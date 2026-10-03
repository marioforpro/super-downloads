use std::{
    collections::{HashMap, HashSet},
    fs,
    io::ErrorKind,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Mutex, OnceLock},
    thread,
};

use tauri::{AppHandle, Emitter, LogicalSize, Manager, Runtime, Size, Window};
use tauri_plugin_updater::UpdaterExt;

mod instagram_fallback;

const WINDOW_LOGICAL_WIDTH: f64 = 480.0;
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

static ACTIVE_DOWNLOADS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();
static CANCELLED_DOWNLOADS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn active_downloads() -> &'static Mutex<HashMap<String, u32>> {
    ACTIVE_DOWNLOADS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cancelled_downloads() -> &'static Mutex<HashSet<String>> {
    CANCELLED_DOWNLOADS.get_or_init(|| Mutex::new(HashSet::new()))
}

fn take_cancelled(download_id: &str) -> bool {
    if let Ok(mut cancelled) = cancelled_downloads().lock() {
        return cancelled.remove(download_id);
    }
    false
}

fn is_auth_required_error(error_text: &str) -> bool {
    let lower = error_text.to_lowercase();
    lower.contains("sign in to confirm your age")
        || lower.contains("age-restricted")
        || lower.contains("age restricted")
        || lower.contains("age gate")
        || lower.contains("age verification")
        || (lower.contains("sign in") && lower.contains("age"))
        || lower.contains("login required")
        || lower.contains("this video is private")
        || lower.contains("private video")
        // YouTube bot-check wall — retrying with a logged-in browser session clears it
        || lower.contains("confirm you're not a bot")
        || lower.contains("confirm you are not a bot")
        || lower.contains("sign in to confirm")
        // Rate limiting — a session with cookies usually gets a higher quota
        || lower.contains("http error 429")
        || lower.contains("too many requests")
        || lower.contains("rate-limit reached")
        || lower.contains("rate limit")
        // Instagram: served to logged-in users only
        || lower.contains("empty media response")
        || lower.contains("requested content is not available")
        // yt-dlp's own hint that cookies would help
        || lower.contains("use --cookies")
}

// Errors that TLS-fingerprint impersonation (curl_cffi) is known to clear —
// Facebook's "Cannot parse data" is the canonical case.
fn is_impersonation_fixable_error(error_text: &str) -> bool {
    let lower = error_text.to_lowercase();
    lower.contains("cannot parse data")
        || lower.contains("unable to extract")
        || (lower.contains("http error 403") && !lower.contains("fragment"))
}

// Vimeo revoked the anonymous OAuth bootstrap used by yt-dlp's `macos` client
// (yt-dlp #17271, 2026-07-20). The fix (#17272) lives on master only — no stable
// release carries it as of 2026-08-17, so the bundled/self-updated engine cannot
// pick it up. The embed-player URL is a different extractor path that still
// works anonymously (verified live 2026-08-17), so on this exact signature we
// rewrite `vimeo.com/<id>` → `player.vimeo.com/video/<id>` and retry once.
//
// 2026-08-19 engine (2026.08.19): yt-dlp changed the anonymous-401 error text
// to "The web client only works when logged-in." — no "401" and no
// "macos api json"/"oauth token" substrings survive, so the old check stopped
// matching and the retry silently stopped firing. Match both the old and new
// text, case-insensitively, on stable fragments; keep the old patterns since
// older bundled/system engines in the field still emit them.
fn is_vimeo_oauth_401_error(error_text: &str) -> bool {
    let lower = error_text.to_lowercase();
    if !lower.contains("vimeo") {
        return false;
    }
    let old_signature = lower.contains("401")
        && (lower.contains("macos api json") || lower.contains("oauth token"));
    let new_signature =
        lower.contains("only works when logged-in") || lower.contains("only works when logged in");
    old_signature || new_signature
}

// `https://vimeo.com/76979871` → `https://player.vimeo.com/video/76979871`
// (`vimeo.com/<id>/<hash>` unlisted links keep their hash as `?h=<hash>`).
// Returns None when the URL is already an embed URL or carries no numeric id.
fn vimeo_embed_url(url: &str) -> Option<String> {
    if url.contains("player.vimeo.com") {
        return None;
    }
    let path = url
        .split(['?', '#'])
        .next()
        .unwrap_or(url)
        .trim_end_matches('/');
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let idx = segments
        .iter()
        .position(|seg| seg.len() >= 6 && seg.chars().all(|c| c.is_ascii_digit()))?;
    let id = segments[idx];
    let hash = segments
        .get(idx + 1)
        .filter(|h| h.len() >= 8 && h.chars().all(|c| c.is_ascii_alphanumeric()));
    Some(match hash {
        Some(h) => format!("https://player.vimeo.com/video/{}?h={}", id, h),
        None => format!("https://player.vimeo.com/video/{}", id),
    })
}

// Whether the resolved yt-dlp supports --impersonate (standalone yt-dlp_macos
// builds bundle curl_cffi; Homebrew/pip builds usually don't). Cached once.
static IMPERSONATION_SUPPORTED: OnceLock<bool> = OnceLock::new();

fn ytdlp_supports_impersonation(ytdlp_path: &str) -> bool {
    *IMPERSONATION_SUPPORTED.get_or_init(|| {
        Command::new(ytdlp_path)
            .arg("--list-impersonate-targets")
            .output()
            .ok()
            .map(|out| {
                let text = format!(
                    "{}{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
                text.contains("curl_cffi") && !text.contains("unavailable")
            })
            .unwrap_or(false)
    })
}

// Map the user's default browser to a name yt-dlp's --cookies-from-browser
// accepts. Falls back to chrome (the most common cookie store). Cached once.
static COOKIE_BROWSER: OnceLock<String> = OnceLock::new();

fn default_cookie_browser() -> &'static str {
    COOKIE_BROWSER.get_or_init(|| {
        // LaunchServices stores the default https handler. `defaults read`
        // prints the handler list as text; find the block that declares the
        // https scheme and take its LSHandlerRoleAll bundle id.
        let handlers = Command::new("/usr/bin/defaults")
            .args([
                "read",
                "com.apple.LaunchServices/com.apple.launchservices.secure",
                "LSHandlers",
            ])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).to_lowercase())
            .unwrap_or_default();

        let mut bundle_id = String::new();
        for block in handlers.split("}") {
            if block.contains("lshandlerurlscheme = https") {
                if let Some(role_line) = block.lines().find(|l| l.contains("lshandlerroleall =")) {
                    bundle_id = role_line
                        .split('=')
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .trim_matches(|c| c == '"' || c == ';')
                        .trim_matches('"')
                        .to_string();
                }
                break;
            }
        }

        let browser = match bundle_id.as_str() {
            id if id.contains("com.google.chrome") => "chrome",
            id if id.contains("com.apple.safari") => "safari",
            id if id.contains("org.mozilla.firefox") => "firefox",
            id if id.contains("com.brave.browser") => "brave",
            id if id.contains("com.microsoft.edgemac") => "edge",
            id if id.contains("com.vivaldi.vivaldi") => "vivaldi",
            id if id.contains("org.chromium.chromium") => "chromium",
            // Arc and anything unknown: yt-dlp cannot read their cookie store —
            // fall back to chrome, the most likely one to exist alongside.
            _ => "chrome",
        };
        eprintln!(
            "Cookie browser resolved: {} (default https handler: {})",
            browser,
            if bundle_id.is_empty() {
                "unknown"
            } else {
                &bundle_id
            }
        );
        browser.to_string()
    })
}

fn height_to_resolution_label(height: u64) -> String {
    match height {
        2160 => "2160p".to_string(),
        1440 => "1440p".to_string(),
        1080 => "1080p".to_string(),
        720 => "720p".to_string(),
        480 => "480p".to_string(),
        360 => "360p".to_string(),
        h if (240..=4320).contains(&h) => format!("{}p", h),
        _ => String::new(),
    }
}

fn is_error_line(line: &str) -> bool {
    let has_error = line.contains("ERROR")
        || line.contains("error")
        || line.contains("Error")
        || line.contains("WARNING")
        || line.contains("Failed")
        || line.contains("failed")
        || line.contains("unavailable")
        || line.contains("Unavailable")
        || line.contains("Private video")
        || line.contains("Video unavailable")
        || line.contains("HTTP Error")
        || line.contains("403")
        || line.contains("404")
        || line.contains("429")
        || line.contains("Unsupported URL")
        || line.contains("Sign in")
        || line.contains("age-restricted")
        || line.contains("impersonation")
        || line.contains("impersonate")
        || line.contains("unauthentic")
        || line.contains("Vimeo");
    let is_ignorable =
        line.contains("Operation not permitted") || line.contains("Cookies.binarycookies");
    has_error && !is_ignorable
}

// Get the path to bundled binaries (for distribution builds)
// Returns the path to the binaries directory inside the app bundle
fn get_bundled_binaries_dir() -> Option<PathBuf> {
    // Get the current executable path
    if let Ok(exe_path) = std::env::current_exe() {
        // On macOS, the structure is:
        // App.app/Contents/MacOS/app-binary
        // And bundled binaries are at:
        // App.app/Contents/MacOS/ (same directory as the binary for externalBin)
        if let Some(exe_dir) = exe_path.parent() {
            return Some(exe_dir.to_path_buf());
        }
    }
    None
}

// Find a bundled binary by name
fn find_bundled_binary(name: &str) -> Option<String> {
    if let Some(bin_dir) = get_bundled_binaries_dir() {
        let bundled_path = bin_dir.join(name);
        if bundled_path.exists() {
            if let Some(path_str) = bundled_path.to_str() {
                eprintln!("Found bundled {} at: {}", name, path_str);
                return Some(path_str.to_string());
            }
        }
    }
    None
}

// Managed yt-dlp self-update support.
// A self-updated copy lives in the app-support dir and is preferred over the
// bundled binary so extractor fixes ship in hours without an app release.
const BUNDLE_IDENTIFIER: &str = "com.supermac.super-downloads";
const YTDLP_UPDATE_INTERVAL_SECS: u64 = 7 * 24 * 60 * 60; // weekly

// app-support bin dir: ~/Library/Application Support/<identifier>/bin
fn managed_bin_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    if home.is_empty() {
        return None;
    }
    Some(
        PathBuf::from(home)
            .join("Library/Application Support")
            .join(BUNDLE_IDENTIFIER)
            .join("bin"),
    )
}

// The engine is yt-dlp's "onedir" macOS build (yt-dlp_macos + _internal/), not
// the single-file yt-dlp_macos. The single file unpacks itself into a fresh temp
// dir on every run and macOS scans those new files each time: ~7s per call.
// The onedir build is scanned once, then starts in ~0.2s.
const ENGINE_EXE: &str = "yt-dlp_macos";
const ENGINE_RESOURCE_DIR: &str = "yt-dlp-engine";
const MANAGED_ENGINE_PREFIX: &str = "engine-";

// Bundled engine: Contents/Resources/yt-dlp-engine/ in the .app; next to the
// executable in `tauri dev` (target/debug/yt-dlp-engine/).
fn find_bundled_ytdlp() -> Option<String> {
    let exe_dir = get_bundled_binaries_dir()?;
    [
        exe_dir.join("../Resources").join(ENGINE_RESOURCE_DIR),
        exe_dir.join(ENGINE_RESOURCE_DIR),
    ]
    .iter()
    .map(|dir| dir.join(ENGINE_EXE))
    .find(|p| p.exists())
    .and_then(|p| p.to_str().map(|s| s.to_string()))
}

// Managed engines live in bin/engine-<tag>/ (tags are dates: YYYY.MM.DD[.N]).
// Returns (tag, dir) sorted oldest → newest; string order is version order.
fn managed_engines() -> Vec<(String, PathBuf)> {
    let Some(bin_dir) = managed_bin_dir() else {
        return Vec::new();
    };
    let mut engines: Vec<(String, PathBuf)> = fs::read_dir(&bin_dir)
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_str()?.to_string();
                    let tag = name.strip_prefix(MANAGED_ENGINE_PREFIX)?;
                    if !is_engine_tag(tag) || !e.path().join(ENGINE_EXE).exists() {
                        return None;
                    }
                    Some((tag.to_string(), e.path()))
                })
                .collect()
        })
        .unwrap_or_default();
    engines.sort();
    engines
}

fn is_engine_tag(tag: &str) -> bool {
    !tag.is_empty() && tag.chars().all(|c| c.is_ascii_digit() || c == '.')
}

// Path to the newest managed yt-dlp if one is installed.
fn find_managed_ytdlp() -> Option<String> {
    let (_, dir) = managed_engines().pop()?;
    dir.join(ENGINE_EXE).to_str().map(|s| s.to_string())
}

// Find yt-dlp executable in common locations
// Prioritizes managed self-update > bundled binary > homebrew > pipx > system
fn find_ytdlp() -> Option<String> {
    // FIRST: managed self-updated copy (freshest extractor, decoupled from app releases)
    if let Some(managed) = find_managed_ytdlp() {
        eprintln!("Using managed (self-updated) yt-dlp at: {}", managed);
        return Some(managed);
    }

    // SECOND: bundled engine (for distributed app)
    if let Some(bundled) = find_bundled_ytdlp() {
        eprintln!("Using bundled yt-dlp at: {}", bundled);
        return Some(bundled);
    }

    // Try to get HOME, but use fallback if not available (build mode)
    let home = std::env::var("HOME").unwrap_or_else(|_| {
        // Fallback: try to get user home from /Users
        if let Ok(mut users) = std::fs::read_dir("/Users") {
            if let Some(Ok(entry)) = users.next() {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    return path_str.to_string();
                }
            }
        }
        String::new()
    });

    // Priority order: homebrew > pipx > system > user pip
    // Homebrew first since that's what was working originally
    let mut locations: Vec<String> = vec![
        // Homebrew locations (original working version)
        "/opt/homebrew/bin/yt-dlp".to_string(), // Homebrew (Apple Silicon)
        "/usr/local/bin/yt-dlp".to_string(),    // Homebrew (Intel) or system
        // System-wide
        "/usr/bin/yt-dlp".to_string(),
    ];

    // Add user-specific paths if HOME is available
    if !home.is_empty() {
        locations.push(format!("{}/.local/bin/yt-dlp", home));
        locations.push(format!("{}/.local/pipx/venvs/yt-dlp/bin/yt-dlp", home));
        // Try common Python paths
        for version in &["3.12", "3.11", "3.10", "3.9"] {
            locations.push(format!("{}/Library/Python/{}/bin/yt-dlp", home, version));
        }
    }

    // Check each location in priority order
    for loc in &locations {
        if Path::new(loc.as_str()).exists() {
            eprintln!("Found yt-dlp at: {}", loc);
            return Some(loc.clone());
        }
    }

    // Try to find via which command as last resort (works in dev, may not in build)
    // Use /usr/bin/which or /bin/which for better compatibility
    for which_cmd in &["/usr/bin/which", "/bin/which", "which"] {
        if let Ok(output) = Command::new(which_cmd).arg("yt-dlp").output() {
            if output.status.success() {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    let path = path.trim();
                    if !path.is_empty() && Path::new(path).exists() {
                        eprintln!("Found yt-dlp via which at: {}", path);
                        return Some(path.to_string());
                    }
                }
            }
        }
    }

    eprintln!("yt-dlp not found in any standard location");
    None
}

// Find ffmpeg executable and return its directory
// Prioritizes bundled binary > system locations
fn find_ffmpeg() -> String {
    // FIRST: Try bundled binary (for distributed app)
    if let Some(bundled) = find_bundled_binary("ffmpeg") {
        // Return the directory containing ffmpeg
        if let Some(parent) = Path::new(&bundled).parent() {
            if let Some(parent_str) = parent.to_str() {
                eprintln!("Using bundled ffmpeg directory: {}", parent_str);
                return parent_str.to_string();
            }
        }
    }

    let home = std::env::var("HOME").unwrap_or_default();
    let locations: Vec<String> = vec![
        "/opt/homebrew/bin/ffmpeg".to_string(),
        "/usr/local/bin/ffmpeg".to_string(),
        "/usr/bin/ffmpeg".to_string(),
        format!("{}/.local/bin/ffmpeg", home),
    ];

    for loc in &locations {
        if Path::new(loc.as_str()).exists() {
            // Return the directory containing ffmpeg
            return Path::new(loc.as_str())
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("/opt/homebrew/bin")
                .to_string();
        }
    }

    // Default to homebrew location
    "/opt/homebrew/bin".to_string()
}

// Get video resolution from file using ffprobe
fn get_video_resolution_from_file(file_path: &str, ffmpeg_dir: &str) -> Option<String> {
    // FIRST: Try bundled ffprobe (for distributed app)
    let mut ffprobe_path: Option<String> = find_bundled_binary("ffprobe");

    // If no bundled binary, try system locations
    if ffprobe_path.is_none() {
        let home = std::env::var("HOME").unwrap_or_default();
        let ffprobe_paths = vec![
            format!("{}/ffprobe", ffmpeg_dir),
            "/opt/homebrew/bin/ffprobe".to_string(),
            "/usr/local/bin/ffprobe".to_string(),
            "/usr/bin/ffprobe".to_string(),
            format!("{}/.local/bin/ffprobe", home),
        ];

        // Find first available ffprobe
        for path in &ffprobe_paths {
            if Path::new(path.as_str()).exists() {
                ffprobe_path = Some(path.clone());
                break;
            }
        }
    }

    if let Some(ref probe_path) = ffprobe_path {
        // Use ffprobe to get actual resolution from file
        if let Ok(output) = Command::new(probe_path.as_str())
            .arg("-v")
            .arg("error")
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=height")
            .arg("-of")
            .arg("json")
            .arg(file_path)
            .output()
        {
            if output.status.success() {
                let json_output = String::from_utf8_lossy(&output.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_output) {
                    if let Some(streams) = json["streams"].as_array() {
                        if let Some(stream) = streams.first() {
                            if let Some(height) = stream["height"].as_u64() {
                                let label = height_to_resolution_label(height);
                                if !label.is_empty() {
                                    return Some(label);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// Vimeo needs up to three attempts: the video page anonymously, then the embed
// player anonymously (works for most public videos, no Keychain prompt), then
// the video page with the browser session (owners who restrict embedding).
// DRM-protected videos stop at the DRM error — never circumvented.
#[derive(Clone)]
enum VimeoStep {
    Direct,
    Embed { origin: String },
    DirectWithCookies,
}

// Generic over the runtime so the live E2E test can drive it with tauri::test.
#[tauri::command]
fn download_video<R: Runtime>(
    window: Window<R>,
    url: String,
    download_id: String,
    download_location: Option<String>,
    quality: Option<String>,
    format: Option<String>,
) -> String {
    start_download(
        window,
        url,
        download_id,
        download_location,
        quality,
        format,
        VimeoStep::Direct,
    )
}

fn start_download<R: Runtime>(
    window: Window<R>,
    url: String,
    download_id: String,
    download_location: Option<String>,
    quality: Option<String>,
    format: Option<String>,
    vimeo_step: VimeoStep,
) -> String {
    let download_id_clone = download_id.clone();
    thread::spawn(move || {
        // Determine download location and store base path for later use
        let base_path = if let Some(ref loc) = download_location {
            // Expand ~ if present
            let expanded = if loc.starts_with("~/") {
                let home = std::env::var("HOME").unwrap();
                loc.replacen("~/", &format!("{}/", home), 1)
            } else {
                loc.clone()
            };
            expanded
        } else {
            let home = std::env::var("HOME").unwrap();
            format!("{}/Downloads", home)
        };

        // Determine selected resolution for display
        let selected_resolution = if format.as_deref() == Some("mp3") {
            String::new()
        } else {
            match quality.as_deref() {
                Some("1080p") => "1080p".to_string(),
                Some("720p") => "720p".to_string(),
                Some("480p") => "480p".to_string(),
                Some("360p") => "360p".to_string(),
                Some("best") => String::new(), // "Best Available" - will detect actual quality
                _ => String::new(),            // Will be detected from metadata or download
            }
        };

        // Track if video has 4K+ available (determined during metadata extraction)
        let mut has_4k_available = false;

        // Check if this is "Best Available" mode

        // Determine output format
        let output_format = format.as_deref().unwrap_or("mp4");
        let is_audio_only = output_format.eq_ignore_ascii_case("mp3");
        let final_format_ext = output_format.to_uppercase(); // Store for later use in metadata

        // Find yt-dlp executable
        let ytdlp_path = match find_ytdlp() {
            Some(path) => {
                eprintln!("Using yt-dlp at: {}", path);
                // Verify the path is absolute and executable
                let abs_path = match fs::canonicalize(&path) {
                    Ok(p) => p.to_string_lossy().to_string(),
                    Err(_) => path.clone(),
                };
                eprintln!("Canonical yt-dlp path: {}", abs_path);
                abs_path
            }
            None => {
                let error_msg = "The download engine is missing. Try 'Update downloader engine' in Settings, or reinstall the app from superdownloads.app.".to_string();
                eprintln!("ERROR: {}", error_msg);
                let _ = window.emit("download-error", (download_id.clone(), error_msg));
                return;
            }
        };

        // Find ffmpeg location
        let ffmpeg_dir = find_ffmpeg();

        // Get full metadata including thumbnail, duration, etc.
        let mut video_title = String::new();
        let mut thumbnail_url = String::new();
        let mut duration_seconds: Option<f64> = None;
        let mut size_bytes: Option<u64> = None;
        let mut format_ext = String::new();
        let mut fps: Option<f64> = None;
        let mut metadata_extraction_failed = false;
        let mut title_error = String::new();
        let mut initial_resolution = String::new(); // Resolution from metadata

        // Try metadata extraction for all videos, including Vimeo
        // For Vimeo, try without cookies first (many videos are public)
        let is_vimeo = url.contains("vimeo.com");
        let is_instagram = url.contains("instagram.com") || url.contains("instagr.am");

        // Ensure PATH includes common locations in build mode
        let current_path = std::env::var("PATH").unwrap_or_default();
        let enhanced_path = if !current_path.contains("/opt/homebrew/bin") {
            format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin", current_path)
        } else {
            current_path
        };

        // Use --dump-json to get full metadata (including thumbnails for Vimeo)
        let build_metadata_cmd = |with_cookies: bool| {
            let mut metadata_cmd = Command::new(&ytdlp_path);
            metadata_cmd.arg("--no-playlist");
            metadata_cmd.env("PATH", &enhanced_path);
            metadata_cmd.arg("--dump-json").arg("--skip-download");

            // Add user-agent for all platforms to help with access
            metadata_cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);

            if with_cookies {
                metadata_cmd
                    .arg("--cookies-from-browser")
                    .arg(default_cookie_browser());
            }

            // Facebook metadata needs browser impersonation too (TLS fingerprinting)
            if (url.contains("facebook.com") || url.contains("fb.watch") || url.contains("fb.com"))
                && ytdlp_supports_impersonation(&ytdlp_path)
            {
                metadata_cmd.arg("--impersonate").arg("chrome");
            }

            metadata_cmd.arg(&url);
            metadata_cmd
        };

        // LinkedIn: try logged-out first (public posts work, the logged-in path
        // fails on yt-dlp 2026.08.19); fall back to browser cookies only if that fails.
        let is_linkedin_metadata = url.contains("linkedin.com") || url.contains("lnkd.in");
        // Vimeo's session step already uses the browser session for the download;
        // use it for metadata too so the file gets its real title.
        let metadata_with_cookies = matches!(vimeo_step, VimeoStep::DirectWithCookies);
        let metadata_result = match build_metadata_cmd(metadata_with_cookies).output() {
            Ok(output) if !output.status.success() && is_linkedin_metadata => {
                eprintln!("LinkedIn metadata failed logged-out, retrying with browser cookies");
                build_metadata_cmd(true).output().or(Ok(output))
            }
            other => other,
        };

        match metadata_result {
            Ok(output) => {
                if output.status.success() {
                    let json_output = String::from_utf8_lossy(&output.stdout);
                    // Parse JSON to extract metadata
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_output) {
                        video_title = json["title"].as_str().unwrap_or("").to_string();
                        thumbnail_url = json["thumbnail"].as_str().unwrap_or("").to_string();
                        duration_seconds = json["duration"].as_f64();
                        size_bytes = json["filesize"]
                            .as_u64()
                            .or_else(|| json["filesize_approx"].as_u64());
                        fps = json["fps"].as_f64();

                        // Check if 4K+ is available by looking at all formats
                        if let Some(formats) = json["formats"].as_array() {
                            for fmt in formats {
                                if let Some(height) = fmt["height"].as_u64() {
                                    if height >= 1440 {
                                        has_4k_available = true;
                                        eprintln!(
                                            "Detected 4K+ format available (height: {})",
                                            height
                                        );
                                    }
                                }
                            }
                        }
                        if !has_4k_available {
                            eprintln!("No 4K+ formats detected, max resolution is 1080p or lower");
                        }
                        // Get format extension (MP4, WEBM, etc.)
                        // Use the output format we'll use (from settings), not the video's native format
                        format_ext = final_format_ext.clone();
                        // But if output format not set, fall back to video's native format
                        if format_ext.is_empty() {
                            if let Some(ext) = json["ext"].as_str() {
                                format_ext = ext.to_uppercase();
                            } else if let Some(container) = json["container"].as_str() {
                                format_ext = container.to_uppercase();
                            }
                        }
                        // Try to extract resolution from metadata if available
                        // But prioritize user-selected quality if available
                        let is_youtube_metadata =
                            url.contains("youtube.com") || url.contains("youtu.be");
                        if !selected_resolution.is_empty() {
                            // Use the user's selected quality (e.g., from settings) - this is what was actually downloaded
                            initial_resolution = selected_resolution.clone();
                            eprintln!(
                                "Using selected resolution (what was downloaded): {}",
                                initial_resolution
                            );
                        } else if !is_youtube_metadata {
                            // For non-YouTube sites, use metadata resolution
                            // For YouTube, skip metadata resolution - it's often wrong (shows 360p when actual is 4K)
                            if let Some(height) = json["height"].as_u64() {
                                let label = height_to_resolution_label(height);
                                if !label.is_empty() {
                                    eprintln!("Detected resolution from metadata: {}", label);
                                    initial_resolution = label;
                                }
                            }
                        } else {
                            // YouTube: Skip metadata resolution - will use actual file resolution after download
                            eprintln!("YouTube: Skipping metadata resolution (will use actual file resolution after download)");
                        }
                    } else {
                        // Fallback: try --get-title if JSON parsing fails
                        let mut title_cmd = Command::new(&ytdlp_path);
                        title_cmd.arg("--no-playlist");
                        title_cmd.arg("--get-title").arg("--skip-download");
                        title_cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);
                        title_cmd.arg(&url);
                        if let Ok(title_output) = title_cmd.output() {
                            if title_output.status.success() {
                                video_title = String::from_utf8_lossy(&title_output.stdout)
                                    .trim()
                                    .to_string();
                            }
                        }
                    }
                } else {
                    // If metadata extraction failed for Vimeo, just continue without retry
                    // We don't use cookies by default to avoid keychain prompts
                    // The download will still work for most public videos
                    if is_vimeo && video_title.is_empty() && thumbnail_url.is_empty() {
                        eprintln!("Vimeo metadata extraction failed, continuing without metadata (to avoid keychain prompts)");
                        // Note: For private Vimeo videos that require login, users would need to
                        // manually run yt-dlp with --cookies-from-browser chrome
                    }

                    // If still no metadata, capture the error
                    if video_title.is_empty() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        if !stderr.trim().is_empty() {
                            title_error = stderr.trim().to_string();
                        } else if !stdout.trim().is_empty() {
                            title_error = stdout.trim().to_string();
                        }
                        metadata_extraction_failed = true;
                    }
                }
            }
            Err(e) => {
                // For Vimeo, command spawn failure is non-fatal - continue without metadata
                // For other sites, this might be a critical error
                if !is_vimeo {
                    let error_msg = format!("Failed to run the download engine ({}): {}\n\nTry 'Update downloader engine' in Settings, or reinstall the app.", ytdlp_path, e);
                    eprintln!("Metadata extraction error: {}", error_msg);
                    let _ = window.emit("download-error", (download_id.clone(), error_msg.clone()));
                    return;
                } else {
                    // For Vimeo, just log and continue
                    eprintln!("Vimeo metadata extraction failed (non-fatal): {}", e);
                    metadata_extraction_failed = true;
                }
            }
        }

        // If metadata extraction failed, log it but don't prevent download
        // Video-specific errors (403, impersonation, etc.) should be handled during actual download
        // Only return early for critical system errors (yt-dlp not found, etc.)
        if metadata_extraction_failed && !title_error.is_empty() {
            eprintln!(
                "Metadata extraction failed (will still attempt download): {}",
                title_error
            );
            // Don't return - let the download attempt proceed
            // The actual download will handle and report video-specific errors
        }

        // For Vimeo, use URL as title since we skipped metadata extraction
        if is_vimeo && video_title.is_empty() {
            video_title = url.clone();
        }

        // If title extraction failed, use URL as fallback
        if video_title.is_empty() {
            video_title = url.clone();
        }

        let output_extension = if is_audio_only {
            "mp3".to_string()
        } else {
            output_format.to_string()
        };
        let planned_output_path =
            build_unique_output_path(&base_path, &video_title, &output_extension);

        // Store metadata for later use
        let metadata_duration = duration_seconds;
        let metadata_size = size_bytes.map(format_file_size).unwrap_or_default();
        let metadata_fps = fps;
        // Use the format we determined (output format or video's native format)
        let metadata_format = if !format_ext.is_empty() {
            format_ext.clone()
        } else {
            final_format_ext.clone()
        };
        let mut metadata_thumbnail = thumbnail_url.clone();
        if is_instagram {
            if let Some(local_thumbnail) =
                cache_thumbnail_for_download(window.app_handle(), &ytdlp_path, &url, &download_id)
            {
                metadata_thumbnail = local_thumbnail;
            }
        }

        // Emit download-started event with thumbnail
        let _ = window.emit(
            "download-started",
            (
                download_id.clone(),
                video_title.clone(),
                metadata_thumbnail.clone(),
            ),
        );

        // Emit metadata if we have it (including resolution if available from metadata)
        // For YouTube, don't show metadata resolution during download - it's often wrong (shows 360p)
        // We'll wait until we get the actual file resolution after download completes
        let is_youtube_for_metadata = url.contains("youtube.com") || url.contains("youtu.be");
        if metadata_duration.is_some()
            || !metadata_format.is_empty()
            || metadata_fps.is_some()
            || (!initial_resolution.is_empty() && !is_youtube_for_metadata)
        {
            let duration_str = metadata_duration.map(format_duration).unwrap_or_default();
            let fps_str = metadata_fps
                .map(|f| format!("{}fps", f.round() as u32))
                .unwrap_or_default();
            // Emit resolution in metadata event so it shows up early
            let _ = window.emit(
                "download-metadata",
                (
                    download_id.clone(),
                    duration_str,
                    metadata_size.clone(),
                    metadata_format.clone(),
                    fps_str,
                    metadata_thumbnail.clone(),
                ),
            );
            // Also emit resolution separately if we have it (but skip for YouTube - metadata is unreliable)
            if !initial_resolution.is_empty() && !is_youtube_for_metadata {
                // Update download with resolution via progress event
                let _ = window.emit(
                    "download-progress",
                    (
                        download_id.clone(),
                        0,
                        initial_resolution.clone(),
                        String::new(),
                        "queued",
                    ),
                );
            }
        }

        // Now run the actual download
        let mut cmd = Command::new(&ytdlp_path);

        let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
        let is_linkedin = url.contains("linkedin.com") || url.contains("lnkd.in");

        // Highest resolution within the quality cap first; at equal resolution
        // prefer H.264/AAC so most downloads need no conversion. Anything that
        // still isn't H.264 is converted after the download (ensure_h264), so
        // the user never gets a lower resolution just to avoid a conversion.
        let format_selector = if is_audio_only {
            "bestaudio/best"
        } else {
            "bv*+ba/b"
        };
        let format_sort = match quality.as_deref() {
            Some("1080p") => "res:1080,vcodec:h264,acodec:m4a",
            Some("720p") => "res:720,vcodec:h264,acodec:m4a",
            _ => "res,vcodec:h264,acodec:m4a",
        };

        eprintln!(
            "Quality setting: {:?}, has_4k: {}, Format: {} sorted by {}",
            quality, has_4k_available, format_selector, format_sort
        );

        cmd.arg("-f").arg(format_selector);
        if !is_audio_only {
            cmd.arg("-S").arg(format_sort);
        }
        // A watch?v=…&list=… link must download the one video, not the playlist.
        cmd.arg("--no-playlist");

        // Network resilience options for unstable connections
        cmd.arg("--continue")
            .arg("--retries")
            .arg("15")
            .arg("--fragment-retries")
            .arg("15")
            .arg("--file-access-retries")
            .arg("8")
            .arg("--extractor-retries")
            .arg("3")
            .arg("--retry-sleep")
            .arg("2")
            .arg("--concurrent-fragments")
            .arg("4")
            .arg("--buffer-size")
            .arg("16K")
            .arg("--socket-timeout")
            .arg("30");

        cmd.arg("--ffmpeg-location")
            .arg(&ffmpeg_dir)
            .arg("--newline");
        if is_audio_only {
            cmd.arg("-x")
                .arg("--audio-format")
                .arg("mp3")
                .arg("--audio-quality")
                .arg("0");
        } else {
            cmd.arg("--merge-output-format").arg(output_format);
        }

        eprintln!(
            "Using format selector: {} for quality: {:?}",
            format_selector, quality
        );

        // User agent for all platforms (no cookies by default - auto-retry handles auth)
        cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);
        cmd.arg("--no-warnings");
        if matches!(vimeo_step, VimeoStep::DirectWithCookies) {
            cmd.arg("--cookies-from-browser")
                .arg(default_cookie_browser());
        }

        // Facebook blocks non-browser TLS fingerprints ("Cannot parse data") —
        // impersonate a real browser when the engine supports it (curl_cffi).
        let is_facebook =
            url.contains("facebook.com") || url.contains("fb.watch") || url.contains("fb.com");
        if is_facebook && ytdlp_supports_impersonation(&ytdlp_path) {
            cmd.arg("--impersonate").arg("chrome");
        }

        // LinkedIn: no cookies on the first attempt. Public posts extract fine
        // logged-out, and on yt-dlp 2026.08.19 the logged-in path fails with
        // "Unable to extract video" — forcing cookies broke every user signed in
        // to LinkedIn. Login-only posts get cookies via the retry below.

        cmd.arg("-o")
            .arg(&planned_output_path)
            .arg(&url)
            .stderr(Stdio::piped());

        // Ensure PATH includes common locations in build mode
        // This helps when the app is sandboxed and PATH might be restricted
        let current_path = std::env::var("PATH").unwrap_or_default();
        let enhanced_path = if !current_path.contains("/opt/homebrew/bin") {
            format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin", current_path)
        } else {
            current_path
        };
        cmd.env("PATH", &enhanced_path);

        // Log key info for debugging (especially in build mode)
        eprintln!("Executing yt-dlp from: {}", ytdlp_path);
        eprintln!("URL: {}", url);
        eprintln!("Format selector: {}", format_selector);
        eprintln!("Output path: {}", planned_output_path);

        // Cancelled while the metadata pass was running: never start the download.
        if take_cancelled(&download_id) {
            eprintln!("Download {} cancelled before it started", download_id);
            return;
        }

        let mut child = match own_process_group(cmd.stdout(Stdio::piped())).spawn() {
            Ok(child) => {
                eprintln!("yt-dlp process started successfully");
                child
            }
            Err(e) => {
                let error_msg = format!("Failed to start the download engine ({}): {}\n\nTry 'Update downloader engine' in Settings, or reinstall the app.", ytdlp_path, e);
                eprintln!("ERROR starting yt-dlp: {}", error_msg);
                let _ = window.emit("download-error", (download_id.clone(), error_msg));
                return;
            }
        };
        if let Ok(mut map) = active_downloads().lock() {
            map.insert(download_id.clone(), child.id());
        }

        let stdout = child.stdout.take().expect("No stdout");
        let stderr = child.stderr.take().expect("No stderr");
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // Initialize resolution from metadata if we got it, otherwise detect during download
        let mut current_resolution = initial_resolution.clone();
        let mut final_file_path = String::new();
        let mut is_merging = false;
        let mut stdout_errors = Vec::new();
        let mut all_stdout = String::new();

        // For Vimeo, if we don't have metadata yet, try to extract it from download output
        let extract_metadata_from_output =
            is_vimeo && (video_title.is_empty() || video_title == url || thumbnail_url.is_empty());

        let mut progress = ProgressTracker::default();

        // Read stdout for progress and errors
        for line in lossy_lines(stdout_reader) {
            all_stdout.push_str(&line);
            all_stdout.push('\n');
            progress.observe_line(&line);

            if is_error_line(&line) {
                stdout_errors.push(line.clone());
            }

            // Detect merging status. The merge (and the H.264 re-encode, which
            // runs inside it) prints no progress, so tell the UI the phase changed.
            if line.contains("[Merger]") || line.contains("Merging") {
                if !is_merging {
                    let resolution = if !current_resolution.is_empty() {
                        current_resolution.clone()
                    } else {
                        selected_resolution.clone()
                    };
                    let status = "merging";
                    let _ = window.emit(
                        "download-progress",
                        (
                            download_id.clone(),
                            100u8,
                            resolution,
                            String::new(),
                            status,
                        ),
                    );
                }
                is_merging = true;
                if let Some(path) = extract_file_path(&line) {
                    final_file_path = path;
                }
            }

            // Extract file path when download completes
            if line.contains("[download]")
                && (line.contains("has already been downloaded") || line.contains("Destination:"))
            {
                if let Some(path) = extract_file_path(&line) {
                    final_file_path = path;
                }
            }

            if line.contains("[Merger]") && line.contains("Merging formats into") {
                if let Some(path) = extract_file_path(&line) {
                    final_file_path = path;
                }
            }

            // Audio-only mode final output is often reported by ExtractAudio postprocessor.
            if line.contains("[ExtractAudio]") && line.contains("Destination:") {
                if let Some(path) = extract_file_path(&line) {
                    final_file_path = path;
                }
            }

            // Extract final file path from download completion
            if line.contains("[download]") && line.contains("100%") {
                // Try to extract path from the line
                if let Some(path) = extract_file_path(&line) {
                    final_file_path = path;
                } else if line.contains(&base_path) {
                    // Try to extract filename from the line
                    if let Some(filename_start) = line.find(&base_path) {
                        let rest = &line[filename_start..];
                        if let Some(end) = rest.find(' ') {
                            final_file_path = rest[..end].to_string();
                        } else {
                            final_file_path = rest.trim().to_string();
                        }
                    }
                }
            }

            // Detect resolution - look for patterns like "1080p", "720p", "2160p", etc.
            // Check multiple patterns to catch resolution in different formats
            // Also look for "4K" which is 2160p
            // For Vimeo, be more aggressive in detection
            if line.contains("p")
                || line.contains("4K")
                || line.contains("4k")
                || (is_vimeo && (line.contains("x") || line.contains("format")))
            {
                // Pattern 0: "4K" or "4k" (convert to 2160p)
                if current_resolution.is_empty() && (line.contains("4K") || line.contains("4k")) {
                    current_resolution = "2160p".to_string();
                    eprintln!("Detected resolution from download output: 2160p (4K)");
                }

                // Pattern 1: "1080p", "720p", etc. (standalone) - check this first for Vimeo
                if current_resolution.is_empty() {
                    for part in line.split_whitespace() {
                        if part.ends_with('p') && part.len() <= 5 && part.len() >= 3 {
                            // Check if it's a valid resolution (e.g., "360p", "480p", "720p", "1080p", "2160p")
                            if part[..part.len() - 1].parse::<u32>().is_ok() {
                                current_resolution = part.to_string();
                                eprintln!(
                                    "Detected resolution from download output: {}",
                                    current_resolution
                                );
                                break;
                            }
                        }
                    }
                }

                // Pattern 2: "1920x1080" or "3840x2160" format - convert to standard resolution
                // This is especially important for Vimeo
                if current_resolution.is_empty() {
                    for part in line.split_whitespace() {
                        if part.contains('x') {
                            let parts: Vec<&str> = part.split('x').collect();
                            if parts.len() == 2 {
                                if let (Ok(_width), Ok(height)) =
                                    (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                                {
                                    // Convert to standard resolution format based on height
                                    match height {
                                        2160 => {
                                            current_resolution = "2160p".to_string();
                                            eprintln!("Detected resolution from dimensions: 2160p ({}x{})", parts[0], parts[1]);
                                            break;
                                        }
                                        1440 => {
                                            current_resolution = "1440p".to_string();
                                            eprintln!("Detected resolution from dimensions: 1440p");
                                            break;
                                        }
                                        1080 => {
                                            current_resolution = "1080p".to_string();
                                            eprintln!("Detected resolution from dimensions: 1080p");
                                            break;
                                        }
                                        720 => {
                                            current_resolution = "720p".to_string();
                                            eprintln!("Detected resolution from dimensions: 720p");
                                            break;
                                        }
                                        480 => {
                                            current_resolution = "480p".to_string();
                                            eprintln!("Detected resolution from dimensions: 480p");
                                            break;
                                        }
                                        360 => {
                                            current_resolution = "360p".to_string();
                                            eprintln!("Detected resolution from dimensions: 360p");
                                            break;
                                        }
                                        _ => {
                                            // Use height as resolution if it's a common video height
                                            if (240..=4320).contains(&height) {
                                                current_resolution = format!("{}p", height);
                                                eprintln!(
                                                    "Detected resolution from dimensions: {}p",
                                                    height
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Pattern 3: Look for resolution in format strings like "[download] 1080p video"
                // Or Vimeo format descriptions
                if current_resolution.is_empty()
                    && (line.contains("video") || line.contains("format") || is_vimeo)
                {
                    for part in line.split_whitespace() {
                        if part.ends_with('p')
                            && part.len() <= 5
                            && part.len() >= 3
                            && part[..part.len() - 1].parse::<u32>().is_ok()
                        {
                            current_resolution = part.to_string();
                            eprintln!(
                                "Detected resolution from format string: {}",
                                current_resolution
                            );
                            break;
                        }
                    }
                }
            }

            // For Vimeo, try to extract metadata from download output if we don't have it
            if extract_metadata_from_output {
                // Look for title in output (yt-dlp sometimes prints it)
                if (video_title.is_empty() || video_title == url)
                    && line.contains("[download]")
                    && line.contains("Destination:")
                {
                    // Try to extract title from filename
                    if let Some(dest_start) = line.find("Destination:") {
                        let dest_part = &line[dest_start + 12..].trim();
                        if let Some(last_slash) = dest_part.rfind('/') {
                            let filename = &dest_part[last_slash + 1..];
                            if let Some(dot_pos) = filename.rfind('.') {
                                let potential_title = &filename[..dot_pos];
                                if !potential_title.is_empty() && potential_title != url {
                                    video_title = potential_title
                                        .replace("%20", " ")
                                        .replace("%27", "'")
                                        .replace("%28", "(")
                                        .replace("%29", ")");
                                    // Emit updated title
                                    let _ = window.emit(
                                        "download-started",
                                        (
                                            download_id.clone(),
                                            video_title.clone(),
                                            metadata_thumbnail.clone(),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }

                // For Vimeo, also try to detect resolution from format strings in output
                // Vimeo sometimes shows resolution in format like "1080p" or "1920x1080"
                // This is already handled by the main resolution detection above, but we can add more patterns
                if is_vimeo
                    && current_resolution.is_empty()
                    && initial_resolution.is_empty()
                    && selected_resolution.is_empty()
                {
                    // Look for Vimeo-specific resolution patterns
                    // Check for patterns like "1080p" in Vimeo format descriptions
                    if line.contains("format") && line.contains("p") {
                        for part in line.split_whitespace() {
                            if part.ends_with('p')
                                && part.len() <= 5
                                && part.len() >= 3
                                && part[..part.len() - 1].parse::<u32>().is_ok()
                            {
                                current_resolution = part.to_string();
                                eprintln!(
                                    "Detected Vimeo resolution from format string: {}",
                                    current_resolution
                                );
                                break;
                            }
                        }
                    }
                }
            }

            // Progress - emit resolution if we detected it
            if let Some((raw_pct, speed)) = parse_progress(&line) {
                let pct = progress.overall(raw_pct);
                let status = if is_merging { "merging" } else { "downloading" };

                // Priority: detected resolution from output > selected quality (what user chose) > metadata resolution
                // For YouTube, skip metadata resolution during download - it's often wrong (shows 360p)
                let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
                let resolution_to_send = if !current_resolution.is_empty() {
                    current_resolution.clone()
                } else if !selected_resolution.is_empty() {
                    selected_resolution.clone()
                } else if !is_youtube {
                    // Only use metadata resolution for non-YouTube sites during download
                    initial_resolution.clone()
                } else {
                    // For YouTube, leave empty during download - will be set after completion with actual file resolution
                    String::new()
                };
                let _ = window.emit(
                    "download-progress",
                    (download_id.clone(), pct, resolution_to_send, speed, status),
                );
            }
        }

        // Read stderr for errors - capture ALL stderr output
        let mut error_messages = Vec::new();
        let mut all_stderr = String::new();
        for line in stderr_reader.lines().map_while(Result::ok) {
            all_stderr.push_str(&line);
            all_stderr.push('\n');
            if is_error_line(&line) {
                error_messages.push(line.clone());
            }
        }

        // Combine stdout and stderr errors
        error_messages.extend(stdout_errors);

        let exit_status = child.wait();
        if let Ok(mut map) = active_downloads().lock() {
            map.remove(&download_id);
        }

        match exit_status {
            Ok(status) if status.success() => {
                // Try to find the actual downloaded file if path not found
                if (final_file_path.is_empty() || !Path::new(&final_file_path).exists())
                    && Path::new(&planned_output_path).exists()
                {
                    final_file_path = planned_output_path.clone();
                }

                if final_file_path.is_empty() || !Path::new(&final_file_path).exists() {
                    let expected_extension = if is_audio_only { "mp3" } else { output_format };
                    // Try to find the most recently created file in the download directory
                    if let Ok(entries) = fs::read_dir(&base_path) {
                        let mut latest_file: Option<String> = None;
                        let mut latest_time: Option<std::time::SystemTime> = None;

                        for entry in entries.flatten() {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.is_file() {
                                    let path = entry.path();
                                    let ext_matches = path
                                        .extension()
                                        .and_then(|ext| ext.to_str())
                                        .map(|ext| ext.eq_ignore_ascii_case(expected_extension))
                                        .unwrap_or(false);
                                    if !ext_matches {
                                        continue;
                                    }
                                    if let Ok(modified) = metadata.modified() {
                                        if latest_time.is_none() || modified > latest_time.unwrap()
                                        {
                                            if let Some(path_str) = path.to_str() {
                                                latest_file = Some(path_str.to_string());
                                                latest_time = Some(modified);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(path) = latest_file {
                            final_file_path = path;
                        } else {
                            // Fallback: construct path
                            final_file_path =
                                format!("{}/{}.{}", base_path, video_title, expected_extension);
                        }
                    } else {
                        final_file_path =
                            format!("{}/{}.{}", base_path, video_title, expected_extension);
                    }
                }

                // Canonicalize the file path to ensure it's absolute and properly formatted
                let canonical_path = if Path::new(&final_file_path).exists() {
                    if let Ok(canonical) = fs::canonicalize(&final_file_path) {
                        if let Some(path_str) = canonical.to_str() {
                            path_str.to_string()
                        } else {
                            final_file_path.clone()
                        }
                    } else {
                        final_file_path.clone()
                    }
                } else {
                    final_file_path.clone()
                };

                // Cancelled right as yt-dlp finished: drop the result quietly.
                if take_cancelled(&download_id) {
                    return;
                }
                let Some(canonical_path) = ensure_h264(
                    &window,
                    &download_id,
                    &canonical_path,
                    &ffmpeg_dir,
                    &current_resolution,
                ) else {
                    return; // cancelled during conversion
                };

                // Get file size (use canonical_path for consistency)
                let file_size_str = if Path::new(&canonical_path).exists() {
                    if let Ok(metadata) = fs::metadata(&canonical_path) {
                        format_file_size(metadata.len())
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                // Format duration and fps (use stored metadata)
                let duration_str = metadata_duration.map(format_duration).unwrap_or_default();
                let fps_str = metadata_fps
                    .map(|f| format!("{}fps", f.round() as u32))
                    .unwrap_or_default();

                // Get ACTUAL resolution from the downloaded file using ffprobe
                // This is the most accurate - it reads the actual file, not what we selected
                let actual_file_resolution = if Path::new(&canonical_path).exists() {
                    if let Some(res) = get_video_resolution_from_file(&canonical_path, &ffmpeg_dir)
                    {
                        eprintln!("Detected ACTUAL resolution from file: {}", res);
                        Some(res)
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Priority: actual file resolution > detected from output > selected quality > metadata resolution
                let final_resolution = if let Some(ref actual) = actual_file_resolution {
                    actual.clone()
                } else if !current_resolution.is_empty() {
                    current_resolution.clone()
                } else if !selected_resolution.is_empty() {
                    selected_resolution.clone()
                } else {
                    initial_resolution.clone()
                };

                eprintln!("Final resolution for download-finished: {} (file: {}, detected: {}, selected: {}, metadata: {})", 
                    final_resolution,
                    if let Some(ref actual) = actual_file_resolution { actual.clone() } else { "none".to_string() },
                    if !current_resolution.is_empty() { current_resolution.clone() } else { "none".to_string() },
                    if !selected_resolution.is_empty() { selected_resolution.clone() } else { "none".to_string() },
                    if !initial_resolution.is_empty() { initial_resolution.clone() } else { "none".to_string() }
                );

                eprintln!("Sending canonical path to frontend: {}", canonical_path);

                let _ = window.emit(
                    "download-finished",
                    (
                        download_id.clone(),
                        video_title.clone(),
                        canonical_path.clone(), // Use canonicalized path
                        duration_str,
                        file_size_str,
                        metadata_format.clone(),
                        final_resolution,
                        fps_str,
                        metadata_thumbnail.clone(),
                    ),
                );
            }
            Ok(status) => {
                if take_cancelled(&download_id) {
                    eprintln!("Download {} cancelled by user", download_id);
                    cleanup_partial_files(&planned_output_path);
                    return;
                }

                // Check if this is an auth-required error that can be retried with cookies
                let combined_error_text =
                    format!("{} {} {}", error_messages.join(" "), all_stderr, all_stdout);

                // Vimeo: the anonymous page hits a login wall (yt-dlp #17271) →
                // embed player; an embed blocked by its owner (HTTP 401) → the page
                // again with the browser session. See VimeoStep.
                if is_vimeo && !combined_error_text.contains("DRM protected") {
                    let next = match &vimeo_step {
                        VimeoStep::Direct if is_vimeo_oauth_401_error(&combined_error_text) => {
                            vimeo_embed_url(&url).map(|embed| {
                                (
                                    embed,
                                    VimeoStep::Embed {
                                        origin: url.clone(),
                                    },
                                )
                            })
                        }
                        VimeoStep::Embed { origin } => {
                            Some((origin.clone(), VimeoStep::DirectWithCookies))
                        }
                        _ => None,
                    };
                    if let Some((next_url, next_step)) = next {
                        eprintln!("Vimeo: retrying via {}", next_url);
                        cleanup_partial_files(&planned_output_path);
                        release_output_path(&planned_output_path);
                        start_download(
                            window.clone(),
                            next_url,
                            download_id.clone(),
                            download_location.clone(),
                            quality.clone(),
                            format.clone(),
                            next_step,
                        );
                        return;
                    }
                }

                let cookie_retry_platform = is_youtube
                    || is_vimeo
                    || is_instagram
                    || url.contains("facebook.com")
                    || url.contains("fb.watch")
                    || url.contains("tiktok.com")
                    || url.contains("twitter.com")
                    || url.contains("x.com");
                // LinkedIn's login wall has no stable error text, so any failed
                // cookie-less attempt earns one retry with the browser session.
                let should_retry_with_cookies = (is_auth_required_error(&combined_error_text)
                    && cookie_retry_platform)
                    || is_linkedin;
                // Fingerprint-blocked extraction (e.g. Facebook "Cannot parse
                // data") is cleared by impersonation, not cookies.
                let should_retry_with_impersonation = !should_retry_with_cookies
                    && is_impersonation_fixable_error(&combined_error_text)
                    && ytdlp_supports_impersonation(&ytdlp_path);

                if should_retry_with_cookies || should_retry_with_impersonation {
                    if should_retry_with_cookies {
                        eprintln!(
                            "Auth-required content detected, retrying with browser cookies..."
                        );
                    } else {
                        eprintln!("Fingerprint-blocked extraction detected, retrying with browser impersonation...");
                    }
                    let _ = window.emit(
                        "download-progress",
                        (
                            download_id.clone(),
                            2,
                            current_resolution.clone(),
                            String::new(),
                            "downloading",
                        ),
                    );

                    // Build retry command with cookies
                    let mut retry_cmd = Command::new(&ytdlp_path);
                    retry_cmd.arg("--no-playlist");
                    if !is_audio_only {
                        retry_cmd.arg("-S").arg(format_sort);
                    }
                    retry_cmd
                        .arg("-f")
                        .arg(format_selector)
                        .arg("--continue")
                        .arg("--retries")
                        .arg("10")
                        .arg("--fragment-retries")
                        .arg("10")
                        .arg("--concurrent-fragments")
                        .arg("4")
                        .arg("--socket-timeout")
                        .arg("30")
                        .arg("--ffmpeg-location")
                        .arg(&ffmpeg_dir)
                        .arg("--newline");

                    if is_audio_only {
                        retry_cmd
                            .arg("-x")
                            .arg("--audio-format")
                            .arg("mp3")
                            .arg("--audio-quality")
                            .arg("0");
                    } else {
                        retry_cmd.arg("--merge-output-format").arg(output_format);
                    }

                    retry_cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);
                    if should_retry_with_cookies {
                        retry_cmd
                            .arg("--cookies-from-browser")
                            .arg(default_cookie_browser());
                    }
                    // Impersonation helps both retry flavors when available.
                    if ytdlp_supports_impersonation(&ytdlp_path) {
                        retry_cmd.arg("--impersonate").arg("chrome");
                    }
                    retry_cmd
                        .arg("--no-warnings")
                        .arg("-o")
                        .arg(&planned_output_path)
                        .arg(&url);

                    let current_path = std::env::var("PATH").unwrap_or_default();
                    let enhanced_path = if !current_path.contains("/opt/homebrew/bin") {
                        format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin", current_path)
                    } else {
                        current_path
                    };
                    retry_cmd.env("PATH", &enhanced_path);

                    match run_streaming_attempt(
                        &window,
                        &download_id,
                        &mut retry_cmd,
                        &current_resolution,
                    ) {
                        Some((true, retry_stdout, _)) => {
                            eprintln!("Cookie retry succeeded for {}", download_id);
                            // Find the downloaded file
                            let mut retry_file_path = planned_output_path.clone();
                            for line in retry_stdout.lines() {
                                if let Some(path) = extract_file_path(line) {
                                    if Path::new(&path).exists() {
                                        retry_file_path = path;
                                    }
                                }
                            }
                            // Canonicalize path
                            let canonical = fs::canonicalize(&retry_file_path)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or(retry_file_path);
                            let Some(canonical) = ensure_h264(
                                &window,
                                &download_id,
                                &canonical,
                                &ffmpeg_dir,
                                &current_resolution,
                            ) else {
                                return; // cancelled during conversion
                            };

                            let duration_str = duration_seconds
                                .map(|d| {
                                    let mins = (d / 60.0).floor() as u64;
                                    let secs = (d % 60.0).round() as u64;
                                    format!("{:02}:{:02}", mins, secs)
                                })
                                .unwrap_or_default();
                            let file_size_str = fs::metadata(&canonical)
                                .map(|m| format_file_size(m.len()))
                                .unwrap_or_default();
                            let fps_str = fps
                                .map(|f| format!("{}fps", f.round() as u64))
                                .unwrap_or_default();
                            let metadata_format = if is_audio_only {
                                "MP3".to_string()
                            } else {
                                final_format_ext.clone()
                            };

                            let _ = window.emit(
                                "download-finished",
                                (
                                    download_id.clone(),
                                    video_title.clone(),
                                    canonical,
                                    duration_str,
                                    file_size_str,
                                    metadata_format,
                                    current_resolution.clone(),
                                    fps_str,
                                    metadata_thumbnail.clone(),
                                ),
                            );
                            return;
                        }
                        Some((false, _, retry_stderr)) => {
                            eprintln!("Cookie retry also failed: {}", retry_stderr);
                            // Fall through to emit original error
                        }
                        None => {
                            eprintln!("Cookie retry process could not start");
                        }
                    }
                    if take_cancelled(&download_id) {
                        cleanup_partial_files(&planned_output_path);
                        return;
                    }
                }

                // Instagram-specific fallback: yt-dlp's extractor is broken
                // for Instagram (HTTP 400 signature — yt-dlp #13626/#16311,
                // see docs/PLATFORM-HEALTH.md runbook step 3). Try the
                // native, login-free endpoint-format fallback
                // (`instagram_fallback`) for single public posts/reels
                // before giving up. This is the wiring: without this call
                // site the fallback module is built but never runs.
                if is_instagram {
                    let combined_error_text_for_ig =
                        format!("{} {} {}", error_messages.join(" "), all_stderr, all_stdout);
                    if instagram_fallback::should_attempt_fallback(&combined_error_text_for_ig) {
                        eprintln!(
                            "Instagram extractor failed with the known HTTP-400 signature, \
                             trying the native endpoint-format fallback..."
                        );
                        match instagram_fallback::fetch_instagram_direct_video_url(&url) {
                            Ok(direct_video_url) => {
                                eprintln!(
                                    "Instagram fallback resolved a direct video URL; downloading \
                                     it through the normal yt-dlp pipeline (filenames/progress/ffmpeg unchanged)."
                                );

                                let mut fallback_cmd = Command::new(&ytdlp_path);
                                fallback_cmd.arg("--no-playlist");
                                fallback_cmd
                                    .arg("--no-warnings")
                                    .arg("--newline")
                                    .arg("--socket-timeout")
                                    .arg("30")
                                    .arg("-o")
                                    .arg(&planned_output_path);

                                if is_audio_only {
                                    fallback_cmd
                                        .arg("-x")
                                        .arg("--audio-format")
                                        .arg("mp3")
                                        .arg("--audio-quality")
                                        .arg("0");
                                } else {
                                    fallback_cmd.arg("--merge-output-format").arg(output_format);
                                }

                                fallback_cmd.arg("--ffmpeg-location").arg(&ffmpeg_dir);
                                fallback_cmd.arg(&direct_video_url);

                                let fb_current_path = std::env::var("PATH").unwrap_or_default();
                                let fb_enhanced_path =
                                    if !fb_current_path.contains("/opt/homebrew/bin") {
                                        format!(
                                            "{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin",
                                            fb_current_path
                                        )
                                    } else {
                                        fb_current_path
                                    };
                                fallback_cmd.env("PATH", &fb_enhanced_path);

                                match run_streaming_attempt(
                                    &window,
                                    &download_id,
                                    &mut fallback_cmd,
                                    &current_resolution,
                                ) {
                                    Some((true, fb_stdout, _)) => {
                                        eprintln!(
                                            "Instagram fallback download succeeded for {}",
                                            download_id
                                        );
                                        let mut fb_file_path = planned_output_path.clone();
                                        for line in fb_stdout.lines() {
                                            if let Some(path) = extract_file_path(line) {
                                                if Path::new(&path).exists() {
                                                    fb_file_path = path;
                                                }
                                            }
                                        }
                                        let canonical = fs::canonicalize(&fb_file_path)
                                            .map(|p| p.to_string_lossy().to_string())
                                            .unwrap_or(fb_file_path);
                                        let Some(canonical) = ensure_h264(
                                            &window,
                                            &download_id,
                                            &canonical,
                                            &ffmpeg_dir,
                                            &current_resolution,
                                        ) else {
                                            return; // cancelled during conversion
                                        };

                                        let duration_str = metadata_duration
                                            .map(format_duration)
                                            .unwrap_or_default();
                                        let file_size_str = if Path::new(&canonical).exists() {
                                            fs::metadata(&canonical)
                                                .map(|m| format_file_size(m.len()))
                                                .unwrap_or_default()
                                        } else {
                                            metadata_size.clone()
                                        };
                                        let fps_str = metadata_fps
                                            .map(|f| format!("{}fps", f.round() as u64))
                                            .unwrap_or_default();
                                        let fb_format = if is_audio_only {
                                            "MP3".to_string()
                                        } else {
                                            final_format_ext.clone()
                                        };

                                        let _ = window.emit(
                                            "download-finished",
                                            (
                                                download_id.clone(),
                                                video_title.clone(),
                                                canonical,
                                                duration_str,
                                                file_size_str,
                                                fb_format,
                                                current_resolution.clone(),
                                                fps_str,
                                                metadata_thumbnail.clone(),
                                            ),
                                        );
                                        return;
                                    }
                                    Some((false, _, fb_stderr)) => {
                                        eprintln!(
                                            "Instagram fallback download failed: {}",
                                            fb_stderr
                                        );
                                        // Fall through to emit the original yt-dlp error below.
                                    }
                                    None => {
                                        eprintln!("Instagram fallback process could not start");
                                        // Fall through to emit the original yt-dlp error below.
                                    }
                                }
                                if take_cancelled(&download_id) {
                                    cleanup_partial_files(&planned_output_path);
                                    return;
                                }
                            }
                            Err(instagram_fallback::IgFallbackError::Fatal(msg)) => {
                                eprintln!("Instagram fallback: fatal — {}", msg);
                                let _ = window.emit("download-error", (download_id.clone(), msg));
                                return;
                            }
                            Err(instagram_fallback::IgFallbackError::Transient(msg)) => {
                                eprintln!("Instagram fallback: exhausted all endpoints — {}", msg);
                                // Fall through to emit the original yt-dlp error below.
                            }
                        }
                    }
                }

                // Process exited with non-zero status - build error message
                let drm_protected = error_messages.iter().any(|m| m.contains("DRM protected"))
                    || all_stderr.contains("DRM protected");
                let error_msg = if drm_protected {
                    // Encrypted streams (Vimeo since 2026-10). Never circumvented.
                    "This video is DRM-protected by the site, so it can't be downloaded."
                        .to_string()
                } else if !error_messages.is_empty() {
                    let msg = error_messages.join("; ");
                    if msg.contains("403") || msg.contains("Forbidden") || msg.contains("SABR") {
                        let mut final_msg = msg.clone();
                        if msg.contains("SABR") || msg.contains("youtube") {
                            final_msg = format!("{}\n\nYouTube download failed — usually a YouTube-side change. Click 'Update downloader engine' in Settings and try again.", msg);
                        }
                        truncate_chars(&final_msg, 400)
                    } else if msg.contains("impersonation")
                        || msg.contains("impersonate")
                        || msg.contains("Vimeo")
                    {
                        truncate_chars(&msg, 500)
                    } else {
                        truncate_chars(&msg, 300)
                    }
                } else if !all_stderr.trim().is_empty() {
                    let stderr_trimmed = all_stderr.trim();
                    let stderr_lines: Vec<&str> = stderr_trimmed.lines().collect();
                    let msg = if stderr_lines.len() > 5 {
                        let last_lines: Vec<&str> =
                            stderr_lines.iter().rev().take(5).rev().copied().collect();
                        last_lines.join("\n")
                    } else {
                        stderr_trimmed.to_string()
                    };
                    truncate_chars(&msg, 500)
                } else if !all_stdout.trim().is_empty() {
                    let stdout_lines: Vec<&str> = all_stdout.trim().lines().collect();
                    let msg = if stdout_lines.len() > 5 {
                        let last_lines: Vec<&str> =
                            stdout_lines.iter().rev().take(5).rev().copied().collect();
                        last_lines.join("\n")
                    } else {
                        all_stdout.trim().to_string()
                    };
                    truncate_chars(&msg, 300)
                } else {
                    format!("Download failed (exit code {}). Try again, or use Update in Settings → Engine.", status.code().unwrap_or(-1))
                };
                eprintln!("Download error: {}", error_msg);
                let _ = window.emit("download-error", (download_id.clone(), error_msg.clone()));
            }
            Err(e) => {
                if take_cancelled(&download_id) {
                    eprintln!("Download {} cancelled by user", download_id);
                    cleanup_partial_files(&planned_output_path);
                    return;
                }
                let _ = window.emit(
                    "download-error",
                    (download_id.clone(), format!("Process error: {}", e)),
                );
            }
        }
    });

    download_id_clone
}

fn extract_file_path(line: &str) -> Option<String> {
    // Extract file path from yt-dlp output
    // Try quoted paths first
    if let Some(start) = line.find('"') {
        if let Some(end) = line.rfind('"') {
            if end > start {
                return Some(line[start + 1..end].to_string());
            }
        }
    }
    // Try "Destination:" pattern
    if let Some(pos) = line.find("Destination:") {
        let path_part = &line[pos + 12..].trim();
        if !path_part.is_empty() {
            return Some(path_part.to_string());
        }
    }
    None
}

fn format_duration(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

fn format_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= GB {
        format!("{:.1} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.1} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.1} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

fn sanitize_filename_for_fs(input: &str) -> String {
    let mut cleaned = String::with_capacity(input.len());
    for ch in input.chars() {
        let forbidden = matches!(ch, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|');
        if forbidden || ch.is_control() {
            cleaned.push('_');
        } else {
            cleaned.push(ch);
        }
    }

    // macOS caps a file name at 255 bytes; long titles failed with
    // "[Errno 63] File name too long". 150 chars leaves room for the
    // extension, a " (copy N)" suffix and yt-dlp's temp suffixes.
    let capped: String = cleaned.trim().chars().take(150).collect();
    let trimmed = capped.trim().trim_matches('.');
    if trimmed.is_empty() {
        "download".to_string()
    } else {
        trimmed.to_string()
    }
}

// Paths handed out in this session. Two downloads with the same title (or the
// same URL twice) would otherwise share one path and its .part files.
static RESERVED_OUTPUT_PATHS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn build_unique_output_path(base_path: &str, title: &str, extension: &str) -> String {
    let safe_title = sanitize_filename_for_fs(title);
    let mut copy_index = 0usize;
    let reserved = RESERVED_OUTPUT_PATHS.get_or_init(|| Mutex::new(HashSet::new()));

    loop {
        let filename = if copy_index == 0 {
            format!("{}.{}", safe_title, extension)
        } else {
            format!("{} (copy {}).{}", safe_title, copy_index, extension)
        };
        let candidate = Path::new(base_path).join(filename);
        let candidate_str = candidate.to_string_lossy().to_string();
        if !candidate.exists() {
            if let Ok(mut set) = reserved.lock() {
                if set.insert(candidate_str.clone()) {
                    return candidate_str;
                }
            } else {
                return candidate_str;
            }
        }
        copy_index += 1;
    }
}

// A failed attempt that is retried under the same download frees its name.
fn release_output_path(path: &str) {
    if let Some(reserved) = RESERVED_OUTPUT_PATHS.get() {
        if let Ok(mut set) = reserved.lock() {
            set.remove(path);
        }
    }
}

// Error text is shown in the UI; cut by characters, never by bytes (a byte cut
// inside an accented letter or emoji panics and leaves the row stuck).
fn truncate_chars(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max).collect::<String>())
    }
}

// Remove yt-dlp's partial/intermediate files for a planned output after an
// error or cancel (.part, .ytdl, per-stream .fNNN files, .temp.mp4).
fn cleanup_partial_files(planned_output_path: &str) {
    let planned = Path::new(planned_output_path);
    let (Some(dir), Some(stem)) = (
        planned.parent(),
        planned.file_stem().and_then(|s| s.to_str()),
    ) else {
        return;
    };
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(rest) = name.strip_prefix(stem) else {
            continue;
        };
        let is_partial = rest.ends_with(".part")
            || rest.ends_with(".ytdl")
            || rest.contains(".temp.")
            || rest.contains(".part-Frag")
            || (rest.starts_with(".f")
                && rest[2..].chars().next().is_some_and(|c| c.is_ascii_digit()));
        if is_partial {
            let _ = fs::remove_file(entry.path());
        }
    }
}

// Run each external process in its own process group so cancel can stop the
// whole tree: yt-dlp spawns ffmpeg, and killing only yt-dlp re-parents ffmpeg
// to launchd, where it keeps encoding.
fn own_process_group(cmd: &mut Command) -> &mut Command {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    cmd
}

fn probe_video_codec_and_duration(path: &str, ffmpeg_dir: &str) -> (Option<String>, Option<f64>) {
    let ffprobe =
        find_bundled_binary("ffprobe").unwrap_or_else(|| format!("{}/ffprobe", ffmpeg_dir));
    let run = |args: &[&str]| -> Option<String> {
        Command::new(&ffprobe)
            .args(args)
            .arg(path)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty())
    };
    let codec = run(&[
        "-v",
        "error",
        "-select_streams",
        "V:0",
        "-show_entries",
        "stream=codec_name",
        "-of",
        "csv=p=0",
    ]);
    let duration = run(&[
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "csv=p=0",
    ])
    .and_then(|d| d.parse::<f64>().ok());
    (codec, duration)
}

// Premiere-ready guarantee: after the download, re-encode to H.264/AAC only when
// the file's video isn't H.264 already (YouTube 4K VP9/AV1, TikTok HEVC…). The
// codec is read from the actual file, so H.264 sources are never re-encoded,
// and progress comes from ffmpeg itself instead of a timer.
// Returns the final path, or None when the download was cancelled meanwhile.
fn ensure_h264<R: Runtime>(
    window: &Window<R>,
    download_id: &str,
    path: &str,
    ffmpeg_dir: &str,
    resolution: &str,
) -> Option<String> {
    let (codec, duration) = probe_video_codec_and_duration(path, ffmpeg_dir);
    let Some(codec) = codec else {
        return Some(path.to_string()); // no video stream (audio-only) or unreadable
    };
    if codec == "h264" {
        return Some(path.to_string());
    }
    eprintln!("Converting {} ({}) to H.264 for editing", path, codec);
    let emit = |pct: u8| {
        let _ = window.emit(
            "download-progress",
            (
                download_id.to_string(),
                pct,
                resolution.to_string(),
                String::new(),
                "converting",
            ),
        );
    };
    emit(0);

    let source = Path::new(path);
    let final_path = source.with_extension("mp4");
    let tmp_path = source.with_extension("converting.mp4");
    let ffmpeg = find_bundled_binary("ffmpeg").unwrap_or_else(|| format!("{}/ffmpeg", ffmpeg_dir));
    let mut cmd = Command::new(ffmpeg);
    cmd.args(["-y", "-v", "error", "-nostats", "-progress", "pipe:1", "-i"])
        .arg(path)
        .args([
            "-map",
            "0:V:0",
            "-map",
            "0:a?",
            "-threads",
            "0",
            "-c:v",
            "h264_videotoolbox",
            "-q:v",
            "70",
            "-profile:v",
            "high",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-b:a",
            "320k",
            "-movflags",
            "+faststart",
        ])
        .arg(&tmp_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let Ok(mut child) = own_process_group(&mut cmd).spawn() else {
        eprintln!("Could not start ffmpeg for H.264 conversion; keeping the original file");
        return Some(path.to_string());
    };
    if let Ok(mut map) = active_downloads().lock() {
        map.insert(download_id.to_string(), child.id());
    }
    let mut last = 0u8;
    if let (Some(stdout), Some(total)) = (child.stdout.take(), duration.filter(|d| *d > 0.0)) {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            // out_time_us is in microseconds (out_time_ms is too, despite its name).
            if let Some(us) = line
                .strip_prefix("out_time_us=")
                .and_then(|v| v.parse::<f64>().ok())
            {
                let pct = ((us / 1_000_000.0 / total) * 100.0).clamp(0.0, 99.0) as u8;
                if pct > last {
                    last = pct;
                    emit(pct);
                }
            }
        }
    }
    let ok = child.wait().map(|s| s.success()).unwrap_or(false);
    if let Ok(mut map) = active_downloads().lock() {
        map.remove(download_id);
    }
    if take_cancelled(download_id) {
        // The user cancelled: leave nothing behind, not even the unconverted file.
        let _ = fs::remove_file(&tmp_path);
        let _ = fs::remove_file(source);
        return None;
    }
    if !ok || fs::rename(&tmp_path, &final_path).is_err() {
        let _ = fs::remove_file(&tmp_path);
        eprintln!("H.264 conversion failed; keeping the original file");
        return Some(path.to_string());
    }
    if source != final_path {
        let _ = fs::remove_file(source);
    }
    Some(final_path.to_string_lossy().to_string())
}

// Run a retry/fallback yt-dlp attempt with live progress (same mapping as the
// first attempt) and cancel support. The old `.output()` call showed a frozen
// bar until the whole retry finished. Returns (success, stdout, stderr), or
// None when the process could not start.
fn run_streaming_attempt<R: Runtime>(
    window: &Window<R>,
    download_id: &str,
    cmd: &mut Command,
    resolution: &str,
) -> Option<(bool, String, String)> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = own_process_group(cmd).spawn().ok()?;
    if let Ok(mut map) = active_downloads().lock() {
        map.insert(download_id.to_string(), child.id());
    }
    // Drain stderr on its own thread so a chatty stderr can never block stdout.
    let stderr_handle = child.stderr.take().map(|err| {
        thread::spawn(move || {
            let mut text = String::new();
            let _ = std::io::Read::read_to_string(&mut BufReader::new(err), &mut text);
            text
        })
    });
    let mut stdout_text = String::new();
    let mut progress = ProgressTracker::default();
    if let Some(out) = child.stdout.take() {
        for line in lossy_lines(out) {
            progress.observe_line(&line);
            if let Some((raw, speed)) = parse_progress(&line) {
                let pct = progress.overall(raw);
                let _ = window.emit(
                    "download-progress",
                    (
                        download_id.to_string(),
                        pct,
                        resolution.to_string(),
                        speed,
                        "downloading",
                    ),
                );
            }
            stdout_text.push_str(&line);
            stdout_text.push('\n');
        }
    }
    let ok = child.wait().map(|s| s.success()).unwrap_or(false);
    if let Ok(mut map) = active_downloads().lock() {
        map.remove(download_id);
    }
    let stderr_text = stderr_handle
        .and_then(|h| h.join().ok())
        .unwrap_or_default();
    Some((ok, stdout_text, stderr_text))
}

// Line iterator that survives non-UTF-8 output (`lines()` stops at the first
// invalid byte, leaving the pipe unread and the process blocked).
fn lossy_lines<T: std::io::Read>(reader: T) -> impl Iterator<Item = String> {
    BufReader::new(reader)
        .split(b'\n')
        .map_while(Result::ok)
        .map(|bytes| {
            String::from_utf8_lossy(&bytes)
                .trim_end_matches('\r')
                .to_string()
        })
}

// Overall progress for the UI. yt-dlp downloads merged formats one stream after
// another (video, then audio), each going 0→100%, and with concurrent fragments
// the per-stream percentage jitters backwards. This maps streams onto one bar
// (video ≈85%, audio the rest) and never lets it move backwards.
#[derive(Default)]
struct ProgressTracker {
    streams: u32,
    stream: u32,
    stream_has_progress: bool,
    emitted: u8,
}

impl ProgressTracker {
    fn observe_line(&mut self, line: &str) {
        // "[info] <id>: Downloading 1 format(s): 137+140"
        if line.starts_with("[info]") && line.contains("format(s):") {
            let ids = line.rsplit("format(s):").next().unwrap_or("");
            self.streams = (ids.trim().matches('+').count() as u32 + 1).min(4);
        }
        // Each stream announces its own destination file.
        if line.starts_with("[download] Destination:") && self.stream_has_progress {
            self.stream = (self.stream + 1).min(self.streams.max(1) - 1);
            self.stream_has_progress = false;
        }
    }

    fn overall(&mut self, raw: u8) -> u8 {
        self.stream_has_progress = true;
        let raw = raw.min(100) as u32;
        let pct = if self.streams <= 1 {
            raw
        } else if self.stream == 0 {
            raw * 85 / 100
        } else {
            let span = 15 / (self.streams - 1);
            85 + span * (self.stream - 1) + raw * span / 100
        };
        self.emitted = self.emitted.max(pct.min(100) as u8);
        self.emitted
    }
}

fn parse_progress(line: &str) -> Option<(u8, String)> {
    if !line.starts_with("[download]") || !line.contains('%') {
        return None;
    }

    let mut percent: Option<u8> = None;
    let mut speed = String::new();

    for part in line.split_whitespace() {
        if part.ends_with('%') {
            percent = part
                .trim_end_matches('%')
                .parse::<f32>()
                .ok()
                .map(|v| v.round() as u8);
        }

        if part.contains("MiB/s")
            || part.contains("KiB/s")
            || part.contains("MB/s")
            || part.contains("KB/s")
        {
            speed = part.to_string();
        }
    }

    percent.map(|p| (p, speed))
}

#[cfg(test)]
mod e2e_tests;

#[cfg(test)]
mod tests {
    // Live (network) end-to-end check of the engine self-update in a throwaway
    // HOME: legacy cleanup → onedir install → fast second start → no re-download.
    // Mutates HOME, so run it alone:
    //   cargo test engine_self_update_live -- --ignored --nocapture
    #[test]
    #[ignore]
    fn engine_self_update_live() {
        use std::time::Instant;
        let home = std::env::temp_dir().join(format!("sd-engine-test-{}", std::process::id()));
        let bin = home.join("Library/Application Support/com.supermac.super-downloads/bin");
        std::fs::create_dir_all(bin.join("engine-2000.01.01.tmp")).unwrap();
        std::fs::write(bin.join("yt-dlp"), b"legacy onefile").unwrap();
        std::fs::write(bin.join("yt-dlp.tmp"), b"interrupted").unwrap();
        std::env::set_var("HOME", &home);

        super::prepare_ytdlp_engine();
        assert!(
            !bin.join("yt-dlp").exists(),
            "legacy onefile engine removed"
        );
        assert!(!bin.join("yt-dlp.tmp").exists(), "interrupted file removed");
        assert!(
            !bin.join("engine-2000.01.01.tmp").exists(),
            "interrupted dir removed"
        );

        let tag = tauri::async_runtime::block_on(super::download_latest_ytdlp()).unwrap();
        let managed = super::find_managed_ytdlp().expect("managed engine installed");
        assert!(managed.ends_with(&format!("engine-{}/yt-dlp_macos", tag)));

        let t = Instant::now();
        assert_eq!(
            super::binary_version(&managed).as_deref(),
            Some(tag.as_str())
        );
        let warm = t.elapsed();
        println!("LIVE engine {} · warm --version {:?}", tag, warm);
        assert!(
            warm.as_secs() < 3,
            "onedir engine should start fast once scanned"
        );

        let t = Instant::now();
        let again = tauri::async_runtime::block_on(super::download_latest_ytdlp()).unwrap();
        assert_eq!(again, tag);
        println!("LIVE second update call (no download) {:?}", t.elapsed());

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn vimeo_oauth_401_signature_is_recognised() {
        let err = "ERROR: [vimeo] 76979871: Unable to download macos API JSON: HTTP Error 401: Unauthorized";
        assert!(super::is_vimeo_oauth_401_error(err));
        assert!(!super::is_vimeo_oauth_401_error(
            "ERROR: [vimeo] 1: This video is private"
        ));
        assert!(!super::is_vimeo_oauth_401_error(
            "ERROR: [youtube] x: HTTP Error 401"
        ));
    }

    #[test]
    fn vimeo_login_wall_2026_08_19_signature_is_recognised() {
        // yt-dlp 2026.08.19+ text — no "401", no "macos api json"/"oauth token".
        let err = "ERROR: [vimeo] 76979871: The web client only works when logged-in. \
                    Use --cookies, --cookies-from-browser, --username and --password, \
                    --netrc-cmd, or --netrc (vimeo) to provide account credentials.";
        assert!(super::is_vimeo_oauth_401_error(err));
        // Case-insensitive match.
        assert!(super::is_vimeo_oauth_401_error(
            "ERROR: [Vimeo] 1: THE WEB CLIENT ONLY WORKS WHEN LOGGED-IN."
        ));
        // Space variant (no hyphen), just in case the wording drifts again.
        assert!(super::is_vimeo_oauth_401_error(
            "ERROR: [vimeo] 1: The web client only works when logged in."
        ));
        // Still must not match an unrelated Vimeo error.
        assert!(!super::is_vimeo_oauth_401_error(
            "ERROR: [vimeo] 1: This video is private"
        ));
    }

    #[test]
    fn vimeo_embed_url_rewrites_public_and_unlisted_links() {
        assert_eq!(
            super::vimeo_embed_url("https://vimeo.com/76979871").as_deref(),
            Some("https://player.vimeo.com/video/76979871")
        );
        assert_eq!(
            super::vimeo_embed_url("https://vimeo.com/channels/staffpicks/76979871?share=copy")
                .as_deref(),
            Some("https://player.vimeo.com/video/76979871")
        );
        assert_eq!(
            super::vimeo_embed_url("https://vimeo.com/76979871/a1b2c3d4e5").as_deref(),
            Some("https://player.vimeo.com/video/76979871?h=a1b2c3d4e5")
        );
        assert!(super::vimeo_embed_url("https://player.vimeo.com/video/76979871").is_none());
        assert!(super::vimeo_embed_url("https://vimeo.com/user12/videos").is_none());
    }

    use super::{extract_file_path, format_duration, parse_progress};

    #[test]
    fn parse_progress_extracts_percent_and_speed() {
        let line = "[download]  42.3% of 123.45MiB at 2.31MiB/s ETA 00:15";
        let progress = parse_progress(line).expect("progress should parse");
        assert_eq!(progress.0, 42);
        assert_eq!(progress.1, "2.31MiB/s");
    }

    #[test]
    fn extract_file_path_from_destination_line() {
        let line = "[download] Destination: /Users/me/Downloads/video.mp4";
        let path = extract_file_path(line).expect("path should parse");
        assert_eq!(path, "/Users/me/Downloads/video.mp4");
    }

    #[test]
    fn format_duration_handles_hours() {
        assert_eq!(format_duration(65.0), "01:05");
        assert_eq!(format_duration(3661.0), "01:01:01");
    }
}

#[tauri::command]
async fn pick_folder() -> Result<Option<String>, String> {
    // Use native macOS file dialog via osascript (AppleScript)
    // This is the most reliable way to get a folder picker on macOS
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg("POSIX path of (choose folder with prompt \"Select Download Location\")")
            .output();

        match output {
            Ok(result) if result.status.success() => {
                let path = String::from_utf8_lossy(&result.stdout).trim().to_string();
                if !path.is_empty() {
                    Ok(Some(path))
                } else {
                    Ok(None)
                }
            }
            Ok(_) => Ok(None), // User cancelled
            Err(e) => Err(format!("Failed to open folder picker: {}", e)),
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("Folder picker not implemented for this platform".to_string())
    }
}

// Async so the kill/pkill processes never run on the UI thread.
#[tauri::command]
async fn cancel_download(download_id: String) -> Result<(), String> {
    if let Ok(mut cancelled) = cancelled_downloads().lock() {
        cancelled.insert(download_id.clone());
    }

    let pid = {
        let map = active_downloads()
            .lock()
            .map_err(|_| "Failed to access active downloads".to_string())?;
        map.get(&download_id).copied()
    };

    let Some(pid) = pid else {
        // Race-safe: process may have just exited while UI still shows active.
        return Ok(());
    };

    #[cfg(target_family = "unix")]
    {
        let pid_arg = pid.to_string();
        // Every download process runs in its own group (own_process_group), so
        // signal the whole group: yt-dlp and the ffmpeg it spawned.
        let _ = Command::new("kill")
            .args(["-TERM", "--", &format!("-{}", pid)])
            .status();
        let term_output = Command::new("kill")
            .arg("-TERM")
            .arg(&pid_arg)
            .output()
            .map_err(|e| format!("Failed to execute kill -TERM: {}", e))?;

        let _ = Command::new("pkill")
            .arg("-TERM")
            .arg("-P")
            .arg(&pid_arg)
            .status();

        if let Ok(mut map) = active_downloads().lock() {
            map.remove(&download_id);
        }

        if !term_output.status.success() {
            let stderr = String::from_utf8_lossy(&term_output.stderr).to_lowercase();
            if stderr.contains("no such process") {
                return Ok(());
            }
            if let Ok(mut cancelled) = cancelled_downloads().lock() {
                cancelled.remove(&download_id);
            }
            return Err(format!("Failed to terminate download process {}", pid));
        }

        Ok(())
    }

    #[cfg(not(target_family = "unix"))]
    {
        let taskkill_status = Command::new("taskkill")
            .arg("/PID")
            .arg(pid.to_string())
            .arg("/F")
            .status()
            .map_err(|e| format!("Failed to execute taskkill: {}", e))?;

        if !taskkill_status.success() {
            if let Ok(mut cancelled) = cancelled_downloads().lock() {
                cancelled.remove(&download_id);
            }
            return Err(format!("Failed to terminate download process {}", pid));
        }

        if let Ok(mut map) = active_downloads().lock() {
            map.remove(&download_id);
        }

        Ok(())
    }
}

#[tauri::command]
fn resize_window_height(window: Window, height: u32) -> Result<(), String> {
    let target_inner_height = height.clamp(220, 1200) as f64;
    let scale_factor = window
        .scale_factor()
        .map_err(|e| format!("Failed to read scale factor: {}", e))?;
    let outer_size = window
        .outer_size()
        .map_err(|e| format!("Failed to read current window size: {}", e))?;
    let inner_size = window
        .inner_size()
        .map_err(|e| format!("Failed to read inner window size: {}", e))?;

    // Convert physical pixel delta to logical pixels
    let chrome_delta_height =
        (outer_size.height.saturating_sub(inner_size.height) as f64) / scale_factor;
    let target_outer_height = (target_inner_height + chrome_delta_height).clamp(220.0, 1200.0);

    window
        .set_size(Size::Logical(LogicalSize::new(
            WINDOW_LOGICAL_WIDTH,
            target_outer_height,
        )))
        .map_err(|e| format!("Failed to resize window: {}", e))
}

#[tauri::command]
fn set_min_window_height(window: Window, height: u32) -> Result<(), String> {
    let target_inner_height = height.clamp(220, 1200) as f64;
    let scale_factor = window
        .scale_factor()
        .map_err(|e| format!("Failed to read scale factor: {}", e))?;
    let outer_size = window
        .outer_size()
        .map_err(|e| format!("Failed to read current window size: {}", e))?;
    let inner_size = window
        .inner_size()
        .map_err(|e| format!("Failed to read inner window size: {}", e))?;

    // Convert physical pixel delta to logical pixels
    let chrome_delta_height =
        (outer_size.height.saturating_sub(inner_size.height) as f64) / scale_factor;
    let target_outer_min_height = (target_inner_height + chrome_delta_height).clamp(220.0, 1200.0);
    window
        .set_min_size(Some(Size::Logical(LogicalSize::new(
            400.0,
            target_outer_min_height,
        ))))
        .map_err(|e| format!("Failed to set minimum window size: {}", e))
}

fn thumbnail_cache_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let cache_root = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Failed to resolve app cache directory: {}", e))?;
    let thumb_dir = cache_root.join("thumbnails");
    fs::create_dir_all(&thumb_dir)
        .map_err(|e| format!("Failed to create thumbnail cache directory: {}", e))?;
    Ok(thumb_dir)
}

fn cache_thumbnail_for_download<R: Runtime>(
    app: &AppHandle<R>,
    ytdlp_path: &str,
    url: &str,
    download_id: &str,
) -> Option<String> {
    let cache_dir = thumbnail_cache_dir(app).ok()?;
    let prefix = format!("thumb-{}", download_id);

    if let Ok(entries) = fs::read_dir(&cache_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem == prefix {
                let _ = fs::remove_file(path);
            }
        }
    }

    let template = cache_dir.join(format!("{}.%(ext)s", prefix));
    let output = Command::new(ytdlp_path)
        .arg("--skip-download")
        .arg("--no-playlist")
        .arg("--write-thumbnail")
        .arg("--convert-thumbnails")
        .arg("jpg")
        .arg("-o")
        .arg(template.to_string_lossy().to_string())
        .arg(url)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let preferred_exts = ["jpg", "jpeg", "webp", "png"];
    for ext in preferred_exts {
        let candidate = cache_dir.join(format!("{}.{}", prefix, ext));
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    None
}

fn download_history_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data directory: {}", e))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    Ok(app_data_dir.join("download-history.json"))
}

#[tauri::command]
fn save_download_history(app: AppHandle, downloads_json: String) -> Result<(), String> {
    let history_path = download_history_path(&app)?;
    fs::write(&history_path, downloads_json)
        .map_err(|e| format!("Failed to save download history: {}", e))
}

#[tauri::command]
fn load_download_history(app: AppHandle) -> Result<String, String> {
    let history_path = download_history_path(&app)?;
    match fs::read_to_string(&history_path) {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok("[]".to_string()),
        Err(err) => Err(format!("Failed to load download history: {}", err)),
    }
}

#[tauri::command]
fn delete_cached_thumbnail(app: AppHandle, path: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Ok(());
    }

    let cache_dir = thumbnail_cache_dir(&app)?;
    let cache_dir_canonical = fs::canonicalize(&cache_dir).unwrap_or(cache_dir.clone());
    let candidate = PathBuf::from(path);

    if !candidate.exists() {
        return Ok(());
    }

    let candidate_canonical = fs::canonicalize(&candidate)
        .map_err(|e| format!("Failed to validate thumbnail path: {}", e))?;

    if !candidate_canonical.starts_with(&cache_dir_canonical) {
        return Err("Refusing to delete thumbnail outside cache directory".to_string());
    }

    if candidate_canonical.is_file() {
        fs::remove_file(candidate_canonical)
            .map_err(|e| format!("Failed to delete cached thumbnail: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn reveal_in_finder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("open").arg("-R").arg(&path).output();

        match output {
            Ok(result) if result.status.success() => Ok(()),
            Ok(result) => {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                Err(format!("Failed to reveal in Finder: {}", error_msg))
            }
            Err(e) => Err(format!("Failed to execute open command: {}", e)),
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("reveal_in_finder is only supported on macOS".to_string())
    }
}

#[tauri::command]
fn read_clipboard_text() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to read clipboard: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Clipboard read failed: {}", stderr.trim()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("read_clipboard_text is only supported on macOS".to_string())
    }
}

#[tauri::command]
fn show_notification(title: String, body: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            body.replace('\\', "\\\\").replace('"', "\\\""),
            title.replace('\\', "\\\\").replace('"', "\\\"")
        );
        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| format!("Failed to show notification: {}", e))?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

// --- License validation (LemonSqueezy) ---

#[derive(serde::Serialize)]
struct LicenseResult {
    valid: bool,
    error: String,
    license_key_id: Option<u64>,
    instance_id: Option<String>,
    customer_name: Option<String>,
}

fn get_instance_name() -> String {
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string())
}

// --- Email activation (free mode) ---

fn activation_arch() -> &'static str {
    match std::env::consts::ARCH {
        "aarch64" => "aarch64",
        "x86_64" => "x86_64",
        _ => "unknown",
    }
}

fn activation_os_version() -> String {
    std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

#[tauri::command]
async fn register_activation(
    app: tauri::AppHandle,
    email: String,
    marketing_opt_in: bool,
) -> Result<(), String> {
    let body = serde_json::json!({
        "email": email.trim(),
        "appVersion": app.package_info().version.to_string(),
        "arch": activation_arch(),
        "osVersion": activation_os_version(),
        "instance": get_instance_name(),
        "marketingOptIn": marketing_opt_in,
        "source": "app",
    });
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;
    let resp = client
        .post("https://superdownloads.app/api/activate")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(format!("Activation failed: HTTP {}", resp.status()))
    }
}

#[tauri::command]
async fn activate_license(key: String) -> Result<LicenseResult, String> {
    let instance_name = get_instance_name();
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.lemonsqueezy.com/v1/licenses/activate")
        .form(&[
            ("license_key", key.as_str()),
            ("instance_name", instance_name.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    let activated = body
        .get("activated")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let error_msg = body
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let instance_id = body
        .get("instance")
        .and_then(|i| i.get("id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let license_key_id = body
        .get("license_key")
        .and_then(|l| l.get("id"))
        .and_then(|v| v.as_u64());
    let customer_name = body
        .get("meta")
        .and_then(|m| m.get("customer_name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(LicenseResult {
        valid: activated,
        error: error_msg,
        license_key_id,
        instance_id,
        customer_name,
    })
}

#[tauri::command]
async fn validate_license(key: String, instance_id: String) -> Result<LicenseResult, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.lemonsqueezy.com/v1/licenses/validate")
        .form(&[
            ("license_key", key.as_str()),
            ("instance_id", instance_id.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    let valid = body.get("valid").and_then(|v| v.as_bool()).unwrap_or(false);
    let error_msg = body
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let license_key_id = body
        .get("license_key")
        .and_then(|l| l.get("id"))
        .and_then(|v| v.as_u64());
    let customer_name = body
        .get("meta")
        .and_then(|m| m.get("customer_name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(LicenseResult {
        valid,
        error: error_msg,
        license_key_id,
        instance_id: Some(instance_id),
        customer_name,
    })
}

#[tauri::command]
async fn deactivate_license(key: String, instance_id: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.lemonsqueezy.com/v1/licenses/deactivate")
        .form(&[
            ("license_key", key.as_str()),
            ("instance_id", instance_id.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    let deactivated = body
        .get("deactivated")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    Ok(deactivated)
}

// ─────────────────────────────────────────────────────────────────────────
// Auto-update (Tauri updater plugin + static latest.json on GitHub Releases)
// ─────────────────────────────────────────────────────────────────────────

#[derive(serde::Serialize)]
struct UpdateInfo {
    version: String,
    notes: String,
}

/// Check the configured updater endpoint for a newer release.
/// Returns Some(info) when an update is available, None when up to date.
#[tauri::command]
async fn check_for_update(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => Ok(Some(UpdateInfo {
            version: update.version.clone(),
            notes: update.body.clone().unwrap_or_default(),
        })),
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Download + install the available update, emitting `update-progress`
/// (downloaded, total) events, then relaunch the app into the new version.
#[tauri::command]
async fn install_update(app: AppHandle, window: Window) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater
        .check()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No update available".to_string())?;

    let win = window.clone();
    let mut downloaded: u64 = 0;
    update
        .download_and_install(
            move |chunk_len, content_len| {
                downloaded += chunk_len as u64;
                let _ = win.emit("update-progress", (downloaded, content_len));
            },
            move || {},
        )
        .await
        .map_err(|e| e.to_string())?;

    // download_and_install replaced the app bundle; relaunch into the new version.
    app.restart();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[derive(serde::Serialize)]
struct YtdlpUpdateResult {
    updated: bool,
    version: Option<String>,
    message: String,
}

fn ytdlp_last_check_path() -> Option<PathBuf> {
    Some(managed_bin_dir()?.join(".last-update-check"))
}

// Weekly throttle: true if never checked or the interval has elapsed.
fn should_check_ytdlp_update() -> bool {
    match ytdlp_last_check_path() {
        Some(path) => match fs::metadata(&path).and_then(|m| m.modified()) {
            Ok(modified) => modified
                .elapsed()
                .map(|e| e.as_secs() >= YTDLP_UPDATE_INTERVAL_SECS)
                .unwrap_or(true),
            Err(_) => true, // never checked yet
        },
        None => false, // no HOME → skip silently
    }
}

fn touch_ytdlp_check() {
    if let Some(path) = ytdlp_last_check_path() {
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(&path, b"");
    }
}

fn binary_version(path: &str) -> Option<String> {
    Command::new(path)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|v| !v.is_empty())
}

// Launch-time engine housekeeping (background thread, never on the UI path):
// 1. Remove leftovers: the single-file engine of v1.2–1.3 and interrupted
//    downloads (*.tmp).
// 2. Keep only the newest managed engine. Older ones are removed here, at
//    launch, never during an update, so a running download keeps its files.
// 3. After an app update the bundled engine may be fresher than the managed
//    one; drop the managed copy when it lost the race. Versions are dates
//    (YYYY.MM.DD), so string order is version order.
// 4. Warm the active engine: its first run after install/update is scanned by
//    macOS (~7s); paying that here keeps it off the user's first download.
fn prepare_ytdlp_engine() {
    if let Some(bin_dir) = managed_bin_dir() {
        let _ = fs::remove_file(bin_dir.join("yt-dlp"));
        if let Ok(entries) = fs::read_dir(&bin_dir) {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().ends_with(".tmp") {
                    let path = entry.path();
                    let _ = fs::remove_dir_all(&path).or_else(|_| fs::remove_file(&path));
                }
            }
        }
    }

    let mut engines = managed_engines();
    if let Some((newest_tag, newest_dir)) = engines.pop() {
        for (_, dir) in engines {
            let _ = fs::remove_dir_all(dir);
        }
        let bundled_version = find_bundled_ytdlp().and_then(|p| binary_version(&p));
        if let Some(bv) = bundled_version {
            if newest_tag < bv {
                eprintln!(
                    "Managed yt-dlp {} is older than bundled {} — removing managed copy",
                    newest_tag, bv
                );
                let _ = fs::remove_dir_all(newest_dir);
            }
        }
    }

    if let Some(active) = find_ytdlp() {
        let _ = binary_version(&active);
    }
}

#[derive(serde::Serialize)]
struct YtdlpVersionInfo {
    version: Option<String>,
    source: String,
}

// Engine version shown in Settings next to the "Update engine" button.
// Async + spawn_blocking: an engine's first run after install is scanned by
// macOS (~7s). A sync command would run on the main thread and freeze the UI.
#[tauri::command]
async fn get_ytdlp_version() -> YtdlpVersionInfo {
    tauri::async_runtime::spawn_blocking(|| {
        let source = if find_managed_ytdlp().is_some() {
            "self-updated"
        } else if find_bundled_ytdlp().is_some() {
            "bundled"
        } else {
            "system"
        };
        YtdlpVersionInfo {
            version: find_ytdlp().and_then(|p| binary_version(&p)),
            source: source.to_string(),
        }
    })
    .await
    .unwrap_or(YtdlpVersionInfo {
        version: None,
        source: "unknown".to_string(),
    })
}

// Install the latest onedir macOS yt-dlp into bin/engine-<tag>/.
// Atomic: download zip → extract into engine-<tag>.tmp → verify --version →
// rename into place. Skips the download when that tag is already installed.
// Returns the version tag on success. Never touches the bundled engine.
async fn download_latest_ytdlp() -> Result<String, String> {
    // One update at a time: the launch-time auto-update and the Settings button
    // would otherwise delete each other's half-extracted .tmp folder.
    static UPDATING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if UPDATING.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return Err("An engine update is already running".to_string());
    }
    let result = download_latest_ytdlp_inner().await;
    UPDATING.store(false, std::sync::atomic::Ordering::SeqCst);
    result
}

async fn download_latest_ytdlp_inner() -> Result<String, String> {
    let bin_dir = managed_bin_dir().ok_or("Could not resolve app-support dir")?;
    fs::create_dir_all(&bin_dir).map_err(|e| format!("Could not create bin dir: {}", e))?;

    // Timeouts so the Settings "Update" button can never spin forever.
    let client = reqwest::Client::builder()
        .user_agent("super-downloads")
        .connect_timeout(std::time::Duration::from_secs(15))
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let release: serde_json::Value = client
        .get("https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Invalid release response: {}", e))?;

    let tag = release
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or("No tag_name in latest release")?
        .to_string();
    // The tag becomes a directory name — accept only YYYY.MM.DD[.N].
    if !is_engine_tag(&tag) {
        return Err(format!("Unexpected yt-dlp tag: {}", tag));
    }

    let final_dir = bin_dir.join(format!("{}{}", MANAGED_ENGINE_PREFIX, tag));
    if final_dir.join(ENGINE_EXE).exists() {
        return Ok(tag);
    }

    let url = format!(
        "https://github.com/yt-dlp/yt-dlp/releases/download/{}/yt-dlp_macos.zip",
        tag
    );
    let bytes = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Download error: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    // Sanity: the onedir zip is tens of MB; reject truncated/HTML responses.
    if bytes.len() < 10_000_000 {
        return Err(format!(
            "Downloaded yt-dlp too small ({} bytes)",
            bytes.len()
        ));
    }

    let zip_path = bin_dir.join(format!("engine-{}.zip.tmp", tag));
    let tmp_dir = bin_dir.join(format!("{}{}.tmp", MANAGED_ENGINE_PREFIX, tag));
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::write(&zip_path, &bytes).map_err(|e| format!("Write failed: {}", e))?;

    // ditto ships with macOS and keeps the executable bits stored in the zip.
    let extracted = Command::new("/usr/bin/ditto")
        .args(["-x", "-k"])
        .arg(&zip_path)
        .arg(&tmp_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let _ = fs::remove_file(&zip_path);

    // Verify the freshly extracted engine actually runs before promoting it.
    // This first run is also the slow macOS scan, so the user never pays it.
    let runs = extracted
        && tmp_dir
            .join(ENGINE_EXE)
            .to_str()
            .and_then(binary_version)
            .is_some();
    if !runs {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err("Downloaded yt-dlp failed its --version check".to_string());
    }

    // Atomic promote — a partial/corrupt download never becomes the active engine.
    fs::rename(&tmp_dir, &final_dir).map_err(|e| format!("Promote failed: {}", e))?;
    Ok(tag)
}

// Manual, user-initiated update (ignores the weekly throttle).
#[tauri::command]
async fn update_ytdlp() -> Result<YtdlpUpdateResult, String> {
    let version = download_latest_ytdlp().await?;
    touch_ytdlp_check();
    Ok(YtdlpUpdateResult {
        updated: true,
        message: format!("yt-dlp updated to {}", version),
        version: Some(version),
    })
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|_app| {
            // Engine maintenance runs off the main thread: each yt-dlp --version
            // costs ~7s, and setup() blocks the window until it returns.
            tauri::async_runtime::spawn(async {
                // A stale managed copy must never shadow a fresher bundled binary
                // (e.g. right after an app update). Prune before self-updating.
                let _ = tauri::async_runtime::spawn_blocking(prepare_ytdlp_engine).await;

                // Weekly yt-dlp self-update; on any failure the bundled binary
                // remains the fallback (see find_ytdlp). The check is stamped
                // only on success, so an update interrupted by quitting the app
                // retries on the next launch instead of a week later.
                if should_check_ytdlp_update() {
                    match download_latest_ytdlp().await {
                        Ok(v) => {
                            touch_ytdlp_check();
                            eprintln!("yt-dlp self-update: updated to {}", v);
                        }
                        Err(e) => eprintln!("yt-dlp self-update skipped: {}", e),
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            download_video,
            reveal_in_finder,
            read_clipboard_text,
            pick_folder,
            cancel_download,
            resize_window_height,
            set_min_window_height,
            save_download_history,
            load_download_history,
            delete_cached_thumbnail,
            show_notification,
            activate_license,
            register_activation,
            validate_license,
            deactivate_license,
            check_for_update,
            install_update,
            update_ytdlp,
            get_ytdlp_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
