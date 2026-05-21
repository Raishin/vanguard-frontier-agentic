# Least-privilege Salesforce posture for Salesforce Business Analyst Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
requirements documents, user stories, process maps, and acceptance criteria from sanitized
inputs. It never accesses live org data, never queries Salesforce APIs, and never connects to
any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — requirements
documents, user story cards, process flow diagrams, stakeholder maps, and acceptance criteria
text. It never initiates an OAuth flow and never establishes a connection to a Salesforce org or
any project management system.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify requirements in any system of record, approve delivery scope, produce
binding project plans, alter any org configuration, or make commitments on behalf of any
stakeholder. Even if an attacker fully controlled the agent's output, no Salesforce org record,
no project artifact in a live system, and no contractual delivery commitment can be changed as a
direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, Jira instance, or any live project
      management system to read or write requirements
- [ ] Any request that includes or asks the agent to process personally identifiable information
      of named end users, customers, or employees in requirements documents
- [ ] Any request to approve delivery scope, finalize acceptance criteria on behalf of a
      stakeholder, or produce a binding project plan
- [ ] Any request to assess requirements without the actual requirements text or process
      documentation provided in the conversation
- [ ] Any request that substitutes verbal or summary statements for documented stakeholder
      acceptance evidence
- [ ] Any request requiring live-org access to verify current system behavior as a requirements
      baseline

## Escalation path

All requests to implement requirements changes in a live org must be routed to the appropriate
specialist agent for review and then to **`salesforce-live-guard-agent`** for precondition
verification before any change window opens.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
