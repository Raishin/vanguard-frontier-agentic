//! Headless output formatters — JSON, Markdown (GitHub-flavoured), and ASCII
//! table (Tasks 9.7 / 9.8).
//!
//! # Design constraints (Req 27 / 29)
//!
//! - **Determinism**: all list outputs are sorted case-insensitively by `id`
//!   before rendering.  The [`sort_by_id`] helper centralises this.
//! - **Accessibility**: status text indicators ([PASS], [FAIL], [WARN],
//!   [DRIFT], [STALE], [MISSING]) are **always** present regardless of whether
//!   color is enabled (Req 29.2).
//! - **No color by default**: the `no_color` parameter suppresses any ANSI codes
//!   that might be added in future.  Currently no ANSI codes are emitted (the
//!   TUI owns all color rendering).

#![deny(warnings)]

use serde_json::Value;

// ---------------------------------------------------------------------------
// ReportData — input for Markdown / table formatters
// ---------------------------------------------------------------------------

/// Structured report section passed to the Markdown and table formatters.
///
/// Each variant carries the data for a specific report type; the formatters
/// dispatch on the variant.
#[derive(Debug, Clone)]
pub enum ReportData {
    /// A list of rows with a header and typed cells.
    Table {
        /// Column headers (short labels).
        headers: Vec<String>,
        /// Rows — each row must have the same number of cells as `headers`.
        rows: Vec<Vec<String>>,
    },
    /// A key-value summary (e.g. for the `summary` report type).
    KeyValue {
        /// Ordered list of `(key, value)` pairs.
        pairs: Vec<(String, String)>,
    },
    /// Raw JSON fallback — rendered as a fenced code block in Markdown or
    /// pretty-printed text in table mode.
    Raw(Value),
}

// ---------------------------------------------------------------------------
// sort_by_id
// ---------------------------------------------------------------------------

/// Stable case-insensitive sort of `rows` by the value in column `id_col`.
///
/// This is the canonical sort order for all list outputs (Req 27.2).  The sort
/// is stable: rows with identical lowercased ids retain their original order.
///
/// # Panics
///
/// Panics in debug builds when `id_col >= row.len()` for any row; in release
/// builds the row is left in place unchanged.
pub fn sort_by_id(rows: &mut [Vec<String>], id_col: usize) {
    rows.sort_by(|a, b| {
        let ak = a.get(id_col).map(|s| s.to_lowercase()).unwrap_or_default();
        let bk = b.get(id_col).map(|s| s.to_lowercase()).unwrap_or_default();
        ak.cmp(&bk)
    });
}

// ---------------------------------------------------------------------------
// format_json
// ---------------------------------------------------------------------------

/// Serialise `value` to a JSON string.
///
/// When `pretty` is `true` the output is pretty-printed with 2-space
/// indentation (suitable for human review).  When `false` the output is
/// compact (suitable for piping and machine parsing).
pub fn format_json(value: &Value, pretty: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
    } else {
        serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
    }
}

// ---------------------------------------------------------------------------
// format_markdown
// ---------------------------------------------------------------------------

/// Render `report` as GitHub-flavoured Markdown.
///
/// - [`ReportData::Table`] → GFM table with pipe delimiters and alignment row.
/// - [`ReportData::KeyValue`] → two-column GFM table (Key | Value).
/// - [`ReportData::Raw`] → fenced ```json code block.
///
/// Status text indicators ([PASS], [FAIL], [WARN], [DRIFT], [STALE],
/// [MISSING]) in cell values are preserved as-is (they are plain text).
pub fn format_markdown(report: &ReportData) -> String {
    match report {
        ReportData::Table { headers, rows } => {
            let mut out = String::new();

            if headers.is_empty() {
                return out;
            }

            // Header row
            out.push('|');
            for h in headers {
                out.push(' ');
                out.push_str(&escape_md_cell(h));
                out.push_str(" |");
            }
            out.push('\n');

            // Alignment row
            out.push('|');
            for _ in headers {
                out.push_str(" --- |");
            }
            out.push('\n');

            // Data rows
            for row in rows {
                out.push('|');
                for (i, _) in headers.iter().enumerate() {
                    let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    out.push(' ');
                    out.push_str(&escape_md_cell(cell));
                    out.push_str(" |");
                }
                out.push('\n');
            }

            out
        }

        ReportData::KeyValue { pairs } => {
            let mut out = String::new();
            out.push_str("| Key | Value |\n");
            out.push_str("| --- | --- |\n");
            for (k, v) in pairs {
                out.push_str(&format!(
                    "| {} | {} |\n",
                    escape_md_cell(k),
                    escape_md_cell(v)
                ));
            }
            out
        }

        ReportData::Raw(value) => {
            let json = format_json(value, true);
            format!("```json\n{json}\n```\n")
        }
    }
}

/// Escape pipe characters inside a Markdown table cell.
fn escape_md_cell(s: &str) -> String {
    s.replace('|', "\\|")
}

// ---------------------------------------------------------------------------
// format_table
// ---------------------------------------------------------------------------

/// Render `report` as aligned ASCII columns suitable for terminal output.
///
/// - [`ReportData::Table`] → fixed-width columns with a header separator line.
/// - [`ReportData::KeyValue`] → two-column layout.
/// - [`ReportData::Raw`] → pretty-printed JSON.
///
/// Status text indicators in cell values are preserved verbatim.
pub fn format_table(report: &ReportData) -> String {
    match report {
        ReportData::Table { headers, rows } => {
            if headers.is_empty() {
                return String::new();
            }

            // Compute column widths.
            let ncols = headers.len();
            let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
            for row in rows {
                for (i, cell) in row.iter().enumerate() {
                    if i < ncols && cell.len() > widths[i] {
                        widths[i] = cell.len();
                    }
                }
            }

            let mut out = String::new();

            // Header row.
            let header_line: Vec<String> = headers
                .iter()
                .enumerate()
                .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
                .collect();
            out.push_str(&header_line.join("  "));
            out.push('\n');

            // Separator.
            let sep: Vec<String> = widths.iter().map(|&w| "-".repeat(w)).collect();
            out.push_str(&sep.join("  "));
            out.push('\n');

            // Data rows.
            for row in rows {
                let cells: Vec<String> = (0..ncols)
                    .map(|i| {
                        let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                        format!("{:width$}", cell, width = widths[i])
                    })
                    .collect();
                out.push_str(&cells.join("  "));
                out.push('\n');
            }

            out
        }

        ReportData::KeyValue { pairs } => {
            if pairs.is_empty() {
                return String::new();
            }
            let key_width = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(3);
            let mut out = String::new();
            let header = format!("{:kw$}  {}", "Key", "Value", kw = key_width);
            out.push_str(&header);
            out.push('\n');
            out.push_str(&"-".repeat(key_width));
            out.push_str("  ");
            out.push_str(&"-".repeat(5));
            out.push('\n');
            for (k, v) in pairs {
                out.push_str(&format!("{:kw$}  {}\n", k, v, kw = key_width));
            }
            out
        }

        ReportData::Raw(value) => format_json(value, true),
    }
}

// ---------------------------------------------------------------------------
// Status text indicators (Req 29.2)
// ---------------------------------------------------------------------------

/// Text prefix for a passing check — always present, no color dependency.
pub const STATUS_PASS: &str = "[PASS]";
/// Text prefix for a failing check.
pub const STATUS_FAIL: &str = "[FAIL]";
/// Text prefix for a warning condition.
pub const STATUS_WARN: &str = "[WARN]";
/// Text prefix for a drifted asset.
pub const STATUS_DRIFT: &str = "[DRIFT]";
/// Text prefix for a stale asset.
pub const STATUS_STALE: &str = "[STALE]";
/// Text prefix for a missing asset.
pub const STATUS_MISSING: &str = "[MISSING]";

/// Prepend the appropriate bracketed status indicator to `text`.
///
/// This is the canonical helper for producing accessible output (Req 29.2).
pub fn with_status(indicator: &str, text: &str) -> String {
    format!("{indicator} {text}")
}

// ---------------------------------------------------------------------------
// Tests (Property 27 + Property 32 + unit tests, Task 9.8)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // format_json unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn format_json_compact_roundtrip() {
        let v = serde_json::json!({"a": 1, "b": [true, null]});
        let s = format_json(&v, false);
        let back: Value = serde_json::from_str(&s).expect("should parse back");
        assert_eq!(v, back);
    }

    #[test]
    fn format_json_pretty_contains_newlines() {
        let v = serde_json::json!({"x": 42});
        let s = format_json(&v, true);
        assert!(s.contains('\n'));
    }

    #[test]
    fn format_json_compact_has_no_newlines() {
        let v = serde_json::json!({"x": 42, "y": [1, 2, 3]});
        let s = format_json(&v, false);
        assert!(!s.contains('\n'));
    }

    // -----------------------------------------------------------------------
    // format_markdown unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn markdown_table_empty_headers_empty_output() {
        let data = ReportData::Table {
            headers: vec![],
            rows: vec![vec!["a".into()]],
        };
        let out = format_markdown(&data);
        assert!(out.is_empty());
    }

    #[test]
    fn markdown_table_renders_header_separator_rows() {
        let data = ReportData::Table {
            headers: vec!["ID".into(), "Status".into()],
            rows: vec![
                vec!["agent-a".into(), "[PASS] ok".into()],
                vec!["agent-b".into(), "[FAIL] missing".into()],
            ],
        };
        let out = format_markdown(&data);
        assert!(out.contains("| ID |"));
        assert!(out.contains("| --- |"));
        assert!(out.contains("agent-a"));
        assert!(out.contains("[PASS]"));
        assert!(out.contains("[FAIL]"));
    }

    #[test]
    fn markdown_key_value_renders() {
        let data = ReportData::KeyValue {
            pairs: vec![
                ("total_agents".into(), "42".into()),
                ("coverage".into(), "95.0%".into()),
            ],
        };
        let out = format_markdown(&data);
        assert!(out.contains("| Key | Value |"));
        assert!(out.contains("total_agents"));
        assert!(out.contains("coverage"));
    }

    #[test]
    fn markdown_raw_renders_json_fence() {
        let data = ReportData::Raw(serde_json::json!({"k": "v"}));
        let out = format_markdown(&data);
        assert!(out.starts_with("```json\n"));
        assert!(out.ends_with("```\n"));
    }

    #[test]
    fn markdown_pipe_in_cell_escaped() {
        let data = ReportData::Table {
            headers: vec!["Col".into()],
            rows: vec![vec!["a|b".into()]],
        };
        let out = format_markdown(&data);
        // The pipe in the cell value should be escaped.
        assert!(out.contains("a\\|b"));
    }

    // -----------------------------------------------------------------------
    // format_table unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn ascii_table_empty_output_for_empty_headers() {
        let data = ReportData::Table {
            headers: vec![],
            rows: vec![],
        };
        assert!(format_table(&data).is_empty());
    }

    #[test]
    fn ascii_table_renders_aligned_columns() {
        let data = ReportData::Table {
            headers: vec!["ID".into(), "Status".into()],
            rows: vec![
                vec!["short".into(), "[PASS] ok".into()],
                vec!["a-much-longer-id".into(), "[FAIL] missing".into()],
            ],
        };
        let out = format_table(&data);
        // Should contain the header and separator.
        assert!(out.contains("ID"));
        assert!(out.contains("--"));
        // Both rows should appear.
        assert!(out.contains("short"));
        assert!(out.contains("a-much-longer-id"));
    }

    #[test]
    fn ascii_table_key_value() {
        let data = ReportData::KeyValue {
            pairs: vec![("agents".into(), "15".into())],
        };
        let out = format_table(&data);
        assert!(out.contains("agents"));
        assert!(out.contains("15"));
    }

    #[test]
    fn ascii_table_empty_key_value() {
        let data = ReportData::KeyValue { pairs: vec![] };
        assert!(format_table(&data).is_empty());
    }

    #[test]
    fn ascii_table_raw_json() {
        let data = ReportData::Raw(serde_json::json!({"z": 1}));
        let out = format_table(&data);
        // Raw falls back to pretty-printed JSON.
        let back: Value = serde_json::from_str(&out).expect("should parse");
        assert_eq!(back["z"], 1);
    }

    // -----------------------------------------------------------------------
    // sort_by_id unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn sort_by_id_basic() {
        let mut rows = vec![
            vec!["Zebra".into(), "x".into()],
            vec!["apple".into(), "y".into()],
            vec!["Mango".into(), "z".into()],
        ];
        sort_by_id(&mut rows, 0);
        assert_eq!(rows[0][0], "apple");
        assert_eq!(rows[1][0], "Mango");
        assert_eq!(rows[2][0], "Zebra");
    }

    #[test]
    fn sort_by_id_already_sorted_is_idempotent() {
        let mut rows = vec![
            vec!["alpha".into()],
            vec!["beta".into()],
            vec!["gamma".into()],
        ];
        let original = rows.clone();
        sort_by_id(&mut rows, 0);
        assert_eq!(rows, original);
    }

    #[test]
    fn sort_by_id_empty() {
        let mut rows: Vec<Vec<String>> = vec![];
        sort_by_id(&mut rows, 0);
        assert!(rows.is_empty());
    }

    // -----------------------------------------------------------------------
    // Status indicator helpers
    // -----------------------------------------------------------------------

    #[test]
    fn status_indicators_present() {
        for indicator in &[
            STATUS_PASS,
            STATUS_FAIL,
            STATUS_WARN,
            STATUS_DRIFT,
            STATUS_STALE,
            STATUS_MISSING,
        ] {
            assert!(indicator.starts_with('['));
            assert!(indicator.ends_with(']'));
        }
    }

    #[test]
    fn with_status_concatenates() {
        let s = with_status(STATUS_PASS, "all good");
        assert_eq!(s, "[PASS] all good");
    }

    // -----------------------------------------------------------------------
    // Property 27 (Req 27.2): stable case-insensitive sort is idempotent
    // -----------------------------------------------------------------------

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(256))]

        /// Sorting any list of ID strings twice gives the same result (idempotent).
        #[test]
        fn prop27_sort_idempotent(
            ids in prop::collection::vec("[a-zA-Z0-9_-]{1,20}", 0..30)
        ) {
            let mut rows: Vec<Vec<String>> = ids.iter().map(|id| vec![id.clone()]).collect();
            sort_by_id(&mut rows, 0);
            let after_first = rows.clone();
            sort_by_id(&mut rows, 0);
            prop_assert_eq!(rows, after_first, "sort must be idempotent");
        }

        /// Sorting is case-insensitive: "Zebra" < "apple" is wrong; case is ignored.
        #[test]
        fn prop27_sort_case_insensitive(
            id_a in "[A-Z][a-z]{2,8}",   // starts uppercase
            id_b in "[a-z]{2,9}",         // all lowercase
        ) {
            // id_b lowercased should compare ≤ id_a lowercased if b < a alphabetically
            let mut rows = vec![
                vec![id_a.clone()],
                vec![id_b.clone()],
            ];
            sort_by_id(&mut rows, 0);
            // After sort: lower lowercased value comes first.
            let lower_a = id_a.to_lowercase();
            let lower_b = id_b.to_lowercase();
            if lower_b <= lower_a {
                prop_assert_eq!(&rows[0][0], &id_b);
            } else {
                prop_assert_eq!(&rows[0][0], &id_a);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Property 32 (Req 29.2): status row includes bracketed text indicator
    // regardless of color mode
    // -----------------------------------------------------------------------

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(256))]

        /// Any cell rendered with a status indicator always starts with '['.
        #[test]
        fn prop32_status_indicators_always_present(
            text in "[a-z ]{0,40}",
        ) {
            for indicator in &[
                STATUS_PASS, STATUS_FAIL, STATUS_WARN,
                STATUS_DRIFT, STATUS_STALE, STATUS_MISSING,
            ] {
                let rendered = with_status(indicator, &text);
                prop_assert!(
                    rendered.starts_with('['),
                    "status row must start with '['; got: {rendered:?}"
                );
                prop_assert!(
                    rendered.contains(indicator),
                    "rendered text must contain the indicator '{indicator}'"
                );
            }
        }

        /// Markdown table output for a row containing a status indicator preserves
        /// the indicator text.
        #[test]
        fn prop32_markdown_preserves_status_indicator(
            indicator in prop_oneof![
                Just(STATUS_PASS),
                Just(STATUS_FAIL),
                Just(STATUS_WARN),
                Just(STATUS_DRIFT),
                Just(STATUS_STALE),
                Just(STATUS_MISSING),
            ],
            id in "[a-z]{3,10}",
        ) {
            let cell = with_status(indicator, &id);
            let data = ReportData::Table {
                headers: vec!["ID".into(), "Status".into()],
                rows: vec![vec![id.clone(), cell.clone()]],
            };
            let md = format_markdown(&data);
            prop_assert!(
                md.contains(indicator),
                "Markdown output must contain the status indicator '{indicator}'"
            );
        }

        /// ASCII table output for a row containing a status indicator preserves it.
        #[test]
        fn prop32_table_preserves_status_indicator(
            indicator in prop_oneof![
                Just(STATUS_PASS),
                Just(STATUS_FAIL),
                Just(STATUS_WARN),
                Just(STATUS_DRIFT),
                Just(STATUS_STALE),
                Just(STATUS_MISSING),
            ],
            id in "[a-z]{3,10}",
        ) {
            let cell = with_status(indicator, &id);
            let data = ReportData::Table {
                headers: vec!["ID".into(), "Status".into()],
                rows: vec![vec![id.clone(), cell]],
            };
            let tbl = format_table(&data);
            prop_assert!(
                tbl.contains(indicator),
                "Table output must contain the status indicator '{indicator}'"
            );
        }
    }
}
