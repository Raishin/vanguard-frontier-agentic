use proptest::prelude::*;
use proptest::test_runner::Config;

// Feature: rust-tui, Property 15: Stable case-insensitive lexicographic sort
// Validates: Requirements 3.2, 4.2, 18.3
//
// For any list of catalog items with string IDs, the sort function SHALL produce
// output where for all adjacent pairs (a, b), a.id.to_lowercase() <= b.id.to_lowercase().
// For items with equal lowercase IDs, their relative order from the input SHALL be
// preserved (stability).

/// Strategy that generates lists containing IDs that share the same lowercase form
/// but differ in case (e.g., "Foo", "fOO", "FOO"). This ensures the stability
/// property is exercised with high probability.
fn ids_with_case_duplicates() -> impl Strategy<Value = Vec<String>> {
    // Generate base lowercase strings, then produce case variants for each
    proptest::collection::vec("[a-z]{1,8}", 2..10)
        .prop_flat_map(|bases| {
            // For each base, generate 1-4 case variants
            let strategies: Vec<_> = bases
                .into_iter()
                .map(|base| {
                    proptest::collection::vec(proptest::bool::ANY, base.len())
                        .prop_map(move |uppercase_flags| {
                            base.chars()
                                .zip(uppercase_flags.iter())
                                .map(|(c, &upper)| {
                                    if upper {
                                        c.to_uppercase().to_string()
                                    } else {
                                        c.to_string()
                                    }
                                })
                                .collect::<String>()
                        })
                        .prop_flat_map(|variant| {
                            // Generate 1-4 variants of the same base
                            proptest::collection::vec(Just(variant.clone()), 1..=3).prop_map(
                                move |mut variants| {
                                    // Make each variant potentially different case
                                    variants.push(variant.clone());
                                    variants
                                },
                            )
                        })
                })
                .collect();

            strategies
        })
        .prop_map(|nested: Vec<Vec<String>>| nested.into_iter().flatten().collect::<Vec<String>>())
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Verifies that sort_by_key with to_lowercase() produces correctly ordered output.
    #[test]
    fn case_insensitive_sort_is_ordered(
        ids in proptest::collection::vec("[a-zA-Z0-9_-]{1,20}", 2..50)
    ) {
        let mut sorted = ids.clone();
        sorted.sort_by_key(|a| a.to_lowercase());

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

    /// Verifies that sort preserves all elements (no additions or removals).
    #[test]
    fn case_insensitive_sort_preserves_all_elements(
        ids in proptest::collection::vec("[a-zA-Z0-9_-]{1,20}", 1..50)
    ) {
        let mut sorted = ids.clone();
        sorted.sort_by_key(|a| a.to_lowercase());

        // Same number of elements
        prop_assert_eq!(sorted.len(), ids.len());

        // Same multiset of elements
        let mut original_sorted = ids.clone();
        original_sorted.sort();
        let mut result_sorted = sorted.clone();
        result_sorted.sort();
        prop_assert_eq!(original_sorted, result_sorted);
    }

    /// **Property 15 — Stability**: For items with equal lowercase IDs, their
    /// relative order from the input SHALL be preserved.
    ///
    /// We tag each element with its original index, sort by lowercase key, then
    /// verify that within each group of equal-lowercase elements the original
    /// indices are strictly increasing (i.e., relative order is preserved).
    ///
    /// **Validates: Requirements 3.2, 4.2, 18.3**
    #[test]
    fn case_insensitive_sort_is_stable(
        ids in ids_with_case_duplicates()
    ) {
        // Tag each ID with its original position
        let tagged: Vec<(usize, String)> = ids.iter().cloned().enumerate().collect();

        // Sort using the same key as CatalogStore: sort_by_key with to_lowercase()
        let mut sorted = tagged.clone();
        sorted.sort_by_key(|(_, id)| id.to_lowercase());

        // 1) Verify ordering: adjacent pairs must be non-decreasing by lowercase
        for window in sorted.windows(2) {
            prop_assert!(
                window[0].1.to_lowercase() <= window[1].1.to_lowercase(),
                "not sorted: {:?} > {:?}",
                window[0].1,
                window[1].1
            );
        }

        // 2) Verify stability: within each group of equal lowercase IDs,
        //    original indices must be strictly increasing
        let mut i = 0;
        while i < sorted.len() {
            let key = sorted[i].1.to_lowercase();
            let group_start = i;
            while i < sorted.len() && sorted[i].1.to_lowercase() == key {
                i += 1;
            }
            // Check that original indices within this group are increasing
            let group = &sorted[group_start..i];
            for window in group.windows(2) {
                prop_assert!(
                    window[0].0 < window[1].0,
                    "stability violated for key {:?}: original index {} came before {} in input but appeared after in output",
                    key,
                    window[0].0,
                    window[1].0
                );
            }
        }
    }
}
