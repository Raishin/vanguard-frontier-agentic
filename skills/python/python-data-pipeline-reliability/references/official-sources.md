# Official Sources

Primary Airflow and Spark documentation and Context7 provenance.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://airflow.apache.org/docs/apache-airflow/stable/core-concepts/tasks.html
- https://airflow.apache.org/docs/apache-airflow/stable/authoring-and-scheduling/catchup.html
- https://airflow.apache.org/docs/apache-airflow/stable/best-practices.html
- https://spark.apache.org/docs/latest/

## Provenance notes

- airflow.apache.org and spark.apache.org are the authoritative upstreams for the idempotency, catchup, and partitioning claims here; Dagster and Prefect behaviour must be confirmed against their own documentation when the pipeline uses them.
- Context7 MCP provenance — library ID `/websites/airflow_apache` (Apache Airflow, source reputation High), retrieved 2026-07-26. Query: idempotent/deterministic tasks, catchup/backfill, retries, resumable execution. Confirmed: Airflow tasks should be idempotent; operators support retries with exponential backoff; the ResumableJobMixin reconnects to an in-flight external job on retry, with a documented window between job submission and state persistence where a worker failure can cause a fresh submission. Limitation: catchup/scheduler behaviour differs across Airflow 2.x/3.x — the applicable version must be confirmed from the user's environment.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
