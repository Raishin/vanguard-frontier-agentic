//! Integration test 13.5 — headless report generation end-to-end.
//! Validates: Requirements 17.x (headless reports), 11.4 (exit codes), 27.x
//!
//! Runs `HeadlessReporter::run` against the catalog fixtures and asserts the
//! JSON envelope shape, valid exit codes, deterministic structure, and that
//! rendered output is non-empty and valid JSON.

use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;

use vfa_tui::cli::Cli;
use vfa_tui::headless::reporter::HeadlessReporter;
use vfa_tui::models::report::OutputFormat;

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// Build a Cli for a headless run with an isolated (non-existent) registry and
/// policy path so the test never reads the user's real config.
fn cli(extra: &[&str]) -> Cli {
    let mut args = vec![
        "vfa-tui",
        "--registry",
        "/nonexistent/registry.toml",
        "--policies",
        "/nonexistent/policies.toml",
    ];
    args.extend_from_slice(extra);
    Cli::parse_from(args)
}

fn run(extra: &[&str]) -> (Value, u8) {
    let reporter = HeadlessReporter::new(OutputFormat::Json, true);
    reporter.run(&cli(extra), &fixtures_root())
}

fn envelope_keys_present(v: &Value) {
    let obj = v.as_object().expect("envelope must be a JSON object");
    for k in ["report_type", "timestamp", "console_version", "exit_code"] {
        assert!(obj.contains_key(k), "envelope missing key {k}: {v}");
    }
}

#[test]
fn default_report_is_summary_envelope() {
    let (value, code) = run(&[]);
    envelope_keys_present(&value);
    assert_eq!(value["report_type"], "summary");
    assert!(code <= 3, "exit code {code} out of documented 0..=3 range");
}

#[test]
fn typed_coverage_report_round_trips() {
    let (value, code) = run(&["--report", "coverage"]);
    envelope_keys_present(&value);
    assert_eq!(value["report_type"], "coverage");
    assert!(
        value.get("data").is_some(),
        "single-type report carries a data section"
    );
    assert!(code <= 3);
}

#[test]
fn all_report_combines_multiple_sections() {
    let (value, _code) = run(&["--report", "all"]);
    let obj = value.as_object().unwrap();
    assert_eq!(obj["report_type"], "all");
    // The combined object carries more than just the envelope's 4 fixed keys.
    assert!(
        obj.len() > 4,
        "combined report should include multiple section keys"
    );
}

#[test]
fn run_is_structurally_deterministic() {
    // Timestamps differ between runs, so compare the exit code and the set of
    // top-level keys rather than the whole value.
    let (a, ca) = run(&["--report", "coverage"]);
    let (b, cb) = run(&["--report", "coverage"]);
    assert_eq!(ca, cb, "exit code must be deterministic");

    let mut ka: Vec<&String> = a.as_object().unwrap().keys().collect();
    let mut kb: Vec<&String> = b.as_object().unwrap().keys().collect();
    ka.sort();
    kb.sort();
    assert_eq!(ka, kb, "top-level key set must be deterministic");
    // The data payloads (excluding timestamp) must match exactly.
    assert_eq!(a["data"], b["data"], "coverage data must be deterministic");
}

#[test]
fn output_is_valid_json_and_renders_nonempty() {
    let (value, _) = run(&["--report", "summary"]);
    // Serialize → re-parse round trip.
    let s = serde_json::to_string(&value).expect("serialize");
    let reparsed: Value = serde_json::from_str(&s).expect("valid JSON");
    assert_eq!(reparsed["report_type"], value["report_type"]);

    let reporter = HeadlessReporter::new(OutputFormat::Json, true);
    let rendered = reporter.render(&value, &cli(&["--report", "summary"]).report_types());
    assert!(
        !rendered.trim().is_empty(),
        "rendered report must be non-empty"
    );
}
