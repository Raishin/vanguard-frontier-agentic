# Least-privilege NetSuite posture for NetSuite AI Foundations Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite AI Foundations Reviewer (custom)
- **Copy from standard role:** Accountant (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** AI Features, OAuth 2.0, Server SuiteScript, REST Web Services
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **MCP Server Connection** (View) — Minimum required permission for AI Connector Service; must be present (evidence-matrix row 6b)
- **Log in using OAuth 2.0 Access Tokens** (View) — Required for OAuth 2.0-based AI Connector authentication; distinct from 'Log in using Access Tokens' (evidence-matrix row 6c)
- **Setup** (View) — Inspect AI feature enablement flags and account preferences for AI governance review
- **Lists** (View) — Review record type and field access configuration for AI-assisted feature scope

## Forbidden

- Administrator role
- Any role with full permissions to access NetSuite features
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, activating AI features, or modifying role permissions in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Request to assert AI Specialist or AI Professional certification as available — those are COMING SOON; refuse with explicit citation of evidence-matrix row 1b
- Claim that the Administrator role can be used for AI Connector — refuse; evidence-matrix row 6a explicitly prohibits Administrator or full-permissions roles for AI Connector

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

`netsuite-ai-foundations-skill` — NetSuite AI Foundations Skill
