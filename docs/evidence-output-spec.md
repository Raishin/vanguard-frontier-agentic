# Evidence Output Specification

> Canonical response shape for all VFA live-guard and review agents, and its mapping to compliance framework controls.

## Purpose

Every live-guard and review agent in this repo produces a **structured verdict response**. This document defines the required fields and maps each field to the specific compliance controls it satisfies — so the response itself becomes an audit artifact without post-processing.

The five fields below are the minimum set required on every agent response. Agents may add provider-specific fields (e.g., `cluster_context`, `assignment_scope`) but must not omit required fields.

---

## Required Response Fields

| Field | Type | Description |
|---|---|---|
| `verdict` | `approved` \| `blocked` \| `needs-review` | Binary gate decision. |
| `evidence_level` | `verified` \| `partial` \| `assumed` | Confidence in the captured baseline. `verified` = live state confirmed via CLI read; `partial` = snapshot exists but may be stale; `assumed` = no current-state capture possible. |
| `blockers` | `string[]` | Each item is a named violation that must be resolved before the change is approved. Empty array if verdict is `approved`. |
| `safe_next_actions` | `string[]` | Ordered remediation steps if blocked, or post-approval verification steps if approved. |
| `open_questions` | `string[]` | Ambiguities requiring human clarification. May be empty. |

### Example — Blocked Response

```json
{
  "verdict": "blocked",
  "evidence_level": "verified",
  "blockers": [
    "verb 'escalate' present — requires platform-team sign-off",
    "wildcard resource '*' on ClusterRole — never approved without CISO justification"
  ],
  "safe_next_actions": [
    "Remove 'escalate' verb; request platform-team approval if needed",
    "Replace '*' resource with explicit list: ['pods', 'services', 'configmaps']",
    "Re-submit for review after scoping changes"
  ],
  "open_questions": [
    "Is this ClusterRole intended to be namespace-scoped? If so, use a Role instead."
  ]
}
```

### Example — Approved Response

```json
{
  "verdict": "approved",
  "evidence_level": "verified",
  "blockers": [],
  "safe_next_actions": [
    "kubectl apply -f role.yaml",
    "kubectl auth can-i list pods --as system:serviceaccount:default:my-sa -n production",
    "Confirm binding propagated: kubectl get rolebinding my-binding -n production"
  ],
  "open_questions": []
}
```

---

## Compliance Framework Mapping

The table below maps each response field to the compliance controls it satisfies. A single structured response from a live-guard or review agent is sufficient evidence for all mapped controls — no additional documentation required.

| Response Field | SOC 2 (CC) | PCI DSS v4 | NIS 2 (Article) | NIST CSF (PR) | ISO 27001 (A.) |
|---|---|---|---|---|---|
| `verdict` | CC6.1 — logical access controls | Req 7.2 — access control systems | Art. 21(2)(e) — access control | PR.AC-4 — access permissions managed | A.9.1.1 — access control policy |
| `evidence_level` | CC7.2 — monitoring activities | Req 10.2 — audit log completeness | Art. 21(2)(b) — incident handling | PR.IP-1 — baseline configuration | A.12.4.1 — event logging |
| `blockers` | CC6.3 — removal of access | Req 7.3 — least privilege enforcement | Art. 21(2)(i) — supply chain security | PR.AC-6 — identities proofed | A.9.2.3 — privileged access rights |
| `safe_next_actions` | CC8.1 — change management | Req 6.5 — secure development | Art. 21(2)(f) — security procedures | PR.IP-3 — configuration change control | A.12.1.2 — change management |
| `open_questions` | CC4.1 — COSO monitoring | Req 12.3 — targeted risk analysis | Art. 21(1) — risk management | ID.RA-3 — threats identified | A.6.1.2 — segregation of duties |

### How This Creates Audit Evidence

A reviewer or auditor can:

1. Export the structured response as a JSON artifact at change time.
2. Hash the artifact and store it alongside the change record (Git commit, JIRA ticket, ServiceNow change).
3. Reference the artifact in a SOC 2 or PCI DSS audit by mapping `verdict=approved` + `evidence_level=verified` + empty `blockers` to the controls in the table above.

This eliminates manual evidence collection for the five most common cloud access-control audit questions:
- "Did you review permissions before granting access?" → `verdict` field + agent name
- "Did you capture the baseline before mutating?" → `evidence_level: verified`
- "Were escalation paths blocked?" → `blockers` field
- "What was the approved remediation path?" → `safe_next_actions`
- "Were ambiguities escalated for human review?" → `open_questions`

---

## Three Enforcement Layers

VFA agents cover three layers of every critical decision point. The evidence output spec applies to all three:

| Layer | Agent Type | Timing | Compliance Role |
|---|---|---|---|
| **BEFORE** | Review agents (e.g., `kubernetes-rbac-review`) | Pre-change, during design | SOC2 CC6.1 design evidence |
| **AT** | Live-guard agents (e.g., `kubernetes-live-rbac-mutation-guard`) | At execution, blocking | NIST CSF PR.AC-4 enforcement control |
| **AFTER** | Verification agents (e.g., `kubectl auth can-i` in safe_next_actions) | Post-change, audit trail | ISO 27001 A.12.4.1 event logging |

---

## Five Critical Decision Points

The live-guard agents cover the five decision points where unguarded automation creates the highest Fortune 50 compliance risk:

| Decision Point | Provider Coverage | Primary Control |
|---|---|---|
| IAM/RBAC change | AWS IAM, Azure Entra ID, OCI IAM, Kubernetes RBAC | SOC2 CC6.1, PCI Req 7 |
| Network exposure | AWS Security Groups, Azure NSGs, OCI Security Lists/NSGs | NIST CSF PR.AC-4 |
| Production deployment | AWS ECS/Lambda, Azure App Service/AKS, OCI OKE | SOC2 CC8.1, PCI Req 6.5 |
| Secret/key lifecycle | AWS KMS, Azure Key Vault, OCI Vault | ISO 27001 A.9.2.3 |
| Permanent privilege escalation | Azure PIM, OCI Resource Manager, Kubernetes escalate/bind | NIS2 Art. 21(2)(e) |

---

## Extending This Spec

To add a new compliance framework (e.g., FedRAMP, HIPAA, CIS Controls):

1. Add a column to the framework mapping table above.
2. Map each response field to the most specific control in the new framework.
3. Add a row to the Five Critical Decision Points table if the new framework introduces a sixth decision point not already covered.
4. Do not add framework columns without concrete control references — `assumed` mappings create audit risk.
