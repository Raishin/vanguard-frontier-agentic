# Workflow and output contract

Use this reference only when performing the full MLOps review, cost audit, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Training job inventory: status, accelerator type, estimated cost, max_run_time presence
- Pipeline execution: component status, container charges, failed runs, retry behavior
- Model Registry: version count, deployed versions, aliases, deprecation policy
- Endpoints: traffic split percentages, latency percentiles, autoscaling config
- Feature Store: online/offline sync lag, feature freshness, write validation gates
- Gemini API: which endpoint (Vertex AI vs. AI Studio), SLA tier, data privacy commitment
- IAM: service account permissions, Workload Identity Federation, over-privilege gaps

## Safe workflow

1. **Frame scope**
   - Project/region/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/SDK read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What training jobs lack max_run_time and could hang indefinitely?
   - What Feature Store writes lack validation and could silently corrupt training data?
   - What IAM bindings grant broader access than needed?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Vertex AI MLOps Engineer: <scope>
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
