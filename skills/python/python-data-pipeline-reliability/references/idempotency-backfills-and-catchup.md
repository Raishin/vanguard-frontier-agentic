# Idempotency, Backfills, And Catchup

Safe task idempotency, backfill sequencing, and catchup scheduling in Airflow.

- Airflow's guidance is that tasks be idempotent and deterministic so retries and backfills are safe.
- `catchup=True` backfills every missed interval on deploy, so non-idempotent/side-effecting tasks fire repeatedly — catchup must be deliberate.
- Safe re-run means overwrite-by-partition or merge-by-key, not blind append.

## Sources

- https://airflow.apache.org/docs/apache-airflow/stable/authoring-and-scheduling/catchup.html
- https://airflow.apache.org/docs/apache-airflow/stable/best-practices.html
