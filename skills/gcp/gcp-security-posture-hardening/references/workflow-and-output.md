# Workflow and output contract

Use this reference only when performing the full posture review, implementation guidance, compliance triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- SCC tier (Standard vs. Premium) and finding coverage gaps
- SCC finding inventory: CRITICAL, HIGH, MEDIUM, LOW by category (IAM, networking, storage, logging, compute)
- CIS GCP Benchmark v2.0 controls: IAM, logging, networking, VMs, storage, Kubernetes
- Org policy baseline: key creation, domain restriction, public IP, OS Login, audit log config
- Assured Workloads: folder-level compliance boundary, framework (FedRAMP/HIPAA/IL4), data residency
- Binary Authorization: policy mode (enforced/dry-run/disabled), attestors, attestation chain
- VPC Service Controls: present or absent, scope (refer to VPC-SC agent for deep analysis)

## Safe workflow

1. **Frame scope**
   - Org/folder/project and environment (prod/staging/dev):
   - Compliance framework (FedRAMP, HIPAA, PCI, SOC2, none):
   - Business criticality and owner:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized SCC findings export, `gcloud organizations get-iam-policy`, `gcloud resource-manager org-policies list`, or Terraform state exports.
   - Otherwise inspect repository IaC/config or structured user descriptions.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What CRITICAL/HIGH SCC findings exist and are they remediated?
   - Which CIS benchmark controls are failing and what is the blast radius?
   - Which org policies are absent and what do they protect against?
   - Is Binary Authorization enforced or in dry-run/disabled mode?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout (dry-run before enforcement), validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# GCP Security Posture Hardening: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- SCC tier:
- Confirmed:
- Unknown:
- Out of scope:
## SCC finding summary
| Severity | Count | Top categories | Remediated |
|---|---|---|---|
## CIS benchmark gaps
| Domain | Control | Status | Risk |
|---|---|---|---|
## Org policy baseline
| Constraint | Status | Risk if absent |
|---|---|---|
## Binary Authorization posture
- Policy mode:
- Attestors configured:
- Gap:
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
