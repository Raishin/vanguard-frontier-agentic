# Auto Loader Ingestion And Schema Evolution

Decision tree for Auto Loader vs COPY INTO, schema-evolution modes, and `_rescued_data` handling.

- Auto Loader tracks ingested files via checkpoint metadata in RocksDB, guaranteeing exactly-once file processing; two queries must never share one checkpoint location.
- Auto Loader supports two detection modes — directory listing (default, slower scans) and file notification (lower cloud cost at scale) — and the choice should align to ingestion volume and cloud-provider cost model.
- Schema inference samples the first 50 GB or first 1000 files (whichever is crossed first); files beyond the sample are not inspected for new columns unless schema is re-inferred explicitly.
- `cloudFiles.schemaEvolutionMode` has five values: `addNewColumns` (default, adds new columns), `addNewColumnsWithTypeWidening` (allows type widening for compatibility), `rescue` (moves mismatches to `_rescued_data`), `failOnNewColumns` (fails if new columns detected), and `none` (evolution off).
- The `_rescued_data` column captures type mismatches, missing columns, and case differences, preventing silent data loss — when schema is explicitly supplied, schema auto-evolution is blocked for all modes.
- COPY INTO suits ingestion of up to thousands of files; Auto Loader is designed for millions of files per hour.

## Sources

- https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/
- https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/schema
