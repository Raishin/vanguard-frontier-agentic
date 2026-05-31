// Feature: rust-tui, Property 11: Catalog string sanitization removes control bytes
// Feature: rust-tui, Property 12: Subprocess output escape filtering

use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::security::sanitize::{sanitize_catalog_string, sanitize_subprocess_output};

// =============================================================================
// Property 11: Catalog string sanitization removes control bytes
// **Validates: Requirements 10.1**
//
// For any string, `sanitize_catalog_string` SHALL replace all bytes in ranges
// 0x00–0x08, 0x0B–0x0C, 0x0E–0x1F, 0x7F, and Unicode C1 controls U+0080-U+009F
// with U+FFFD. Bytes 0x09 (tab) and 0x0A (newline) SHALL be preserved.
// All other characters SHALL be preserved unchanged.
// =============================================================================

/// Strategy that generates strings containing a mix of normal characters,
/// control bytes, tabs, newlines, and C1 controls.
fn arbitrary_string_with_controls() -> impl Strategy<Value = String> {
    proptest::collection::vec(
        prop_oneof![
            // Normal ASCII printable characters
            (0x20u32..=0x7Eu32).prop_map(|c| char::from_u32(c).unwrap()),
            // Control bytes 0x00-0x08 (should be replaced)
            (0x00u32..=0x08u32).prop_map(|c| char::from_u32(c).unwrap()),
            // Tab (0x09) and newline (0x0A) - should be preserved
            Just('\t'),
            Just('\n'),
            // 0x0B-0x0C (should be replaced)
            (0x0Bu32..=0x0Cu32).prop_map(|c| char::from_u32(c).unwrap()),
            // 0x0D (carriage return) - in range 0x0E-0x1F? No, 0x0D is between 0x0C and 0x0E
            // Actually 0x0D is NOT in the disallowed range per spec (0x0E-0x1F), let's check:
            // The spec says 0x00-0x08, 0x0B-0x0C, 0x0E-0x1F. So 0x0D (CR) is NOT replaced.
            // Wait - looking at the implementation: is_disallowed_control checks:
            // b <= 0x08 || (0x0B..=0x0C) || (0x0E..=0x1F) || b == 0x7F || (0x80..=0x9F)
            // So 0x0D (CR) is NOT disallowed. But the spec says "0x00-0x08, 0x0B-0x0C, 0x0E-0x1F"
            // which excludes 0x09 (tab), 0x0A (newline), and 0x0D (CR).
            // Let's include 0x0D as a preserved character.
            Just('\r'),
            // 0x0E-0x1F (should be replaced)
            (0x0Eu32..=0x1Fu32).prop_map(|c| char::from_u32(c).unwrap()),
            // 0x7F DEL (should be replaced)
            Just('\x7F'),
            // Unicode C1 controls U+0080-U+009F (should be replaced)
            (0x80u32..=0x9Fu32).prop_map(|c| char::from_u32(c).unwrap()),
            // Some Unicode characters beyond ASCII (should be preserved)
            (0x00A0u32..=0x00FFu32).prop_map(|c| char::from_u32(c).unwrap()),
        ],
        1..100,
    )
    .prop_map(|chars| chars.into_iter().collect::<String>())
}

/// Helper: returns true if a character should be replaced by U+FFFD
fn should_be_replaced(c: char) -> bool {
    let b = c as u32;
    b <= 0x08
        || (0x0B..=0x0C).contains(&b)
        || (0x0E..=0x1F).contains(&b)
        || b == 0x7F
        || (0x80..=0x9F).contains(&b)
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Property 11a: No disallowed control bytes remain in output.
    /// For any input string, the output of sanitize_catalog_string SHALL NOT
    /// contain any bytes in the disallowed ranges.
    #[test]
    fn no_control_bytes_remain_in_output(input in arbitrary_string_with_controls()) {
        let result = sanitize_catalog_string(&input);
        for c in result.chars() {
            prop_assert!(
                !should_be_replaced(c),
                "Disallowed control byte U+{:04X} found in output",
                c as u32
            );
        }
    }

    /// Property 11b: Tab (0x09) and newline (0x0A) are preserved.
    /// The count of tabs and newlines in the output SHALL equal the count in the input.
    #[test]
    fn tabs_and_newlines_preserved(input in arbitrary_string_with_controls()) {
        let result = sanitize_catalog_string(&input);
        let input_tabs = input.chars().filter(|&c| c == '\t').count();
        let input_newlines = input.chars().filter(|&c| c == '\n').count();
        let result_tabs = result.chars().filter(|&c| c == '\t').count();
        let result_newlines = result.chars().filter(|&c| c == '\n').count();
        prop_assert_eq!(input_tabs, result_tabs, "Tab count mismatch");
        prop_assert_eq!(input_newlines, result_newlines, "Newline count mismatch");
    }

    /// Property 11c: All other characters are preserved unchanged.
    /// The output has the same number of characters as the input (1:1 replacement).
    /// Each non-control character in the input appears at the same position in the output.
    #[test]
    fn non_control_chars_preserved_unchanged(input in arbitrary_string_with_controls()) {
        let result = sanitize_catalog_string(&input);
        // Output length (in chars) must equal input length (1:1 mapping)
        prop_assert_eq!(
            input.chars().count(),
            result.chars().count(),
            "Character count mismatch: input has {} chars, output has {}",
            input.chars().count(),
            result.chars().count()
        );
        // Each character is either preserved or replaced with U+FFFD
        for (orig, out) in input.chars().zip(result.chars()) {
            if should_be_replaced(orig) {
                prop_assert_eq!(out, '\u{FFFD}', "Expected U+FFFD for U+{:04X}", orig as u32);
            } else {
                prop_assert_eq!(out, orig, "Expected char U+{:04X} to be preserved", orig as u32);
            }
        }
    }

    /// Property 11d: Unicode C1 controls (U+0080-U+009F) are specifically replaced.
    /// Generate strings that specifically include C1 controls and verify replacement.
    #[test]
    fn c1_controls_replaced(
        prefix in "[a-z]{0,5}",
        c1_byte in 0x80u32..=0x9Fu32,
        suffix in "[a-z]{0,5}"
    ) {
        let c1_char = char::from_u32(c1_byte).unwrap();
        let input = format!("{prefix}{c1_char}{suffix}");
        let result = sanitize_catalog_string(&input);
        // The C1 control should be replaced with U+FFFD
        let expected = format!("{prefix}\u{FFFD}{suffix}");
        prop_assert_eq!(result, expected, "C1 control U+{:04X} not replaced", c1_byte);
    }

    /// Property 11e: Arbitrary Unicode strings (no controls) pass through unchanged.
    #[test]
    fn safe_strings_unchanged(input in "[\\PC&&[^\\x00-\\x08\\x0B\\x0C\\x0E-\\x1F\\x7F\\u0080-\\u009F]]{1,50}") {
        let result = sanitize_catalog_string(&input);
        prop_assert_eq!(&result, &input, "Safe string should pass through unchanged");
    }
}

// =============================================================================
// Property 12: Subprocess output escape filtering
// **Validates: Requirements 10.2**
//
// For any string containing ANSI escape sequences or Unicode C1 controls,
// `sanitize_subprocess_output` SHALL preserve SGR sequences (CSI + numeric
// parameters + `m`) and SHALL remove all other escape sequences (OSC, DCS, SOS,
// PM, APC) plus C1 controls. All non-escape content SHALL be preserved unchanged
// in its original order.
// =============================================================================

/// Strategy to generate valid SGR parameter strings (digits and semicolons)
fn sgr_params() -> impl Strategy<Value = String> {
    proptest::collection::vec(
        prop_oneof![
            Just("0".to_string()),
            Just("1".to_string()),
            Just("31".to_string()),
            Just("32".to_string()),
            Just("38;5;196".to_string()),
            Just("48;2;255;128;0".to_string()),
            (0u32..=107u32).prop_map(|n| n.to_string()),
        ],
        1..4,
    )
    .prop_map(|parts| parts.join(";"))
}

/// Strategy to generate non-SGR CSI final bytes (A-L, N-Z, a-l, n-z — anything except 'm')
fn non_sgr_final_byte() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('A'), // Cursor Up
        Just('B'), // Cursor Down
        Just('C'), // Cursor Forward
        Just('D'), // Cursor Back
        Just('H'), // Cursor Position
        Just('J'), // Erase Display
        Just('K'), // Erase Line
        Just('S'), // Scroll Up
        Just('T'), // Scroll Down
        Just('f'), // Horizontal Vertical Position
        Just('h'), // Set Mode
        Just('l'), // Reset Mode
    ]
}

/// Strategy to generate plain text content (no escape chars or C1 controls)
fn plain_text() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 .,!?:;_-]{1,20}"
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Property 12a: SGR sequences (CSI + numeric params + 'm') are preserved.
    #[test]
    fn sgr_sequences_preserved(
        params in sgr_params(),
        text in plain_text()
    ) {
        let sgr_open = format!("\x1B[{}m", params);
        let sgr_close = "\x1B[0m";
        let input = format!("{sgr_open}{text}{sgr_close}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            result.contains(&sgr_open),
            "SGR open sequence should be preserved: {:?} not in {:?}",
            sgr_open, result
        );
        prop_assert!(
            result.contains(sgr_close),
            "SGR close sequence should be preserved"
        );
        prop_assert!(
            result.contains(&text),
            "Text content should be preserved"
        );
    }

    /// Property 12b: Non-SGR CSI sequences are stripped.
    #[test]
    fn non_sgr_csi_stripped(
        params in "[0-9;]{0,5}",
        final_byte in non_sgr_final_byte(),
        before in plain_text(),
        after in plain_text()
    ) {
        let csi_seq = format!("\x1B[{params}{final_byte}");
        let input = format!("{before}{csi_seq}{after}");
        let result = sanitize_subprocess_output(&input);
        // The CSI sequence should be stripped
        prop_assert!(
            !result.contains(&csi_seq),
            "Non-SGR CSI should be stripped: {:?} found in {:?}",
            csi_seq, result
        );
        // Both text segments should be preserved
        prop_assert!(
            result.contains(&before),
            "Text before CSI should be preserved: {:?} not in {:?}",
            before, result
        );
        prop_assert!(
            result.contains(&after),
            "Text after CSI should be preserved: {:?} not in {:?}",
            after, result
        );
    }

    /// Property 12c: OSC sequences (ESC ]) are stripped.
    #[test]
    fn osc_sequences_stripped(
        osc_content in "[a-zA-Z0-9;]{0,20}",
        before in plain_text(),
        after in plain_text()
    ) {
        // OSC terminated by BEL (0x07)
        let input = format!("{before}\x1B]{osc_content}\x07{after}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            !result.contains("\x1B]"),
            "OSC sequence should be stripped"
        );
        prop_assert!(
            result.contains(&before),
            "Text before OSC should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after OSC should be preserved"
        );
    }

    /// Property 12d: DCS sequences (ESC P) are stripped.
    #[test]
    fn dcs_sequences_stripped(
        dcs_content in "[a-zA-Z0-9]{0,15}",
        before in plain_text(),
        after in plain_text()
    ) {
        // DCS terminated by ST (ESC \)
        let input = format!("{before}\x1BP{dcs_content}\x1B\\{after}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            !result.contains("\x1BP"),
            "DCS sequence should be stripped"
        );
        prop_assert!(
            result.contains(&before),
            "Text before DCS should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after DCS should be preserved"
        );
    }

    /// Property 12e: SOS sequences (ESC X) are stripped.
    #[test]
    fn sos_sequences_stripped(
        sos_content in "[a-zA-Z0-9]{0,15}",
        before in plain_text(),
        after in plain_text()
    ) {
        let input = format!("{before}\x1BX{sos_content}\x1B\\{after}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            !result.contains("\x1BX"),
            "SOS sequence should be stripped"
        );
        prop_assert!(
            result.contains(&before),
            "Text before SOS should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after SOS should be preserved"
        );
    }

    /// Property 12f: PM sequences (ESC ^) are stripped.
    #[test]
    fn pm_sequences_stripped(
        pm_content in "[a-zA-Z0-9]{0,15}",
        before in plain_text(),
        after in plain_text()
    ) {
        let input = format!("{before}\x1B^{pm_content}\x1B\\{after}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            !result.contains("\x1B^"),
            "PM sequence should be stripped"
        );
        prop_assert!(
            result.contains(&before),
            "Text before PM should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after PM should be preserved"
        );
    }

    /// Property 12g: APC sequences (ESC _) are stripped.
    #[test]
    fn apc_sequences_stripped(
        apc_content in "[a-zA-Z0-9]{0,15}",
        before in plain_text(),
        after in plain_text()
    ) {
        let input = format!("{before}\x1B_{apc_content}\x1B\\{after}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(
            !result.contains("\x1B_"),
            "APC sequence should be stripped"
        );
        prop_assert!(
            result.contains(&before),
            "Text before APC should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after APC should be preserved"
        );
    }

    /// Property 12h: Unicode C1 controls (U+0080-U+009F) are stripped from subprocess output.
    #[test]
    fn c1_controls_stripped_from_subprocess(
        c1_byte in 0x80u32..=0x9Fu32,
        before in plain_text(),
        after in plain_text()
    ) {
        let c1_char = char::from_u32(c1_byte).unwrap();
        let input = format!("{before}{c1_char}{after}");
        let result = sanitize_subprocess_output(&input);
        // C1 control should be removed
        prop_assert!(
            !result.contains(c1_char),
            "C1 control U+{:04X} should be stripped from output: {:?}",
            c1_byte, result
        );
        // Surrounding text preserved
        prop_assert!(
            result.contains(&before),
            "Text before C1 should be preserved"
        );
        prop_assert!(
            result.contains(&after),
            "Text after C1 should be preserved"
        );
    }

    /// Property 12i: Non-escape content is preserved in original order.
    /// Generate multiple text segments separated by various escape sequences
    /// and verify all text segments appear in the output in order.
    #[test]
    fn non_escape_content_preserved_in_order(
        seg1 in "[a-z]{3,8}",
        seg2 in "[A-Z]{3,8}",
        seg3 in "[0-9]{3,8}",
        escape_type in 0u8..4
    ) {
        let escape_seq = match escape_type {
            0 => "\x1B[2J".to_string(),       // Erase display (non-SGR CSI)
            1 => "\x1B]0;title\x07".to_string(), // OSC
            2 => "\x1BPdata\x1B\\".to_string(),  // DCS
            _ => "\x1B_app\x1B\\".to_string(),   // APC
        };
        let input = format!("{seg1}{escape_seq}{seg2}{escape_seq}{seg3}");
        let result = sanitize_subprocess_output(&input);

        // All segments should be present
        prop_assert!(result.contains(&seg1), "seg1 should be in output");
        prop_assert!(result.contains(&seg2), "seg2 should be in output");
        prop_assert!(result.contains(&seg3), "seg3 should be in output");

        // Segments should appear in order
        let pos1 = result.find(&seg1).unwrap();
        let pos2 = result.find(&seg2).unwrap();
        let pos3 = result.find(&seg3).unwrap();
        prop_assert!(pos1 < pos2, "seg1 should come before seg2");
        prop_assert!(pos2 < pos3, "seg2 should come before seg3");
    }

    /// Property 12j: Plain text without any escape sequences passes through unchanged.
    #[test]
    fn plain_text_unchanged(input in "[a-zA-Z0-9 .,!?:;_\\-\\n\\t]{1,100}") {
        let result = sanitize_subprocess_output(&input);
        prop_assert_eq!(&result, &input, "Plain text should pass through unchanged");
    }

    /// Property 12k: SGR sequences mixed with stripped sequences — SGR preserved, others stripped.
    #[test]
    fn mixed_sgr_and_non_sgr(
        sgr_param in sgr_params(),
        text in plain_text(),
        non_sgr_final in non_sgr_final_byte()
    ) {
        let sgr = format!("\x1B[{}m", sgr_param);
        let non_sgr = format!("\x1B[5{non_sgr_final}");
        let input = format!("{sgr}{text}{non_sgr}more");
        let result = sanitize_subprocess_output(&input);
        // SGR should be preserved
        prop_assert!(
            result.contains(&sgr),
            "SGR should be preserved in mixed input"
        );
        // Non-SGR should be stripped
        prop_assert!(
            !result.contains(&non_sgr),
            "Non-SGR CSI should be stripped in mixed input"
        );
        // Text content preserved
        prop_assert!(result.contains(&text), "Text should be preserved");
        prop_assert!(result.contains("more"), "'more' should be preserved");
    }
}
