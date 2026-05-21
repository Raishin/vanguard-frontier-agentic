# Least-privilege Salesforce posture for Salesforce Platform Admin Review Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
org configuration — objects, fields, layouts, permissions, flows, reports, dashboards, user
administration, and release-impact — from sanitized metadata exports and pasted excerpts. It
never requests live-org access and never invokes Salesforce APIs or the sf CLI.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — metadata
exports, profile and permission set XML, layout definitions, object and field configurations,
report and dashboard configuration descriptions, and release notes. It never initiates an OAuth
flow and never establishes a connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify object definitions, alter permission sets, deploy layouts, change
user license assignments, activate or deactivate users, modify sharing rules, or affect any
platform administration configuration in any org. Even if an attacker fully controlled the
agent's output, no admin configuration, no user record, and no platform setting can change as
a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, invoke Salesforce APIs, or run the
      sf CLI against any org
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or user personal data beyond what appears in sanitized metadata exports
- [ ] Any request to approve, deploy, or execute any org configuration change — including
      permission set assignments, profile changes, or user license changes
- [ ] Any configuration review where the actual metadata export or sanitized configuration
      excerpt has not been provided in the conversation
- [ ] Any permission review that approves over-permissioned profiles or permission sets without
      documenting the business justification and risk acknowledgment
- [ ] Any release-impact assessment that treats verbal confirmation of change scope as
      sufficient evidence

## Escalation path

All requests to implement configuration changes, assign permission sets, modify profiles, or
make any live-org admin change must be routed to **`salesforce-live-guard-agent`** with a named
human decision owner and a complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
