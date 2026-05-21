# Least-privilege Salesforce posture for Salesforce Live Guard Agent

## Execution tier

**T0 — Static Review** (advisory checklist emitter only)

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. Despite its name and
its position as the gateway for live-org change decisions, this agent is itself a static-review
agent. It never executes org mutations, never invokes the sf CLI, and never calls Salesforce
APIs. Its output is a structured refusal or a precondition checklist for a qualified human
operator to act on. Production mutation (T3) is PROHIBITED for this and every agent in this
portfolio.

## Identity model

No live identity required. This agent operates entirely on documentary evidence submitted by
human operators in the conversation. It never initiates an OAuth flow, never receives a session
token, and never establishes a connection to any Salesforce org.

Any attempt to supply org credentials, client secrets, or session tokens to this agent must
trigger an immediate refusal. The agent must return those inputs unprocessed and emit a
structured warning.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## Human-in-the-loop requirements

This agent enforces, but does not replace, the following human controls:

1. **Named change owner** — every change envelope must include the Salesforce username of the
   human operator who will execute the change and their role.
2. **Dual control** — for security configuration changes (permissions, sharing, Shield,
   Connected Apps), a second named approver with documented authority must be present in the
   change evidence.
3. **Change-window enforcement** — the change ticket must reference an approved change window;
   the agent refuses to emit a CHECKLIST READY state outside a documented approved window.
4. **Rollback protocol** — a documented rollback procedure with a named rollback owner must be
   present before the checklist can be marked ready. The rollback owner must be a different
   person from the change executor when the change risk is HIGH or CRITICAL.

## Ten required preconditions

All ten must be present with documentary evidence before a CHECKLIST READY state is emitted:

```json
{
  "requiredPreconditions": [
    "target_org_identity_and_type",
    "environment_type_confirmed",
    "user_identity_and_role",
    "permission_scope_in_effect",
    "approved_change_ticket_reference",
    "change_advisory_board_approval",
    "dry_run_or_validation_deploy_evidence",
    "backup_and_rollback_plan",
    "test_evidence_with_coverage",
    "post_change_verification_plan"
  ],
  "evidenceStandard": "documentary — verbal or summary statements are not accepted for any precondition"
}
```

## MCP server binding

None. No MCP server is permitted for this agent.

## Blast-radius bound

This agent cannot execute deployments, invoke APIs, modify org configuration, approve changes,
or authorize any production operation. Even if an attacker fully controlled the agent's output,
no org state changes because this agent never issues executable commands. The blast radius is
limited to the advisory quality of its checklist — a compromised checklist might omit a
precondition, but the human change owner retains execution authority and must independently
verify the checklist before acting.

## Refusal triggers

- [ ] Any of the ten required preconditions is absent, insufficient, or unverifiable from the
      evidence provided
- [ ] Org identity or org type is undeclared or cannot be verified from documentary evidence —
      treat as production and apply full preconditions
- [ ] No change ticket or change advisory board approval evidence is provided
- [ ] No dry-run or validation-only deploy output is provided
- [ ] No backup or rollback plan with a named rollback owner is documented
- [ ] Any request to execute, simulate, describe execution of, or "just do" an org mutation
- [ ] Any input containing org credentials, session tokens, client secrets, or API keys

## Escalation path

This agent is the escalation terminus for live-org change requests from all other Salesforce
agents in this portfolio. It does not escalate further — it emits a structured refusal or a
precondition checklist and routes the decision exclusively to the **named human change owner**
identified in the change envelope. No agent in this portfolio has authority to bypass or
override the Live Guard gate.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
