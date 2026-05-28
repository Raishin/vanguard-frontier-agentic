use proptest::prelude::*;
use proptest::test_runner::Config;

/// Property 15: Generate lists of string IDs with mixed case, verify sort is
/// case-insensitive and stable.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn case_insensitive_sort_is_ordered(
        ids in proptest::collection::vec("[a-zA-Z0-9_-]{1,20}", 2..50)
    ) {
        let mut sorted = ids.clone();
        sorted.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        // Verify the result is actually sorted case-insensitively
        for window in sorted.windows(2) {
            prop_assert!(
                window[0].to_lowercase() <= window[1].to_lowercase(),
                "not sorted: {:?} > {:?}",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn case_insensitive_sort_preserves_all_elements(
        ids in proptest::collection::vec("[a-zA-Z0-9_-]{1,20}", 1..50)
    ) {
        let mut sorted = ids.clone();
        sorted.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        // Same number of elements
        prop_assert_eq!(sorted.len(), ids.len());

        // Same multiset of elements
        let mut original_sorted = ids.clone();
        original_sorted.sort();
        let mut result_sorted = sorted.clone();
        result_sorted.sort();
        prop_assert_eq!(original_sorted, result_sorted);
    }
}
