# Review Workflow And Output Contract

The event-loop reliability review workflow and the required output shape.

## Workflow

1. Identify the concurrency model: which coroutines run on the event loop, and where work is offloaded to a thread or process pool.
2. Scan every coroutine for blocking calls (sync I/O, `time.sleep`, blocking clients, heavy CPU) and confirm each is offloaded via `run_in_executor` or replaced with an async client.
3. Check cancellation: `CancelledError` is never swallowed, cleanup is in `finally`, and `shield` is used only where justified.
4. Check every external await has a deadline, every task is supervised (awaited/`TaskGroup`), and fan-out is bounded by a semaphore or bounded queue.
5. Trace context propagation across `await` and executor boundaries, and record every timing/throughput claim that needs measurement.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the concurrency model assumed.
- Blocking-in-loop, cancellation/timeout, task-lifecycle, and backpressure/context-propagation findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any timing/throughput claim the user must confirm by measurement.
