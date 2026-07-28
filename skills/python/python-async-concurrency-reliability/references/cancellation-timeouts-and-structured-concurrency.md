# Cancellation, Timeouts, And Structured Concurrency

Cancellation semantics, the timeout context manager, and TaskGroup supervision.

- `asyncio.CancelledError` inherits from `BaseException` (since Python 3.8) specifically so that a normal `except Exception` does not swallow it; catching it must be deliberate and must re-raise after cleanup.
- The `asyncio.timeout()` context manager (Python 3.11+) applies a deadline to its enclosed block, cancels the operations inside it on expiry, and raises `TimeoutError`, while code outside the block continues unaffected; `asyncio.wait_for` provides the equivalent per-await deadline.
- `asyncio.TaskGroup` (Python 3.11+) supervises child tasks: any non-`CancelledError` exception in a child cancels the remaining children and, on exit, the collected exceptions are raised together as an `ExceptionGroup`.

## Sources

- https://docs.python.org/3/library/asyncio-task.html#timeouts
- https://docs.python.org/3/library/asyncio-task.html#task-groups
