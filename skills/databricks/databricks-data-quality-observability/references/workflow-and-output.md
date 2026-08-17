# Workflow And Output

Data-quality and observability review sequence and output contract.

## Workflow

1. Establish the pipeline's source, target tables, and known quality requirements — refuse if missing.
2. Audit expectations: identify all expectations, their violation modes (warn/drop/fail), and whether the modes match the risk profile.
3. Audit table constraints: confirm NOT NULL and CHECK are declared (enforced); confirm primary/foreign/unique are used only as informational hints, not relying on them for correctness.
4. Verify Lakehouse Monitoring: confirm a monitor is configured for each target table; check which columns are profiled; confirm drift metrics and distance measures align to the domain.
5. Assess freshness and staleness: confirm a monitor is configured for freshness anomaly detection; understand the learned staleness threshold; verify the SLA is based on learned baselines, not hardcoded wall-clock times.
6. Review event-log interrogation: identify which event types are queried for quality results; confirm `flow_progress` events are used for data-quality data; verify the pipeline's event log is retained and accessible (60-day retention).
7. Validate quality SLA and alerting: confirm the SLA is explicit and documented; verify alerting is configured when breached; confirm the SLA is communicated to downstream consumers.
8. Check downstream quality signaling: identify what quality metrics or constraint satisfaction is published to downstream consumers; verify evidence is actionable.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (compliant / compliant-with-enhancements / non-compliant-fix-required) and the scope of this review.
- Expectations, constraints, monitoring, freshness, event-log, and SLA findings.
- A severity-labelled finding list (critical / high / medium / low), each with evidence basis, and safe next actions for the user.
