# OCI Cloud Guard Responder Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Closing or dismissing problems because they are noisy.
- Enabling responder automation without checking what it can mutate.
- Treating Oracle-managed recipe defaults as static forever.
- Using root-compartment or subtree queries without understanding partial-access behavior.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows Cloud Guard problem list operations expose compartment, subtree/access-level, detector type, target, problem category, resource, risk level, lifecycle, and detection-time filters. Treat this as API shape evidence, not proof of full tenancy posture.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate detection evidence from remediation opinion.
- Never close, dismiss, suppress, or auto-remediate without owner, rationale, and blast-radius review.
- Review detector and responder recipes before enabling automated actions.
- Label Cloud Guard results as sampled evidence unless every target/scope was intentionally covered.
- Keep current-state evidence separate from official docs.

## Minimal safe implementation flow

- Confirm target, compartment, resource type, environment, owner, and requested action.
- Collect read-only problem, target, detector, and responder evidence when available.
- Prioritize by risk level, resource criticality, exploitability, and recurrence.
- Review responder side effects and IAM before remediation.
- Return verdict, problem table, safe next actions, approvals, and open questions.

## High-risk assumptions to kill

- “Cloud Guard enabled means the tenancy is secure.”
- “A problem can be dismissed because it appears expected.”
- “Responder recipes are safe because Oracle provides defaults.”
- “A list query from one compartment proves full-tenancy posture.”
- “Compliance evidence is valid without target coverage and exception rationale.”

Those are lazy assumptions.

## Safe command/code verification targets

- List problems only for confirmed scope and document access-level/subtree behavior.
- Check detector type, resource, risk, first/last detected time, lifecycle detail, and target ID.
- Review target coverage, detector recipe customizations, responder recipe enablement, and suppression rationale.
- Require owner approval and rollback path before responder actions or problem status updates.
- Correlate repeated problems with policy, IAM, network, and resource-owner remediation plans.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
