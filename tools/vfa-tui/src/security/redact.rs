use std::ffi::OsString;

/// Check if an environment variable name indicates it contains a secret value.
/// Case-insensitive match for exact names and substring patterns.
/// For `_KEY`, only matches when the name ends with `_KEY` or contains `_KEY_`
/// to avoid false positives on names like `KEYBOARD_LAYOUT` or `KEYRING_CONTROL`.
pub fn is_secret_env_var(name: &str) -> bool {
    let upper = name.to_uppercase();

    // Exact matches
    if upper == "AWS_SECRET_ACCESS_KEY" || upper == "GITHUB_TOKEN" || upper == "NPM_TOKEN" {
        return true;
    }

    // Substring matches (these are unambiguous)
    if upper.contains("_SECRET")
        || upper.contains("_TOKEN")
        || upper.contains("_PASSWORD")
        || upper.contains("_CREDENTIAL")
    {
        return true;
    }

    // _KEY: only match as suffix or when followed by _ (e.g., _KEY_ID)
    // This catches API_KEY, SECRET_KEY, ACCESS_KEY_ID but not KEYBOARD, KEYRING
    if upper.ends_with("_KEY") || upper.contains("_KEY_") {
        return true;
    }

    false
}

/// Redact known secret patterns in a string, replacing them with `[REDACTED]`.
///
/// Patterns matched:
/// - `ghp_` followed by 36+ alphanumeric characters (GitHub PAT)
/// - `npm_` followed by 36+ characters (npm token)
/// - `sk-` followed by 20+ characters (API key)
/// - `AKIA` followed by 16+ uppercase alphanumeric characters (AWS access key ID)
/// - Base64-like strings >40 contiguous characters from `[A-Za-z0-9+/=]`
pub fn redact_secrets(input: &str) -> String {
    // First pass: identify spans to redact (start, end) using pattern matching.
    let mut redactions: Vec<(usize, usize)> = Vec::new();

    // Find ghp_ tokens
    find_prefixed_spans(input, "ghp_", 36, is_alnum, &mut redactions);
    // Find npm_ tokens
    find_prefixed_spans(input, "npm_", 36, is_npm_token_char, &mut redactions);
    // Find sk- keys
    find_prefixed_spans(input, "sk-", 20, is_sk_char, &mut redactions);
    // Find AKIA keys
    find_akia_spans(input, &mut redactions);
    // Find base64 spans >40 chars
    find_base64_spans(input, &mut redactions);

    if redactions.is_empty() {
        return input.to_string();
    }

    // Sort by start position and merge overlapping spans
    redactions.sort_by_key(|&(start, _)| start);
    let merged = merge_spans(&redactions);

    // Build result
    let mut result = String::with_capacity(input.len());
    let mut pos = 0;
    for (start, end) in merged {
        if pos < start {
            result.push_str(&input[pos..start]);
        }
        result.push_str("[REDACTED]");
        pos = end;
    }
    if pos < input.len() {
        result.push_str(&input[pos..]);
    }

    result
}

fn is_alnum(c: u8) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_npm_token_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-'
}

fn is_sk_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_'
}

fn is_base64_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'+' || c == b'/' || c == b'='
}

fn find_prefixed_spans(
    input: &str,
    prefix: &str,
    min_suffix_len: usize,
    is_valid: fn(u8) -> bool,
    spans: &mut Vec<(usize, usize)>,
) {
    let bytes = input.as_bytes();
    let prefix_bytes = prefix.as_bytes();
    let prefix_len = prefix_bytes.len();

    let mut start = 0;
    while let Some(pos) = find_substr(bytes, prefix_bytes, start) {
        let suffix_start = pos + prefix_len;
        let mut end = suffix_start;
        while end < bytes.len() && is_valid(bytes[end]) {
            end += 1;
        }
        if end - suffix_start >= min_suffix_len {
            spans.push((pos, end));
        }
        start = pos + 1;
    }
}

fn find_akia_spans(input: &str, spans: &mut Vec<(usize, usize)>) {
    let bytes = input.as_bytes();
    let prefix = b"AKIA";

    let mut start = 0;
    while let Some(pos) = find_substr(bytes, prefix, start) {
        let suffix_start = pos + 4;
        let mut end = suffix_start;
        while end < bytes.len() && (bytes[end].is_ascii_uppercase() || bytes[end].is_ascii_digit())
        {
            end += 1;
        }
        if end - suffix_start >= 16 {
            spans.push((pos, end));
        }
        start = pos + 1;
    }
}

fn find_base64_spans(input: &str, spans: &mut Vec<(usize, usize)>) {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if is_base64_byte(bytes[i]) {
            let start = i;
            let mut has_base64_special = false;
            while i < bytes.len() && is_base64_byte(bytes[i]) {
                if bytes[i] == b'+' || bytes[i] == b'/' || bytes[i] == b'=' {
                    has_base64_special = true;
                }
                i += 1;
            }
            // Only treat as base64 if >40 chars AND contains at least one +, /, or =
            // This avoids false positives on pure hex strings and plain alphanumeric text
            if i - start > 40 && has_base64_special {
                spans.push((start, i));
            }
        } else {
            i += 1;
        }
    }
}

fn find_substr(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start + needle.len() > haystack.len() {
        return None;
    }
    haystack[start..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + start)
}

fn merge_spans(spans: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if spans.is_empty() {
        return Vec::new();
    }
    let mut merged = vec![spans[0]];
    for &(start, end) in &spans[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

/// Collect environment variables, filtering out entries where the key is identified as a secret.
pub fn sanitized_child_env() -> Vec<(OsString, OsString)> {
    std::env::vars_os()
        .filter(|(key, _)| {
            if let Some(name) = key.to_str() {
                !is_secret_env_var(name)
            } else {
                // Keep non-UTF-8 env var names (cannot check them)
                true
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_secret_exact_matches() {
        assert!(is_secret_env_var("AWS_SECRET_ACCESS_KEY"));
        assert!(is_secret_env_var("GITHUB_TOKEN"));
        assert!(is_secret_env_var("NPM_TOKEN"));
    }

    #[test]
    fn is_secret_case_insensitive() {
        assert!(is_secret_env_var("aws_secret_access_key"));
        assert!(is_secret_env_var("github_token"));
        assert!(is_secret_env_var("Npm_Token"));
    }

    #[test]
    fn is_secret_substring_matches() {
        assert!(is_secret_env_var("MY_SECRET_VALUE"));
        assert!(is_secret_env_var("API_KEY"));
        assert!(is_secret_env_var("AUTH_TOKEN"));
        assert!(is_secret_env_var("DB_PASSWORD"));
        assert!(is_secret_env_var("CLOUD_CREDENTIAL"));
        assert!(is_secret_env_var("ACCESS_KEY_ID"));
        assert!(is_secret_env_var("SECRET_KEY"));
    }

    #[test]
    fn is_secret_non_secret() {
        assert!(!is_secret_env_var("PATH"));
        assert!(!is_secret_env_var("HOME"));
        assert!(!is_secret_env_var("LANG"));
        assert!(!is_secret_env_var("SHELL"));
        assert!(!is_secret_env_var("KEYBOARD_LAYOUT"));
        assert!(!is_secret_env_var("GNOME_KEYRING_CONTROL"));
        assert!(!is_secret_env_var("XKB_DEFAULT_LAYOUT"));
    }

    #[test]
    fn redact_github_pat() {
        let token = "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmn";
        let result = redact_secrets(&format!("token: {token}"));
        assert_eq!(result, "token: [REDACTED]");
    }

    #[test]
    fn redact_npm_token() {
        let token = "npm_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmn";
        let result = redact_secrets(&format!("npm: {token}"));
        assert_eq!(result, "npm: [REDACTED]");
    }

    #[test]
    fn redact_sk_key() {
        let key = "sk-abcdefghijklmnopqrstuvwxyz";
        let result = redact_secrets(&format!("key: {key}"));
        assert_eq!(result, "key: [REDACTED]");
    }

    #[test]
    fn redact_aws_key_id() {
        let key = "AKIAIOSFODNN7EXAMPLE1";
        let result = redact_secrets(&format!("aws: {key}"));
        assert_eq!(result, "aws: [REDACTED]");
    }

    #[test]
    fn redact_base64_long() {
        let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijk+mnopqrs=";
        assert!(b64.len() > 40);
        let result = redact_secrets(&format!("data: {b64} end"));
        assert_eq!(result, "data: [REDACTED] end");
    }

    #[test]
    fn redact_preserves_short_strings() {
        let input = "hello world PATH=/usr/bin";
        assert_eq!(redact_secrets(input), input);
    }

    #[test]
    fn redact_preserves_normal_text() {
        let input = "This is a normal log message with no secrets.";
        assert_eq!(redact_secrets(input), input);
    }

    #[test]
    fn redact_preserves_pure_hex_string() {
        // SHA-256 hex digests should NOT be redacted (no +, /, or = characters)
        let hex = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert!(hex.len() > 40);
        let result = redact_secrets(&format!("hash: {hex}"));
        assert_eq!(result, format!("hash: {hex}"));
    }

    #[test]
    fn redact_preserves_long_alphanumeric() {
        // Pure alphanumeric strings >40 chars without base64 specials should not be redacted
        let long_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs";
        assert!(long_str.len() > 40);
        let result = redact_secrets(&format!("data: {long_str} end"));
        assert_eq!(result, format!("data: {long_str} end"));
    }

    #[test]
    fn sanitized_env_filters_secrets() {
        // This test just verifies the function runs without panic.
        // Actual filtering depends on the running environment.
        let env = sanitized_child_env();
        for (key, _) in &env {
            if let Some(name) = key.to_str() {
                assert!(
                    !is_secret_env_var(name),
                    "secret env var {name} was not filtered"
                );
            }
        }
    }

    #[test]
    fn redact_secrets_after_ansi_strip() {
        use crate::security::sanitize::sanitize_subprocess_output;
        // A GitHub PAT token with ANSI color codes inserted in the middle
        // After sanitize strips the ANSI, the full token pattern should be visible and redacted
        let token = "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmn";
        let with_ansi = format!("secret: \x1B[31m{}\x1B[0m end", token);
        let sanitized = sanitize_subprocess_output(&with_ansi);
        let redacted = redact_secrets(&sanitized);
        assert!(
            !redacted.contains("ghp_"),
            "Token should be redacted but found: {redacted}"
        );
        assert!(redacted.contains("[REDACTED]"));
    }
}
