# Least-privilege NetSuite posture for NetSuite Application Developer Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite App Developer Reviewer (custom)
- **Copy from standard role:** Developer (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** SuiteScript, SuiteBuilder, SuiteFlow, SuiteCloud Development Framework
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **SuiteScript** (View) — Read script files and deployment records without modification
- **SuiteFlow** (View) — Inspect workflow definitions and action configurations
- **Custom Record Types** (View) — Review custom record definitions and field configurations
- **Custom Fields** (View) — Validate custom field internal IDs and types
- **Script Deployments** (View) — Inspect deployment records and run-as role bindings
- **Log in using OAuth 2.0 Access Tokens** (Full) — Required for authenticated read-only API calls if used as run-as context; least-privilege scope only

## Forbidden

- Administrator role
- Full permissions to any module
- Edit or Create on Script Deployments for review-only sessions
- Access Token Management permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to deploy, activate, schedule, or execute any script or workflow in a live or sandbox account
- Request to assume Administrator role or any role granting full account access
- Request to run security penetration tests or exploit discovery — use netsuite-suitescript-secure-code-review-agent
- Request to perform SDF project deployment or SuiteScript 1.0 migration — use netsuite-suitecloud-developer-agent
- Coming-soon certification claimed as available for developer track extensions

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

`netsuite-application-developer-skill` — NetSuite Application Developer Skill
