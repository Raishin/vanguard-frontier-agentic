# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A non-idempotent charge task with `acks_late` double-charges a customer when a worker crashes after the charge but before the ack.
- A no-backoff retry against a down dependency turns one outage into a self-inflicted retry storm.
- A poison message with infinite retry pins a worker forever and blocks the queue.
- A task enqueued inside a transaction that rolls back runs on data that never existed.
- A beat task firing on two workers sends every scheduled email twice.
