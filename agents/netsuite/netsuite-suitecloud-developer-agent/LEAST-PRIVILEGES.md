# Least-privilege NetSuite posture for NetSuite SuiteCloud Developer Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite SuiteCloud Developer Reviewer (custom)
- **Copy from standard role:** Developer (or closest available standard role with SuiteScript and SDF access) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Server SuiteScript, Client SuiteScript, SuiteCloud Development Framework, Custom Records
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **SuiteScript** (View) — Required to review SuiteScript file configurations and deployment records
- **SuiteCloud Development Framework** (View) — Required to inspect SDF project configurations and object definitions
- **Custom Record Types** (View) — Required to review custom record and field definitions
- **Script Deployments** (View) — Required to review script deployment configuration and run-as settings
- **SuiteApps** (View) — Required to inspect SuiteApp manifest and packaging configuration

## Forbidden

- Administrator role
- Full permission roles
- Any role with Create/Edit/Full on Script Deployments or SuiteApps

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes credentials, tokens, secrets, hardcoded org IDs, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions for script execution
- Request asks agent to push SDF project, execute deployment commands, or mutate a NetSuite account
- User claims SuiteCloud Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires live execution of SuiteScript or SDF CLI commands

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

`netsuite-suitecloud-developer-skill` — NetSuite SuiteCloud Developer Skill
