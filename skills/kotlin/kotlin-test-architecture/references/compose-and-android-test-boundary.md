# Compose UI Testing And The Robolectric/Instrumented Boundary

How Compose UI tests should assert, and when a local vs instrumented test is the right choice.

- Compose UI tests use `createComposeRule()` (or `createAndroidComposeRule()`) and are documented to assert via semantics — matchers like `onNodeWithText`/`onNodeWithTag` — rather than view-tree structure, and the test framework synchronizes with Compose's own idling/recomposition state.
- Android local (Robolectric-based) tests run on the JVM without a device/emulator and are documented as the faster path for logic that does not need real device/hardware behavior; instrumented tests run on a device or emulator and are reserved for behavior that genuinely depends on it.

## Sources

- https://developer.android.com/develop/ui/compose/testing
- https://developer.android.com/training/testing/local-tests
