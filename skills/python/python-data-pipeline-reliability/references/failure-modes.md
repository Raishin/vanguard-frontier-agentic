# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A backfill triggered by `catchup=True` on a newly-deployed DAG re-sends every historical notification because the send task wasn't idempotent.
- A non-idempotent append task doubles a day's revenue rows after an operator manually retries a failed run.
- A fixed-window aggregation with no watermark drops a batch of late-arriving events that cross midnight, undercounting the metric.
- An upstream column rename silently nulls out a downstream join key because the consumer read the column positionally.
- A multi-hour Spark job with no checkpoint restarts from scratch after a worker failure and re-emits records already committed downstream.
