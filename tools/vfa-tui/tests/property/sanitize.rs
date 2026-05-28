use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::security::sanitize::{sanitize_catalog_string, sanitize_subprocess_output};

/// Property 11: Generate strings with control bytes, verify sanitize_catalog_string
/// replaces them with U+FFFD. Tab/newline preserved.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn control_bytes_replaced(bytes in proptest::collection::vec(0u8..128, 1..100)) {
        let input: String = bytes.iter().map(|&b| b as char).collect();
        let result = sanitize_catalog_string(&input);

        for (orig, replacement) in input.chars().zip(result.chars()) {
            let b = orig as u32;
            if b <= 0x08 || (0x0B..=0x0C).contains(&b) || (0x0E..=0x1F).contains(&b) || b == 0x7F {
                prop_assert_eq!(replacement, '\u{FFFD}', "byte 0x{:02X} should be replaced", b);
            } else {
                prop_assert_eq!(replacement, orig, "byte 0x{:02X} should be preserved", b);
            }
        }
    }

    #[test]
    fn tab_and_newline_preserved(
        prefix in "[a-z]{0,5}",
        sep_type in 0u8..2,
        suffix in "[a-z]{0,5}"
    ) {
        let sep = if sep_type == 0 { '\t' } else { '\n' };
        let input = format!("{prefix}{sep}{suffix}");
        let result = sanitize_catalog_string(&input);
        prop_assert_eq!(result, input);
    }
}

/// Property 12: Generate strings with SGR sequences and other escape sequences,
/// verify sanitize_subprocess_output preserves SGR and strips others.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn sgr_sequences_preserved(
        params in "[0-9;]{0,10}",
        text in "[a-zA-Z ]{1,20}"
    ) {
        let input = format!("\x1B[{params}m{text}\x1B[0m");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(result.contains(&text), "text should be preserved in: {:?}", result);
        prop_assert!(result.contains(&format!("\x1B[{params}m")), "SGR should be preserved");
    }

    #[test]
    fn non_sgr_csi_stripped(
        params in "[0-9;]{0,5}",
        final_byte in prop::sample::select(vec!['A', 'B', 'C', 'D', 'H', 'J', 'K']),
        text in "[a-zA-Z]{1,10}"
    ) {
        let seq = format!("\x1B[{params}{final_byte}");
        let input = format!("{seq}{text}");
        let result = sanitize_subprocess_output(&input);
        // The CSI sequence should be stripped, text preserved
        prop_assert!(result.contains(&text), "text should be preserved in: {:?}", result);
        prop_assert!(!result.contains('\x1B'), "escape should be stripped: {:?}", result);
    }

    #[test]
    fn osc_sequences_stripped(
        osc_content in "[a-zA-Z0-9;]{0,20}",
        text in "[a-zA-Z]{1,10}"
    ) {
        let input = format!("\x1B]{osc_content}\x07{text}");
        let result = sanitize_subprocess_output(&input);
        prop_assert!(result.contains(&text), "text should be preserved in: {:?}", result);
        prop_assert!(!result.contains('\x1B'), "escape should be stripped: {:?}", result);
    }
}
