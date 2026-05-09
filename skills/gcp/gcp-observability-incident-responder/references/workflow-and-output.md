# Workflow and output contract

Use this reference only when performing the full incident review, observability design, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Incident scope: service affected, timeline, user-visible impact, blast radius
- Cloud Monitoring: alerting policy types, SLO burn rate alert coverage, dashboard scope
- Cloud Logging: log sink configuration (GCS/BigQuery/Pub/Sub), log-based metrics, retention
- Cloud Trace: sampling rate, trace coverage for affected services, latency breakdown
- Error Reporting: structured logging with stack traces, aggregation correctness
- SLO status: error budget remaining, burn rate, window type (fast/slow burn)
- Hypothesis: evidence strength per hypothesis, confirmed vs. inferred causation

## Safe workflow

1. **Frame scope**
   - Project/region/environment/service:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What can expose data or escalate the incident scope?
   - What log sinks are missing that create compliance gaps?
   - What evidence is missing that prevents root cause determination?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Observability Incident Responder: <scope>
## Executive verdict
- Status: CONTAINED / ACTIVE / INVESTIGATING / NEEDS EVIDENCE
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
