# Task-Reliability Review Checklist

The per-concern checklist applied to every task-system review.

- Idempotency: every side-effecting task is guarded by a dedup/idempotency key checked before the effect.
- Acks: acknowledgement timing matches the work; `acks_late` is only on idempotent tasks.
- Retries: bounded exponential backoff with jitter and a max-retries cap; only transient errors auto-retry.
- Poison: an always-failing message is dead-lettered after N attempts with an alert, not retried forever.
- Outbox: task enqueue and database write are coordinated (transactional outbox), never a bare enqueue inside a transaction.
- Scheduling: periodic tasks are single-fire and idempotent; no logic assumes exactly-once or ordered delivery.
