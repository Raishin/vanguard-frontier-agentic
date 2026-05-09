# Workflow and output contract

Use this reference only when performing the full change impact analysis, dependency mapping, or safe sequencing review.

## Analysis domains

Check these areas before giving a verdict:
- Org policy cascade: constraint type (list vs. boolean), affected folder/project hierarchy, deny-override risk
- Shared VPC impact: host project identification, enumeration of all service projects, subnet/firewall/route change scope
- Service Account dependency chain: all IAM policy bindings, workload identity bindings, impersonation chains
- Cloud Asset Inventory coverage: confirming `roles/cloudasset.viewer` is available before dependency analysis
- VPC peering topology: direct peers only (non-transitive), route advertisement changes, firewall rule propagation
- Change sequencing: dependencies between steps, blast radius reduction through staged rollout
- Rollback design: reversibility of each change step, approval gates before irreversible actions

## Safe workflow

1. **Frame scope**
   - Change type and target resource:
   - Environment (prod/non-prod):
   - Business criticality and owner:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test blast radius**
   - What org folders and projects are in scope for org policy cascade?
   - What Shared VPC service projects are attached to the affected host project?
   - What workloads bind to or impersonate the affected Service Account?
   - What VPC peers are directly affected (not transitively)?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - Require explicit approval before irreversible changes (SA deletion, org policy enforcement mode).
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Change Impact Advisor: <change description>
## Executive verdict
- Status: SAFE TO PROCEED / PROCEED WITH GATES / HIGH RISK / NEEDS EVIDENCE
- Blast radius: <summary of affected resources and projects>
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Safe change sequencing
1. <step> — approval gate: <gate>, rollback: <rollback>
## Rollback plan
- <step rollback or explicit irreversible note>
## Residual risk
- <risk or explicit none>
```
