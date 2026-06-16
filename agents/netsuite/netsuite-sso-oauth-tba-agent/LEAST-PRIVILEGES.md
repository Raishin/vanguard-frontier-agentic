# Least-privilege NetSuite posture for NetSuite SSO OAuth TBA Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Auth Configuration Reviewer (custom)
- **Copy from standard role:** Auditor (standard NetSuite role — read-only, no transaction entry) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Setup
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Integrated Applications** (View) — Required to read OAuth 2.0 integration record settings (no client secret visible at View level)
- **User Access Tokens** (View) — Required to confirm TBA setup without accessing token values
- **OAuth 2.0 Authorized Applications Management** (View) — Required to verify authorized application list per environment; triggers mandatory 2FA (evidence 5c)
- **Single Sign-on** (View) — Required to review SSO/SAML configuration excerpts

## Forbidden

- Administrator role
- Log in using Access Tokens (do not confuse with 'Log in using OAuth 2.0 Access Tokens')
- Edit or Full on any Setup permission listed above
- Any transaction or record entry permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes or asks for access tokens, refresh tokens, client secrets, TBA token values, SAML assertions, or session cookies
- Request asks the agent to generate OAuth 2.0 authorization codes, client credentials, or TBA token pairs
- Request asks the agent to perform a live sandbox refresh, authorize an OAuth application in a live account, or create TBA tokens
- Request asks to act as or use Administrator role
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for authentication context
- Scope creep: role and permission questions route to netsuite-identity-access-role-permission-agent

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

`netsuite-sso-oauth-tba-skill` — NetSuite SSO OAuth TBA Skill
