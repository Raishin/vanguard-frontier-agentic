# Review Workflow And Output Contract

The task-reliability review workflow and the required output shape.

## Workflow

1. Identify the task framework, the broker/result backend, and every task with an external side effect.
2. For each side-effecting task, confirm idempotency (a dedup/idempotency key checked before the effect) given at-least-once delivery.
3. Check acknowledgement timing (`acks_late` vs early ack) matches idempotency, and that retries use bounded exponential backoff with a max-retries cap.
4. Check poison messages are dead-lettered after N attempts, and that task-and-database writes use a transactional outbox.
5. Check scheduling is single-fire and idempotent, and record every claim needing a real broker to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the task framework and broker assumed.
- Idempotency/duplicate-execution, ack-timing/retry, poison-message/dead-letter, and outbox/scheduling findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any delivery-count/duplicate-execution claim the user must confirm against a real broker.
