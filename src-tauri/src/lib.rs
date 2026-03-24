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

use tauri::{AppHandle, Emitter, LogicalSize, Manager, Size, Window};

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
    let is_ignorable = line.contains("Operation not permitted")
        || line.contains("Cookies.binarycookies");
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

// Find yt-dlp executable in common locations
// Prioritizes bundled binary > homebrew > pipx > system
fn find_ytdlp() -> Option<String> {
    // FIRST: Try bundled binary (for distributed app)
    if let Some(bundled) = find_bundled_binary("yt-dlp") {
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

#[tauri::command]
fn download_video(
    window: Window,
    url: String,
    download_id: String,
    download_location: Option<String>,
    quality: Option<String>,
    format: Option<String>,
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
        let is_best_available = quality.as_deref() == Some("best") || quality.is_none();

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
                let error_msg = "yt-dlp not found. Please install it using:\nbrew install yt-dlp\nor\npip3 install yt-dlp[impersonate]\n\nIf installed, ensure it's in your PATH or at /opt/homebrew/bin/yt-dlp".to_string();
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
        let requested_max_height: Option<u64> = match quality.as_deref() {
            Some("1080p") => Some(1080),
            Some("720p") => Some(720),
            Some("480p") => Some(480),
            Some("360p") => Some(360),
            _ => None,
        };
        let mut has_h264_for_requested = false;

        // Try metadata extraction for all videos, including Vimeo
        // For Vimeo, try without cookies first (many videos are public)
        let is_vimeo = url.contains("vimeo.com");
        let is_instagram = url.contains("instagram.com") || url.contains("instagr.am");

        // Use --dump-json to get full metadata (including thumbnails for Vimeo)
        let mut metadata_cmd = Command::new(&ytdlp_path);

        // Ensure PATH includes common locations in build mode
        let current_path = std::env::var("PATH").unwrap_or_default();
        let enhanced_path = if !current_path.contains("/opt/homebrew/bin") {
            format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin", current_path)
        } else {
            current_path
        };
        metadata_cmd.env("PATH", &enhanced_path);

        metadata_cmd.arg("--dump-json").arg("--skip-download");

        // Add user-agent for all platforms to help with access
        metadata_cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);

        // LinkedIn always requires authentication cookies for metadata
        if url.contains("linkedin.com") || url.contains("lnkd.in") {
            metadata_cmd.arg("--cookies-from-browser").arg("chrome");
        }

        metadata_cmd.arg(&url);

        match metadata_cmd.output() {
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

                                    if let Some(vcodec) = fmt["vcodec"].as_str() {
                                        let vcodec_lower = vcodec.to_lowercase();
                                        let is_h264 = (vcodec_lower.contains("avc1")
                                            || vcodec_lower.contains("h264"))
                                            && vcodec_lower != "none";
                                        if is_h264 {
                                            if let Some(max_height) = requested_max_height {
                                                if height <= max_height {
                                                    has_h264_for_requested = true;
                                                }
                                            } else {
                                                has_h264_for_requested = true;
                                            }
                                        }
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
                    let error_msg = format!("Failed to run yt-dlp ({}): {}\n\nPlease ensure yt-dlp is installed:\nbrew install yt-dlp\nor\npip3 install yt-dlp[impersonate]", ytdlp_path, e);
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

        // Determine format selector based on quality setting and available resolutions
        // For "Best Available":
        //   - If 4K+ available: get best quality (VP9/AV1) and convert to H.264
        //   - If max is 1080p or lower: prefer H.264 directly (fast, no conversion)
        // For specific resolutions: prefer H.264 directly
        let format_selector = match quality.as_deref() {
            _ if is_audio_only => "bestaudio/best",
            Some("1080p") => "bestvideo[vcodec^=avc1][height<=1080]+bestaudio[acodec^=mp4a]/bestvideo[vcodec^=avc1][height<=1080]+bestaudio/bestvideo[height<=1080]+bestaudio/best[height<=1080]/best",
            Some("720p") => "bestvideo[vcodec^=avc1][height<=720]+bestaudio[acodec^=mp4a]/bestvideo[vcodec^=avc1][height<=720]+bestaudio/bestvideo[height<=720]+bestaudio/best[height<=720]/best",
            _ => {
                // "Best Available" - check if 4K+ is available
                if has_4k_available {
                    // 4K available: get best quality (will convert to H.264)
                    "bestvideo+bestaudio/best"
                } else {
                    // Max is 1080p or lower: prefer H.264 directly (no conversion needed)
                    "bestvideo[vcodec^=avc1]+bestaudio[acodec^=mp4a]/bestvideo[vcodec^=avc1]+bestaudio/bestvideo+bestaudio/best"
                }
            }
        };

        eprintln!(
            "Quality setting: {:?}, has_4k: {}, Format selector: {}",
            quality, has_4k_available, format_selector
        );

        cmd.arg("-f").arg(format_selector);

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

        // Only convert when needed:
        // - "Best Available" with 4K+ content: convert VP9/AV1 to H.264
        // - No H.264 stream available for requested quality: convert for Premiere compatibility
        let needs_conversion =
            !is_audio_only && ((is_best_available && has_4k_available) || !has_h264_for_requested);

        if needs_conversion {
            // Re-encode video to H.264 using GPU (VideoToolbox) - optimized for Apple Silicon
            // -threads 0: Use all CPU cores for decoding
            // -c:v h264_videotoolbox: Apple's hardware H.264 encoder (uses Media Engine)
            // -realtime true: Prioritize encoding speed
            // -prio_speed true: Speed over power efficiency
            // -q:v 70: High quality balance for editing pipeline
            // -profile:v high: H.264 High profile for Premiere Pro compatibility
            cmd.arg("--postprocessor-args")
                .arg("ffmpeg:-threads 0 -c:v h264_videotoolbox -realtime true -prio_speed true -q:v 70 -profile:v high -pix_fmt yuv420p -c:a aac -b:a 320k -movflags +faststart");
        }

        eprintln!(
            "Using format selector: {} for quality: {:?}",
            format_selector, quality
        );

        // User agent for all platforms (no cookies by default - auto-retry handles auth)
        cmd.arg("--user-agent").arg(DEFAULT_USER_AGENT);
        cmd.arg("--no-warnings");

        // LinkedIn always requires authentication cookies
        if is_linkedin {
            cmd.arg("--cookies-from-browser").arg("chrome");
        }

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

        let mut child = match cmd.stdout(Stdio::piped()).spawn() {
            Ok(child) => {
                eprintln!("yt-dlp process started successfully");
                child
            }
            Err(e) => {
                let error_msg = format!("Failed to start download ({}): {}\n\nPlease ensure yt-dlp is installed:\nbrew install yt-dlp\nor\npip3 install yt-dlp[impersonate]\n\nError details: {:?}", ytdlp_path, e, e);
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

        // Track if we've sent the converting status
        let mut conversion_status_sent = false;

        // Read stdout for progress and errors
        for line in stdout_reader.lines().map_while(Result::ok) {
            all_stdout.push_str(&line);
            all_stdout.push('\n');

            if is_error_line(&line) {
                stdout_errors.push(line.clone());
            }

            // Detect merging status
            if line.contains("[Merger]") || line.contains("Merging") {
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
            if let Some((pct, speed)) = parse_progress(&line) {
                // Detect when download hits 100% - if we need conversion, show converting status
                if pct >= 100 && needs_conversion && !conversion_status_sent {
                    conversion_status_sent = true;
                    eprintln!("Download complete at 100%, starting H.264 conversion...");
                }

                // Determine status
                let status = if conversion_status_sent {
                    "converting"
                } else if is_merging {
                    "merging"
                } else {
                    "downloading"
                };

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

        // If we're in best quality mode and download seems complete, emit converting status
        // This handles the case where the last progress line was already 100%
        if needs_conversion && !conversion_status_sent {
            let resolution_to_send = if !current_resolution.is_empty() {
                current_resolution.clone()
            } else if !selected_resolution.is_empty() {
                selected_resolution.clone()
            } else {
                initial_resolution.clone()
            };
            let _ = window.emit(
                "download-progress",
                (
                    download_id.clone(),
                    100,
                    resolution_to_send,
                    String::new(),
                    "converting",
                ),
            );
            eprintln!(
                "Download phase complete, now converting to H.264 (this may take a while)..."
            );
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
                    return;
                }

                // Check if this is an auth-required error that can be retried with cookies
                let combined_error_text = format!(
                    "{} {} {}",
                    error_messages.join(" "),
                    all_stderr,
                    all_stdout
                );
                let should_retry_with_cookies = is_auth_required_error(&combined_error_text)
                    && (is_youtube || is_vimeo || url.contains("facebook.com") || url.contains("fb.watch"));

                if should_retry_with_cookies {
                    eprintln!("Auth-required content detected, retrying with browser cookies...");
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
                    retry_cmd
                        .arg("-f").arg(format_selector)
                        .arg("--continue")
                        .arg("--retries").arg("10")
                        .arg("--fragment-retries").arg("10")
                        .arg("--concurrent-fragments").arg("4")
                        .arg("--socket-timeout").arg("30")
                        .arg("--ffmpeg-location").arg(&ffmpeg_dir)
                        .arg("--newline");

                    if is_audio_only {
                        retry_cmd
                            .arg("-x")
                            .arg("--audio-format").arg("mp3")
                            .arg("--audio-quality").arg("0");
                    } else {
                        retry_cmd.arg("--merge-output-format").arg(output_format);
                    }

                    if needs_conversion {
                        retry_cmd.arg("--postprocessor-args")
                            .arg("ffmpeg:-threads 0 -c:v h264_videotoolbox -realtime true -prio_speed true -q:v 70 -profile:v high -pix_fmt yuv420p -c:a aac -b:a 320k -movflags +faststart");
                    }

                    retry_cmd
                        .arg("--user-agent").arg(DEFAULT_USER_AGENT)
                        .arg("--cookies-from-browser").arg("chrome")
                        .arg("--no-warnings")
                        .arg("-o").arg(&planned_output_path)
                        .arg(&url)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped());

                    let current_path = std::env::var("PATH").unwrap_or_default();
                    let enhanced_path = if !current_path.contains("/opt/homebrew/bin") {
                        format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin", current_path)
                    } else {
                        current_path
                    };
                    retry_cmd.env("PATH", &enhanced_path);

                    match retry_cmd.output() {
                        Ok(retry_output) if retry_output.status.success() => {
                            eprintln!("Cookie retry succeeded for {}", download_id);
                            // Find the downloaded file
                            let mut retry_file_path = planned_output_path.clone();
                            let retry_stdout = String::from_utf8_lossy(&retry_output.stdout);
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

                            let duration_str = duration_seconds
                                .map(|d| {
                                    let mins = (d / 60.0).floor() as u64;
                                    let secs = (d % 60.0).round() as u64;
                                    format!("{:02}:{:02}", mins, secs)
                                })
                                .unwrap_or_default();
                            let file_size_str = size_bytes
                                .map(|b| {
                                    if b >= 1_073_741_824 {
                                        format!("{:.1} GB", b as f64 / 1_073_741_824.0)
                                    } else {
                                        format!("{:.1} MB", b as f64 / 1_048_576.0)
                                    }
                                })
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
                        Ok(retry_output) => {
                            let retry_stderr = String::from_utf8_lossy(&retry_output.stderr);
                            eprintln!("Cookie retry also failed: {}", retry_stderr);
                            // Fall through to emit original error
                        }
                        Err(e) => {
                            eprintln!("Cookie retry process error: {}", e);
                            // Fall through to emit original error
                        }
                    }
                }

                // Process exited with non-zero status - build error message
                let error_msg = if !error_messages.is_empty() {
                    let msg = error_messages.join("; ");
                    if msg.contains("403") || msg.contains("Forbidden") || msg.contains("SABR") {
                        let mut final_msg = msg.clone();
                        if msg.contains("SABR") || msg.contains("youtube") {
                            final_msg = format!("{}\n\nYouTube download failed. This is often due to YouTube's recent changes.\nTry updating yt-dlp: brew upgrade yt-dlp", msg);
                        }
                        if final_msg.len() > 400 {
                            format!("{}...", &final_msg[..400])
                        } else {
                            final_msg
                        }
                    } else if msg.contains("impersonation")
                        || msg.contains("impersonate")
                        || msg.contains("Vimeo")
                    {
                        if msg.len() > 500 {
                            format!("{}...", &msg[..500])
                        } else {
                            msg
                        }
                    } else {
                        if msg.len() > 300 {
                            format!("{}...", &msg[..300])
                        } else {
                            msg
                        }
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
                    if msg.len() > 500 {
                        format!("{}...", &msg[..500])
                    } else {
                        msg
                    }
                } else if !all_stdout.trim().is_empty() {
                    let stdout_lines: Vec<&str> = all_stdout.trim().lines().collect();
                    let msg = if stdout_lines.len() > 5 {
                        let last_lines: Vec<&str> =
                            stdout_lines.iter().rev().take(5).rev().copied().collect();
                        last_lines.join("\n")
                    } else {
                        all_stdout.trim().to_string()
                    };
                    if msg.len() > 300 {
                        format!("{}...", &msg[..300])
                    } else {
                        msg
                    }
                } else {
                    format!("Download failed (exit code: {}). Please check if yt-dlp is working correctly.", status.code().unwrap_or(-1))
                };
                eprintln!("Download error: {}", error_msg);
                let _ = window.emit("download-error", (download_id.clone(), error_msg.clone()));
            }
            Err(e) => {
                if take_cancelled(&download_id) {
                    eprintln!("Download {} cancelled by user", download_id);
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

    let trimmed = cleaned.trim().trim_matches('.');
    if trimmed.is_empty() {
        "download".to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_unique_output_path(base_path: &str, title: &str, extension: &str) -> String {
    let safe_title = sanitize_filename_for_fs(title);
    let mut copy_index = 0usize;

    loop {
        let filename = if copy_index == 0 {
            format!("{}.{}", safe_title, extension)
        } else {
            format!("{} (copy {}).{}", safe_title, copy_index, extension)
        };
        let candidate = Path::new(base_path).join(filename);
        if !candidate.exists() {
            return candidate.to_string_lossy().to_string();
        }
        copy_index += 1;
    }
}

fn parse_progress(line: &str) -> Option<(u8, String)> {
    if !line.contains('%') {
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
mod tests {
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

#[tauri::command]
fn cancel_download(download_id: String) -> Result<(), String> {
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
    let target_outer_height =
        (target_inner_height + chrome_delta_height).clamp(220.0, 1200.0);

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
    let target_outer_min_height =
        (target_inner_height + chrome_delta_height).clamp(220.0, 1200.0);
    window
        .set_min_size(Some(Size::Logical(LogicalSize::new(
            400.0,
            target_outer_min_height,
        ))))
        .map_err(|e| format!("Failed to set minimum window size: {}", e))
}

fn thumbnail_cache_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let cache_root = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Failed to resolve app cache directory: {}", e))?;
    let thumb_dir = cache_root.join("thumbnails");
    fs::create_dir_all(&thumb_dir)
        .map_err(|e| format!("Failed to create thumbnail cache directory: {}", e))?;
    Ok(thumb_dir)
}

fn cache_thumbnail_for_download(
    app: &AppHandle,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            delete_cached_thumbnail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
