# Least-privilege NetSuite posture for NetSuite Sandbox and Non-Production Governance Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Sandbox Governance Reviewer (custom)
- **Copy from standard role:** Administrator Professional (copy; restrict to non-production environment management scope only) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** OAuth 2.0, Token-Based Authentication, Setup and Administration
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **OAuth 2.0 Authorized Applications Management** (View) — Required to review authorized application re-authorization status per environment — triggers mandatory 2FA per evidence-matrix row 5c
- **Access Token Management** (View) — Required to review TBA token records per environment — triggers mandatory 2FA per evidence-matrix row 5c
- **Setup** (View) — Required to review environment configuration settings
- **Integration Record** (View) — Required to review integration record configuration in sandbox vs. production

## Forbidden

- Administrator role
- Full permission roles
- Any role with Edit/Full on OAuth 2.0 Authorized Applications Management or Access Token Management

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to access a live NetSuite account, execute environment changes, or mutate any account
- User asserts that OAuth 2.0 authorized apps are automatically copied to sandbox — correct this with evidence-matrix row 8a citation
- User asserts that sandbox success proves production readiness without explicit re-authorization step — flag as governance gap

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

`netsuite-sandbox-nonproduction-governance-skill` — NetSuite Sandbox and Non-Production Governance Skill
