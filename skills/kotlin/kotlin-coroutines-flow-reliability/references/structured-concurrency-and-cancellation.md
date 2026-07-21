# Structured Concurrency And Cancellation

How scope choice and cancellation cooperation determine leak-freedom.

- `coroutineScope` cancels all children and rethrows on the first child failure (fail-fast); `supervisorScope` isolates child failures and still awaits siblings — choose by whether one failure should cancel the batch.
- `CancellationException` must be rethrown; swallowing it in a broad catch breaks cancellation propagation and orphans children.
- `isActive` is a non-throwing check for loops; `ensureActive()` throws immediately on cancellation; `yield()` suspends and re-checks — long CPU work must call one of them to stay cancellable.
- `GlobalScope.launch` has no lifecycle owner and leaks work; bind launches to a scope cancelled with the consumer.

## Sources

- https://kotlinlang.org/docs/coroutines-basics.html
- https://kotlinlang.org/docs/cancellation-and-timeouts.html
