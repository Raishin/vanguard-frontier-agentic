# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Pipeline topology (streaming vs. batch, bounded vs. unbounded sources)
- Dataflow job health (auto-scaling, worker errors, stale watermarks)
- Pub/Sub subscription lag, dead letter topic configuration, and message schema
- Dataproc cluster lifecycle (ephemeral vs. long-running, version, autoscaling)
- Cloud Composer environment version, DAG health, task failure rate
- Dataplex zones, assets, data quality rules, and access policies
- Cost profile (Dataproc cluster hours, Dataflow worker scaling, Pub/Sub throughput)

## Safe workflow

1. **Frame scope**
   - Project/region and pipeline name:
   - Pipeline type (streaming/batch):
   - SLA (latency, throughput, data freshness):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Are Pub/Sub subscriptions missing dead letter topics?
   - Are Dataproc clusters long-running when ephemeral would suffice?
   - Are Dataflow jobs experiencing stale watermarks or worker errors?
   - Are Composer DAGs retrying silently without alerting?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Data Pipeline Engineer: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
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
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
