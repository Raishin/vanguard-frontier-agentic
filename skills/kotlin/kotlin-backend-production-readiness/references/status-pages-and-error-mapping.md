# StatusPages And Typed Error Mapping

How centralized exception handling shapes client-visible errors and shutdown-time resource discipline.

- The `StatusPages` plugin installs a centralized handler that maps exception types and status codes to a defined response body, preventing an unhandled exception from leaking a raw stack trace or falling through to an ambiguous default error.
- A single overly broad exception handler collapses distinct failure classes (client error, downstream dependency failure, genuine defect) into one response shape, which degrades the operational signal available for triage and alerting.
- DI-managed and `AutoCloseable` resources (connection pools, schedulers) should be registered for closure during the application's stop-preparing/stopping lifecycle rather than relying on JVM shutdown-hook ordering, which is not guaranteed to run before the process is forcibly terminated.

## Sources

- https://ktor.io/docs/server-status-pages.html
- https://ktor.io/docs/server-lifecycle.html
