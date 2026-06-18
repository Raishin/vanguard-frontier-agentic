# Workflow and output contract

Use this reference only when performing the full Microsoft Fabric analytics engineering review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Dimensional modeling: fact table granularity, surrogate key presence, dimension key integrity, degenerate dimensions, role-playing dimensions, bridge tables for many-to-many, SCD strategy
- Fabric Data Warehouse T-SQL: DDL correctness, missing column statistics, implicit type conversions, anti-patterns (SELECT *, cross-join without filter, missing WHERE on large scans), materialized views, stored procedures, COPY INTO usage
- Semantic model storage mode: Direct Lake vs Import vs DirectQuery selection rationale, framing behavior, V-Order dependency, DirectQuery fallback risk (SQL views in Direct Lake), composite model design
- Relationship quality: cardinality correctness (1:*, *:1, *:*), cross-filter direction (prefer single), inactive relationships, role-playing dimension handling, bridge table patterns
- DAX measure quality: explicit vs implicit measures, filter context isolation (CALCULATE, ALL, REMOVEFILTERS), iterator correctness (SUMX vs SUM), variable usage, calculation groups, dynamic format strings, time-intelligence correctness
- Data preparation: T-SQL view and stored-procedure quality as transformation layer, Dataflows Gen2 prep for analytics, deduplication and null handling, star-schema load patterns
- Reusable semantic model design: shared certified model structure, Build-permission readiness, single-source-of-truth design as a build concern (distinct from governance enforcement)

## Safe workflow

1. **Frame scope**
   - Warehouse tables/views, semantic model, or DAX measures in scope:
   - Required outcome (model correctness / DAX quality / warehouse design / Direct Lake readiness / shared-model design):
   - Available evidence (DDL scripts, PBIP/TMDL metadata, DAX measure definitions, query plans):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer user-provided sanitized DDL, semantic model metadata (PBIP or TMDL format), or DAX measure definitions.
   - Otherwise use official Microsoft Learn documentation to assess likely behavior.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test correctness and performance**
   - Which fact tables lack surrogate keys, have wrong granularity, or embed dimension attributes (denormalization)?
   - Which DAX measures use implicit aggregation, incorrect filter context, or calculated columns for aggregation?
   - Which Direct Lake models use SQL views that will trigger DirectQuery fallback?
   - Which relationships have incorrect cardinality or bidirectional cross-filter that creates ambiguous paths?
   - Which warehouse queries carry implicit type conversions, missing statistics, or anti-pattern joins?

4. **Recommend the smallest safe action**
   - Prefer non-breaking changes: add explicit measures, fix relationship direction, add surrogate key columns to dimension tables.
   - Production warehouse DDL changes and semantic-model deployment require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# Fabric Analytics Engineering Review: <scope>
## Analytics engineering verdict
- Status: SOUND / SOUND WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or tests to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
