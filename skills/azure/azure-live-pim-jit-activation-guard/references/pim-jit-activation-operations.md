# Azure PIM JIT Activation Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating eligible assignment as active access.
- Activating the broadest eligible scope instead of reducing scope to the actual resource needed.
- Skipping justification, ticket, MFA, or approval because the task is urgent.
- Assuming deactivation instantly removes access from every downstream application cache.
- Trying to activate another user’s eligible role from an agent context.

## Officially grounded service shape

Microsoft Learn evidence says PIM provides just-in-time, time-bound, approval-based privileged access for Microsoft Entra and Azure resources. Azure resource role activation can require MFA, reduced scope, start time, duration, and reason; approval can leave a request pending. PIM temporarily adds active assignment and later removes it, but applications can cache role state so access changes may not appear immediately.

- PIM role settings are configured per role and per resource.
- Eligible members activate roles only when needed and within configured maximum duration.
- Activation can require MFA, justification, ticket information, Conditional Access, and approval.
- Users can view pending requests, cancel pending requests, and deactivate active assignments subject to timing rules.
- PIM audit and notification signals are part of the control, not optional decoration.

## Non-negotiable design rules

- Confirm the eligible principal is the requester or authorized approver; do not impersonate activation.
- Require the narrowest scope, shortest duration, and explicit business reason.
- Require approval state before treating access as usable.
- Document cache and sign-out/sign-in caveats for access add/remove.
- Refuse activation when target scope, role, principal, approval, or rollback/deactivation path is ambiguous.

## Minimal safe implementation flow

- Scope principal, role, resource scope, duration, approval workflow, and task ticket.
- Collect read-only evidence of eligibility, current active assignments, PIM role settings, and request status.
- Check MFA, Conditional Access, approval, justification, and reduced-scope requirements.
- If mutation is requested, require explicit user approval before activation, approval, cancellation, or deactivation.
- Verify request status, active assignment, expiry time, audit evidence, and deactivation plan.

## Safe verification targets

- Eligible assignment exists for the requested principal and scope.
- Activation duration is within policy and no broader than operational need.
- MFA/approval/justification/ticket controls are satisfied where required.
- Audit or request status proves pending, active, denied, canceled, expired, or deactivated state.
- Access-cache caveat and sign-out/sign-in mitigation are stated.

## When to push back

- The user asks the agent to activate someone else’s role.
- Scope or role is broader than the task requires.
- Approval workflow is bypassed or approval owner is unclear.
- The user needs permanent access but calls it JIT.
