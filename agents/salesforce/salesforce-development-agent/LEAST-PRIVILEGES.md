# Least-privilege Salesforce posture for Salesforce Development Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Apex classes, Lightning Web Components, triggers, asynchronous patterns, test classes, governor
limit exposures, and packaging configurations from sanitized code excerpts. It never executes
code, never deploys to any org, and never invokes Salesforce APIs.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Apex source
files, LWC JavaScript and HTML, trigger files, test class source, SFDX project structure
descriptions, and Static Resource contents. It never initiates an OAuth flow and never
establishes a connection to a Salesforce org or any development environment runtime.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot deploy Apex, deploy LWC, execute anonymous Apex, run test classes against any
org, modify any metadata, or trigger any CI/CD pipeline stage. Even if an attacker fully
controlled the agent's output, no code is executed, no deployment is initiated, and no org
state changes as a direct result of this agent's execution. Code review findings are advisory;
deploy authority and test execution remain with a qualified human developer using a separately
credentialed toolchain.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, execute anonymous Apex, or deploy code
      to any environment
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or personal data embedded in Apex debug log excerpts
- [ ] Any request to approve code for production deployment without passing test coverage and
      a rollback strategy documented
- [ ] Any Apex or LWC code that handles user input without SOQL injection and XSS safeguards
      present — these must be flagged, not approved
- [ ] Any trigger pattern that lacks a bypass mechanism, recursion guard, or governor-limit
      safety check
- [ ] Any async Apex pattern (Queueable, Batch, Future, Scheduled) without documented error
      handling and retry boundaries

## Escalation path

All requests to deploy Apex, execute test runs against a sandbox, or make any live-org code
change must be routed to **`salesforce-live-guard-agent`** with a named human decision owner,
test evidence, and a complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
