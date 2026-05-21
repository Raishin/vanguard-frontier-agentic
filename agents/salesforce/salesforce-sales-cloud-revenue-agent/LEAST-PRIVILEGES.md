# Least-privilege Salesforce posture for Salesforce Sales Cloud Revenue Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
lead-to-cash configurations, opportunity lifecycle settings, forecasting hierarchies, territory
models, CPQ/Revenue Cloud pricing rules, and pipeline integrity controls from sanitized
configuration exports and process descriptions. It never accesses live pipeline data, never
queries opportunity records, and never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Sales Cloud
configuration exports, forecasting hierarchy definitions, territory model documentation, CPQ
product catalog and pricing rule descriptions, Revenue Cloud contract lifecycle descriptions,
and sales process documentation. It never initiates an OAuth flow and never establishes a
connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input containing live pipeline data, real opportunity
amounts, named account revenue figures, or compensation-sensitive pricing information.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify opportunity stage definitions, alter forecasting configurations,
change territory assignments, update CPQ pricing rules, deploy Revenue Cloud contract settings,
or affect any sales process in any org. Even if an attacker fully controlled the agent's
output, no opportunity record, no pipeline figure, and no revenue configuration can change as a
direct result of this agent's execution. The agent does not approve pricing, discount, or
revenue decisions.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to query opportunity pipeline, revenue
      figures, or forecasting data
- [ ] Any input that includes or asks the agent to process live pipeline data, real opportunity
      amounts, named account revenue figures, or compensation-sensitive pricing information
- [ ] Any request to approve pricing decisions, discount authorities, or revenue recognition
      policies — these require qualified finance and legal review
- [ ] Any CPQ or Revenue Cloud configuration review where the actual pricing rule definitions
      or product catalog configuration has not been provided in the conversation
- [ ] Any forecasting configuration that does not include territory hierarchy and sharing model
      documentation
- [ ] Any revenue process change that would affect recognized revenue without documented
      finance controller sign-off

## Escalation path

All requests to implement Sales Cloud configuration changes, modify CPQ pricing rules, alter
revenue recognition settings, or make any live-org sales process change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner and a complete change
envelope. Pricing and revenue recognition decisions must additionally be reviewed by qualified
finance and legal stakeholders.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting Sales Cloud and revenue configuration for review by this agent:

- [ ] Sales Cloud configuration exports describe process settings and field definitions, not live opportunity records with deal values
- [ ] Forecasting hierarchy definitions identify roles and quota methodology, not individual rep targets or compensation data
- [ ] CPQ product catalog descriptions identify product families, option groups, and configuration rules — not customer-specific price book entries
- [ ] Revenue Cloud contract lifecycle descriptions identify stage definitions and automation logic, not live contract records with customer names
- [ ] Territory model documentation describes hierarchy levels and assignment criteria, not individual rep-to-account assignments

## Companion skill

`salesforce-org-assessment-skill` — use before invoking this agent to establish the Sales Cloud
configuration baseline. The skill's revenue process, object model, and automation sections
provide the dependency context this agent uses to evaluate forecasting accuracy, CPQ pricing
integrity, and pipeline leakage risks in submitted Sales Cloud artifacts.
