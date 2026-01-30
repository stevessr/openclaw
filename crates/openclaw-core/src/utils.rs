/// Utility functions
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a unique ID (simple implementation)
pub fn generate_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", timestamp)
}

/// Sanitize string for logging
pub fn sanitize_for_log(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// Validate phone number format
pub fn is_valid_phone(phone: &str) -> bool {
    phone.starts_with('+') && phone.len() > 10 && phone[1..].chars().all(|c| c.is_numeric())
}

/// Format duration in human-readable form
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id1 = generate_id();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id2 = generate_id();
        assert_ne!(id1, id2);
        assert!(!id1.is_empty());
    }

    #[test]
    fn test_sanitize_for_log() {
        assert_eq!(sanitize_for_log("hello", 10), "hello");
        assert_eq!(sanitize_for_log("hello world!", 5), "hello...");
    }

    #[test]
    fn test_is_valid_phone() {
        assert!(is_valid_phone("+1234567890"));
        assert!(is_valid_phone("+8613800138000"));
        assert!(!is_valid_phone("1234567890"));
        assert!(!is_valid_phone("+123abc"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
    }
}
