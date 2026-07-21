# Ktor Lifecycle And Graceful Shutdown

Which lifecycle events gate readiness and how the engine drains in-flight work.

- Ktor's application lifecycle publishes `ApplicationStarting`, `ApplicationStarted`, `ServerReady`, `ApplicationStopPreparing`, `ApplicationStopping`, and `ApplicationStopped` events; readiness signals and shutdown-time cleanup should hook these events rather than infer server state indirectly.
- The Netty and CIO engines each expose a configurable grace period/timeout that bounds how long in-flight connections are given to complete before the engine forces a stop; leaving this unconfigured defers to a framework default that has not been reviewed against real request latencies.
- Graceful shutdown should cancel and join the application-level coroutine scope alongside connection draining, so in-flight coroutine work is not abandoned mid-execution when the process exits.

## Sources

- https://ktor.io/docs/server-lifecycle.html
- https://ktor.io/docs/server-events.html
