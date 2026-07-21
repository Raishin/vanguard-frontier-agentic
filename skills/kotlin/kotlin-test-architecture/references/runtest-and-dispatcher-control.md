# runTest And Dispatcher Control

How runTest, StandardTestDispatcher, UnconfinedTestDispatcher, and Main-dispatcher overrides establish deterministic tests.

- `runTest` (kotlinx-coroutines-test) runs a suspend test body on a test coroutine scope and automatically skips real `delay` calls using virtual time, whereas `runBlocking` executes real delays and provides no virtual-time control.
- `StandardTestDispatcher` queues coroutines for execution and requires the test to explicitly call `advanceUntilIdle()`, `runCurrent()`, or `advanceTimeBy()` to progress them before an assertion is valid.
- `UnconfinedTestDispatcher` runs launched coroutines eagerly up to their first suspension point, which is convenient but can mask ordering bugs a `StandardTestDispatcher` test would catch.
- `Dispatchers.setMain(dispatcher)` overrides the Main dispatcher for a test and must be paired with `Dispatchers.resetMain()` in teardown, or the override leaks into later tests.

## Sources

- https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-test/
