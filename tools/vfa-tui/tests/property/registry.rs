//! Feature: rust-tui-v2, Properties 17, 29, 31 — workspace registry validation,
//! safe env-var expansion, and glob filtering.
//! Validates: Requirements 6.7, 30.5
//!
//! P17: `validate` flags missing paths and duplicate canonical paths.
//! P29: `expand_env_with` substitutes `${VAR}` / `$VAR` from an injected lookup,
//!      leaves unknown vars literal, and never runs a shell.
//! P31: `glob_match` honours `*` (any run) and `?` (one byte) semantics, and
//!      `filter` includes a workspace when its name or path matches.

use std::path::PathBuf;
use std::time::Instant;

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::error::TuiError;
use vfa_tui::federation::registry::{glob_match, WorkspaceRegistry};
use vfa_tui::models::workspace::{ResolvedWorkspace, WorkspaceEntry, WorkspaceStatus};

fn entry(path: &str, name: Option<&str>) -> WorkspaceEntry {
    WorkspaceEntry {
        path: path.to_string(),
        name: name.map(|s| s.to_string()),
        team: None,
        tags: None,
        policy_overrides: None,
    }
}

fn registry(entries: Vec<WorkspaceEntry>) -> WorkspaceRegistry {
    WorkspaceRegistry {
        entries,
        path: PathBuf::from("registry.toml"),
        last_loaded: Instant::now(),
    }
}

fn resolved(name: &str, path: &str) -> ResolvedWorkspace {
    ResolvedWorkspace {
        canonical_path: PathBuf::from(path),
        name: name.to_string(),
        team: None,
        tags: vec![],
        status: WorkspaceStatus::Available,
    }
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    // ---- P29: env expansion -------------------------------------------------

    /// `${VAR}` and `$VAR` both expand from the injected lookup.
    #[test]
    fn expands_known_var_both_forms(val in "[A-Za-z0-9/_-]{0,16}") {
        let lookup = |name: &str| (name == "ROOT").then(|| val.clone());
        prop_assert_eq!(
            WorkspaceRegistry::expand_env_with("/p/${ROOT}/s", lookup),
            format!("/p/{val}/s")
        );
        prop_assert_eq!(
            WorkspaceRegistry::expand_env_with("$ROOT/s", lookup),
            format!("{val}/s")
        );
    }

    /// Unknown variables are left verbatim (never blanked, never shelled out).
    #[test]
    fn unknown_var_left_literal(name in "[A-Z][A-Z0-9_]{0,10}") {
        let s = format!("/x/${{{name}}}/y");
        prop_assert_eq!(
            WorkspaceRegistry::expand_env_with(&s, |_| None),
            s.clone()
        );
    }

    /// A path with no `$` is returned unchanged.
    #[test]
    fn no_dollar_is_identity(s in "[A-Za-z0-9/_.-]{0,32}") {
        prop_assert_eq!(WorkspaceRegistry::expand_env_with(&s, |_| Some("X".into())), s.clone());
    }

    // ---- P31: glob matching -------------------------------------------------

    /// `*` matches any text.
    #[test]
    fn star_matches_anything(text in "[a-zA-Z0-9/_.-]{0,24}") {
        prop_assert!(glob_match("*", &text));
    }

    /// A literal pattern (no metachars) matches exactly itself.
    #[test]
    fn literal_matches_only_itself(text in "[a-zA-Z0-9/_.-]{1,16}", other in "[a-zA-Z0-9/_.-]{1,16}") {
        prop_assert!(glob_match(&text, &text));
        prop_assume!(text != other);
        prop_assert_eq!(glob_match(&text, &other), false);
    }

    /// `?` matches exactly one byte: `?{n}` matches iff the (ASCII) text has len n.
    #[test]
    fn question_marks_match_exact_length(text in "[a-z]{0,10}") {
        let pat = "?".repeat(text.len());
        prop_assert!(glob_match(&pat, &text));
        let too_many = "?".repeat(text.len() + 1);
        prop_assert!(!glob_match(&too_many, &text));
    }

    /// `prefix*` matches iff text starts with the literal prefix.
    #[test]
    fn prefix_star(prefix in "[a-z]{1,8}", rest in "[a-z]{0,8}", other_first in "[A-Z]{1,1}") {
        let text = format!("{prefix}{rest}");
        let pat = format!("{prefix}*");
        prop_assert!(glob_match(&pat, &text));
        // A text that cannot start with the (lowercase) prefix must not match.
        let non_matching = format!("{other_first}{rest}");
        prop_assume!(!non_matching.starts_with(&prefix));
        prop_assert!(!glob_match(&pat, &non_matching));
    }

    /// `filter` returns exactly the workspaces whose name OR canonical path
    /// matches the pattern (Req 6.7).
    #[test]
    fn filter_matches_name_or_path(
        stem in "[a-z]{2,6}",
        names in proptest::collection::vec("[a-zA-Z0-9/_-]{1,12}", 1..8),
        paths in proptest::collection::vec("/[a-zA-Z0-9/_-]{1,16}", 1..8),
    ) {
        let n = names.len().min(paths.len());
        let ws: Vec<_> = (0..n).map(|i| resolved(&names[i], &paths[i])).collect();
        let reg = registry(vec![]);
        let pat = format!("*{stem}*");

        let got = reg.filter(&pat, &ws);

        // The filtered set must equal the set selected by the glob oracle.
        // Compare by pointer identity to stay correct even if names collide.
        for w in &ws {
            let in_got = got.iter().any(|g| std::ptr::eq(*g, w));
            let expected = glob_match(&pat, &w.name)
                || glob_match(&pat, &w.canonical_path.to_string_lossy());
            prop_assert_eq!(in_got, expected,
                "name {} path {:?} pattern {}", w.name, w.canonical_path, pat);
        }
    }
}

// ---- P17: validation (deterministic unit cases) ----------------------------

#[test]
fn validate_flags_empty_path() {
    let reg = registry(vec![
        entry("   ", Some("blank")),
        entry("/ok", Some("good")),
    ]);
    let errors = reg.validate();
    assert!(errors
        .iter()
        .any(|e| matches!(e, TuiError::RegistryFieldMissing { field, .. } if field == "path")));
}

#[test]
fn validate_flags_duplicate_paths() {
    // Two entries pointing at the same (non-existent) path collapse to one key.
    let dup = "/does/not/exist/ws";
    let reg = registry(vec![entry(dup, Some("a")), entry(dup, Some("b"))]);
    let errors = reg.validate();
    assert!(errors
        .iter()
        .any(|e| matches!(e, TuiError::RegistryDuplicate { .. })));
}

#[test]
fn validate_clean_registry_has_no_errors() {
    let reg = registry(vec![
        entry("/does/not/exist/one", Some("one")),
        entry("/does/not/exist/two", Some("two")),
    ]);
    assert!(reg.validate().is_empty());
}
