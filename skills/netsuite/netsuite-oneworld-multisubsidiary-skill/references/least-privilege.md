# Least-privilege NetSuite posture for NetSuite OneWorld Multi-Subsidiary Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite OneWorld Reviewer (custom)
- **Copy from standard role:** Accountant (standard role — copy and restrict) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** General Ledger, Multi-Currency, Tax, OneWorld
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Subsidiaries** (View) — Required to inspect subsidiary hierarchy and configuration
- **Intercompany Journal Entries** (View) — Required to review intercompany elimination account coverage
- **Currency** (View) — Required to review base-currency assignments and exchange-rate types
- **Tax Schedules** (View) — Required to review nexus and tax-jurisdiction configurations
- **General Ledger** (View) — Required to review intercompany due-to/due-from account pairings
- **Roles** (View) — Required to review cross-subsidiary role restrictions

## Forbidden

- Administrator role
- Full access to any transaction entry type
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- Edit or Create level on Subsidiaries or Intercompany records

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete subsidiaries, legal entities, or intercompany accounts in a live account
- Request provides unredacted tax registration numbers, VAT/GST IDs, or legal-entity bank account data — flag and ask for redacted version
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available

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

`netsuite-oneworld-multisubsidiary-skill` — NetSuite OneWorld Multi-Subsidiary Skill
