//! Headless reporter data models — output format, report types, and structured output.

use serde::Serialize;

/// Which kind of report to produce in headless mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportType {
    Coverage,
    Violations,
    Drift,
    Stale,
    Gates,
    Integrity,
    Versions,
    Dependencies,
    Lifecycle,
    Summary,
    All,
}

/// Output format requested by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Markdown,
    Table,
}

/// Top-level envelope for a single headless report section.
///
/// `data` is flattened into the JSON output so that consumers receive a single
/// object with `report_type`, `timestamp`, `console_version`, `exit_code`, and
/// all report-specific keys at the top level.
#[derive(Debug, Serialize)]
pub struct HeadlessOutput {
    /// String representation of the [`ReportType`] that produced this output.
    pub report_type: String,
    /// ISO 8601 timestamp of when the report was generated.
    pub timestamp: String,
    /// Version of the `vfa-tui` binary that produced this output.
    pub console_version: String,
    /// Proposed exit code for this report section.
    pub exit_code: i32,
    /// Arbitrary report payload — flattened into the enclosing JSON object.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_type_equality() {
        assert_eq!(ReportType::Coverage, ReportType::Coverage);
        assert_ne!(ReportType::Coverage, ReportType::Violations);
        assert_eq!(ReportType::All, ReportType::All);
    }

    #[test]
    fn output_format_equality() {
        assert_eq!(OutputFormat::Json, OutputFormat::Json);
        assert_ne!(OutputFormat::Json, OutputFormat::Markdown);
        assert_eq!(OutputFormat::Table, OutputFormat::Table);
    }

    #[test]
    fn headless_output_serializes_correctly() {
        let output = HeadlessOutput {
            report_type: "coverage".to_string(),
            timestamp: "2025-01-01T00:00:00.000Z".to_string(),
            console_version: "1.0.0".to_string(),
            exit_code: 0,
            data: serde_json::json!({"aggregate_score": 95.0}),
        };
        let json = serde_json::to_string(&output).expect("serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");

        assert_eq!(parsed["report_type"], "coverage");
        assert_eq!(parsed["timestamp"], "2025-01-01T00:00:00.000Z");
        assert_eq!(parsed["console_version"], "1.0.0");
        assert_eq!(parsed["exit_code"], 0);
        // flattened key from `data`:
        assert_eq!(parsed["aggregate_score"], 95.0);
    }

    #[test]
    fn headless_output_flatten_merges_data() {
        // Confirm that `#[serde(flatten)]` on `data` merges keys at the top level.
        let output = HeadlessOutput {
            report_type: "gates".to_string(),
            timestamp: "2025-06-01T00:00:00.000Z".to_string(),
            console_version: "2.0.0".to_string(),
            exit_code: 1,
            data: serde_json::json!({"gates_passed": 3, "gates_failed": 1}),
        };
        let json = serde_json::to_string(&output).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["exit_code"], 1);
        assert_eq!(parsed["gates_passed"], 3);
        assert_eq!(parsed["gates_failed"], 1);
        // ensure no nested "data" key leaks through
        assert!(parsed.get("data").is_none());
    }

    #[test]
    fn report_type_all_variants_are_distinct() {
        let variants = [
            ReportType::Coverage,
            ReportType::Violations,
            ReportType::Drift,
            ReportType::Stale,
            ReportType::Gates,
            ReportType::Integrity,
            ReportType::Versions,
            ReportType::Dependencies,
            ReportType::Lifecycle,
            ReportType::Summary,
            ReportType::All,
        ];
        // Each variant is equal only to itself.
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
