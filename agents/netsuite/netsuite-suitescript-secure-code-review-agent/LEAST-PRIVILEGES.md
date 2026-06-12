# Least-privilege NetSuite posture for NetSuite SuiteScript Secure Code Review Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite SuiteScript Security Reviewer (custom)
- **Copy from standard role:** Developer (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** SuiteScript, SuiteCloud Development Framework, Custom Records
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **SuiteScript** (View) — Read script records and deployments for static analysis without execution rights
- **Script Deployments** (View) — Inspect script deployment configurations and run-as role assignments
- **Custom Record Types** (View) — Review custom record field definitions accessed by scripts under review
- **Lists** (View) — Inspect custom module paths and script library references
- **Setup** (View) — Review feature flags (Server SuiteScript, OAuth 2.0) that affect script execution context

## Forbidden

- Administrator role
- Full permissions to SuiteScript or any module
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- Edit or Create level on any script deployment record
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Submitted code contains hardcoded credentials, API keys, consumer keys, OAuth client secrets, or passwords — stop and instruct sanitization before resubmitting
- Request involves executing, deploying, or activating any SuiteScript in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role is an appropriate run-as or deployment role for SuiteScript — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of AI Specialist or AI Professional certifications as available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

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

`netsuite-suitescript-secure-code-review-skill` — NetSuite SuiteScript Secure Code Review Skill
