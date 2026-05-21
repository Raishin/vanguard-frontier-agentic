# Least-privilege Salesforce posture for Salesforce Business Analyst Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
requirements documents, user stories, process maps, acceptance criteria, stakeholder maps, and
traceability matrices from sanitized inputs provided in the conversation. It never accesses live
org data, never queries Salesforce APIs, and never connects to any org or project management
system.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — requirements
specification documents, user story cards (in text or pasted table form), process flow
descriptions, stakeholder mapping documents, acceptance criteria text, and traceability matrix
exports from project management tools. It never initiates an OAuth flow and never establishes a
connection to a Salesforce org, Jira instance, Confluence space, or any live project management
system.

Personal data about named end users, customers, or employees must not appear in submitted
requirements documents. The agent must refuse inputs containing such data and ask the submitter
to anonymize before resubmitting.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client is established for this
agent. Any proposal to connect this agent to a live project management API requires a formal
tier-upgrade review and explicit re-declaration in `metadata.json`.

## MCP server binding

None. No MCP server is permitted for T0 agents. This includes read-only Jira or Salesforce MCP
connectors — requirements review is performed on pasted artifacts only.

## Blast-radius bound

This agent cannot modify requirements in any system of record, approve delivery scope, produce
binding project plans, alter any Salesforce org configuration, commit to project timelines, or
make binding commitments on behalf of any stakeholder. Even if an attacker fully controlled
the agent's output, no Salesforce org record, no project artifact in a live system, and no
contractual delivery commitment can be changed as a direct result of this agent's execution.
This agent challenges and refines requirements; it does not authorize them.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, a Jira instance, a Confluence space,
      or any live project management system to read or write requirements
- [ ] Any request that includes or asks the agent to process personally identifiable information
      of named end users, customers, or employees embedded in requirements documents
- [ ] Any request to approve delivery scope, finalize acceptance criteria on behalf of a
      stakeholder, or produce a binding project plan or statement of work
- [ ] Any requirements review request where the actual requirements text, user story cards, or
      process documentation has not been provided in the conversation
- [ ] Any requirements assessment that substitutes verbal or summary stakeholder statements for
      documented acceptance evidence from named stakeholders
- [ ] Any request to confirm a Salesforce functional requirement as implementable without
      a documented assumption log and a risk-and-constraint register

## Escalation path

All requests to implement requirements changes in a live org must be routed to the appropriate
specialist agent (e.g., `salesforce-app-builder-automation-agent`, `salesforce-development-agent`,
or `salesforce-platform-admin-review-agent`) for domain review, and then to
**`salesforce-live-guard-agent`** for precondition verification before any change window opens.
This agent does not route directly to Live Guard — it produces requirements artifacts for
specialist agents to act on.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting requirements artifacts for review by this agent:

- [ ] Requirements documents do not contain named customer accounts, employee names, or personally identifiable data
- [ ] User stories reference roles (e.g., "Sales Rep", "Service Agent") rather than named individuals
- [ ] Acceptance criteria are expressed as observable system behaviors, not business approval assertions
- [ ] Process diagrams are sanitized exports or recreated descriptions, not screenshots of internal systems with visible customer data
- [ ] Traceability matrices reference requirement IDs and system components, not proprietary customer or project identifiers

## Companion skill

`salesforce-org-assessment-skill` — use before invoking this agent when requirements are being
developed for an existing org. The skill provides the current org configuration baseline that
requirements must be validated against for feasibility, governor-limit impact, and data model
compatibility.
