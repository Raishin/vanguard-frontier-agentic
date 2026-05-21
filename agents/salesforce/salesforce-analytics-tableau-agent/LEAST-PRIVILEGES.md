# Least-privilege Salesforce posture for Salesforce Analytics and Tableau Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
CRM Analytics, Tableau, and Einstein Discovery configurations from sanitized exports and
excerpts. It never queries live dashboards, never fetches live dataset rows, and never connects
to a Salesforce or Tableau server.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — CRM Analytics
dashboard JSON exports, Tableau workbook XML, KPI definition documents, semantic layer
descriptions, and dataset lineage documentation. It never initiates an OAuth flow and never
establishes a connection to a CRM Analytics org, a Tableau Cloud site, or a Tableau Server
instance.

Einstein Discovery product naming is explicitly drift-prone. The agent must verify
Einstein-specific terminology against current official Salesforce documentation before including
it in a review finding.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

If a future tier upgrade is considered for live dashboard telemetry observation, the minimum
allowed object reads would be scoped to:

```json
{
  "AllowedObjectReads": ["Report", "Dashboard", "ForecastingItem"],
  "ExplicitDenials": [
    "ModifyAllData",
    "ViewAllData",
    "ViewEncryptedData",
    "ModifyMetadata",
    "AuthorApex",
    "ManageConnectedApps"
  ]
}
```

No such upgrade is authorized under the current T0 declaration. This block is documented for
planning purposes only.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot create or modify dashboards, alter dataset definitions, change KPI governance
policies, add or remove Tableau users, adjust CRM Analytics sharing settings, or affect any
analytics configuration. Even if an attacker fully controlled the agent's output, no dashboard,
dataset, or executive report can be changed as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live CRM Analytics org, Tableau Cloud site, or Tableau
      Server instance
- [ ] Any request that includes or asks the agent to process org credentials, Tableau Personal
      Access Tokens, session tokens, or personal data appearing in dashboard screenshots
- [ ] Any request to approve, publish, or deploy a dashboard, dataset, or KPI definition
- [ ] Any Einstein Discovery or CRM Analytics feature claim that cannot be verified against
      current official Salesforce documentation
- [ ] Any request to define or certify an executive KPI without a documented business owner and
      semantic definition
- [ ] Any request to authorize uncontrolled executive data export without a documented data
      classification and access justification

## Escalation path

All requests to publish dashboards, alter dataset bindings, change sharing settings, or make any
live-org analytics change must be routed to **`salesforce-live-guard-agent`** with a named human
decision owner and a structured change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
