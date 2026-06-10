# Least-privilege NetSuite posture for NetSuite Financial Foundations Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Financial Foundations Reviewer (custom)
- **Copy from standard role:** Accountant (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Accounts Payable, Accounts Receivable, Financial Management, Banking
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Vendors** (View) — Inspect AP vendor record defaults and payment term configuration
- **Customers** (View) — Inspect AR customer record defaults, invoicing templates, and payment method mapping
- **Accounting Lists** (View) — Review chart of accounts structure, account types, and sub-account hierarchy
- **Accounting Preferences** (View) — Inspect base currency, fiscal year, accounting method, and tax defaults
- **Bank Accounts** (View) — Review bank account record type, currency, and GL mapping (masked account numbers only)
- **Reconcile Account Statement** (View) — Inspect bank reconciliation configuration and statement format settings

## Forbidden

- Administrator role
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- Edit or Full level on any live financial record type

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, vendor bank account numbers, payment tokens, credit card numbers, or any authentication or financial account material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for AP/AR review or accounting configuration — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
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

`netsuite-financial-foundations-skill` — NetSuite Financial Foundations Skill
