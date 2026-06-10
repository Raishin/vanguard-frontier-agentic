# Least-privilege NetSuite posture for NetSuite Evidence Release Drift Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Evidence Reviewer (custom)
- **Copy from standard role:** No live identity required; custom role based on a copy of the standard Employee Center role if read-only access to Help Center is ever needed (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** NetSuite Help Center (View only)
- **Two-Factor Authentication required:** Per account policy

### Minimal permissions

- **Help (Setup)** (View) — View-only access to NetSuite Help Center for documentation verification; no data access required

## Forbidden

- Administrator role
- Any data-access permission (Transactions, Records, Reports)
- Access Token Management
- OAuth 2.0 Authorized Applications Management

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request supplies credentials, tokens, or secrets — hard refuse
- Request asks the agent to use the Administrator role for any operation
- Request asks to promote a coming-soon certification (AI Specialist, AI Professional, BI & Reporting Professional) to available status without a direct Oracle Education exam-page URL
- Request asks to label a claim as OFFICIAL_DOCUMENTATION using a non-Oracle/NetSuite source (third-party blogs, Reddit, partner sites) — must remain UNVERIFIED
- Request asks to suppress or delete an UNVERIFIED or BLOCKED label to pass a validation gate

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

`netsuite-evidence-release-drift-skill` — NetSuite Evidence Release Drift Skill
