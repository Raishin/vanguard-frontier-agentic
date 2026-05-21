# Least-privilege Salesforce posture for Salesforce Marketing Cloud Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Marketing Cloud Engagement and Account Engagement journey designs, segmentation configurations,
deliverability settings, consent management setups, and campaign governance documentation from
sanitized excerpts. It never connects to a Marketing Cloud tenant, never accesses subscriber
data, and never sends test messages.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Journey
Builder canvas exports, Email Studio configuration descriptions, segmentation data extension
schemas, preference center configuration documents, and consent management policy
documentation. It never initiates an OAuth flow and never establishes a connection to a
Marketing Cloud or Account Engagement tenant.

This agent refuses to perform a product-specific review when the specific Marketing Cloud
product (Marketing Cloud Engagement vs. Account Engagement vs. Marketing Cloud Growth/Advanced)
has not been explicitly declared — the products have materially different data models and
compliance obligations.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input containing subscriber PII (email addresses, phone
numbers, device identifiers) even if described as sample or anonymized data.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot send messages, activate journeys, modify subscriber lists, alter consent
records, change deliverability settings, deploy email templates, or affect any Marketing Cloud
configuration. Even if an attacker fully controlled the agent's output, no message is sent, no
subscriber record is modified, and no campaign is activated as a direct result of this agent's
execution. Consent and regulatory obligations are advisory findings only; legal interpretation
must be escalated to qualified privacy counsel.

## Refusal triggers

- [ ] Any request to connect to a live Marketing Cloud or Account Engagement tenant, access
      subscriber data, or send test or live messages
- [ ] Any input that includes or asks the agent to process subscriber PII (email addresses,
      phone numbers, device identifiers) even if described as sample or anonymized
- [ ] Any product-specific review where the specific Marketing Cloud product has not been
      explicitly declared at the start of the conversation
- [ ] Any request to approve, activate, or deploy a journey, campaign, or email send
- [ ] Any consent or regulatory obligation assessment that substitutes the agent's output for
      advice from qualified privacy counsel
- [ ] Any deliverability configuration review without the actual sending domain, IP warming
      plan, and authentication record (SPF, DKIM, DMARC) documentation provided

## Escalation path

All requests to activate journeys, modify subscriber consent records, deploy campaigns, or
make any live Marketing Cloud configuration change must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner and a structured change envelope. Consent and regulatory
obligations must be escalated to qualified privacy counsel independently.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
