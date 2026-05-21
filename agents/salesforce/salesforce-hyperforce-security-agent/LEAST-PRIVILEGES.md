# Least-privilege Salesforce posture for Salesforce Hyperforce Security Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Hyperforce deployment security posture, data residency commitments, Hyperforce Infrastructure
Access (HIA) controls, and shared responsibility boundaries from sanitized documentation and
configuration excerpts. It never connects to any org, cloud provider console, or Hyperforce
management plane.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Hyperforce
region configuration documentation, data residency attestation documents, HIA access policy
exports, security baseline documents, and shared responsibility matrix artifacts. It never
initiates an OAuth flow and never establishes a connection to a Salesforce org, AWS console,
Azure portal, or any Hyperforce management interface.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify Hyperforce region assignments, alter data residency configurations,
change HIA access policies, reconfigure shared responsibility controls, or affect any
Hyperforce infrastructure setting. Even if an attacker fully controlled the agent's output,
no Hyperforce configuration, no data residency commitment, and no HIA policy can change as a
direct result of this agent's execution. The agent reviews the Salesforce customer's
configuration posture only; it has no access to Salesforce's own Hyperforce infrastructure
management systems.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, a Hyperforce management console, or
      any cloud provider control plane
- [ ] Any request that includes or asks the agent to process org credentials, API keys,
      cloud-provider access keys, or HIA service-account credentials
- [ ] Any request to approve, configure, or change a Hyperforce region assignment or data
      residency commitment
- [ ] Any Hyperforce feature or compliance claim that cannot be verified against current
      official Salesforce documentation
- [ ] Any request to confirm Hyperforce data residency compliance without the official
      Salesforce data residency attestation document provided
- [ ] Any shared responsibility boundary assessment where the Salesforce Trust site or
      official Hyperforce documentation has not been consulted

## Escalation path

All requests to change Hyperforce region configuration, alter HIA policies, or make any
Hyperforce-related change in a live org must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner. Changes affecting data residency commitments must also be
escalated to qualified legal and compliance counsel before the change envelope is submitted.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
