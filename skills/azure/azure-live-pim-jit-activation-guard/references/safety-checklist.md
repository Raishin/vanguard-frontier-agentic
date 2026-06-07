# Safety Checklist

## Evidence labels

- `documentation-based`: grounded in Microsoft Learn or listed official documentation.
- `sampled-current-state`: grounded in read-only Azure observations from the user's configured tools.
- `user-provided`: grounded in sanitized snippets supplied by the user.
- `inference`: reasoned from evidence but not directly proven.

## Mutation boundary

- Default to read-only review.
- Do not perform create, update, delete, activate, approve, cancel, deactivate, migrate, cut over, route, peer, deploy, alert, suppress, or configuration changes unless the user explicitly asks and approval is clear.
- Prefer preview, assessment, status, list, show, query, activity-log, dependency, and diagnostic evidence before any mutation.

## Credential and data boundary

- Never ask users to paste credentials, tokens, tenant IDs, subscription IDs, customer data, private keys, appliance secrets, migration inventory dumps, log payload secrets, or raw environment dumps.
- Summarize sensitive evidence by field presence, control state, and risk; do not reproduce secret material.

## Risk gates

- Stop on ambiguous target, ambiguous principal, missing approval, missing owner, missing rollback, stale assessment, incomplete dependency mapping, unclear routing, or missing telemetry for high-impact assets.
- Separate documented product behavior from sampled configured-environment evidence.

## Asset-specific hard line

Never activate or approve PIM privileged access without confirming eligible principal, scope, role, activation duration, MFA/Conditional Access requirement, justification or ticket, approval status, and deactivation/expiry behavior.
