//! Native, login-free fallback extraction for single Instagram posts/reels.
//!
//! yt-dlp's Instagram extractor currently fails with an HTTP 400 signature
//! (yt-dlp issues #13626 / #16311 — see `docs/PLATFORM-HEALTH.md` runbook
//! step 3). This module ports the *technique*, not the code, of
//! `instaloader/instaloader`'s endpoint-format-fallback approach
//! (`instaloadercontext.py` `get_json()` / `default_iphone_headers()`,
//! PR #2706; analysis: Vault `13_Sources/Repos/instaloader-instaloader.md`,
//! founder-approved 2026-08-10): try an ordered list of Instagram endpoint
//! shapes, with iPhone-app-style headers, until one yields a direct video
//! URL.
//!
//! Scope is intentionally narrow (legal positioning 2026-07-16): **single
//! public posts/reels only** — no bulk fetching, no private content, no
//! profile scraping. There is no session/login here at all; every request
//! is anonymous, so login-walled or private content simply fails fatally.

use std::fmt;
use std::time::Duration;

/// iPhone-app User-Agent, matching the shape Instagram's own app sends
/// (see instaloader's `default_iphone_headers()`).
const IPHONE_USER_AGENT: &str =
    "Instagram 309.0.0.20.115 (iPhone14,3; iOS 17_4; en_US; en-US; scale=3.00; 1284x2778; 566217994) AppleWebKit/420+";

/// Public-web X-IG-App-ID constant used by the `www.instagram.com` endpoint
/// family (task spec 2026-08-17).
const IG_APP_ID: &str = "936619743392459";

/// GraphQL `doc_id` for `xdt_api__v1__media__shortcode__web_info`, the
/// modern replacement for the old `query_hash`-based `shortcode_media`
/// query. Sourced from instaloader `structures.py` (`Post._obtain_metadata`,
/// master branch, read 2026-08-17) — the same doc_id Instagram's own web
/// client uses to fetch a single post by shortcode.
const GRAPHQL_DOC_ID: &str = "27128499623469141";

/// Retry-vs-fatal error hierarchy for the fallback pipeline.
///
/// - `Transient`: rate-limited, a timeout, a 5xx, or an endpoint shape that
///   didn't pan out (wrong JSON shape, no video field). Worth trying the
///   next endpoint format.
/// - `Fatal`: login-wall, private account, or the post is gone (404). No
///   endpoint format will fix this — surface the message to the user as-is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IgFallbackError {
    Transient(String),
    Fatal(String),
}

impl fmt::Display for IgFallbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IgFallbackError::Transient(msg) => write!(f, "transient: {}", msg),
            IgFallbackError::Fatal(msg) => write!(f, "fatal: {}", msg),
        }
    }
}

impl std::error::Error for IgFallbackError {}

/// A parsed Instagram post/reel reference: the URL "kind" segment
/// (`p` | `reel` | `reels` | `tv`) and the shortcode itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedShortcode {
    pub kind: String,
    pub shortcode: String,
}

/// Whether a download failure looks like the known Instagram HTTP-400
/// extractor breakage this fallback targets (yt-dlp #13626/#16311), so the
/// caller knows when it's worth trying the native path at all.
pub fn should_attempt_fallback(error_text: &str) -> bool {
    let lc = error_text.to_lowercase();
    (lc.contains("400") && lc.contains("bad request"))
        || lc.contains("empty media response")
        || lc.contains("requested content is not available")
}

/// Parse a single-post/reel Instagram URL into its kind + shortcode.
/// Supports `/p/`, `/reel/`, `/reels/`, `/tv/`, with or without a trailing
/// slash or query string, on both `instagram.com` and `instagr.am`.
/// Returns `None` for anything else (profile URLs, story URLs, non-IG
/// hosts, etc.) — this fallback only ever targets single posts/reels.
pub fn parse_instagram_shortcode(url: &str) -> Option<ParsedShortcode> {
    let without_scheme = url.split("://").nth(1).unwrap_or(url);
    let mut parts = without_scheme.splitn(2, '/');
    let host = parts.next()?.to_lowercase();
    let rest = parts.next()?;

    let host_ok = host.ends_with("instagram.com") || host.ends_with("instagr.am");
    if !host_ok {
        return None;
    }

    let path = rest.trim_start_matches('/');
    let mut segments = path.splitn(3, '/');
    let kind = segments.next()?.to_lowercase();
    if !matches!(kind.as_str(), "p" | "reel" | "reels" | "tv") {
        return None;
    }

    let raw_code = segments.next()?;
    let code = raw_code.split(['?', '#']).next().unwrap_or(raw_code);
    if code.is_empty() {
        return None;
    }

    Some(ParsedShortcode {
        kind,
        shortcode: code.to_string(),
    })
}

fn base64url_value(ch: char) -> Option<u8> {
    match ch {
        'A'..='Z' => Some(ch as u8 - b'A'),
        'a'..='z' => Some(ch as u8 - b'a' + 26),
        '0'..='9' => Some(ch as u8 - b'0' + 52),
        '-' => Some(62),
        '_' => Some(63),
        _ => None,
    }
}

/// Convert an Instagram shortcode to its numeric media id, following the
/// same algorithm as instaloader's `Post.shortcode_to_mediaid`: left-pad the
/// shortcode with `'A'` to 12 characters, base64url-decode (no padding
/// character, none needed at exactly 12 chars -> 9 bytes), and interpret the
/// 9 decoded bytes as a big-endian integer. Shortcodes longer than 11
/// characters (carousel sub-item shortcodes) are not supported by this
/// conversion — same limitation instaloader has — and return `None`.
pub fn shortcode_to_media_id(shortcode: &str) -> Option<u64> {
    if shortcode.is_empty() || shortcode.len() > 11 || !shortcode.is_ascii() {
        return None;
    }

    let mut padded = String::with_capacity(12);
    for _ in 0..(12 - shortcode.len()) {
        padded.push('A');
    }
    padded.push_str(shortcode);

    let mut bits: u32 = 0;
    let mut bit_count: u32 = 0;
    let mut bytes: Vec<u8> = Vec::with_capacity(9);
    for ch in padded.chars() {
        let value = base64url_value(ch)?;
        bits = (bits << 6) | value as u32;
        bit_count += 6;
        if bit_count >= 8 {
            bit_count -= 8;
            bytes.push(((bits >> bit_count) & 0xFF) as u8);
        }
    }

    if bytes.len() != 9 || bytes[0] != 0 {
        // A real media id fits in 64 bits, so the leading byte of the 9-byte
        // big-endian value must be zero.
        return None;
    }

    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[1..9]);
    Some(u64::from_be_bytes(buf))
}

/// One endpoint shape to try, in order.
struct EndpointCandidate {
    label: &'static str,
    url: String,
    /// `Some(form_fields)` for a POST with form-encoded body; `None` for GET.
    form: Option<Vec<(&'static str, String)>>,
}

/// Build the ordered list of endpoint candidates for a parsed shortcode.
/// Endpoint (b) (the private-API media-id lookup) is only included when the
/// shortcode fits the 11-char conversion (true for virtually all real posts
/// and reels).
fn build_endpoint_candidates(parsed: &ParsedShortcode) -> Vec<EndpointCandidate> {
    let mut endpoints = Vec::with_capacity(3);

    // (a) Legacy AJAX-style post page endpoint.
    endpoints.push(EndpointCandidate {
        label: "www __a=1",
        url: format!(
            "https://www.instagram.com/{}/{}/?__a=1&__d=dis",
            parsed.kind, parsed.shortcode
        ),
        form: None,
    });

    // (b) Private API media-info endpoint, keyed by numeric media id.
    if let Some(media_id) = shortcode_to_media_id(&parsed.shortcode) {
        endpoints.push(EndpointCandidate {
            label: "i.instagram.com media info",
            url: format!("https://i.instagram.com/api/v1/media/{}/info/", media_id),
            form: None,
        });
    }

    // (c) GraphQL doc_id query — the modern web client's own endpoint for
    // fetching a single post by shortcode. Last resort: it is the most
    // likely to be rate-limited/blocked for a non-browser client.
    let variables = format!(
        "{{\"shortcode\":\"{}\",\"__relay_internal__pv__PolarisAIGMMediaWebLabelEnabledrelayprovider\":false}}",
        parsed.shortcode
    );
    endpoints.push(EndpointCandidate {
        label: "graphql doc_id",
        url: "https://www.instagram.com/graphql/query/".to_string(),
        form: Some(vec![
            ("doc_id", GRAPHQL_DOC_ID.to_string()),
            ("variables", variables),
        ]),
    });

    endpoints
}

/// Randomized backoff before each request (400-1200ms), so the fallback
/// doesn't hammer Instagram with back-to-back requests across endpoint
/// shapes. Seeded from the current time — good enough for jitter, no `rand`
/// dependency needed.
fn random_backoff_ms(min_ms: u64, max_ms: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    let span = max_ms.saturating_sub(min_ms).max(1);
    min_ms + (nanos % span)
}

fn backoff_before_request() {
    let ms = random_backoff_ms(400, 1200);
    std::thread::sleep(Duration::from_millis(ms));
}

fn build_headers() -> reqwest::header::HeaderMap {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static(IPHONE_USER_AGENT),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("*/*"),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9"),
    );
    if let Ok(name) = HeaderName::from_bytes(b"X-IG-App-ID") {
        headers.insert(name, HeaderValue::from_static(IG_APP_ID));
    }
    headers
}

/// Classify a completed HTTP response into a JSON payload or a typed error,
/// mirroring instaloader's `get_json()` status-code handling (400 -> bad
/// request, 404 -> not found, 429 -> too-many-requests, redirect-to-login ->
/// login required).
fn classify_response(
    status: reqwest::StatusCode,
    body: &str,
    label: &str,
) -> Result<serde_json::Value, IgFallbackError> {
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(IgFallbackError::Fatal(
            "This Instagram post appears to be unavailable or has been deleted.".to_string(),
        ));
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        return Err(IgFallbackError::Fatal(
            "This Instagram content requires a login and can't be downloaded anonymously."
                .to_string(),
        ));
    }
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(IgFallbackError::Transient(format!(
            "{}: rate-limited (429)",
            label
        )));
    }
    if status.as_u16() == 400 {
        let lc = body.to_lowercase();
        if lc.contains("feedback_required")
            || lc.contains("checkpoint_required")
            || lc.contains("challenge_required")
            || lc.contains("login_required")
        {
            return Err(IgFallbackError::Fatal(
                "Instagram is asking for a login/checkpoint to view this content — it can't be downloaded anonymously.".to_string(),
            ));
        }
        // A plain 400 with no login/checkpoint marker is the same shape as
        // the yt-dlp breakage this fallback exists to route around — treat
        // this endpoint format as a miss and let the caller try the next.
        return Err(IgFallbackError::Transient(format!(
            "{}: HTTP 400 (no video payload)",
            label
        )));
    }
    if !status.is_success() {
        return Err(IgFallbackError::Transient(format!(
            "{}: HTTP {}",
            label,
            status.as_u16()
        )));
    }

    serde_json::from_str::<serde_json::Value>(body).map_err(|_| {
        IgFallbackError::Transient(format!(
            "{}: response was not JSON (likely an HTML login/interstitial page)",
            label
        ))
    })
}

fn fetch_json_from_endpoint(
    client: &reqwest::blocking::Client,
    endpoint: &EndpointCandidate,
) -> Result<serde_json::Value, IgFallbackError> {
    let response = if let Some(form) = &endpoint.form {
        client
            .post(&endpoint.url)
            .headers(build_headers())
            .form(form)
            .send()
    } else {
        client.get(&endpoint.url).headers(build_headers()).send()
    };

    let response = response.map_err(|e| {
        if e.is_timeout() {
            IgFallbackError::Transient(format!("{}: request timed out ({})", endpoint.label, e))
        } else {
            IgFallbackError::Transient(format!("{}: network error ({})", endpoint.label, e))
        }
    })?;

    let status = response.status();
    let body = response
        .text()
        .map_err(|e| IgFallbackError::Transient(format!("{}: failed to read body ({})", endpoint.label, e)))?;

    classify_response(status, &body, endpoint.label)
}

/// Pick the best (highest-resolution) `url` out of an Instagram
/// `video_versions` array, which looks like:
/// `[{"url": "...", "width": 1080, "height": 1920}, ...]`.
fn best_from_video_versions(video_versions: &serde_json::Value) -> Option<String> {
    let arr = video_versions.as_array()?;
    arr.iter()
        .filter_map(|v| {
            let url = v.get("url")?.as_str()?.to_string();
            let width = v.get("width").and_then(|w| w.as_u64()).unwrap_or(0);
            let height = v.get("height").and_then(|h| h.as_u64()).unwrap_or(0);
            Some((width * height, url))
        })
        .max_by_key(|(area, _)| *area)
        .map(|(_, url)| url)
}

/// Pull the best available video URL out of a private-API-shaped `item`
/// object (used directly by endpoint (b), and nested under
/// `data.xdt_api__v1__media__shortcode__web_info.items[0]` for endpoint (c)).
/// Handles single-video items and video items inside a carousel.
fn best_from_item(item: &serde_json::Value) -> Option<String> {
    if let Some(video_versions) = item.get("video_versions") {
        if let Some(url) = best_from_video_versions(video_versions) {
            return Some(url);
        }
    }
    if let Some(carousel) = item.get("carousel_media").and_then(|c| c.as_array()) {
        for media in carousel {
            if let Some(video_versions) = media.get("video_versions") {
                if let Some(url) = best_from_video_versions(video_versions) {
                    return Some(url);
                }
            }
        }
    }
    None
}

/// Extract the best video URL out of any of the three supported endpoint
/// response shapes. Returns `None` when the JSON parsed fine but carries no
/// usable video (wrong shape, or a genuinely photo-only post) — the caller
/// treats that as "try the next endpoint format".
pub fn extract_best_video_url(json: &serde_json::Value) -> Option<String> {
    // Shape (a) — legacy `graphql.shortcode_media`.
    if let Some(media) = json.get("graphql").and_then(|g| g.get("shortcode_media")) {
        if let Some(url) = media.get("video_url").and_then(|v| v.as_str()) {
            return Some(url.to_string());
        }
        if let Some(edges) = media
            .get("edge_sidecar_to_children")
            .and_then(|e| e.get("edges"))
            .and_then(|e| e.as_array())
        {
            for edge in edges {
                if let Some(url) = edge
                    .get("node")
                    .and_then(|n| n.get("video_url"))
                    .and_then(|v| v.as_str())
                {
                    return Some(url.to_string());
                }
            }
        }
    }

    // Shape (c) — modern GraphQL doc_id response.
    if let Some(items) = json
        .get("data")
        .and_then(|d| d.get("xdt_api__v1__media__shortcode__web_info"))
        .and_then(|w| w.get("items"))
        .and_then(|i| i.as_array())
    {
        if let Some(first) = items.first() {
            if let Some(url) = best_from_item(first) {
                return Some(url);
            }
        }
    }

    // Shape (b) — private API media-info (`{"items": [...]}` at the top level).
    if let Some(items) = json.get("items").and_then(|i| i.as_array()) {
        if let Some(first) = items.first() {
            if let Some(url) = best_from_item(first) {
                return Some(url);
            }
        }
    }

    // Top-level fallback some endpoints use directly.
    if let Some(url) = json.get("video_url").and_then(|v| v.as_str()) {
        return Some(url.to_string());
    }

    None
}

/// Whether the parsed JSON positively confirms this is a photo-only post
/// (no video anywhere), as opposed to just an endpoint shape we couldn't
/// parse. Used to give a clear, honest fatal message instead of a generic
/// "no video URL found" after exhausting every endpoint.
pub fn post_is_definitely_not_a_video(json: &serde_json::Value) -> bool {
    if let Some(media) = json.get("graphql").and_then(|g| g.get("shortcode_media")) {
        if let Some(is_video) = media.get("is_video").and_then(|v| v.as_bool()) {
            if !is_video && media.get("edge_sidecar_to_children").is_none() {
                return true;
            }
        }
    }
    if let Some(item) = json
        .get("items")
        .and_then(|i| i.as_array())
        .and_then(|a| a.first())
    {
        // media_type: 1 = photo, 2 = video, 8 = carousel.
        if item.get("media_type").and_then(|m| m.as_u64()) == Some(1) {
            return true;
        }
    }
    false
}

/// Run the endpoint-format fallback for a single Instagram post/reel URL and
/// return the best direct video URL found. Anonymous, single-post only — no
/// cookies, no session, no bulk fetching.
pub fn fetch_instagram_direct_video_url(url: &str) -> Result<String, IgFallbackError> {
    let parsed = parse_instagram_shortcode(url).ok_or_else(|| {
        IgFallbackError::Fatal(
            "Could not find an Instagram post/reel shortcode in this URL.".to_string(),
        )
    })?;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(20))
        // Instagram-bound requests only — never route through a system
        // HTTP/SOCKS proxy. reqwest auto-detects the OS proxy config by
        // default, and a slow/misconfigured system proxy silently turns
        // every request into a full-timeout hang instead of a fast
        // success/failure, which defeats the point of the endpoint-format
        // fallback (try the next shape quickly).
        .no_proxy()
        .build()
        .map_err(|e| {
            IgFallbackError::Transient(format!("Could not create an HTTP client: {}", e))
        })?;

    let endpoints = build_endpoint_candidates(&parsed);

    let mut last_err = IgFallbackError::Transient(
        "Instagram did not return a usable video from any known endpoint.".to_string(),
    );
    let mut saw_photo_only = false;

    for endpoint in &endpoints {
        backoff_before_request();
        match fetch_json_from_endpoint(&client, endpoint) {
            Ok(json) => match extract_best_video_url(&json) {
                Some(video_url) => return Ok(video_url),
                None => {
                    if post_is_definitely_not_a_video(&json) {
                        saw_photo_only = true;
                    }
                    last_err = IgFallbackError::Transient(format!(
                        "{}: JSON parsed but no video field present",
                        endpoint.label
                    ));
                }
            },
            Err(IgFallbackError::Fatal(msg)) => return Err(IgFallbackError::Fatal(msg)),
            Err(err @ IgFallbackError::Transient(_)) => {
                last_err = err;
            }
        }
    }

    if saw_photo_only {
        return Err(IgFallbackError::Fatal(
            "This Instagram post doesn't contain a video (photo-only post) — nothing to download."
                .to_string(),
        ));
    }
    Err(last_err)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- shortcode parsing -------------------------------------------------

    #[test]
    fn parses_post_url_with_trailing_slash() {
        let parsed = parse_instagram_shortcode("https://www.instagram.com/p/CqzZ0HwI9bA/").unwrap();
        assert_eq!(parsed.kind, "p");
        assert_eq!(parsed.shortcode, "CqzZ0HwI9bA");
    }

    #[test]
    fn parses_reel_url_without_trailing_slash() {
        let parsed = parse_instagram_shortcode("https://www.instagram.com/reel/AbC123_45-6").unwrap();
        assert_eq!(parsed.kind, "reel");
        assert_eq!(parsed.shortcode, "AbC123_45-6");
    }

    #[test]
    fn parses_reels_and_tv_kinds() {
        assert_eq!(
            parse_instagram_shortcode("https://www.instagram.com/reels/XyZ9988/").unwrap().kind,
            "reels"
        );
        assert_eq!(
            parse_instagram_shortcode("https://www.instagram.com/tv/QwErTy1/").unwrap().kind,
            "tv"
        );
    }

    #[test]
    fn parses_url_with_query_string() {
        let parsed = parse_instagram_shortcode(
            "https://www.instagram.com/p/CqzZ0HwI9bA/?utm_source=ig_web_copy_link",
        )
        .unwrap();
        assert_eq!(parsed.shortcode, "CqzZ0HwI9bA");
    }

    #[test]
    fn parses_url_with_query_string_no_trailing_slash() {
        let parsed =
            parse_instagram_shortcode("https://www.instagram.com/p/CqzZ0HwI9bA?taken-by=nasa")
                .unwrap();
        assert_eq!(parsed.shortcode, "CqzZ0HwI9bA");
    }

    #[test]
    fn parses_instagr_am_short_domain() {
        let parsed = parse_instagram_shortcode("https://instagr.am/p/CqzZ0HwI9bA/").unwrap();
        assert_eq!(parsed.shortcode, "CqzZ0HwI9bA");
    }

    #[test]
    fn rejects_non_instagram_host() {
        assert!(parse_instagram_shortcode("https://www.example.com/p/CqzZ0HwI9bA/").is_none());
    }

    #[test]
    fn rejects_profile_urls() {
        // No /p//reel//reels//tv/ segment — a profile, not a single post.
        assert!(parse_instagram_shortcode("https://www.instagram.com/nasa/").is_none());
    }

    #[test]
    fn rejects_story_urls() {
        assert!(parse_instagram_shortcode("https://www.instagram.com/stories/nasa/12345/").is_none());
    }

    // ---- shortcode -> media id ----------------------------------------------

    #[test]
    fn converts_known_shortcode_to_media_id() {
        // Verified against instaloader's Post.shortcode_to_mediaid algorithm
        // (pad to 12 chars with leading 'A', base64url-decode, big-endian
        // int) run independently in Python for this exact shortcode.
        assert_eq!(shortcode_to_media_id("CqzZ0HwI9bA"), Some(3_076_916_503_323_596_480));
    }

    #[test]
    fn rejects_shortcode_longer_than_eleven_chars() {
        assert_eq!(shortcode_to_media_id("ThisIsWayTooLongForAShortcode"), None);
    }

    #[test]
    fn rejects_shortcode_with_invalid_characters() {
        assert_eq!(shortcode_to_media_id("has space!"), None);
    }

    // ---- JSON extraction: endpoint (a), legacy graphql shape ---------------

    #[test]
    fn extracts_video_url_from_legacy_graphql_shape() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"graphql":{"shortcode_media":{"is_video":true,"video_url":"https://cdn.example.com/a.mp4"}}}"#,
        )
        .unwrap();
        assert_eq!(
            extract_best_video_url(&json),
            Some("https://cdn.example.com/a.mp4".to_string())
        );
    }

    #[test]
    fn extracts_video_from_legacy_graphql_carousel() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"graphql":{"shortcode_media":{"is_video":false,
                "edge_sidecar_to_children":{"edges":[
                    {"node":{"is_video":false}},
                    {"node":{"is_video":true,"video_url":"https://cdn.example.com/carousel2.mp4"}}
                ]}}}}"#,
        )
        .unwrap();
        assert_eq!(
            extract_best_video_url(&json),
            Some("https://cdn.example.com/carousel2.mp4".to_string())
        );
    }

    #[test]
    fn detects_photo_only_legacy_graphql_post() {
        let json: serde_json::Value =
            serde_json::from_str(r#"{"graphql":{"shortcode_media":{"is_video":false}}}"#).unwrap();
        assert_eq!(extract_best_video_url(&json), None);
        assert!(post_is_definitely_not_a_video(&json));
    }

    // ---- JSON extraction: endpoint (b), private-API media info -------------

    #[test]
    fn extracts_highest_resolution_from_private_api_shape() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"items":[{"media_type":2,"video_versions":[
                {"url":"https://cdn.example.com/low.mp4","width":480,"height":852},
                {"url":"https://cdn.example.com/high.mp4","width":1080,"height":1920}
            ]}]}"#,
        )
        .unwrap();
        assert_eq!(
            extract_best_video_url(&json),
            Some("https://cdn.example.com/high.mp4".to_string())
        );
    }

    #[test]
    fn extracts_video_from_private_api_carousel_item() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"items":[{"media_type":8,"carousel_media":[
                {"media_type":1},
                {"media_type":2,"video_versions":[{"url":"https://cdn.example.com/c2.mp4","width":720,"height":1280}]}
            ]}]}"#,
        )
        .unwrap();
        assert_eq!(
            extract_best_video_url(&json),
            Some("https://cdn.example.com/c2.mp4".to_string())
        );
    }

    #[test]
    fn detects_photo_only_private_api_post() {
        let json: serde_json::Value =
            serde_json::from_str(r#"{"items":[{"media_type":1}]}"#).unwrap();
        assert_eq!(extract_best_video_url(&json), None);
        assert!(post_is_definitely_not_a_video(&json));
    }

    // ---- JSON extraction: endpoint (c), modern graphql doc_id shape --------

    #[test]
    fn extracts_video_from_graphql_doc_id_shape() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"data":{"xdt_api__v1__media__shortcode__web_info":{"items":[
                {"media_type":2,"video_versions":[{"url":"https://cdn.example.com/docid.mp4","width":1080,"height":1920}]}
            ]}}}"#,
        )
        .unwrap();
        assert_eq!(
            extract_best_video_url(&json),
            Some("https://cdn.example.com/docid.mp4".to_string())
        );
    }

    // ---- classification / no-video shapes -----------------------------------

    #[test]
    fn returns_none_for_unrelated_json() {
        let json: serde_json::Value = serde_json::from_str(r#"{"unexpected":"shape"}"#).unwrap();
        assert_eq!(extract_best_video_url(&json), None);
        assert!(!post_is_definitely_not_a_video(&json));
    }

    // ---- should_attempt_fallback --------------------------------------------

    #[test]
    fn recognizes_the_known_instagram_400_signature() {
        assert!(should_attempt_fallback(
            "ERROR: [Instagram] abc123: Unable to download JSON metadata: HTTP Error 400: Bad Request"
        ));
    }

    #[test]
    fn ignores_unrelated_errors() {
        assert!(!should_attempt_fallback("HTTP Error 403: Forbidden"));
        assert!(!should_attempt_fallback("Video unavailable"));
    }

    // ---- endpoint candidate construction ------------------------------------

    #[test]
    fn builds_all_three_endpoint_shapes_for_a_valid_shortcode() {
        let parsed = ParsedShortcode {
            kind: "p".to_string(),
            shortcode: "CqzZ0HwI9bA".to_string(),
        };
        let endpoints = build_endpoint_candidates(&parsed);
        assert_eq!(endpoints.len(), 3);
        assert!(endpoints[0].url.contains("__a=1"));
        assert!(endpoints[1].url.contains("i.instagram.com/api/v1/media/"));
        assert!(endpoints[2].url.contains("graphql/query"));
        assert!(endpoints[2].form.is_some());
    }

    // ---- backoff --------------------------------------------------------------

    #[test]
    fn random_backoff_stays_within_bounds() {
        for _ in 0..20 {
            let ms = random_backoff_ms(400, 1200);
            assert!((400..=1200).contains(&ms), "backoff {} out of range", ms);
        }
    }

    // ---- live test (network, opt-in only) ------------------------------------
    //
    // Run explicitly with: cargo test -- --ignored
    // Hits the real Instagram endpoints for the probe post from
    // docs/PLATFORM-HEALTH.md and a public reel. Prints the outcome —
    // Instagram may block a datacenter-shaped client, in which case this is
    // expected to return an error, not a video URL; the point is to record
    // what actually happens, honestly.
    #[test]
    #[ignore]
    fn live_fallback_against_real_instagram_posts() {
        let probe_post = "https://www.instagram.com/p/CqzZ0HwI9bA/";
        let probe_reel = "https://www.instagram.com/reel/C1z0j9nIQwB/";

        for url in [probe_post, probe_reel] {
            match fetch_instagram_direct_video_url(url) {
                Ok(video_url) => {
                    println!("LIVE PASS [{}] -> {}", url, video_url);
                }
                Err(err) => {
                    println!("LIVE RESULT (not a video URL) [{}] -> {}", url, err);
                }
            }
        }
    }
}
