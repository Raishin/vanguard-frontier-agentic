# Least-privilege Salesforce posture for Salesforce App Builder Automation Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Flow metadata XML, validation rule formulas, approval process definitions, and record-triggered
automation configurations from sanitized excerpts. It never deploys flows, never invokes the
Metadata API, and never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Flow metadata
XML, validation rule formula text, approval process configuration exports, and dynamic forms
condition definitions. It never initiates an OAuth flow and never establishes a connection to a
Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot deploy flows, activate or deactivate automation, modify validation rules,
alter approval processes, change record-triggered automation, or affect any declarative
configuration in any org. Even if an attacker fully controlled the agent's output, no flow, no
validation rule, and no automation record can change as a direct result of this agent's
execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org or invoke the Metadata API against a
      running org
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or personal data embedded in flow variable values
- [ ] Any request to approve, activate, or deploy a flow, validation rule, or approval process
- [ ] Any request to assess automation without providing the full flow metadata XML or equivalent
      sanitized export
- [ ] Any request to bypass a governor-limit safeguard or recursion guard without documented
      compensating controls
- [ ] Any request requiring live-org access where org type cannot be confirmed from documentary
      evidence

## Escalation path

All requests to activate, deploy, or modify any flow, validation rule, or approval process in a
live org must be routed to **`salesforce-live-guard-agent`** with a named human decision owner
and a complete change envelope including dry-run validation output.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
