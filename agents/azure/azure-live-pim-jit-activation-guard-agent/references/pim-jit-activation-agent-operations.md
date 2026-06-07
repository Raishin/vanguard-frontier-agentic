# Azure Live PIM JIT Activation Guard operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Never activate or approve privileged access on weak evidence. PIM is just-in-time access control, not proof that the requester, device, ticket, or production change is safe.

## Officially grounded service shape

PIM eligible access is a request workflow with role, scope, activation conditions, duration, justification, and optional approval. That is the key insight: the agent must prove the activation should happen, not merely that activation is technically possible.

## Non-negotiable design rules

1. Treat every activation as a privilege escalation until evidence proves otherwise.
2. Confirm eligible assignment, role, scope, requester, duration, reason, and approval requirement before any activation recommendation.
3. Require MFA or Conditional Access evidence when the role settings require it; do not accept vague statements like MFA is probably enabled.
4. Block duplicate or overlapping active assignments unless the business reason is explicit and time-bound.
5. Never ask for credentials, tokens, device claims, tenant identifiers, or unsanitized identity exports.

## Minimal safe implementation flow

1. Classify whether the request is activation, approval, settings review, or post-activation verification.
2. Ground PIM behavior in Microsoft Learn and label it documentation-based.
3. Request only sanitized assignment, role, scope, ticket, approval, and expiry evidence if live read-only evidence is unavailable.
4. Verify eligibility, activation requirements, duplicate active assignments, justification quality, and expiry.
5. Return go/no-go with blockers and the smallest safe next action.

## High-risk assumptions to kill

- Eligible means safe.
- An approver can approve without ticket, scope, and expiry evidence.
- Role settings are uniform across Entra roles and Azure resource roles.
- MFA presence proves device or change safety.
- Documentation proves the user's current PIM configuration.

## Safe command/code verification targets

- Role settings: activation duration, MFA or Conditional Access, justification, approval, ticketing, and notification requirements.
- Assignments: eligible versus active, principal, role, scope, schedule, and expiry.
- Audit evidence: activation request, approval decision, reason, approver, and final active assignment window.

## When to push back

- The requester wants activation without justification or ticket binding.
- The requested duration exceeds role settings or operational need.
- The role scope is broader than the change scope.
- Approval evidence is missing or self-approved.
