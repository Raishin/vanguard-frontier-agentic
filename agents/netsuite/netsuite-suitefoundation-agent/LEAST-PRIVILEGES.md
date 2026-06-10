# Least-privilege NetSuite posture for NetSuite SuiteFoundation Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite SuiteFoundation Reviewer (custom)
- **Copy from standard role:** Accountant (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Core Administration, Basic Customization, Saved Searches, Custom Fields and Lists
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Lists** (View) — Read saved searches, custom lists, and segment definitions
- **Transactions** (View) — Inspect transaction form layouts and default settings
- **Reports** (View) — Review saved search scheduling and dashboard portlets
- **Setup** (View) — Inspect subsidiary hierarchy, base currency, and custom field definitions
- **Custom Record Types** (View) — Review custom record form and sublist configuration

## Forbidden

- Administrator role
- Full permissions to any module
- Edit or Create level on any live record type
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration or review purposes — refuse and cite least-privilege principle (evidence-matrix row 7a, 7b)
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

`netsuite-suitefoundation-skill` — NetSuite SuiteFoundation Skill
