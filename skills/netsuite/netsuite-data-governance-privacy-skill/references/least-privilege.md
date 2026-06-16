# Least-privilege NetSuite posture for NetSuite Data Governance & Privacy Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Data Governance Reviewer (custom)
- **Copy from standard role:** Full Access (standard role — copy and heavily restrict to View-only on configuration objects) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** CRM, HR / Employees, Saved Searches
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Employee Record** (View) — Required to inspect PII field visibility on employee records
- **Customer** (View) — Required to inspect PII field visibility on customer records
- **Contact** (View) — Required to inspect PII field visibility on contact records
- **Saved Searches** (View) — Required to review saved search audience and PII field exposure
- **Custom Fields** (View) — Required to review custom PII field configurations and field-level security settings
- **Roles** (View) — Required to review role field-access configurations for PII records

## Forbidden

- Administrator role
- View Unencrypted Credit Cards permission
- View Unencrypted ACH Account Numbers permission
- Access Token Management permission
- Edit or Create level on any PII-bearing record type
- Mass Update permission
- CSV Export on employee or customer records without documented justification

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request provides actual personal data (real names, SSNs, email addresses, phone numbers, bank account numbers, or healthcare data) — refuse immediately, do not log or echo, ask for sanitized version
- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete field-security configurations, retention policies, or consent records in a live account
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

`netsuite-data-governance-privacy-skill` — NetSuite Data Governance & Privacy Skill
