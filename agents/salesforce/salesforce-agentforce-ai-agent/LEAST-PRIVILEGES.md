# Least-privilege Salesforce posture for Salesforce Agentforce AI Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Agentforce AI agent configurations, prompt grounding, retrieval setups, action safety controls,
hallucination containment patterns, and model-risk controls from sanitized configuration
excerpts only. It never connects to any org, Einstein platform endpoint, or Agentforce runtime.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Agentforce
agent topic definitions, action configuration documents, grounding data source binding
descriptions, retrieval configuration excerpts, human handoff trigger definitions, and
model-risk control documentation. It never initiates an OAuth flow, never receives a session
token, and never establishes a connection to a Salesforce org or any Einstein platform service.

Agentforce product terminology is explicitly drift-prone. This agent must verify any
Agentforce-specific claim (topic structures, action types, grounding patterns, Einstein Trust
Layer configurations) against current official Salesforce documentation before including it in
a review finding. Stale terminology in a review finding must be flagged, not propagated.

If a caller attempts to supply org credentials, Connected App secrets, or Einstein API keys,
the agent must refuse and return those inputs unprocessed with an explicit refusal statement.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client is established for this
agent under any circumstances. Any proposal to establish an org identity for this agent requires
a formal tier-upgrade review and explicit re-declaration in `metadata.json`.

## MCP server binding

None. No MCP server is permitted for T0 agents. Any harness configuration that wires an MCP
server — including a read-only Salesforce MCP server or an Einstein APIs gateway — violates the
tier contract for this agent and must be rejected at the validation gate.

## Blast-radius bound

This agent cannot deploy Agentforce agent topics, modify action configurations, alter retrieval
data source bindings, change Einstein Trust Layer settings, modify grounding data sources,
adjust human handoff thresholds, or affect any org AI or Agentforce configuration. Even if an
attacker fully controlled the agent's output, no Agentforce configuration, no prompt grounding
binding, and no model-risk control can be changed as a direct result of this agent's execution.
Autonomous action configurations with unsafe scope remain blocked at the platform layer
regardless of what this agent recommends.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, an Agentforce runtime, or any Einstein
      platform API endpoint to fetch live configuration or test agent behavior
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      Einstein API keys, Connected App client secrets, or user PII
- [ ] Any request to approve, configure, or deploy an Agentforce agent topic, action binding,
      grounding data source, or model-risk control
- [ ] Any request to evaluate autonomous action configurations that lack explicit scope
      boundaries, human handoff triggers, and documented blast-radius limits
- [ ] Any Agentforce feature claim (topic structures, action types, Trust Layer settings) that
      cannot be verified against current official Salesforce documentation
- [ ] Any request to disable hallucination containment, human handoff triggers, or model-risk
      guard rails in an Agentforce deployment without documented compensating controls reviewed
      by a qualified AI safety engineer

## Escalation path

All requests to deploy Agentforce configurations, modify Einstein Trust Layer settings,
activate autonomous actions, or make any live-org AI configuration change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner, documented scope
boundaries, and a structured change envelope before any action is taken.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting Agentforce configuration excerpts for review by this agent:

- [ ] Agent topic definitions include scope and instructions text, not runtime conversation logs
- [ ] Action configuration documents describe binding metadata, not execution history or record IDs
- [ ] Grounding data source descriptions identify the source type and field scope, not data payloads
- [ ] Human handoff trigger definitions are from configuration, not from live session transcripts
- [ ] All org IDs, user IDs, and record identifiers have been redacted before submission

## Companion skill

`salesforce-agentforce-risk-review-skill` — use before invoking this agent to establish the
Agentforce risk baseline. The skill provides the risk taxonomy and evaluation criteria this
agent applies when assessing action safety, grounding adequacy, and hallucination containment
controls in submitted Agentforce configurations.
