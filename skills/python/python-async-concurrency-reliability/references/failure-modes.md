# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A single blocking DB call in a hot coroutine freezes every concurrent request on that worker until it returns.
- A swallowed `CancelledError` turns a graceful-shutdown deadline into a hung pod that the orchestrator eventually kills.
- A missing timeout on an upstream call lets one slow dependency exhaust the worker pool and cascade into an outage.
- A fire-and-forget `create_task` loses its exception, so a persistent failure runs silently until the backlog is discovered downstream.
- Unbounded `gather` over a large input allocates every coroutine at once and OOM-kills the process.
