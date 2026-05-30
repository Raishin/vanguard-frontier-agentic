use proptest::prelude::*;
use proptest::test_runner::Config;
use std::path::PathBuf;
use vfa_tui::models::export::{ExportCommand, ExportSelection};

// Shell metacharacters that must never appear in arguments.
const SHELL_METACHARACTERS: &[char] = &[
    ';', '|', '&', '$', '`', '\\', '<', '>', '(', ')', '{', '}', '!', '#', '*', '?', '[', ']',
    '\n', '\r', '\0',
];

// Strategy to generate valid platform names (alphanumeric + hyphens).
fn platform_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{1,15}".prop_map(|s| s)
}

// Strategy to generate valid paths (no metacharacters).
fn path_strategy() -> impl Strategy<Value = PathBuf> {
    "/[a-z]{1,8}(/[a-z0-9-]{1,12}){0,3}".prop_map(PathBuf::from)
}

// Strategy to generate valid identifiers (for roles, providers, agent IDs).
fn identifier_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{1,20}".prop_map(|s| s)
}

// Strategy to generate a valid ExportSelection.
fn selection_strategy() -> impl Strategy<Value = ExportSelection> {
    prop_oneof![
        Just(ExportSelection::All),
        identifier_strategy().prop_map(ExportSelection::Role),
        identifier_strategy().prop_map(ExportSelection::Provider),
        prop::collection::vec(identifier_strategy(), 1..5).prop_map(ExportSelection::Agents),
    ]
}

// Property 6: Export command argument construction.
//
// For any valid ExportCommand (valid platform names, valid paths without metacharacters,
// valid selection), to_args() produces an array that:
// - Contains --platform with the platform value
// - Contains correct selection flag
// - Contains --repo with the path
// - Has --dry-run if dry_run is true
// - Has --force if force is true
// - Has --no-skills if no_skills is true
// - Contains NO empty strings
// - Contains NO shell metacharacters in any element
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn export_args_contain_platform(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
        dry_run in any::<bool>(),
        force in any::<bool>(),
        no_skills in any::<bool>(),
    ) {
        let mut cmd = ExportCommand::new(platform.clone(), selection, target_repo);
        cmd.dry_run = dry_run;
        cmd.force = force;
        cmd.no_skills = no_skills;

        let args = cmd.to_args();

        // Must contain --platform followed by the platform value
        let platform_idx = args.iter().position(|a| a == "--platform").unwrap();
        prop_assert_eq!(&args[platform_idx + 1], &platform);
    }

    #[test]
    fn export_args_contain_correct_selection(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
    ) {
        let cmd = ExportCommand::new(platform, selection.clone(), target_repo);
        let args = cmd.to_args();

        match &selection {
            ExportSelection::All => {
                prop_assert!(args.contains(&"--all".to_string()));
            }
            ExportSelection::Role(role) => {
                let role_idx = args.iter().position(|a| a == "--role").unwrap();
                prop_assert_eq!(&args[role_idx + 1], role);
            }
            ExportSelection::Provider(provider) => {
                let prov_idx = args.iter().position(|a| a == "--provider").unwrap();
                prop_assert_eq!(&args[prov_idx + 1], provider);
            }
            ExportSelection::Agents(ids) => {
                let expected = format!("--agents={}", ids.join(","));
                prop_assert!(
                    args.contains(&expected),
                    "missing --agents flag: expected {:?}, got {:?}",
                    expected,
                    args
                );
            }
        }
    }

    #[test]
    fn export_args_contain_repo(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
    ) {
        let cmd = ExportCommand::new(platform, selection, target_repo.clone());
        let args = cmd.to_args();

        let repo_idx = args.iter().position(|a| a == "--repo").unwrap();
        prop_assert_eq!(&args[repo_idx + 1], &target_repo.to_string_lossy().to_string());
    }

    #[test]
    fn export_args_flags_match_options(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
        dry_run in any::<bool>(),
        force in any::<bool>(),
        no_skills in any::<bool>(),
    ) {
        let mut cmd = ExportCommand::new(platform, selection, target_repo);
        cmd.dry_run = dry_run;
        cmd.force = force;
        cmd.no_skills = no_skills;

        let args = cmd.to_args();

        prop_assert_eq!(args.contains(&"--dry-run".to_string()), dry_run);
        prop_assert_eq!(args.contains(&"--force".to_string()), force);
        prop_assert_eq!(args.contains(&"--no-skills".to_string()), no_skills);
    }

    #[test]
    fn export_args_no_empty_strings(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
        dry_run in any::<bool>(),
        force in any::<bool>(),
        no_skills in any::<bool>(),
    ) {
        let mut cmd = ExportCommand::new(platform, selection, target_repo);
        cmd.dry_run = dry_run;
        cmd.force = force;
        cmd.no_skills = no_skills;

        let args = cmd.to_args();

        for arg in &args {
            prop_assert!(!arg.is_empty(), "found empty string in args");
        }
    }

    #[test]
    fn export_args_no_shell_metacharacters(
        platform in platform_strategy(),
        selection in selection_strategy(),
        target_repo in path_strategy(),
        dry_run in any::<bool>(),
        force in any::<bool>(),
        no_skills in any::<bool>(),
    ) {
        let mut cmd = ExportCommand::new(platform, selection, target_repo);
        cmd.dry_run = dry_run;
        cmd.force = force;
        cmd.no_skills = no_skills;

        let args = cmd.to_args();

        for arg in &args {
            for c in arg.chars() {
                prop_assert!(
                    !SHELL_METACHARACTERS.contains(&c),
                    "found metacharacter {:?} in arg {:?}",
                    c,
                    arg
                );
            }
        }
    }
}
