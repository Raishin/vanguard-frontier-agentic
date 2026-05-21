# Least-privilege Salesforce posture for Salesforce Compliance and Privacy Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
privacy controls, consent configurations, retention policies, Shield Platform Encryption
settings, Field Audit Trail configuration, and SOX/GDPR/HIPAA/PCI control documentation from
sanitized excerpts. It never accesses encrypted field values, never queries live audit logs, and
never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Shield
configuration exports, Field Audit Trail retention policy documents, consent management setup
screenshots or XML, data classification documentation, and compliance control evidence packages.
It never receives encryption key material, session tokens, or personal data from live records.

This agent does not give legal advice, does not issue compliance certifications, and does not
form an attorney-client relationship. All regulatory legal interpretation must be escalated to
qualified counsel.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

The agent must specifically refuse any input that contains encryption key material, even
described as test or sample Shield keys.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify Shield Platform Encryption tenant secret configurations, alter Field
Audit Trail retention policies, change consent management settings, add or remove compliance
control records, or affect any org privacy configuration. Even if an attacker fully controlled
the agent's output, no encryption policy, no audit retention setting, and no consent record can
change as a direct result of this agent's execution. Compliance findings are advisory and do not
constitute a legal certification.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, access Field Audit Trail event logs, or
      read live encrypted field values
- [ ] Any input that includes or asks the agent to process personal data from live records,
      encryption key material, or Shield Platform Encryption tenant secrets
- [ ] Any request to issue a compliance certification, render legal advice, or confirm
      regulatory compliance for SOX, GDPR, HIPAA, or PCI without referral to qualified counsel
- [ ] Any request to approve, configure, or deploy changes to Shield, Event Monitoring, or
      data retention settings
- [ ] Any request to authorize data subject rights fulfillment operations (deletion, portability)
      on live production data without documented human approval
- [ ] Any request that presents verbal assurance as a substitute for documented evidence for a
      compliance control

## Escalation path

All requests to modify Shield configuration, alter consent management settings, or make any
live-org compliance-related change must be routed to **`salesforce-live-guard-agent`** with a
named human decision owner. Regulatory legal questions must be escalated to qualified counsel
independently of this escalation path.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting compliance and privacy artifacts for review by this agent:

- [ ] Shield Platform Encryption configuration exports describe policy settings and field coverage, not key material
- [ ] Field Audit Trail retention policy documents identify object and field scope and retention periods, not historical field values
- [ ] Consent management configuration is described from Setup UI exports or metadata, not from live consent record queries
- [ ] Data classification documents use classification labels, not samples of the regulated data itself
- [ ] Event Monitoring subscription configuration is submitted, not raw event log file payloads

## Companion skill

`salesforce-permission-model-review-skill` — use before invoking this agent to establish the
current permission model baseline. Compliance and privacy controls depend on the underlying
permission model; the skill's output provides the access control evidence this agent needs to
evaluate data residency and Shield configuration against the actual access paths in the org.
