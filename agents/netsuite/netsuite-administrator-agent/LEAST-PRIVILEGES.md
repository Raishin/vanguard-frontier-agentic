# Least-privilege NetSuite posture for NetSuite Administrator Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Administrator Reviewer (custom)
- **Copy from standard role:** Full Access (read-only copy, stripped of all Edit/Create/Full levels) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Core Administration, Company Preferences, Currency Management, User Management, Email Management
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Company Information** (View) — Inspect legal entity, tax registration, and nexus settings
- **Accounting Preferences** (View) — Review fiscal year, period, and accounting impact defaults
- **Currency** (View) — Review base currency, multi-currency, and exchange rate source settings
- **Manage Users** (View) — Review user provisioning patterns and role assignment without editing user records
- **Setup** (View) — Review page layout, tab customization, and system preferences
- **Email Preferences** (View) — Inspect email template defaults and bounce handling settings
- **Sandbox Management** (View) — Review sandbox environment list and refresh history (no initiation rights)

## Forbidden

- Administrator role — absolute prohibition regardless of context
- Edit or Full level on any Setup or Users/Roles page
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- Core Administration Permissions bundle

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, passwords, or any authentication material — stop and require sanitization before resubmitting
- Request involves executing, deploying, or activating any configuration change in a live or production account
- Request to use or recommend the Administrator role for any purpose — an absolute refusal; cite evidence-matrix rows 7a and 7b
- Request to connect, authenticate, or log in to any NetSuite environment
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production-environment changes without documented sandbox validation evidence

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

`netsuite-administrator-skill` — NetSuite Administrator Skill
