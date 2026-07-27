# Schema Evolution, Late Data, And Data Quality

Data contracts, watermarking for late-arriving data, and quality/lineage gates.

- Event-time jobs must handle late/out-of-order data via a watermark or reprocessing window or they drop/misassign rows.
- Schema evolution needs an explicit data contract, not positional/implicit access.
- Data-quality gates and lineage at boundaries stop bad data propagating.

## Sources

- https://airflow.apache.org/docs/apache-airflow/stable/core-concepts/tasks.html
- https://spark.apache.org/docs/latest/
