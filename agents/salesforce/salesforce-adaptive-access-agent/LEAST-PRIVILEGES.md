# Least-privilege Salesforce posture for Salesforce Adaptive Access Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent operates
entirely on sanitized configuration excerpts provided in the conversation. It has no MCP server
binding, no OAuth flow, and no live-org access path.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Transaction
Security Policy XML, Shield event monitoring exports, permission set policies, and
Context-Aware Access configuration fragments. It never initiates an OAuth flow, never receives a
session token, and never establishes a connection to a Salesforce org.

If a caller attempts to supply org credentials, session tokens, or Connected App secrets, the
agent must refuse and return those inputs unprocessed.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

Because this agent operates exclusively on static artifacts, no Salesforce identity is
established at runtime and no permission-set restrictions need to be enforced at the platform
layer. The zero blast-radius guarantee is structural, not policy-based.

## MCP server binding

None. No MCP server is permitted for T0 agents. Any harness configuration that wires an MCP
server to this agent violates the tier contract and must be rejected.

## Blast-radius bound

This agent cannot modify records, deploy metadata, invoke SOQL, read live event streams, change
Transaction Security Policy enforcement, alter permission set assignments, or affect any org
configuration. Even if an attacker gained full control of the agent's output channel, the
worst-case outcome is misleading review commentary — no org state can change as a direct result
of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org or retrieve data from a running event
      monitoring stream
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or personal identifiable information
- [ ] Any request to approve, deploy, or recommend execution of any Transaction Security Policy
      or permission set change
- [ ] Any request to disable, downgrade, or bypass a zero-trust access control without
      documented evidence from a qualified human reviewer
- [ ] Any request to evaluate Shield Platform Encryption key material or encrypted field values
      directly
- [ ] Any org-type confirmation that cannot be verified from documentary evidence (treat
      unverifiable org types as production)

## Escalation path

All mutation requests — enabling or disabling Transaction Security Policies, modifying
permission set policies, changing high-assurance session requirements, or any other live-org
operation — must be routed to **`salesforce-live-guard-agent`** with a named human decision
owner and a fully populated change envelope before any action is taken.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
