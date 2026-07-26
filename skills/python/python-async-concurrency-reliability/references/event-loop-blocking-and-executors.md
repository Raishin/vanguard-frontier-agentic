# Event-Loop Blocking And Executors

Why blocking calls stall the loop and how run_in_executor offloads them.

- The event loop is single-threaded: a synchronous blocking call inside a coroutine suspends the loop itself, so no other task can make progress until it returns.
- `loop.run_in_executor(None, fn, *args)` runs a blocking callable in the default thread-pool executor and returns an awaitable; a custom `ThreadPoolExecutor` suits blocking I/O and a `ProcessPoolExecutor` suits CPU-bound work.
- Offloading does not make a non-thread-safe object safe: any object shared with an executor thread must itself be thread-safe, and results must be awaited so exceptions surface.

## Sources

- https://docs.python.org/3/library/asyncio-eventloop.html#asyncio.loop.run_in_executor
- https://docs.python.org/3/library/asyncio-dev.html
