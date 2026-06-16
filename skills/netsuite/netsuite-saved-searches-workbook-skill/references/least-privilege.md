# Least-privilege NetSuite posture for NetSuite Saved Searches Workbook Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Search Workbook Reviewer (custom)
- **Copy from standard role:** Reports Only (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Reports, Analytics, SuiteAnalytics Workbook
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Saved Searches** (View) — Read saved search definitions and criteria without modification
- **SuiteAnalytics Workbook** (View) — Inspect workbook dataset, pivot, and chart configurations
- **Reports** (View) — Cross-reference saved searches used as report data sources
- **Employees** (View) — Validate employee record searches for PII exposure risk
- **Contacts** (View) — Validate contact searches for PII exposure risk
- **Transactions** (View) — Inspect transaction search joins and results for data leakage

## Forbidden

- Administrator role
- Edit or Create on Saved Searches for review-only sessions
- Full permissions to any module
- Access Token Management permission
- Publish Search with write intent

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to execute, run, preview, or schedule a search against a live NetSuite account
- Request to share or publish a search or workbook
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw unmasked PII fields without prior sanitization acknowledgment
- Coming-soon certification claimed as currently available for this domain

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

`netsuite-saved-searches-workbook-skill` — NetSuite Saved Searches Workbook Skill
