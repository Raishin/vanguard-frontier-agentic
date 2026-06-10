# Least-privilege NetSuite posture for NetSuite Maestro Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Maestro Reviewer (custom)
- **Copy from standard role:** No live identity required (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** scoped to remit
- **Two-Factor Authentication required:** Per account policy

### Minimal permissions

- No standing NetSuite permissions required (static review of sanitized excerpts only).

## Forbidden

- Administrator role
- Any live NetSuite identity or session token
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request supplies credentials, tokens, session cookies, client secrets, or any live-org secret — refuse, do not log or echo
- Request asks the maestro to use the Administrator role for any operation
- Request asks the maestro to directly execute a live-org mutation without routing through netsuite-live-org-mutation-guard-agent
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available
- Request contains PII (SSN, credit card, bank account numbers, healthcare data) — refuse and advise sanitization before resubmission

## Escalation path

Route all live-account changes to `netsuite-live-org-mutation-guard-agent` with a named human decision owner and a structured case capsule.

## Role creation steps

1. In the target SANDBOX, copy the standard role named above to a new custom role.
2. Remove every permission not listed under Minimal permissions.
3. Add only the listed permissions at the stated access level.
4. Confirm the role is NOT Administrator and grants no global/cross-subsidiary access beyond remit.
5. Enable 2FA enforcement if the role touches privileged permissions.
6. Test in sandbox, then assign to the integration/review user; monitor for least-privilege drift.

## Companion skill

None (router/structural role).
