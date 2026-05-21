# Least-privilege Salesforce posture for Salesforce Continuous Verification Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
adaptive authentication policies, Always-On MFA configuration, OAuth token lifetime settings,
behavioral anomaly detection rules, and continuous re-validation patterns from sanitized
configuration excerpts. It never connects to any org or identity service.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — session
settings exports, MFA policy configuration screenshots or XML, OAuth Connected App policy
settings, Transaction Security Policy definitions for anomaly-based triggers, and Event
Monitoring subscription configuration. It never initiates an OAuth flow and never establishes a
connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify session timeout settings, alter MFA enforcement policies, change OAuth
token lifetime configurations, activate or deactivate Transaction Security Policies, or affect
any continuous authentication control in any org. Even if an attacker fully controlled the
agent's output, no session policy, no MFA configuration, and no anomaly detection rule can
change as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, access live Event Monitoring streams,
      or query OAuth token activity from a running org
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      refresh tokens, or user behavioral data from live monitoring systems
- [ ] Any request to approve, configure, or deploy changes to MFA enforcement, session
      timeout, or OAuth token lifetime settings
- [ ] Any request to disable, bypass, or reduce continuous verification controls without
      documented compensating controls reviewed by a qualified security engineer
- [ ] Any review request where session settings or MFA configuration excerpts have not been
      provided in the conversation
- [ ] Any request to confirm zero-trust compliance status for an org without the full session
      and MFA policy configuration provided

## Escalation path

All requests to modify MFA enforcement, alter session timeout policies, change OAuth token
lifetime settings, or make any live-org continuous verification change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner and a structured change
envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
