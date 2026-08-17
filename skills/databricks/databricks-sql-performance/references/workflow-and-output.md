# Workflow And Output

Diagnostic sequence and output contract for SQL performance review.

## Workflow

1. Establish the warehouse type (serverless, pro, classic) and current configuration — refuse-and-ask if missing.
2. Collect a query profile (wall-clock, task times, memory, spill, shuffle, task-duration percentiles) or flag if unavailable.
3. Analyse task-duration skew: maximum > 75th percentile + 50% indicates data skew — identify the skewed stage and join or grouping operation.
4. Check data layout: is the table liquid-clustered on the filter/join columns, or is Z-ORDER or partitioning in use? Recommend liquid clustering for all new tables.
5. Verify data-skipping: first 32 columns are indexed by default; flag filter columns beyond position 32 or low-selectivity columns in the index.
6. Review cache status: UI cache (7-day), remote result cache (24-hour, schema-invalidated), local disk cache (per-node, invalidates on file change). Flag stale results.
7. Confirm warehouse type supports the optimization (Photon all types; Predictive I/O serverless/pro only; IWM serverless only).

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (pass / pass-with-conditions / block) and warehouse type assumed.
- Cache, data-layout, query-profile, and materialized-view findings, each with evidence-basis labels.
- Severity-labelled findings (critical / high / medium / low) and safe next actions.
- Any warehouse type, tier, or evidence gaps that would change the verdict.
