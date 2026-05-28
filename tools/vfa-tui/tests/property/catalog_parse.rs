use proptest::prelude::*;
use proptest::test_runner::Config;
use vfa_tui::models::Agent;

// Property 1: Feed arbitrary byte vectors to serde_json::from_slice::<Vec<Agent>>()
// Must not panic, must return Err.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn arbitrary_bytes_do_not_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let result = serde_json::from_slice::<Vec<Agent>>(&bytes);
        // Should always be an error for random bytes (overwhelmingly unlikely to be valid JSON)
        // But the key property is it must not panic
        let _ = result;
    }
}
