# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- Retrying all failed jobs after an outage re-sends every already-delivered notification and charge.
- A job that reports success but never actually applied its business effect passes review unnoticed without reconciliation.
- A non-idempotent payment job re-run after a timeout double-charges the customer.
- Confusing process completion with business completion hides a silent business failure behind a green job status.
- An unbounded, un-approved retry of a poison job repeats the same failure at scale.
