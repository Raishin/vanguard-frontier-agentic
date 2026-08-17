# Workflow And Output

Pipeline-engineering review sequence and output contract.

## Workflow

1. Establish the pipeline's current source files (Python or SQL), table names, medallion tier, and refresh cadence — refusal-trigger if missing.
2. Validate medallion-layer boundaries against the domain and data ownership; flag tier misalignment or implicit ownership.
3. Audit Lakeflow Jobs orchestration: confirm the DAG is acyclic, dependencies are explicit in the job definition, and single-task pipelines are justified.
4. Review Delta table layout: confirm new tables use liquid clustering (not Z-order), check deletion-vector eligibility (DBR 15.4+), and verify Predictive Optimization aligns to the tier.
5. Assess Auto Loader usage: confirm ingestion volume justifies Auto Loader vs COPY INTO, validate `cloudFiles.schemaEvolutionMode` selection, and confirm `_rescued_data` is monitored.
6. Validate materialized-view vs streaming-table selection against refresh cadence and state-schema immutability (if applicable).
7. For backfill designs: confirm checkpoint isolation from incremental pipelines, state-schema immutability across restarts, and partial-backfill correctness.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (align / align-with-design-changes / redesign required) and the scope of this review.
- Medallion-layer, Lakeflow Jobs, Delta table-layout, Auto Loader, and materialized-view vs streaming-table findings.
- A severity-labelled finding list (critical / high / medium / low), each with evidence basis, and safe next actions for the user.
