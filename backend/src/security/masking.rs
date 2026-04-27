//! PII masking utilities (TM-04)
//!
//! Masks email addresses, IP addresses, and API tokens in log output
//! and API responses to comply with AVG/GDPR requirements.

use regex::Regex;
use std::sync::LazyLock;

// Pre-compiled regex patterns for PII detection
#[allow(clippy::unwrap_used)]
static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap());

#[allow(clippy::unwrap_used)]
static IP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{1,3})\.(\d{1,3})\.\d{1,3}\.\d{1,3}\b").unwrap());

#[allow(clippy::unwrap_used)]
static TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(ghp_|gho_|ghu_|ghs_|ghr_|ATCTT|ATATT|Bearer\s)[a-zA-Z0-9_\-]{4,}").unwrap()
});

/// Mask an email address for display to non-admin users.
///
/// john.doe@equans.com -> j***@e***.com
pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let local = &email[..at_pos];
        let domain = &email[at_pos + 1..];

        let masked_local = if local.is_empty() {
            "***".to_string()
        } else {
            format!("{}***", &local[..1])
        };

        let masked_domain = if let Some(dot_pos) = domain.rfind('.') {
            let domain_name = &domain[..dot_pos];
            let tld = &domain[dot_pos..];
            if domain_name.is_empty() {
                format!("***{}", tld)
            } else {
                format!("{}***{}", &domain_name[..1], tld)
            }
        } else {
            "***".to_string()
        };

        format!("{}@{}", masked_local, masked_domain)
    } else {
        "***".to_string()
    }
}

/// Mask an IP address for logging.
///
/// 192.168.1.100 -> 192.168.x.x
pub fn mask_ip(ip: &str) -> String {
    IP_REGEX
        .replace_all(ip, |caps: &regex::Captures| {
            format!("{}.{}.x.x", &caps[1], &caps[2])
        })
        .to_string()
}

/// Mask an API token for logging.
///
/// ghp_abc123def456 -> ghp_***
pub fn mask_token(token: &str) -> String {
    TOKEN_REGEX
        .replace_all(token, |caps: &regex::Captures| {
            let prefix = &caps[1];
            format!("{}***", prefix)
        })
        .to_string()
}

/// Sanitize a string by masking all PII patterns (emails, IPs, tokens).
/// Used for log output sanitization.
pub fn sanitize_for_logging(input: &str) -> String {
    let result = EMAIL_REGEX
        .replace_all(input, |caps: &regex::Captures| mask_email(&caps[0]))
        .to_string();

    let result = IP_REGEX
        .replace_all(&result, |caps: &regex::Captures| mask_ip(&caps[0]))
        .to_string();

    TOKEN_REGEX
        .replace_all(&result, |caps: &regex::Captures| mask_token(&caps[0]))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("john.doe@equans.com"), "j***@e***.com");
        assert_eq!(mask_email("a@b.nl"), "a***@b***.nl");
        assert_eq!(mask_email("test@example.org"), "t***@e***.org");
    }

    #[test]
    fn test_mask_ip() {
        assert_eq!(mask_ip("192.168.1.100"), "192.168.x.x");
        assert_eq!(mask_ip("10.0.0.1"), "10.0.x.x");
    }

    #[test]
    fn test_mask_token() {
        assert_eq!(mask_token("ghp_abc123def456"), "ghp_***");
        assert_eq!(mask_token("ATCTTabc123"), "ATCTT***");
    }

    #[test]
    fn test_sanitize_for_logging() {
        let input = "User john.doe@equans.com from 192.168.1.100 with token ghp_abc123";
        let result = sanitize_for_logging(input);
        assert!(!result.contains("john.doe@equans.com"));
        assert!(!result.contains("192.168.1.100"));
        assert!(!result.contains("ghp_abc123"));
        assert!(result.contains("j***@e***.com"));
        assert!(result.contains("x.x"));
        assert!(result.contains("ghp_***"));
    }
}
