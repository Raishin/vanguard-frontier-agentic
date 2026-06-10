# Least-privilege NetSuite posture for NetSuite Web Services Integration Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite Web Services Integration Reviewer (custom)
- **Copy from standard role:** Integration Manager (or closest available standard role with web services access) (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** REST Web Services, SOAP Web Services, OAuth 2.0, Token-Based Authentication
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **REST Web Services** (View) — Required to review REST integration record configurations
- **SOAP Web Services** (View) — Required to review SOAP configuration for migration-risk assessment
- **Integration Record** (View) — Required to inspect integration record settings and OAuth grant configuration
- **Log in using OAuth 2.0 Access Tokens** (View) — Required to review OAuth 2.0 token grant configuration
- **Access Token Management** (View) — Required to review TBA token records — triggers mandatory 2FA per evidence-matrix row 5c

## Forbidden

- Administrator role
- Full permission roles
- Any role with Create/Edit/Full on Integration Record or Token Management

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to fire live API calls or mutate a NetSuite account
- User claims Web Services Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires evaluating SOAP integration as a long-term strategy without flagging migration risk

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

`netsuite-web-services-integration-skill` — NetSuite Web Services Integration Skill
