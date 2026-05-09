# Workflow and output contract

Use this reference only when performing the full fleet review, policy audit, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Fleet inventory: cluster count by type (GKE/AWS/Azure/bare metal), registration status, Connect Agent health
- Config Management: sync status per cluster, Git repository source, last sync timestamp, error count
- Policy Controller: constraint templates deployed, violations by severity, audit vs. enforce mode per constraint
- Service mesh (ASM): mTLS mode (STRICT/PERMISSIVE), proxy injection coverage, traffic policy state
- Multi-cloud connectivity: Connect Gateway access, cross-cluster routing, network policy gaps
- Fleet IAM: fleet-level bindings, cluster-scoped vs. fleet-scoped permissions
- Workload identity: Workload Identity Federation configuration for non-GCP clusters

## Safe workflow

1. **Frame scope**
   - Fleet name/GCP project:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live gcloud / kubectl / ACM read-only evidence if available.
   - Otherwise inspect repository GitOps configs, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What Policy Controller constraints are in audit mode (detecting but not blocking)?
   - What clusters have Config Management sync errors (drift state)?
   - What clusters expose the Kubernetes API server directly to the internet?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Anthos Multicloud Architect: <scope>
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
