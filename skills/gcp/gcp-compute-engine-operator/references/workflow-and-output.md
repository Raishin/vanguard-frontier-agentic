# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Instance inventory (type, zone, image, service account, scopes)
- MIG configuration (stateless vs. stateful, auto-scaling policy, auto-healing health check)
- OS patch compliance (VM Manager patch job status, OS Config agent health)
- Cost profile (spot/preemptible opportunities, right-sizing, committed use discounts)
- Security posture (OS Login enabled, external IPs, overly broad service account scopes)
- Startup/shutdown scripts (idempotency, error handling, logging)

## Safe workflow

1. **Frame scope**
   - Project/zone/region and instance or MIG name:
   - Workload type (stateless/stateful, batch/serving):
   - Patch compliance requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which instances have external IPs that should be private?
   - Which instances use legacy metadata SSH keys instead of OS Login?
   - Are patch jobs scheduled and completing successfully?
   - Are spot VMs used for workloads that cannot tolerate preemption?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Compute Engine Operator: <scope>
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
