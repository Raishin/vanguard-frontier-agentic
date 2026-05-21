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

## Validation checklist

Before submitting platform administration artifacts for review by this agent:

- [ ] Profile and permission set XML exports are from the Metadata API or SFDX retrieve, not from live user record screens with personal data visible
- [ ] Object and field configuration exports describe metadata structure, not record payloads or sample data
- [ ] Layout definitions are metadata XML, not screenshots of Setup pages with draft changes visible
- [ ] User administration exports identify user license types and permission assignments, not personal user details beyond username format
- [ ] Release impact documentation references the metadata components and business process areas affected, not production data volumes or customer names

## Companion skill

`salesforce-metadata-review-skill` — use before invoking this agent to run the standard
metadata quality review. The skill covers permission set design principles, profile-vs-
permission-set governance, layout design standards, and release-impact categories that this
agent applies when reviewing submitted platform administration configuration artifacts.

## sf CLI example — login with minimum scopes

```bash
sf org login web \
  --instance-url https://login.salesforce.com \
  --scopes "api refresh_token" \
  --set-default
```

This example is shown for reference only. T0 agents never execute this command. If a
T1-or-above upgrade is evaluated for this agent, the Connected App must be created with
exactly these scopes and the org allowlist must be enforced before any CLI invocation.
