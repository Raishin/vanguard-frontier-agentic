# Review Workflow And Output Contract

The production-readiness review workflow and the required output shape.

## Workflow

1. Detect the framework and server model (ASGI vs WSGI, worker class) and load the matching framework reference only if needed.
2. Check every endpoint for a blocking call in an `async def`, and confirm sync vs async is chosen deliberately.
3. Check request validation and per-route authentication/authorization (including object-level authorization) at every method.
4. Check middleware order, error handling, and CORS; check timeouts, worker model, and graceful shutdown on SIGTERM.
5. Check health/readiness reflects real capacity, and record every claim needing runtime confirmation.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the framework and server model detected.
- Async/sync-blocking, validation/authz, middleware/error-handling, and worker/timeout/shutdown/health findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any runtime behavior the user must confirm.
