// Feature: rust-tui, Property 9: Secret environment variable detection
// Feature: rust-tui, Property 10: Secret redaction correctness

use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::security::redact::{is_secret_env_var, redact_secrets};

fn ascii_case_permutations(word: &'static str) -> impl Strategy<Value = String> {
    prop::collection::vec(any::<bool>(), word.len()).prop_map(move |upper_flags| {
        word.chars()
            .zip(upper_flags)
            .map(|(c, upper)| {
                if upper {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
    })
}

// =============================================================================
// Property 9: Secret environment variable detection
// **Validates: Requirements 9.1, 9.2**
//
// For any environment variable name, `is_secret_env_var` SHALL return `true`
// if and only if the name matches (case-insensitive) one of:
// `AWS_SECRET_ACCESS_KEY`, `GITHUB_TOKEN`, `NPM_TOKEN`, or any name containing
// the substrings `_SECRET`, `_KEY`, `_TOKEN`, `_PASSWORD`, or `_CREDENTIAL`.
// =============================================================================

proptest! {
    #![proptest_config(Config::with_cases(256))]

    // --- Property 9: Known exact secret names (with random case) → should return true ---

    #[test]
    fn prop9_exact_aws_secret_access_key_any_case(
        name in ascii_case_permutations("aws_secret_access_key")
    ) {
        prop_assert!(
            is_secret_env_var(&name),
            "AWS_SECRET_ACCESS_KEY in any case should be detected: {}",
            name
        );
    }

    #[test]
    fn prop9_exact_github_token_any_case(
        name in ascii_case_permutations("github_token")
    ) {
        prop_assert!(
            is_secret_env_var(&name),
            "GITHUB_TOKEN in any case should be detected: {}",
            name
        );
    }

    #[test]
    fn prop9_exact_npm_token_any_case(
        name in ascii_case_permutations("npm_token")
    ) {
        prop_assert!(
            is_secret_env_var(&name),
            "NPM_TOKEN in any case should be detected: {}",
            name
        );
    }

    // --- Property 9: Names containing _SECRET, _KEY, _TOKEN, _PASSWORD, _CREDENTIAL → true ---

    #[test]
    fn prop9_contains_secret_substring(
        prefix in "[A-Za-z]{1,8}",
        secret_idx in 0usize..4,
        suffix in "[A-Za-z_]{0,8}"
    ) {
        let secret_substrings = ["_SECRET", "_TOKEN", "_PASSWORD", "_CREDENTIAL"];
        let word = secret_substrings[secret_idx];
        let name = format!("{prefix}{word}{suffix}");
        prop_assert!(
            is_secret_env_var(&name),
            "Name containing {} should be detected as secret: {}",
            word,
            name
        );
    }

    #[test]
    fn prop9_contains_secret_substring_case_insensitive(
        prefix in "[A-Za-z]{1,8}",
        secret_idx in 0usize..4,
        suffix in "[A-Za-z_]{0,8}"
    ) {
        let secret_substrings = ["_secret", "_token", "_password", "_credential"];
        let word = secret_substrings[secret_idx];
        let name = format!("{prefix}{word}{suffix}");
        prop_assert!(
            is_secret_env_var(&name),
            "Name containing {} (lowercase) should be detected as secret: {}",
            word,
            name
        );
    }

    #[test]
    fn prop9_key_suffix_detected(
        prefix in "[A-Z]{1,10}"
    ) {
        let name = format!("{prefix}_KEY");
        prop_assert!(
            is_secret_env_var(&name),
            "Name ending with _KEY should be detected: {}",
            name
        );
    }

    #[test]
    fn prop9_key_infix_detected(
        prefix in "[A-Z]{1,5}",
        suffix in "[A-Z]{1,5}"
    ) {
        let name = format!("{prefix}_KEY_{suffix}");
        prop_assert!(
            is_secret_env_var(&name),
            "Name containing _KEY_ should be detected: {}",
            name
        );
    }

    // --- Property 9: Safe names (PATH, HOME, LANG, etc.) → should return false ---

    #[test]
    fn prop9_safe_names_not_detected(s in "[A-Z]{1,12}") {
        let upper = s.to_uppercase();
        // Filter out names that happen to contain secret substrings or exact matches
        prop_assume!(!upper.contains("_SECRET"));
        prop_assume!(!upper.contains("SECRET_"));
        prop_assume!(!upper.ends_with("_KEY"));
        prop_assume!(!upper.contains("_KEY_"));
        prop_assume!(!upper.contains("_TOKEN"));
        prop_assume!(!upper.contains("TOKEN"));
        prop_assume!(!upper.contains("_PASSWORD"));
        prop_assume!(!upper.contains("PASSWORD"));
        prop_assume!(!upper.contains("_CREDENTIAL"));
        prop_assume!(!upper.contains("CREDENTIAL"));
        prop_assume!(!upper.contains("SECRET"));
        prop_assume!(!upper.contains("KEY"));
        prop_assume!(upper != "AWS_SECRET_ACCESS_KEY");
        prop_assume!(upper != "GITHUB_TOKEN");
        prop_assume!(upper != "NPM_TOKEN");
        prop_assert!(
            !is_secret_env_var(&s),
            "Safe name should NOT be detected as secret: {}",
            s
        );
    }

    #[test]
    fn prop9_known_safe_names_not_detected(idx in 0usize..7) {
        let safe_names = ["PATH", "HOME", "LANG", "SHELL", "USER", "DISPLAY", "TERM"];
        let name = safe_names[idx];
        prop_assert!(
            !is_secret_env_var(name),
            "Known safe name should NOT be detected: {}",
            name
        );
    }
}

// =============================================================================
// Property 10: Secret redaction correctness
// **Validates: Requirements 9.3, 9.5**
//
// For any string containing substrings matching secret patterns (base64-encoded
// strings longer than 40 characters, strings prefixed with `ghp_`, `npm_`,
// `sk-`, or `AKIA`), `redact_secrets` SHALL replace each matching substring
// with the fixed redaction placeholder. For any substring in the input that does
// NOT match a secret pattern, that substring SHALL appear unchanged in the output
// at the same relative position.
// =============================================================================

proptest! {
    #![proptest_config(Config::with_cases(256))]

    // --- Property 10: ghp_ tokens are redacted ---

    #[test]
    fn prop10_ghp_token_redacted(
        prefix in "[a-z ]{0,10}",
        chars in "[a-zA-Z0-9]{36,50}",
        suffix in "[a-z ]{0,10}"
    ) {
        let token = format!("ghp_{chars}");
        let input = format!("{prefix}{token}{suffix}");
        let result = redact_secrets(&input);
        prop_assert!(
            result.contains("[REDACTED]"),
            "ghp_ token should be redacted in: {}",
            result
        );
        prop_assert!(
            !result.contains("ghp_"),
            "ghp_ prefix should not remain after redaction: {}",
            result
        );
    }

    // --- Property 10: npm_ tokens are redacted ---

    #[test]
    fn prop10_npm_token_redacted(
        prefix in "[a-z ]{0,10}",
        chars in "[a-zA-Z0-9_-]{36,50}",
        suffix in "[a-z ]{0,10}"
    ) {
        let token = format!("npm_{chars}");
        let input = format!("{prefix}{token}{suffix}");
        let result = redact_secrets(&input);
        prop_assert!(
            result.contains("[REDACTED]"),
            "npm_ token should be redacted in: {}",
            result
        );
        prop_assert!(
            !result.contains("npm_"),
            "npm_ prefix should not remain after redaction: {}",
            result
        );
    }

    // --- Property 10: sk- keys are redacted ---

    #[test]
    fn prop10_sk_key_redacted(
        prefix in "[a-z ]{0,10}",
        chars in "[a-zA-Z0-9_-]{20,40}",
        suffix in "[a-z ]{0,10}"
    ) {
        let key = format!("sk-{chars}");
        let input = format!("{prefix}{key}{suffix}");
        let result = redact_secrets(&input);
        prop_assert!(
            result.contains("[REDACTED]"),
            "sk- key should be redacted in: {}",
            result
        );
        prop_assert!(
            !result.contains("sk-"),
            "sk- prefix should not remain after redaction: {}",
            result
        );
    }

    // --- Property 10: AKIA keys are redacted ---

    #[test]
    fn prop10_akia_key_redacted(
        prefix in "[a-z ]{0,10}",
        chars in "[A-Z0-9]{16,24}",
        suffix in "[a-z ]{0,10}"
    ) {
        let key = format!("AKIA{chars}");
        let input = format!("{prefix}{key}{suffix}");
        let result = redact_secrets(&input);
        prop_assert!(
            result.contains("[REDACTED]"),
            "AKIA key should be redacted in: {}",
            result
        );
        prop_assert!(
            !result.contains("AKIA"),
            "AKIA prefix should not remain after redaction: {}",
            result
        );
    }

    // --- Property 10: Normal text without secret patterns → output equals input ---

    #[test]
    fn prop10_normal_text_preserved(s in "[a-z ]{1,40}") {
        // Short lowercase text with spaces cannot match any secret pattern
        let result = redact_secrets(&s);
        prop_assert_eq!(
            &result, &s,
            "Normal text should be preserved unchanged"
        );
    }

    #[test]
    fn prop10_normal_words_preserved(
        word1 in "[a-z]{1,10}",
        word2 in "[a-z]{1,10}",
        word3 in "[a-z]{1,10}"
    ) {
        let input = format!("{word1} {word2} {word3}");
        let result = redact_secrets(&input);
        prop_assert_eq!(
            &result, &input,
            "Normal words should be preserved unchanged"
        );
    }

    // --- Property 10: Surrounding text is preserved unchanged ---

    #[test]
    fn prop10_surrounding_text_preserved_ghp(
        prefix in "[a-z ]{1,15}",
        chars in "[a-zA-Z0-9]{36,50}",
        suffix in "[a-z ]{1,15}"
    ) {
        let token = format!("ghp_{chars}");
        let suffix = format!(" {suffix}");
        let input = format!("{prefix}{token}{suffix}");
        let result = redact_secrets(&input);
        // The prefix should appear before [REDACTED]
        prop_assert!(
            result.starts_with(&prefix),
            "Prefix '{}' should be preserved at start of: {}",
            prefix,
            result
        );
        // The suffix should appear after [REDACTED]
        prop_assert!(
            result.ends_with(suffix.as_str()),
            "Suffix '{}' should be preserved at end of: {}",
            suffix,
            result
        );
        // The overall structure should be prefix + [REDACTED] + suffix
        let expected = format!("{prefix}[REDACTED]{suffix}");
        prop_assert_eq!(
            &result, &expected,
            "Result should be prefix + [REDACTED] + suffix"
        );
    }

    #[test]
    fn prop10_surrounding_text_preserved_akia(
        prefix in "[a-z ]{1,15}",
        chars in "[A-Z0-9]{16,24}",
        suffix in "[a-z ]{1,15}"
    ) {
        let key = format!("AKIA{chars}");
        let input = format!("{prefix}{key}{suffix}");
        let result = redact_secrets(&input);
        // The prefix should appear before [REDACTED]
        prop_assert!(
            result.starts_with(&prefix),
            "Prefix '{}' should be preserved at start of: {}",
            prefix,
            result
        );
        // The suffix should appear after [REDACTED]
        prop_assert!(
            result.ends_with(suffix.as_str()),
            "Suffix '{}' should be preserved at end of: {}",
            suffix,
            result
        );
    }

    // --- Property 10: Multiple secrets in one string are all redacted ---

    #[test]
    fn prop10_multiple_secrets_redacted(
        chars1 in "[a-zA-Z0-9]{36,40}",
        chars2 in "[A-Z0-9]{16,20}"
    ) {
        let input = format!("a ghp_{chars1} b AKIA{chars2} c");
        let result = redact_secrets(&input);
        prop_assert!(
            !result.contains("ghp_"),
            "First secret (ghp_) should be redacted: {}",
            result
        );
        prop_assert!(
            !result.contains("AKIA"),
            "Second secret (AKIA) should be redacted: {}",
            result
        );
        // Both surrounding text fragments should be preserved
        prop_assert!(result.contains("a "), "Prefix 'a ' should be preserved: {}", result);
        prop_assert!(result.contains(" b "), "Middle ' b ' should be preserved: {}", result);
        prop_assert!(result.contains(" c"), "Suffix ' c' should be preserved: {}", result);
    }
}
