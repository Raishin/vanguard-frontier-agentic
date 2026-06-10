# Least-privilege NetSuite posture for NetSuite Enterprise Architecture Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Architecture Reviewer (custom)
- **Copy from standard role:** Custom copy of the standard Developer role (read-only access to SuiteScript and SDF objects) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** SuiteCloud (view-only), SuiteScript (view-only), SDF (view-only), Integrations (view-only)
- **Two-Factor Authentication required:** Per account policy

### Minimal permissions

- **SuiteScript (Setup)** (View) — Required to inspect SuiteScript file configurations and deployment objects during architecture review
- **SuiteCloud Development Framework (Setup)** (View) — Required to review SDF project structures and object manifests
- **Custom Record Types (Lists)** (View) — Required to inspect custom record schema during customization architecture review
- **Integration Application (Setup)** (View) — Required to review integration application registrations without modifying them

## Forbidden

- Administrator role
- Access Token Management
- OAuth 2.0 Authorized Applications Management
- Core Administration Permissions
- Any permission level of Full on any module
- Roles with all permissions granted

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request supplies credentials, API keys, OAuth secrets, or TBA tokens — hard refuse
- Request asks for architecture approval of a new SOAP integration post-2026.1 without a migration plan — refuse clearance
- Request asks the agent to use or recommend the Administrator role for automated or integration purposes
- Request cites coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) as currently available in a design justification
- Request asks for production deployment execution rather than architecture review — route to netsuite-live-org-mutation-guard-agent

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

`netsuite-enterprise-architecture-skill` — NetSuite Enterprise Architecture Skill
