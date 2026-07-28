# Official Sources

Primary Celery documentation and Context7 provenance for the delivery/idempotency claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.celeryq.dev/en/stable/userguide/tasks.html
- https://docs.celeryq.dev/en/stable/userguide/optimizing.html
- https://docs.celeryq.dev/en/stable/faq.html
- https://docs.celeryq.dev/en/stable/userguide/configuration.html

## Provenance notes

- docs.celeryq.dev is the authoritative upstream for Celery; RQ and Dramatiq behaviour must be confirmed against their own documentation when the code uses them.
- Context7 MCP provenance — library ID `/websites/celeryq_dev_en_stable` (source reputation High), retrieved 2026-07-26. Query: acks_late at-least-once delivery requiring idempotent tasks; retry with backoff; duplicate execution. Confirmed: tasks should be idempotent; default early-ack prevents re-execution; `acks_late` re-executes on worker crash (idempotent tasks only); `retry_backoff=True` exponential backoff with jitter; `worker_prefetch_multiplier=1` for safely-retriable tasks. Limitation: exactly-once is not provided by the broker; the applicable Celery/broker version must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
