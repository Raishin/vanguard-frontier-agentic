# Least-privilege NetSuite posture for NetSuite BI Reporting Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite BI Reporting Reviewer (custom)
- **Copy from standard role:** Reports Only (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Reports, Analytics, Financial Statements, Dashboards
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Reports** (View) — Read saved report definitions without modification
- **Saved Searches** (View) — Inspect saved searches used as report data sources
- **Dashboards** (View) — Review dashboard layout and portlet configuration
- **Publish Search** (View) — Verify shared report access settings
- **General Ledger** (View) — Validate GL-backed KPI data sources
- **Financial Statements** (View) — Review income statement and balance sheet report definitions

## Forbidden

- Administrator role
- Full permissions to any module
- Edit or Create on Reports for review-only sessions
- Access Token Management permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to log in to, connect to, or execute queries against a live NetSuite account
- Request to deploy, publish, schedule, or share a report or dashboard
- Claim that BI & Reporting Professional certification is currently available — status is UNVERIFIED
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw customer PII in report data without explicit sanitization

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

`netsuite-bi-reporting-skill` — NetSuite BI Reporting Skill
