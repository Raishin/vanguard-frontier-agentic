# Evidence Output Specification

> Canonical response shape for all VFA live-guard and review agents, and how it
> **may support** — but does not by itself constitute — compliance evidence.

## Purpose

Every live-guard and review agent in this repo produces a **structured verdict
response**. This document defines the required fields and describes which compliance
controls each field can help *support*.

> [!IMPORTANT]
> **An agent response may support audit evidence; it is not audit proof.** A prior
> version of this spec claimed a single response "becomes an audit artifact without
> post-processing" and was "sufficient evidence for all mapped controls." That claim
> was wrong and has been removed. The corrected position is stated in
> [What a response does and does not establish](#what-a-response-does-and-does-not-establish)
> and governs the rest of this document.

The five fields below are the minimum set required on every agent response. Agents may
add provider- or tier-specific fields (e.g., `cluster_context`, `assignment_scope`, or
the live-control-plane audit-event fields in
[audit-event.schema.json](../schemas/audit-event.schema.json)) but must not omit
required fields.

---

## What a response does and does not establish

These statements are load-bearing. Every mapping, table, and example in this document
is read subject to them:

- An agent response **may support** audit evidence. It is one input a reviewer or
  auditor may consider — not a finished audit artifact.
- A **control mapping does not establish applicability.** Whether a framework or a
  specific control applies to a given system, process, and jurisdiction is a
  determination made by the accountable owner, not asserted by an agent.
- **Evidence existence does not prove evidence accuracy.** A captured field can be
  stale, incomplete, self-reported, or wrong; the response records a claim, not a proof.
- **One successful execution does not prove continuing operating effectiveness.** A
  control's design and its sustained operation are assessed over time and populations,
  not from a single green verdict.
- **Technical controls do not replace procedural controls.** An automated gate does not
  discharge the human approval, review, segregation-of-duties, and change-governance
  obligations around it.
- **Control design and control operation are separate.** Designing a check is not the
  same as evidence that it operated on the change in question.
- **Internal evidence does not replace independent assessment.** Self-generated output
  is unreviewed until an independent party assesses it.
- **Audit acceptance remains the auditor's decision.** Nothing here obliges any auditor
  to accept any artifact.
- **Legal compliance remains the organization's responsibility.** No agent, response, or
  mapping in this repo establishes legal or regulatory compliance; qualified owners do.

---

## Required Response Fields

| Field | Type | Description |
|---|---|---|
| `verdict` | `approved` \| `blocked` \| `needs-review` | Gate decision. `approved` from an agent means *no blocking condition was detected in what the agent could observe* — never that a change is authorized, safe, or compliant. |
| `evidence_level` | `verified` \| `partial` \| `assumed` | Confidence in the captured baseline. `verified` = live state confirmed via read; `partial` = snapshot exists but may be stale; `assumed` = no current-state capture possible. Retained for compatibility; richer dimensions are in [evidence-quality-model.md](compliance/evidence-quality-model.md). |
| `blockers` | `string[]` | Each item is a named violation that must be resolved before the change proceeds. Empty array if verdict is `approved`. |
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

## Candidate Compliance-Control Support

The table below lists controls each response field **may help support** as one input
among others. A row means "this field is potentially relevant to this control," **not**
"this field satisfies this control." Applicability, accuracy, sufficiency, and operating
effectiveness are determined by the accountable owner and, where required, an independent
assessor — see
[What a response does and does not establish](#what-a-response-does-and-does-not-establish).

| Response Field | SOC 2 (CC) | PCI DSS v4 | NIS 2 (Article) | NIST CSF (PR) | ISO 27001 (A.) |
|---|---|---|---|---|---|
| `verdict` | CC6.1 — logical access controls | Req 7.2 — access control systems | Art. 21(2)(e) — access control | PR.AC-4 — access permissions managed | A.9.1.1 — access control policy |
| `evidence_level` | CC7.2 — monitoring activities | Req 10.2 — audit log completeness | Art. 21(2)(b) — incident handling | PR.IP-1 — baseline configuration | A.12.4.1 — event logging |
| `blockers` | CC6.3 — removal of access | Req 7.3 — least privilege enforcement | Art. 21(2)(i) — supply chain security | PR.AC-6 — identities proofed | A.9.2.3 — privileged access rights |
| `safe_next_actions` | CC8.1 — change management | Req 6.5 — secure development | Art. 21(2)(f) — security procedures | PR.IP-3 — configuration change control | A.12.1.2 — change management |
| `open_questions` | CC4.1 — COSO monitoring | Req 12.3 — targeted risk analysis | Art. 21(1) — risk management | ID.RA-3 — threats identified | A.6.1.2 — segregation of duties |

> [!WARNING]
> This table is a **research aid for control owners**, not a certification. A candidate
> mapping is unverified until an owner confirms the control applies and an assessor
> confirms the evidence is accurate, complete, and operating. Do not add a framework
> column without concrete control references, and never present a mapping as proof of
> compliance.

### How a response can *support* audit evidence

A reviewer or auditor **may**:

1. Export the structured response as a JSON artifact at change time.
2. Hash the artifact and store it alongside the change record (Git commit, ticket,
   change request) with a trusted timestamp.
3. Consider it as **one input** when testing a mapped control — alongside the
   independent evidence that establishes applicability, accuracy, and operating
   effectiveness over the audit period.

A response **cannot** by itself answer an audit question. For example:

- "Did you review permissions before granting access?" → the `verdict` field plus the
  agent name is a *lead*; the auditor still confirms the review actually governed the
  change and that the reviewer was independent of the requester.
- "Did you capture the baseline before mutating?" → `evidence_level: verified` claims a
  capture occurred; the before/after state digests and an independent log establish it.
- "Were escalation paths blocked?" → the `blockers` field records what *this run*
  detected, not that the control operated on every relevant change.

Populate the richer evidence dimensions in
[evidence-quality-model.md](compliance/evidence-quality-model.md) (source, integrity,
freshness, completeness, independence, sensitivity, control stage, retention, assessor
status) so a reviewer can judge how much weight a response can carry.

---

## Three Enforcement Layers

VFA agents cover three layers of every critical decision point. The evidence output spec
applies to all three. Each layer produces *inputs* to control testing, not conclusions.

| Layer | Agent Type | Timing | Control support (candidate) |
|---|---|---|---|
| **BEFORE** | Review agents (e.g., `kubernetes-rbac-review`) | Pre-change, during design | Design-stage input for SOC2 CC6.1 |
| **AT** | Live-guard agents (e.g., `kubernetes-live-rbac-mutation-guard`) | At execution, blocking | Enforcement input for NIST CSF PR.AC-4 |
| **AFTER** | Verification agents (e.g., `kubectl auth can-i` in safe_next_actions) | Post-change, audit trail | Operation input for ISO 27001 A.12.4.1 |

Design-stage, enforcement, and operation inputs are **distinct** and none substitutes for
another (control design and control operation are separate).

---

## Five Critical Decision Points

The live-guard agents cover the five decision points where unguarded automation creates
the highest Fortune 50 compliance risk:

| Decision Point | Provider Coverage | Candidate primary control |
|---|---|---|
| IAM/RBAC change | AWS IAM, Azure Entra ID, OCI IAM, Kubernetes RBAC | SOC2 CC6.1, PCI Req 7 |
| Network exposure | AWS Security Groups, Azure NSGs, OCI Security Lists/NSGs | NIST CSF PR.AC-4 |
| Production deployment | AWS ECS/Lambda, Azure App Service/AKS, OCI OKE | SOC2 CC8.1, PCI Req 6.5 |
| Secret/key lifecycle | AWS KMS, Azure Key Vault, OCI Vault | ISO 27001 A.9.2.3 |
| Permanent privilege escalation | Azure PIM, OCI Resource Manager, Kubernetes escalate/bind | NIS2 Art. 21(2)(e) |

---

## Extending This Spec

To add a new compliance framework (e.g., FedRAMP, HIPAA, CIS Controls):

1. Add a column to the candidate-support table above.
2. Map each response field to the most specific control in the new framework.
3. Add a row to the Five Critical Decision Points table if the new framework introduces
   a sixth decision point not already covered.
4. Do not add framework columns without concrete control references — unverified
   mappings create audit risk and must never be presented as proof.
5. A mapping is a candidate for a control owner to evaluate; it is never a compliance
   claim.
