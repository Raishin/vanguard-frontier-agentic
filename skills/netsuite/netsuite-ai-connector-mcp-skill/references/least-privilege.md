# Least-privilege NetSuite posture for NetSuite AI Connector MCP Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite AI Connector Reviewer (custom)
- **Copy from standard role:** Custom role — no standard role grants only the two required AI Connector permissions; build from scratch or copy a minimal standard role and strip to View-only (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** SuiteScript, OAuth 2.0, REST Web Services
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **MCP Server Connection** (View) — The exact required permission for AI Connector access (evidence row 6b) — reviewer needs View to confirm it is present
- **Log in using OAuth 2.0 Access Tokens** (View) — The exact required permission for AI Connector OAuth 2.0 authentication (evidence row 6c) — reviewer needs View to confirm it is present and is not confused with 'Log in using Access Tokens'
- **Roles** (View) — Required to inspect the AI Connector role configuration and verify it is not the Administrator role
- **Custom Records** (View) — Required to inspect tool allowlist custom record configurations if defined as custom records

## Forbidden

- Administrator role
- Any role with full permissions to access NetSuite features (blocked by AI Connector policy, evidence row 6a)
- Log in using Access Tokens permission (this is NOT the same as Log in using OAuth 2.0 Access Tokens, evidence row 6c)
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full permissions to access NetSuite features for AI Connector configuration (evidence row 6a)
- Request asks the agent to directly activate, modify, or disable the AI Connector Service in a live account
- Request uses 'Log in using Access Tokens' instead of 'Log in using OAuth 2.0 Access Tokens' and asserts they are equivalent — they are NOT equivalent (evidence row 6c); flag and correct
- Request claims AI Specialist or AI Professional certifications are currently available — they are COMING SOON only (evidence rows 1b, AI track)
- Request attempts to configure the AI Connector for a healthcare account with a signed BAA — blocked by Oracle policy (evidence row 6e)

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

`netsuite-ai-connector-mcp-skill` — NetSuite AI Connector MCP Skill
