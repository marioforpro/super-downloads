// Live end-to-end check of the real `download_video` command: real engine,
// real ffmpeg, real network. Records the event timeline exactly as the UI
// receives it (started → metadata → progress… → finished/error) and reports
// per-phase timings, stalls and progress that goes backwards.
//
//   cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture
//
// Cases come from SD_E2E ("label|url|quality|format" per line) or the default
// set below. Files land in a temp dir that is removed afterwards.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tauri::Listener;

const DEFAULT_CASES: &str = "\
youtube-best|https://www.youtube.com/watch?v=jNQXAC9IVRw|best|mp4
youtube-mp3|https://www.youtube.com/watch?v=jNQXAC9IVRw|best|mp3
tiktok-best|https://www.tiktok.com/@tiktok/video/7106594312292453675|best|mp4
x-best|https://x.com/SpaceX/status/1732824684683784516|best|mp4
vimeo-1080p|https://vimeo.com/863362136|1080p|mp4
instagram-best|https://www.instagram.com/reel/Chunk8-jurw/|best|mp4
facebook-best|https://www.facebook.com/watch/?v=10153231379946729|best|mp4
linkedin-best|https://www.linkedin.com/posts/the-mathworks_2_what-is-mathworks-cloud-center-activity-7151241570371948544-4Gu7|best|mp4";

const CASE_TIMEOUT: Duration = Duration::from_secs(600);

// The app resolves its engine and ffmpeg next to the executable. Under
// `cargo test` that is target/debug/deps, so link the project binaries there.
fn link_binaries() {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let bin = Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");
    let arch = if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    };
    let links = [
        ("yt-dlp-engine".to_string(), bin.join("yt-dlp-onedir")),
        (
            "ffmpeg".to_string(),
            bin.join(format!("ffmpeg-{}-apple-darwin", arch)),
        ),
        (
            "ffprobe".to_string(),
            bin.join(format!("ffprobe-{}-apple-darwin", arch)),
        ),
    ];
    for (name, target) in links {
        let link = exe_dir.join(name);
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&target, &link).unwrap();
    }
}

fn probe(path: &str) -> String {
    let ffprobe = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("ffprobe");
    std::process::Command::new(ffprobe)
        .args([
            "-v",
            "error",
            "-show_entries",
            "stream=codec_name,width,height",
            "-of",
            "csv=p=0",
            path,
        ])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .replace('\n', " / ")
        })
        .unwrap_or_default()
}

#[test]
#[ignore]
fn e2e_downloads_live() {
    link_binaries();
    let out_dir: PathBuf = std::env::temp_dir().join(format!("sd-e2e-{}", std::process::id()));
    std::fs::create_dir_all(&out_dir).unwrap();

    let app = tauri::test::mock_builder()
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let window = webview.as_ref().window();

    let (tx, rx) = mpsc::channel::<(String, String, Instant)>();
    for name in [
        "download-started",
        "download-metadata",
        "download-progress",
        "download-finished",
        "download-error",
    ] {
        let tx = tx.clone();
        app.listen_any(name, move |e| {
            let _ = tx.send((name.to_string(), e.payload().to_string(), Instant::now()));
        });
    }

    let cases = std::env::var("SD_E2E").unwrap_or_else(|_| DEFAULT_CASES.to_string());
    let mut summary = Vec::new();
    for (i, line) in cases.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        let (label, url, quality, format) = (parts[0], parts[1], parts[2], parts[3]);
        // Optional 5th field: cancel N seconds after start.
        let cancel_after: Option<u64> = parts.get(4).and_then(|v| v.parse().ok());
        let id = format!("e2e-{}", i);
        while rx.try_recv().is_ok() {}
        if let Some(secs) = cancel_after {
            let cancel_id = id.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(secs));
                let r = tauri::async_runtime::block_on(super::cancel_download(cancel_id));
                println!("E2E cancel sent after {}s: {:?}", secs, r);
            });
        }
        let case_timeout = cancel_after
            .map(|s| Duration::from_secs(s + 25))
            .unwrap_or(CASE_TIMEOUT);

        let t0 = Instant::now();
        super::download_video(
            window.clone(),
            url.to_string(),
            id.clone(),
            Some(out_dir.to_string_lossy().to_string()),
            Some(quality.to_string()),
            Some(format.to_string()),
        );

        let mut timeline: Vec<String> = Vec::new();
        let (mut first_progress, mut started, mut metadata) = (None, None, None);
        let (mut last_pct, mut backwards, mut max_gap, mut last_t) = (-1i64, 0, 0f64, t0);
        let mut statuses: Vec<String> = Vec::new();
        let mut outcome = String::from("TIMEOUT");
        let mut file = String::new();
        loop {
            let Ok((name, payload, t)) = rx.recv_timeout(case_timeout.saturating_sub(t0.elapsed()))
            else {
                break;
            };
            let v: serde_json::Value = serde_json::from_str(&payload).unwrap_or_default();
            if v.get(0).and_then(|x| x.as_str()) != Some(id.as_str()) {
                continue;
            }
            let at = t.duration_since(t0).as_secs_f64();
            max_gap = max_gap.max(t.duration_since(last_t).as_secs_f64());
            last_t = t;
            match name.as_str() {
                "download-started" => {
                    started.get_or_insert(at);
                }
                "download-metadata" => {
                    metadata.get_or_insert(at);
                }
                "download-progress" => {
                    let pct = v.get(1).and_then(|x| x.as_i64()).unwrap_or(-1);
                    let status = v.get(4).and_then(|x| x.as_str()).unwrap_or("").to_string();
                    if pct > 0 {
                        first_progress.get_or_insert(at);
                    }
                    if pct < last_pct {
                        backwards += 1;
                        timeline.push(format!("{:.1}s BACK {}→{} ({})", at, last_pct, pct, status));
                    }
                    last_pct = pct;
                    if statuses.last() != Some(&status) {
                        timeline.push(format!("{:.1}s {} {}%", at, status, pct));
                        statuses.push(status);
                    }
                }
                "download-finished" => {
                    file = v.get(2).and_then(|x| x.as_str()).unwrap_or("").to_string();
                    outcome = format!("OK {:.1}s", at);
                    break;
                }
                "download-error" => {
                    outcome = format!(
                        "ERROR {:.1}s: {}",
                        at,
                        v.get(1).and_then(|x| x.as_str()).unwrap_or("")
                    );
                    break;
                }
                _ => {}
            }
        }
        if cancel_after.is_some() && outcome == "TIMEOUT" {
            outcome = "CANCELLED (no finished/error after cancel)".to_string();
        }
        if cancel_after.is_some() {
            let leftover_procs = std::process::Command::new("pgrep")
                .args(["-f", &out_dir.to_string_lossy()])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
                .unwrap_or(0);
            outcome = format!("{} · processes still running: {}", outcome, leftover_procs);
        }
        let fmt = |o: Option<f64>| o.map(|x| format!("{:.1}s", x)).unwrap_or("-".into());
        let media = if file.is_empty() {
            String::new()
        } else {
            probe(&file)
        };
        let exists = !file.is_empty() && Path::new(&file).exists();
        let line = format!(
            "{label}: {outcome} | started {} · metadata {} · first% {} · max gap {:.1}s · backwards {} | {} | file {} [{}]\n    {}",
            fmt(started),
            fmt(metadata),
            fmt(first_progress),
            max_gap,
            backwards,
            statuses.join("→"),
            if exists { "ok" } else { "MISSING" },
            media,
            timeline.join(" · ")
        );
        println!("E2E {}", line);
        summary.push(line);
    }
    // Everything left in the output dir: finished files plus any temp/partial leftovers.
    let files: Vec<String> = std::fs::read_dir(&out_dir)
        .map(|d| {
            d.flatten()
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    println!("E2E files in output dir: {:?}", files);
    let _ = std::fs::remove_dir_all(&out_dir);
}
