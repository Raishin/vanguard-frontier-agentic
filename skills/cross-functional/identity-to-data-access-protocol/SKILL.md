---
name: identity-to-data-access-protocol
description: Use this skill when an identity lifecycle event (joiner, mover, leaver) or an access request must be evaluated end-to-end across Microsoft Entra identity, Conditional Access policy, and data access governance under a Zero Trust posture. Orchestrates m365-identity-zero-trust-agent as primary and m365-copilot-readiness-governance-agent for data-layer governance. Gates include access review sign-off and least-privilege validation before any access grant is recommended. Does not approve access; all recommendations require human owner confirmation. Never requests credentials, tenant IDs, or customer data.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: security
  lifecycle: experimental
---

# Identity to Data Access Protocol

## Purpose
This skill defines how an identity claim — a new hire, a role change, an
external partner request, or a privileged-role activation — is evaluated
against Conditional Access policy and data access governance rules before any
access recommendation is produced. It enforces Zero Trust principles (verify
explicitly, use least privilege, assume breach) across the full access
lifecycle. It does not approve access; it produces a structured recommendation
that a human identity or security owner confirms.

## When to use
- A joiner, mover, or leaver event requires access provisioning or de-provisioning.
- An access request must be assessed against Conditional Access policy and
  least-privilege rules before approval.
- A periodic access review is due for a group, application, or privileged role.
- A Privileged Identity Management (PIM) activation request needs risk context
  before the approver decides.
- Data access governance must be validated before sensitive content is exposed.

## When NOT to use
- The access decision is already made and you only need to execute provisioning
  — use your provisioning runbook, not this protocol.
- The matter is a security incident requiring incident response — route to your
  incident response protocol.
- The identity in question belongs to a non-Microsoft Entra directory — this
  protocol is scoped to Microsoft Entra ID and Microsoft 365.
- You need live tenant configuration changes — escalate to the identity owner;
  this protocol is recommendation-only.

## Participating agents
- `m365-identity-zero-trust-agent` (primary — Entra identity, Conditional Access, PIM)
- `m365-copilot-readiness-governance-agent` (secondary — data access governance, sensitivity label compliance)

## Inputs required
- Identity claim: UPN, identity type (employee, guest, service principal), lifecycle event
- Requested resource or role with business justification
- Applicable Conditional Access policies in scope
- Current group memberships, role assignments, and entitlement packages
- Data classification of the target resource (if data access is in scope)

## Evidence required
- Microsoft Entra tenant has Conditional Access policies configured
- Entitlement management access packages are defined for the relevant resource
- PIM is configured for any privileged roles in scope
- Access review schedule is active for the affected group or application
- Data classification labels (Microsoft Purview) are applied to target resources

## Workflow

1. **Identity verification** — Confirm identity type, lifecycle event, and
   authentication strength. Verify MFA registration and device compliance
   before proceeding.
2. **Conditional Access evaluation** — Map the requested access to applicable
   Conditional Access policies. Identify any policy gaps or exceptions.
3. **Least-privilege check** — Enumerate existing assignments. Identify
   over-privileged roles or group memberships. Flag any permissions not
   required for the stated business purpose.
4. **Gate 1 — Access review sign-off** — If the target resource or role is
   subject to an active access review, confirm the review is current and the
   identity has been attested. Do not recommend access if attestation is
   overdue.
5. **Data governance layer** — Invoke m365-copilot-readiness-governance-agent
   to assess data sensitivity and confirm that the identity's access scope
   does not expose over-shared or unlabelled sensitive content.
6. **Entitlement management** — If access is via an entitlement management
   package, confirm the package policy (approval workflow, expiry, separation
   of duties) is satisfied.
7. **PIM gate (privileged roles only)** — For any privileged role, confirm
   just-in-time activation is in use, the justification is documented, and the
   activation window is bounded.
8. **Gate 2 — Least-privilege validation** — Produce a least-privilege
   attestation: the recommended access grants only the permissions required,
   for only the duration required, with no standing privileged access.
9. **Recommendation** — Produce a structured access recommendation: approve,
   deny, or reduce-scope. Include the evidence basis, open questions, and
   do_not_do_list.
10. **Human confirmation** — Route to the identity owner or security team for
    final approval. This protocol never approves access autonomously.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Access review | Review overdue or attestation not current | Block recommendation; flag for review owner |
| Least-privilege | Requested permissions exceed business justification | Recommend reduced scope; escalate to identity owner |
| MFA / device | MFA not registered or device non-compliant | Block recommendation; direct user to remediation |
| Data sensitivity | Target resource contains unlabelled or over-shared sensitive data | Invoke m365-copilot-readiness-governance-agent; hold access recommendation |
| PIM activation | Privileged role requested without JIT justification | Require JIT activation and documented justification |

## Refusal triggers
- Stop if the identity cannot be authenticated to the required assurance level
  — do not produce an access recommendation on an unverified identity.
- Stop if credentials, session tokens, tenant IDs, or customer PII are
  requested — refuse and escalate to the security team.
- Stop if the access request would grant standing Global Administrator or
  equivalent permissions without PIM — this is unconditionally blocked.
- Stop if separation of duties would be violated — escalate to the security owner.

## Handoff rules
- All handoffs carry: identity_id (anonymised), skill_id, skill_version,
  invoked_by, access_scope, evidence_quality, open_questions, do_not_do_list.
- Human escalations always include the least-privilege gap statement and the
  specific Conditional Access policy in scope.
- Data governance findings from m365-copilot-readiness-governance-agent are
  attached as a sub-report; they are never discarded.

## KPIs
- Access review completion rate (% on schedule)
- Least-privilege attestation pass rate
- Mean time to access decision (request to human confirmation)
- PIM just-in-time activation rate for privileged roles
- Over-privilege remediation rate (flagged vs. resolved)

## References
- https://learn.microsoft.com/entra/id-governance/scenarios/least-privileged
- https://learn.microsoft.com/entra/id-governance/access-reviews-overview
- https://learn.microsoft.com/entra/id-governance/deploy-access-reviews
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-configure
- https://learn.microsoft.com/security/zero-trust/
