# Least-privilege NetSuite posture for NetSuite SuiteFlow Automation Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews sanitized configuration excerpts and never holds a live NetSuite session.

## Identity model

No live NetSuite identity is required for the agent itself. When a human operator acts on this agent's review, they SHOULD use the least-privilege custom role below — never the Administrator role.

## Recommended custom role

- **Custom role name:** NetSuite SuiteFlow Reviewer (custom)
- **Copy from standard role:** Accountant (NetSuite guidance: start from a copy of a standard role, then remove unneeded permissions).
- **Modules in scope:** Workflow (SuiteFlow), Basic Customization, Core Administration
- **Two-Factor Authentication required:** Yes

### Minimal permissions

- **Workflow** (View) — Read workflow definition records and state/transition configuration without edit rights
- **Script Deployments** (View) — Inspect SuiteScript action deployment references embedded in workflow steps
- **Lists** (View) — Review record type and field definitions accessed by workflow conditions and actions
- **Setup** (View) — Inspect workflow-related feature flags and run-as role configuration
- **Transactions** (View) — Review transaction record types on which workflows operate, for trigger alignment validation

## Forbidden

- Administrator role
- Workflow at Edit or Full level
- Ability to activate or enable workflows
- Access Token Management permission
- OAuth 2.0 Authorized Applications Management permission
- View Unencrypted Credit Cards
- View Unencrypted ACH Account Numbers

## Blast-radius bound

Even if fully compromised, this agent cannot mutate a NetSuite account: it has no live session, no API tokens, and no SDF deploy rights. It can only produce review text.

## Refusal triggers

- Request to activate, enable, deploy, test-in-production, or change the status of any workflow in any NetSuite environment — NEVER comply; immediately escalate to netsuite-live-org-mutation-guard-agent
- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used as a workflow run-as role — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
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

`netsuite-suiteflow-automation-skill` — NetSuite SuiteFlow Automation Skill
