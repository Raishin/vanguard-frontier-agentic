# Least-privilege Salesforce posture for Salesforce Sandbox Governance Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
sandbox data governance posture, PII masking strategy, Connected App scope in sandbox
environments, and access controls to prevent regulated data leakage into lower environments.
It never connects to any sandbox or production org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — sandbox
creation configuration documentation, data masking rule definitions, Connected App policy
descriptions for sandbox-scoped apps, sandbox refresh schedule documentation, and access
control policy documents. It never initiates an OAuth flow and never establishes a connection
to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input that contains actual production data samples,
even described as used for sandbox masking rule validation.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot create or refresh sandboxes, apply data masking rules, modify Connected App
scope settings for sandbox environments, alter sandbox access controls, or affect any sandbox
governance configuration. Even if an attacker fully controlled the agent's output, no sandbox
is created, refreshed, or modified and no production data is accessed as a direct result of
this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org (production or sandbox) to fetch live
      configuration or validate masking rules
- [ ] Any input that includes or asks the agent to process actual production data samples,
      even described as used for masking rule validation
- [ ] Any request to approve, initiate, or execute a sandbox creation, refresh, or data
      masking operation
- [ ] Any sandbox governance review where the masking rule definitions and Connected App
      scope documentation have not been provided in the conversation
- [ ] Any Connected App configuration for sandbox that includes `full`, `web`, `chatbot_api`,
      or `sfap_api` scopes without documented exception justification
- [ ] Any sandbox data governance posture assessment that does not verify PII masking coverage
      for all regulated data object types (PHI, PII, PAN) before sandbox refresh

## Escalation path

All requests to create or refresh sandboxes, apply data masking rules, or make any live
sandbox governance change must be routed to **`salesforce-live-guard-agent`** with a named
human decision owner and a complete change envelope including masking rule coverage
documentation.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting sandbox governance documentation for review by this agent:

- [ ] Sandbox creation configuration documentation identifies the sandbox type, org ID template source, and Connected App scope — not production data samples
- [ ] Data masking rule definitions describe the masking strategy and field scope, not before-and-after data value examples
- [ ] Connected App policy descriptions for sandbox-scoped apps identify OAuth scope assignments and IP restriction settings
- [ ] Sandbox refresh schedule documentation identifies cadence and responsible owner by role, not by personal name with contact details
- [ ] Access control policy documents describe role-based access assignments for sandbox environments, not individual user lists with names

## Companion skill

`salesforce-devsecops-pipeline-skill` — use before invoking this agent to establish the
DevSecOps baseline for sandbox governance. The skill's data masking, Connected App scope, and
environment access control sections define the governance criteria this agent applies when
reviewing submitted sandbox governance configuration and policy documents.
