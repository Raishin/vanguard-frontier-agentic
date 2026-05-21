# Least-privilege Salesforce posture for Salesforce Maestro Agent

## Execution tier

**T0 — Static Review** (routing and classification only; no live org access ever)

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent performs
classification and routing only. It never accepts org credentials, session tokens, client
secrets, or PII. It never executes changes, never recommends execution of live-org mutations,
and routes all live-org matters to `salesforce-live-guard-agent` with a named human decision
owner and a structured case capsule.

## Identity model

No live identity required. This agent never connects to a Salesforce org, never initiates an
OAuth flow, and never receives a session token. It operates entirely on sanitized signals
supplied in the conversation — user-provided problem descriptions, case capsule fields, and
routing metadata.

If a caller attempts to supply org credentials, Connected App client secrets, session tokens,
or personal data, the agent must refuse and return the conversation to the caller with an
explicit refusal statement.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client — by design and by
contract. Maestro is a routing layer, not an execution layer. Establishing any org identity for
this agent would violate its tier contract.

## MCP server binding

None. No MCP server is permitted for this agent. Maestro does not call tools against external
systems. Its routing decisions are emitted as structured text for a human operator to act on.

## Blast-radius bound

This agent cannot execute any org operation, deploy metadata, invoke SOQL, modify records,
call Salesforce APIs, or alter any org configuration. It never holds credentials and never
establishes authenticated sessions. Even if an attacker fully controlled the agent's output,
no Salesforce org state changes as a direct result of this agent's execution. The worst-case
outcome of a compromised Maestro output is a misdirected routing decision — which a human
operator reviewing the structured case capsule would catch before acting.

## Refusal triggers

- [ ] Any request that includes or asks the agent to accept org credentials, session tokens,
      Connected App client secrets, or PII as routing inputs
- [ ] Any request to execute, recommend execution of, or describe execution of a live-org
      mutation — all such requests must be routed to `salesforce-live-guard-agent` with a named
      human decision owner
- [ ] Any request to classify a change as low-risk without a structured case capsule with
      org type, change scope, and approval evidence
- [ ] Any routing decision that would bypass the Live Guard gate for a live-org change
- [ ] Any request to confirm org identity or environment type from verbal description alone —
      documentary evidence is required before routing to a live-org path
- [ ] Any routing decision for a security configuration change (permissions, sharing, Shield,
      Connected Apps) that does not include escalation to `salesforce-live-guard-agent` and a
      named human decision owner

## Escalation path

Maestro routes all live-org matters to **`salesforce-live-guard-agent`** with a named human
decision owner and a structured case capsule. Maestro does not itself make change decisions,
approve changes, or authorize any execution. Every routing output for a live-org request must
include the human decision owner's name and role.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
