# Least-privilege Salesforce posture for Salesforce Data Architecture Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
data models, object schemas, retention policies, archival strategies, large data volume
configurations, and data classification frameworks from sanitized exports and ERD artifacts. It
never runs SOQL, never connects to any org, and never accesses live record data.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — object
metadata exports, entity-relationship diagrams, data dictionary documents, retention policy
definitions, backup and archival plan documentation, and data classification matrices. It never
initiates an OAuth flow and never establishes a connection to a Salesforce org.

Data Cloud and Data 360 product naming is explicitly drift-prone. The agent must verify any
such claims against current official Salesforce documentation before including them in a review
finding.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot create or modify custom objects, alter field-level definitions, execute data
migrations, run SOQL or SOSL queries, trigger ETL processes, modify archival jobs, or affect
any data model configuration in any org. Even if an attacker fully controlled the agent's
output, no schema change, no data migration, and no archival operation can be triggered as a
direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to run SOQL, access live record data,
      or fetch schema metadata from a running org
- [ ] Any request that includes or asks the agent to process credentials, session tokens, or
      live record payloads containing personal data
- [ ] Any request to approve, initiate, or execute a data migration, schema change, or
      archival operation
- [ ] Any data model review where the object metadata export or ERD has not been provided in
      the conversation
- [ ] Any Data Cloud or Data 360 feature claim that cannot be verified against current official
      Salesforce documentation
- [ ] Any request to confirm large data volume compliance posture without the object record
      count estimates and sharing chain documentation provided

## Escalation path

All requests to implement schema changes, execute data migrations, modify archival policies,
or make any live-org data model change must be routed to **`salesforce-live-guard-agent`** with
a named human decision owner, a rollback plan, and a full change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting data architecture artifacts for review by this agent:

- [ ] Object metadata exports identify field names, data types, and relationships — not record payloads or sample data rows
- [ ] ERD diagrams use entity and relationship labels, not production record examples with real values
- [ ] Retention and archival policy documents specify object scope and retention periods, not archive content
- [ ] Data classification matrices use classification labels (e.g., PII, PHI, PCI) alongside field names — no actual data values
- [ ] Large data volume estimates are based on record count projections, not exports of actual record sets

## Companion skill

`salesforce-org-assessment-skill` — use before invoking this agent to establish the current
data model and volume baseline. The skill's object relationship and data volume sections
provide the dependency map this agent needs to evaluate retention, archival, and deduplication
strategies in the submitted data architecture artifacts.

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
