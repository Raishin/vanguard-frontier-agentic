# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Cluster type (Standard vs. Autopilot), Kubernetes version, and release channel
- Node pool inventory (machine type, count, version, auto-upgrade, auto-repair)
- Workload Identity: every workload SA annotation vs. mounted key files
- Binary Authorization: policy mode (WARN vs. ENFORCE), attestor configuration
- Network policies enabled and Pod Security Standards enforced
- RBAC: no cluster-admin bindings for non-bootstrap accounts
- Logging and monitoring: Cloud Logging and Cloud Monitoring enabled

## Safe workflow

1. **Frame scope**
   - Project/cluster name/region:
   - Cluster type (Standard/Autopilot):
   - Kubernetes version and release channel:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which pods use mounted SA key files instead of Workload Identity?
   - Is Binary Authorization in WARN or ENFORCE mode? Are all production images attested?
   - Are node pools on a version more than 2 minor versions behind the cluster master?
   - Are network policies deployed to all namespaces?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP GKE Platform Operator: <scope>
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
