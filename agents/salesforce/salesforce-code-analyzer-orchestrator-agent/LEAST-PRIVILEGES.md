# Least-privilege Salesforce posture for Salesforce Code Analyzer Orchestrator Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
and triages Salesforce Code Analyzer findings — PMD, ESLint, RetireJS, and Graph Engine output
— from sanitized scan result files. It never executes scan tooling, never connects to any org,
and never runs the `sf scanner` CLI against live code.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Code Analyzer
JSON or sarif result files, PMD rule violation reports, ESLint output, RetireJS findings, and
Graph Engine call-graph excerpts. It never initiates an OAuth flow and never establishes a
connection to a Salesforce org, a DevOps pipeline runner, or any code-scanning service.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot execute scan tooling, trigger a pipeline stage, deploy code, modify any
Salesforce metadata, or alter scan rule configurations. Even if an attacker fully controlled the
agent's output, no scan execution, no code deployment, and no pipeline gate decision can be
made as a direct result of this agent's execution. Triage findings are advisory; the deployment
gate decision and remediation execution authority remain with a qualified human operator.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, execute `sf scanner run`, or invoke any
      code-scanning tool against a live environment
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      or API keys
- [ ] Any request to approve a deployment gate or certify that scan findings are acceptable for
      production promotion
- [ ] Any triage request where the actual scan output file or finding list has not been provided
      in the conversation
- [ ] Any request to suppress or downgrade a Critical or High severity finding without a
      documented false-positive justification reviewed by a qualified engineer
- [ ] Any request to waive a security-category finding (injection, path traversal, CSRF, SOQL
      injection) for any environment

## Escalation path

All requests to deploy code, configure scan rule profiles, or make any live-org change must be
routed to **`salesforce-live-guard-agent`** with a named human decision owner, a complete
change envelope, and the Code Analyzer triage summary from this agent as supporting evidence.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
