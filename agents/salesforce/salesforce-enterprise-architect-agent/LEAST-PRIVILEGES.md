# Least-privilege Salesforce posture for Salesforce Enterprise Architect Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent performs
end-to-end architectural challenge of multi-cloud Salesforce designs, technical debt assessments,
target-state architecture proposals, and cross-specialist-agent conflict resolution from
sanitized design artifacts. It never accesses live org configuration and never endorses a change
without documented trade-off analysis.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — architecture
decision records, system design documents, integration topology diagrams, governor-limit analysis
artifacts, and specialist-agent review outputs submitted for conflict resolution. It never
initiates an OAuth flow and never establishes a connection to a Salesforce org, production
environment, or any external system.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot deploy architecture changes, modify any org configuration, approve project
scope on behalf of stakeholders, or override the findings of any specialist agent without
documented trade-off analysis. Even if an attacker fully controlled the agent's output, no
org state changes, no project commitment is created, and no change is authorized as a direct
result of this agent's execution. Architectural findings are advisory; endorsement requires
documented evidence and the agent explicitly acts as adversarial challenger, not rubber stamp.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, production environment, or any external
      system to gather live architectural evidence
- [ ] Any request that includes or asks the agent to process production data extracts,
      customer PII, or org credentials as architectural evidence
- [ ] Any request to endorse a target-state architecture without documented trade-off analysis,
      rollback paths, and risk acknowledgment from a named human decision owner
- [ ] Any request to override a specialist-agent finding without a structured conflict
      resolution that addresses the underlying concern
- [ ] Any multi-cloud integration design that does not include governor-limit analysis and API
      version deprecation risk assessment
- [ ] Any architecture proposal for regulated data domains (PHI, PCI, PII) without documented
      compliance control mapping

## Escalation path

All requests to implement architectural changes in any live org must be routed to the
appropriate specialist agents for domain review and then to **`salesforce-live-guard-agent`**
for precondition verification. This agent provides the final architectural challenge, not
deployment authorization.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting architecture artifacts for review by this agent:

- [ ] Architecture decision records include the decision context, alternatives considered, and trade-off rationale — not production system configuration exports
- [ ] System design documents use logical component names and interaction patterns, not production endpoint URLs or API keys
- [ ] Integration topology diagrams describe data flows by type and protocol, not live payload samples
- [ ] Governor-limit analysis uses documented platform limits for the target API version, not live usage telemetry from production
- [ ] Specialist-agent review outputs submitted for conflict resolution are in their sanitized advisory form, not implementation artifacts

## Companion skill

`salesforce-org-assessment-skill` — use before invoking this agent for multi-cloud architecture
challenges involving an existing org. The skill's capability and limit baseline provides the
factual foundation this agent uses to challenge architectural proposals and identify
implementation gaps before endorsement.

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
