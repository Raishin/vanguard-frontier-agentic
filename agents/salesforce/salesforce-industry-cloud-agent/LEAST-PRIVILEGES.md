# Least-privilege Salesforce posture for Salesforce Industry Cloud Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent routes
questions to vertical specialists and reviews sanitized configuration excerpts for Education
Cloud, Nonprofit Cloud, Life Sciences Cloud, B2C Commerce, and Industries CPQ. It never
accesses live PHI, student records, donor PII, or cardholder data and does not perform
substantive compliance certification for any regulated vertical.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — vertical cloud
configuration documentation, industry data model descriptions, regulatory overlap documentation
(HIPAA, FERPA, PCI), and CPQ pricing rule definitions. It never initiates an OAuth flow and
never establishes a connection to any Salesforce org, health system, educational institution, or
payment processor.

All vertical-specific feature claims (Education Cloud, Nonprofit Cloud, Health Cloud, Life
Sciences Cloud, B2C Commerce, Industries CPQ) are explicitly drift-prone and must be verified
against current official Salesforce documentation before inclusion in a review finding.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse inputs containing PHI (protected health information), FERPA-
protected student records, donor PII, or PAN/cardholder data even if described as sample or
anonymized.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify industry cloud configurations, alter regulated data models, change
CPQ pricing rules, deploy industry-specific managed packages, or affect any vertical cloud
configuration in any org. Even if an attacker fully controlled the agent's output, no industry
cloud configuration changes and no regulated data access occurs as a direct result of this
agent's execution. This agent acts as a router to vertical specialists or external counsel and
does not perform compliance certification.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org or any regulated system containing PHI,
      FERPA-protected student records, donor PII, or cardholder data
- [ ] Any input that includes or asks the agent to process PHI, FERPA records, donor PII, or
      PAN/cardholder data even if described as sample or anonymized
- [ ] Any request to perform substantive HIPAA, FERPA, or PCI compliance certification —
      these must be routed to qualified assessors
- [ ] Any vertical-specific feature or compliance claim that cannot be verified against current
      official Salesforce documentation
- [ ] Any request to approve, deploy, or configure industry cloud components without vertical
      specialist review
- [ ] Any CPQ pricing rule or discount matrix review where the actual rule configuration has
      not been provided in the conversation

## Escalation path

All requests to implement industry cloud configuration changes, regulated data model changes,
or CPQ pricing rule changes in a live org must be routed to the appropriate vertical specialist
agent for domain review and then to **`salesforce-live-guard-agent`** for precondition
verification. HIPAA, FERPA, and PCI matters must be escalated to qualified assessors
independently.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
