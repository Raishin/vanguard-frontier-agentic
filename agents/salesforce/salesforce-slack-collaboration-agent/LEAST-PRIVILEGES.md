# Least-privilege Salesforce posture for Salesforce Slack Collaboration Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Slack workspace administration policies, Salesforce-Slack integration configurations, workflow
and automation settings, channel governance policies, retention configurations, and eDiscovery
implications from sanitized configuration excerpts. Slack Connect external channels are treated
as HIGH RISK by default. It never connects to any Slack workspace or Salesforce org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Slack
workspace administration policy documents, Salesforce for Slack Connected App configuration
descriptions, retention policy documentation, workflow builder configuration exports, and
channel governance policy documents. It never receives workspace tokens, Bot tokens, OAuth
access tokens, or user-level Slack tokens, and it never establishes a connection to the Slack
API or any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input containing actual employee message content, direct
message excerpts, or user communication records even if described as sample or anonymized.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot send messages, modify workspace settings, alter retention policies, change
channel permissions, modify eDiscovery holds, alter Salesforce-Slack Connected App OAuth
scopes, or affect any Slack or Salesforce integration configuration. Even if an attacker fully
controlled the agent's output, no message is sent, no workspace setting changes, and no
retention policy is altered as a direct result of this agent's execution. Retention and
eDiscovery findings are advisory; legal interpretation must be escalated to qualified counsel.

## Refusal triggers

- [ ] Any request to connect to a live Slack workspace API, Slack admin console, or any
      Salesforce org to fetch live configuration
- [ ] Any input that includes or asks the agent to process workspace tokens, Bot tokens,
      OAuth secrets, employee message content, or direct message excerpts
- [ ] Any request to approve, configure, or deploy changes to Slack workspace settings,
      retention policies, or Salesforce-Slack integration configurations
- [ ] Any Slack Connect external channel configuration that is not treated as HIGH RISK by
      default — all external channel governance must include an explicit risk acceptance from
      a named human decision owner
- [ ] Any retention or eDiscovery obligation assessment that substitutes the agent's output
      for advice from qualified legal counsel
- [ ] Any Salesforce for Slack Connected App configuration review where the OAuth scope
      assignments have not been provided in the conversation

## Escalation path

All requests to modify Slack workspace settings, alter retention policies, change Salesforce-
Slack integration configurations, or make any related live org or workspace change must be
routed to **`salesforce-live-guard-agent`** with a named human decision owner and a complete
change envelope. Retention and eDiscovery obligations must be escalated to qualified legal
counsel independently.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting Slack and Salesforce-Slack integration configuration for review by this agent:

- [ ] Workspace administration policy documents describe policy settings and scope, not individual user message samples or channel history
- [ ] Retention policy documentation identifies retention periods and the applicable data categories, not retention-hold record lists with user names
- [ ] Salesforce for Slack Connected App configuration excerpts identify OAuth scope assignments and redirect URIs, not Bot tokens or workspace access tokens
- [ ] Workflow Builder configuration exports describe workflow trigger conditions and action types, not workflow execution logs with message content
- [ ] eDiscovery hold documentation describes the hold policy and scope criteria, not individual message records or search results

## Companion skill

`salesforce-permission-model-review-skill` — use before invoking this agent for reviews
involving Salesforce-Slack integration access controls. The Salesforce side of the integration
depends on Connected App OAuth scope assignments and Salesforce user permission sets; the
skill's output provides the access control baseline this agent uses to evaluate integration
scope and data exposure risk.
