# Least-privilege Salesforce posture for Salesforce Integration MuleSoft Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Salesforce API integration designs, MuleSoft flow definitions, event-driven architecture plans,
Platform Event configurations, CDC subscriber designs, and middleware error-handling patterns
from sanitized design documents and API specification excerpts. It never invokes Salesforce APIs,
never connects to MuleSoft Runtime Manager, and never establishes a live middleware connection.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — OpenAPI or
RAML specification files, MuleSoft application topology diagrams, Platform Event schema
definitions, CDC configuration descriptions, Named Credential configuration excerpts, and
integration error-handling documentation. It never initiates an OAuth flow and never
establishes a connection to any Salesforce org, MuleSoft Anypoint Platform, or external
middleware system.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot deploy MuleSoft applications, publish Platform Event schemas, activate CDC
channels, modify Named Credentials, configure Connected Apps for integration, or affect any
integration in any org or middleware runtime. Even if an attacker fully controlled the agent's
output, no API call is made, no integration flow is deployed, and no middleware connection is
established as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, MuleSoft Runtime Manager, Anypoint
      Platform, or any external middleware runtime
- [ ] Any request that includes or asks the agent to process org credentials, MuleSoft Runtime
      Manager credentials, session tokens, or API keys for any connected system
- [ ] Any request to approve, deploy, or execute an integration deployment or middleware
      configuration change
- [ ] Any integration design review where the actual API specification, MuleSoft flow
      definition, or Platform Event schema has not been provided in the conversation
- [ ] Any point-to-point integration design without idempotency, error handling, and retry
      boundary documentation
- [ ] Any integration pattern involving regulated data (PHI, PII, PAN) without documented
      transit encryption and access control requirements

## Escalation path

All requests to deploy integrations, publish Platform Events schemas, activate CDC channels,
or make any live-org integration change must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner and a complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting integration artifacts for review by this agent:

- [ ] API specification files (OpenAPI, RAML) are the design-time contract, not live response payloads with production data
- [ ] MuleSoft application topology diagrams describe component names, protocols, and data flows — not runtime connection configurations with credentials
- [ ] Platform Event schema definitions identify event fields and types, not event payloads with record values
- [ ] Named Credential configuration excerpts describe the authentication type and endpoint pattern, not actual credential values
- [ ] Error handling and retry boundary documentation includes retry counts, backoff strategies, and DLQ configurations

## Companion skill

`salesforce-integration-review-skill` — use before invoking this agent to run the standard
integration review checklist. The skill covers idempotency requirements, error envelope
standards, event-driven ordering guarantees, and API versioning compliance that this agent
evaluates in submitted integration design artifacts.

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
