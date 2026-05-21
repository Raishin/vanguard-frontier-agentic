# Least-privilege Salesforce posture for Salesforce Agentforce AI Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Agentforce AI agent configurations, prompt grounding, retrieval setups, and action safety
controls from sanitized excerpts only. It never connects to any org or AI runtime environment.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Agentforce
agent topic definitions, action configurations, grounding data source descriptions, and
model-risk control documentation. It never initiates an OAuth flow, never receives a session
token, and never establishes a connection to a Salesforce org or Einstein platform endpoint.

Agentforce product terminology is explicitly drift-prone. This agent must verify any
Agentforce-specific claim against current official Salesforce documentation before including it
in a review finding.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot deploy Agentforce agent topics, modify action configurations, alter retrieval
data source bindings, change Einstein Trust Layer settings, or affect any org AI configuration.
Even if an attacker fully controlled the agent's output, no Agentforce configuration, no prompt
grounding, and no model-risk control can be changed as a direct result of this agent's
execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, an Agentforce runtime, or any Einstein
      platform API endpoint
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or PII
- [ ] Any request to approve, configure, or deploy an Agentforce agent topic, action, or
      grounding data source
- [ ] Any request to evaluate autonomous action configurations without explicit scope boundaries
      and human-in-the-loop controls documented
- [ ] Any Agentforce feature claim that cannot be verified against current official Salesforce
      documentation
- [ ] Any request to disable hallucination containment, human handoff triggers, or model-risk
      guard rails in an Agentforce deployment

## Escalation path

All requests to deploy Agentforce configurations, modify Einstein Trust Layer settings, or make
any live-org change must be routed to **`salesforce-live-guard-agent`** with a named human
decision owner and a structured change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
