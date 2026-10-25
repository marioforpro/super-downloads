//! Anonymous daily "this install is active" ping.
//!
//! Once a day, when the user leaves «Share anonymous usage stats» on, the app
//! sends one `app_active` event to PostHog. The event carries a random install
//! id (a v4 UUID created on this Mac, not derived from anything about it), the
//! app version, the CPU architecture and the macOS major version. No email,
//! no computer name, no IP address (sent as 0.0.0.0, GeoIP off), no person
//! profile, nothing about what the user downloads.
//!
//! The frontend (src/main.js) renews the install id every 12 months and
//! deletes it when the switch is turned off.

use serde_json::{json, Value};

pub const CAPTURE_URL: &str = "https://us.i.posthog.com/i/v0/e/";

/// Public project token, the same one superdownloads.app ships in its page.
/// It can only write events, never read data.
pub const PROJECT_TOKEN: &str = "phc_5nkW8LyRDxnxAZxqxOYVqXdOqWHbZ8G9SP1cMy0iAx7";

pub const EVENT: &str = "app_active";

/// Returns the caller's install id when it is a valid UUID, otherwise a new
/// random one. Only a UUID can ever be sent as the distinct id.
pub fn install_id_or_new(candidate: Option<&str>) -> String {
    candidate
        .and_then(|c| uuid::Uuid::parse_str(c.trim()).ok())
        .unwrap_or_else(uuid::Uuid::new_v4)
        .to_string()
}

/// "15.6.1" → "15"; anything unexpected → "unknown".
pub fn macos_major(os_version: &str) -> String {
    match os_version.trim().split('.').next() {
        Some(major) if !major.is_empty() && major.chars().all(|c| c.is_ascii_digit()) => {
            major.to_string()
        }
        _ => "unknown".to_string(),
    }
}

pub fn body(install_id: &str, app_version: &str, arch: &str, os_version: &str) -> Value {
    json!({
        "api_key": PROJECT_TOKEN,
        "event": EVENT,
        "distinct_id": install_id,
        "properties": {
            "product": "super-downloads",
            "version": app_version,
            "arch": arch,
            "macos_major": macos_major(os_version),
            "$process_person_profile": false,
            // PostHog stores the request IP unless an explicit $ip is sent
            // (null or "" are ignored — measured 2026-10-25).
            "$ip": "0.0.0.0",
            "$geoip_disable": true,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_a_valid_install_id() {
        let id = "3f2b8c1e-9a4d-4e6f-8b7a-1c2d3e4f5a6b";
        assert_eq!(install_id_or_new(Some(id)), id);
    }

    #[test]
    fn replaces_anything_that_is_not_a_uuid() {
        for bad in [
            Some("someone@example.com"),
            Some(""),
            Some("Mac-Studio"),
            None,
        ] {
            let id = install_id_or_new(bad);
            assert!(uuid::Uuid::parse_str(&id).is_ok(), "{bad:?} → {id}");
            assert_ne!(Some(id.as_str()), bad);
        }
    }

    #[test]
    fn macos_major_only() {
        assert_eq!(macos_major("15.6.1"), "15");
        assert_eq!(macos_major("10.15.7"), "10");
        assert_eq!(macos_major("unknown"), "unknown");
        assert_eq!(macos_major(""), "unknown");
    }

    #[test]
    fn body_is_anonymous() {
        let b = body(
            "3f2b8c1e-9a4d-4e6f-8b7a-1c2d3e4f5a6b",
            "1.5.5",
            "aarch64",
            "15.6.1",
        );
        assert_eq!(b["event"], EVENT);
        assert_eq!(b["distinct_id"], "3f2b8c1e-9a4d-4e6f-8b7a-1c2d3e4f5a6b");
        let p = b["properties"].as_object().unwrap();
        assert_eq!(p["$process_person_profile"], false);
        assert_eq!(p["$ip"], "0.0.0.0");
        assert_eq!(p["$geoip_disable"], true);
        assert_eq!(p["macos_major"], "15");
        // Exactly these properties, so nothing personal can slip in later unnoticed.
        let mut keys: Vec<_> = p.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "$geoip_disable",
                "$ip",
                "$process_person_profile",
                "arch",
                "macos_major",
                "product",
                "version"
            ]
        );
    }
}
