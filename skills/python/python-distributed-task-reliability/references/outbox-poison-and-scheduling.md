# Transactional Outbox, Poison Messages, And Scheduling

Coordinating task-and-write consistency, dead-lettering, and safe scheduling.

- Enqueuing a task inside a database transaction is unsafe: if the transaction rolls back the task still runs on absent/stale data, and if the commit succeeds but the enqueue fails the work is lost — the transactional-outbox pattern persists the task intent in the same transaction and a separate relay publishes it, giving at-least-once delivery consistent with the write.
- A poison message (one that always fails) must have a stop condition: after a bounded number of attempts it is routed to a dead-letter/parking queue and alerted, rather than retried indefinitely.
- A periodic (beat) schedule must have a single active scheduler and idempotent tasks, because a schedule fired by more than one scheduler, or a beat task that double-fires, produces duplicate side effects.

## Sources

- https://docs.celeryq.dev/en/stable/faq.html
- https://docs.celeryq.dev/en/stable/userguide/configuration.html
