use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::security::redact::{is_secret_env_var, redact_secrets};

// Property 9: Generate env var names with SECRET/KEY/TOKEN/PASSWORD/CREDENTIAL substrings,
// verify is_secret_env_var returns true.
// Generate names without those, verify returns false (except exact matches).
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn secret_substring_detected(
        prefix in "[A-Z]{1,5}",
        secret_word_idx in 0usize..5,
        suffix in "[A-Z]{0,5}"
    ) {
        let secret_words = ["_SECRET", "_TOKEN", "_PASSWORD", "_CREDENTIAL"];
        // For non-_KEY patterns, any suffix works (substring match)
        if secret_word_idx < 4 {
            let word = secret_words[secret_word_idx % secret_words.len()];
            let name = format!("{prefix}{word}{suffix}");
            prop_assert!(is_secret_env_var(&name), "should detect as secret: {}", name);
        } else {
            // For _KEY, test suffix match (ends with _KEY) and _KEY_ pattern
            let name = format!("{prefix}_KEY");
            prop_assert!(is_secret_env_var(&name), "should detect as secret (suffix): {}", name);
            if !suffix.is_empty() {
                let name_with_underscore = format!("{prefix}_KEY_{suffix}");
                prop_assert!(is_secret_env_var(&name_with_underscore), "should detect as secret (_KEY_): {}", name_with_underscore);
            }
        }
    }

    #[test]
    fn non_secret_names_not_detected(s in "[A-Z]{1,10}") {
        // Filter out names that happen to contain secret substrings or exact matches
        let upper = s.to_uppercase();
        prop_assume!(!upper.contains("_SECRET"));
        prop_assume!(!upper.ends_with("_KEY"));
        prop_assume!(!upper.contains("_KEY_"));
        prop_assume!(!upper.contains("_TOKEN"));
        prop_assume!(!upper.contains("_PASSWORD"));
        prop_assume!(!upper.contains("_CREDENTIAL"));
        prop_assume!(upper != "AWS_SECRET_ACCESS_KEY");
        prop_assume!(upper != "GITHUB_TOKEN");
        prop_assume!(upper != "NPM_TOKEN");
        prop_assert!(!is_secret_env_var(&s), "should NOT detect as secret: {}", s);
    }
}

// Property 10: Generate strings with ghp_/npm_/sk-/AKIA prefixes + enough chars,
// verify redact_secrets replaces them. Non-secret text preserved.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn ghp_token_redacted(chars in "[a-zA-Z0-9]{36,50}") {
        let token = format!("ghp_{chars}");
        let input = format!("token={token}");
        let result = redact_secrets(&input);
        prop_assert!(result.contains("[REDACTED]"), "should redact ghp_ token: {}", result);
        prop_assert!(!result.contains("ghp_"));
    }

    #[test]
    fn npm_token_redacted(chars in "[a-zA-Z0-9]{36,50}") {
        let token = format!("npm_{chars}");
        let input = format!("token={token}");
        let result = redact_secrets(&input);
        prop_assert!(result.contains("[REDACTED]"), "should redact npm_ token: {}", result);
        prop_assert!(!result.contains("npm_"));
    }

    #[test]
    fn sk_key_redacted(chars in "[a-zA-Z0-9_-]{20,40}") {
        let key = format!("sk-{chars}");
        let input = format!("key={key}");
        let result = redact_secrets(&input);
        prop_assert!(result.contains("[REDACTED]"), "should redact sk- key: {}", result);
        prop_assert!(!result.contains("sk-"));
    }

    #[test]
    fn akia_key_redacted(chars in "[A-Z0-9]{16,24}") {
        let key = format!("AKIA{chars}");
        let input = format!("key={key}");
        let result = redact_secrets(&input);
        prop_assert!(result.contains("[REDACTED]"), "should redact AKIA key: {}", result);
        prop_assert!(!result.contains("AKIA"));
    }

    #[test]
    fn short_safe_text_preserved(s in "[a-z ]{1,30}") {
        let result = redact_secrets(&s);
        prop_assert_eq!(result, s);
    }
}
