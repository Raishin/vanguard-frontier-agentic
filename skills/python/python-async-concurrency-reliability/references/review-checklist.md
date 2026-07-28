# Async Reliability Review Checklist

The per-concern checklist applied to every asyncio review.

- No coroutine performs a synchronous blocking call; blocking work is offloaded via `run_in_executor` or replaced with an async client.
- `CancelledError` is never swallowed; cleanup runs in `finally` and re-raises.
- Every external await (network, DB, subprocess) is wrapped in `asyncio.timeout()` or `wait_for` with a deadline.
- Every task is supervised: awaited, referenced, or created inside a `TaskGroup`; no exception is silently discarded.
- Fan-out is bounded by an `asyncio.Semaphore` or a bounded queue sized to downstream capacity.
- Trace/log/security context is explicitly propagated across executor and thread boundaries.
