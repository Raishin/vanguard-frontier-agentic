# Least-privilege NetSuite posture for NetSuite Audit Controls SOX Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Audit Controls SOX Reviewer (custom)
- **Copy from standard role:** Accountant (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Financial Management, Accounting, Revenue Recognition, Approval Workflows, Audit Logging
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Manage Accounting Periods** (View) — Inspect posting period lock/unlock status and close calendar without modifying period state
- **Journal Entries** (View) — Review journal entry records and approval chain history for SOX walkthrough
- **Vendor Bills** (View) — Inspect AP approval workflow coverage and SoD separation between invoice entry and payment
- **Revenue Recognition** (View) — Review recognition schedules, deferral accounts, and ASC 606 arrangement allocation
- **Audit Trail (System Notes)** (View) — Verify field-history tracking completeness across financial transaction types
- **Workflow** (View) — Inspect approval workflow definitions and step configurations for SOX control evidence

## Forbidden

- Administrator role
- Manage Accounting Periods at Edit or Full level
- Full access to Journal Entries
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, activating, or unlocking any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration, review, or period-close operations — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

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

`netsuite-audit-controls-sox-skill` — NetSuite Audit Controls SOX Skill
