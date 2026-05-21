# Least-privilege Salesforce posture for Salesforce App Builder Automation Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Flow metadata XML, validation rule formula text, approval process definitions, dynamic forms
condition logic, and record-triggered automation configurations from sanitized excerpts. It
never deploys flows, never invokes the Metadata API against any org, and never connects to any
Salesforce environment.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Flow metadata
XML files, validation rule formula definitions, approval process configuration exports, dynamic
forms visibility condition logic, process builder JSON exports (legacy), and record-triggered
automation configuration descriptions. It never initiates an OAuth flow and never establishes a
connection to any Salesforce org.

Specifically excluded from accepted inputs: live flow execution logs with record IDs, interview
GUIDs, or user context data from production flow runs. These must be anonymized before
submission.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client is established for this
agent. The zero blast-radius guarantee is structural. Any proposal to establish an org identity
requires a formal tier-upgrade review.

## MCP server binding

None. No MCP server is permitted for T0 agents. Any harness configuration that wires a
Salesforce MCP server to this agent violates the tier contract and must be rejected.

## Blast-radius bound

This agent cannot deploy flows, activate or deactivate automation, modify validation rules,
alter approval processes, change record-triggered automation, publish dynamic forms, or affect
any declarative configuration in any Salesforce org. Even if an attacker fully controlled the
agent's output, no flow activates, no validation rule deploys, and no automation record changes
as a direct result of this agent's execution. Recursion, hidden bypasses, and brittle flow
patterns this agent identifies remain in the reviewed artifacts — they do not propagate to any
live environment without a human deployment action through a separate credentialed toolchain.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, invoke the Metadata API, or activate
      or deactivate any flow version against a running org
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      client secrets, or personal data embedded in flow variable payloads or interview logs
- [ ] Any request to approve, activate, or deploy a flow, validation rule, approval process,
      or record-triggered automation to any environment
- [ ] Any flow review request where the full Flow metadata XML or equivalent sanitized export
      has not been provided in the conversation
- [ ] Any automation pattern that lacks a documented governor-limit safety check, recursion
      guard, or bypass mechanism — these must be flagged as blockers, not accepted as-is
- [ ] Any request to approve a flow that bypasses a validation rule or approval process without
      documented business justification and human sign-off from the process owner

## Escalation path

All requests to activate flows, deploy automation changes, modify validation rules, or make any
live-org declarative configuration change must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner, dry-run validation output, and a complete change envelope
before any change window opens.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting flow metadata for review by this agent:

- [ ] Flow metadata XML is from a version-controlled source or Setup export, not a live org API response
- [ ] Validation rule formula text has been extracted from Setup UI or metadata export
- [ ] Approval process configuration is from an export, not from a live process instance with active records
- [ ] Record-triggered automation descriptions identify the trigger object, entry conditions, and action types
- [ ] All record IDs, user IDs, and org-specific references have been redacted before submission

## Companion skill

`salesforce-flow-automation-review-skill` — use before invoking this agent to run the standard
Flow review checklist. The skill covers recursion risk, governor-limit exposure, bulkification
requirements, and bypass mechanism patterns that this agent evaluates in submitted Flow XML.
