# Least-privilege NetSuite posture for NetSuite SDF DevOps Release Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite SDF Release Reviewer (custom)
- **Copy from standard role:** Developer (standard NetSuite role — SuiteCloud access, no financial transaction entry) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** SuiteCloud, Setup
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **SuiteCloud Development Framework** (View) — Required to read SDF project configurations, manifests, and deploy objects
- **Script Deployments** (View) — Required to inspect script deployment records and target environments
- **Roles and Groups** (View) — Required to verify customrole permission XML in deployment objects
- **SuiteScript** (View) — Required to examine script file versions and entry point configurations
- **Custom Records** (View) — Required to inspect custom object definitions included in SDF deployments

## Forbidden

- Administrator role
- Edit or Full on Script Deployments
- Edit or Full on SuiteCloud Development Framework
- Any financial transaction permission
- Deploy to Production permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes or asks for account credentials, tokens, client secrets, or deployment passwords
- Request asks the agent to execute, trigger, or approve a live deployment — escalate to netsuite-live-org-mutation-guard-agent
- Request asks the agent to act as or use Administrator role
- Request asks to bypass documentation gate (deploy without README/ARCHITECTURE/CHANGELOG) — document the risk, do not approve bypass
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for deployment context
- Scope creep: SuiteScript OWASP security review routes to netsuite-suitescript-secure-code-review-agent

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

`netsuite-sdf-devops-release-skill` — NetSuite SDF DevOps Release Skill
