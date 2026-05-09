# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Service/function inventory (region, revision count, current traffic split)
- Revision health (container startup time, error rate, latency p99)
- Cold-start profile (min-instances setting vs. latency SLA)
- Concurrency settings (max-instances, max-concurrency, CPU allocation mode)
- VPC connectivity (Direct VPC Egress vs. VPC connector vs. no VPC)
- Eventarc triggers (source, filter, delivery guarantees)
- IAM (invoker bindings, service account least privilege)

## Safe workflow

1. **Frame scope**
   - Project/region and service or function name:
   - Traffic split and active revisions:
   - Latency SLA and cold-start tolerance:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Are latency-sensitive services running without minimum instances?
   - Are background services missing always-on CPU?
   - Are services that need private VPC access missing Direct VPC Egress?
   - Are all invoker bindings scoped to the minimum necessary identities?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Cloud Run and Functions Operator: <scope>
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
