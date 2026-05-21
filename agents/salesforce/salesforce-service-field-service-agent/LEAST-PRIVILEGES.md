# Least-privilege Salesforce posture for Salesforce Service Field Service Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Service Cloud and Field Service configurations — case management, entitlements, omni-channel
routing, knowledge articles, SLA milestones, work order types, dispatch configurations, and
service analytics — from sanitized configuration exports and process descriptions. It never
accesses live case data, never queries customer records, and never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — entitlement
process configuration exports, SLA milestone definitions, omni-channel routing configuration
descriptions, knowledge article category structures, Field Service resource configuration
documents, work order type definitions, and service analytics configuration descriptions. It
never initiates an OAuth flow and never establishes a connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input containing live case records, customer contact
information, or field service appointment details with identifiable customer data.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify entitlement processes, alter SLA milestones, change omni-channel
routing configurations, publish knowledge articles, assign Field Service resources, create
work orders, or affect any service operation in any org. Even if an attacker fully controlled
the agent's output, no case is modified, no SLA is altered, no field technician is dispatched,
and no customer-facing service process changes as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to access live case data, customer
      contact records, or field service appointment schedules
- [ ] Any input that includes or asks the agent to process live case records, customer PII,
      field service appointment details with identifiable customer data, or SLA breach records
      with named customers
- [ ] Any request to approve, configure, or deploy changes to entitlement processes, SLA
      milestone definitions, or omni-channel routing configurations
- [ ] Any service configuration review where the actual entitlement process definition or
      SLA configuration export has not been provided in the conversation
- [ ] Any SLA configuration that removes or relaxes breach escalation without documented
      customer-impact assessment and stakeholder sign-off
- [ ] Any knowledge article governance review that approves unreviewed content for external
      customer-facing publishing

## Escalation path

All requests to modify entitlement processes, alter SLA configurations, change omni-channel
routing, deploy Field Service configurations, or make any live Service Cloud or Field Service
org change must be routed to **`salesforce-live-guard-agent`** with a named human decision
owner and a complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
