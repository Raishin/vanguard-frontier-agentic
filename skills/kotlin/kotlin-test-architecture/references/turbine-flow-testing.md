# Turbine Flow Testing

How Turbine's test idiom consumes Flow emissions and what an unconsumed event means.

- Turbine's `test { }` extension on a Flow collects emissions inside a coroutine and provides `awaitItem()`/`awaitComplete()`/`awaitError()` assertions with a configurable timeout that defaults to a finite value — so a test relying on the default timeout is fine — replacing manual `toList()`/`first()` collection in tests.
- Every emission a Turbine `test {}` block receives must be consumed (via `awaitItem()`) or explicitly discarded (`cancelAndIgnoreRemainingEvents()`) before the block ends, or Turbine fails the test with an unconsumed-events error rather than silently passing.

## Sources

- https://github.com/cashapp/turbine
