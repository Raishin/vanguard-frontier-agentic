# Web-Service Readiness Review Checklist

The per-concern checklist applied to every web-service review.

- Event loop: no blocking call in an `async def` endpoint; blocking work uses `def` (threadpool) or is offloaded.
- Validation: every path/query/body/header input is validated by the framework's schema; unknown/oversized input is rejected.
- Authz: authentication and object-level authorization are enforced per-route for every method (no IDOR).
- Middleware: auth/CORS/exception middleware are correctly ordered; no stack-trace leak; no permissive CORS with credentials.
- Lifecycle: request and upstream timeouts exist; the worker class matches the workload; SIGTERM drains in-flight requests.
- Health: readiness reflects real dependency and warmup state, not a static 200.
