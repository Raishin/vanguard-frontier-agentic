# Least-privilege NetSuite posture for NetSuite Live Org Mutation Guard Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Live Guard Reviewer (custom)
- **Copy from standard role:** No live identity required for guard evaluation; if future read-only audit logging access is provisioned, base on a custom copy of the standard Auditor role (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** scoped to remit
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Log (Setup)** (View) — Required only if guard needs to inspect SuiteScript execution logs for change evidence; View only

## Forbidden

- Administrator role
- Access Token Management
- OAuth 2.0 Authorized Applications Management
- Core Administration Permissions
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers
- Any permission level of Full on any module

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request supplies credentials, tokens, OAuth client secrets, TBA token values, or session cookies — hard refuse, do not echo or log
- Request asks for or implies use of the Administrator role for any automated or scripted operation
- No authorized live-op protocol or change-management ticket reference is present
- No named human decision owner is identified
- No rollback plan is provided for production-bound changes
- Request proposes building a new SOAP integration after the 2026.1 release (REST+OAuth2 is required for new builds per evidence item 2a)
- Request proposes new TBA for SOAP, REST, or RESTlets after 2027.1 (hard block per evidence item 4d)
- Proposed change would grant permissions that mandate 2FA (Access Token Management, OAuth 2.0 Authorized Applications Management, Core Administration Permissions, View Unencrypted Credit Cards, View Unencrypted ACH Account Numbers) without confirming 2FA enrollment
- Coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) cited as available in the change justification

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

`netsuite-live-operation-safety-skill` — NetSuite Live Operation Safety Skill
