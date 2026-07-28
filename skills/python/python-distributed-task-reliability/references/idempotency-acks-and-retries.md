# Idempotency, Acknowledgements, And Retries

At-least-once delivery, ack timing, and safe retry policy in Celery.

- Celery's documentation states that task functions should ideally be idempotent — callable multiple times with the same arguments without unintended side effects — because delivery is at-least-once.
- By default Celery acknowledges a message just before execution to prevent re-execution of a started task; setting `acks_late=True` acknowledges after execution, so a worker crash mid-task re-delivers it and the task may run multiple times — which is safe only for idempotent tasks (`task_acks_late` with `worker_prefetch_multiplier=1` is the documented pattern for safely-retriable tasks).
- Automatic retries should use exponential backoff (`retry_backoff=True`, jitter on by default) with a bounded max-retries, and only for expected transient errors, to avoid overwhelming a failing dependency.

## Sources

- https://docs.celeryq.dev/en/stable/userguide/tasks.html
- https://docs.celeryq.dev/en/stable/userguide/optimizing.html
