# Least-privilege NetSuite posture for NetSuite Identity Access Role Permission Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Identity Access Reviewer (custom)
- **Copy from standard role:** Auditor (standard NetSuite role — read-only, no transaction entry) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Setup, SuiteCloud
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Roles and Groups** (View) — Required to read role definitions and permission lists for analysis
- **Custom Roles** (View) — Required to inspect custom role configurations and permkey/permlevel assignments
- **User Management** (View) — Required to review role-to-user assignments (no edit access needed)
- **SuiteCloud Development Framework** (View) — Required to read SDF customrole XML exports
- **Audit Trail** (View) — Required to verify role-change history for evidence artifacts

## Forbidden

- Administrator role
- Edit or Full on User Management
- Edit or Full on Roles and Groups
- Any permission not listed above

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes or asks for user passwords, access tokens, TBA token values, OAuth client secrets, or session cookies
- Request asks the agent to act as or assume Administrator role
- Request asks to perform a live role assignment, permission edit, or user account modification — escalate to netsuite-live-org-mutation-guard-agent
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for role alignment context
- Request asks to generate TBA tokens, OAuth authorization codes, or integration credentials
- Scope creep: authentication mechanism design questions belong to netsuite-sso-oauth-tba-agent

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

`netsuite-identity-access-role-permission-skill` — NetSuite Identity Access Role Permission Skill
