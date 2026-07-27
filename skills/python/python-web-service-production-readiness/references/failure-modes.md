# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A blocking DB driver in an `async def` endpoint freezes all concurrent requests on that worker.
- A route missing an object-level authorization check lets a user read another tenant's record by ID (IDOR).
- A shutdown path that ignores SIGTERM drops in-flight requests on every rolling deploy.
- An exception handler that returns the stack trace leaks internal paths and secrets to the client.
- A durable email/payment placed in an in-process background task is lost when the worker restarts.
