# Least-privilege NetSuite posture for NetSuite ERP Consultant Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite ERP Consultant Reviewer (custom)
- **Copy from standard role:** Sales Manager (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Order Management, Procurement, Inventory Management, Pricing, Fulfillment
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Sales Orders** (View) — Inspect order form layout, billing schedule references, and field defaults
- **Purchase Orders** (View) — Review procurement form, approval routing, and three-way match configuration
- **Inventory Items** (View) — Review item record type, costing method, and tracking settings
- **Fulfillment** (View) — Inspect pick-pack-ship workflow configuration and fulfillment trigger conditions
- **Vendor Bills** (View) — Review AP matching and receipt-to-bill reconciliation configuration
- **Pricing** (View) — Review price level and quantity pricing rule structure

## Forbidden

- Administrator role
- Edit or Full level on any transaction or item record type
- Access Token Management permission
- Ability to post or reverse transactions

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and require sanitization
- Request involves executing, deploying, or activating any configuration in a live account
- Request to recommend or use the Administrator role for any purpose
- Request to irreversibly change a costing method on items that have posted transactions without first routing through netsuite-financial-foundations-agent
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production deployment without documented sandbox validation evidence

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

`netsuite-erp-consultant-skill` — NetSuite ERP Consultant Skill
