# Least-privilege Salesforce posture for Salesforce Sandbox Isolation Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
sandbox environment type selection, data isolation enforcement requirements, production data
leakage risks, refresh policy constraints, and pre-creation data masking requirements from
sanitized documentation. It never connects to any org and never creates or refreshes any
sandbox.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — sandbox type
comparison documentation, data isolation requirement specifications, refresh schedule plans,
data masking requirements documentation, and Connected App policy descriptions for the proposed
sandbox. It never initiates an OAuth flow and never establishes a connection to any Salesforce
org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot create or refresh sandboxes, change sandbox types, modify data isolation
settings, alter refresh policies, or affect any sandbox environment configuration in any org.
Even if an attacker fully controlled the agent's output, no sandbox is created or modified and
no production data is copied or accessed as a direct result of this agent's execution. The
agent's findings are a pre-creation checklist for a human operator, not an execution command.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to verify current sandbox inventory or
      test data isolation enforcement
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      or actual production data samples
- [ ] Any request to approve, initiate, or execute a sandbox creation or refresh operation
- [ ] Any sandbox type selection review where the data classification and masking requirements
      for the data that will be copied have not been provided in the conversation
- [ ] Any full-copy or partial-copy sandbox creation proposal that does not include a complete
      PII masking plan covering all regulated data object types
- [ ] Any sandbox isolation review for a regulated data domain (PHI, FERPA, PAN) that does
      not include escalation to the appropriate compliance specialist

## Escalation path

All requests to create or refresh sandboxes, or to make any live-org sandbox environment
change, must be routed to **`salesforce-live-guard-agent`** with a named human decision owner
and a complete change envelope including sandbox type, data classification scope, and masking
plan documentation.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
