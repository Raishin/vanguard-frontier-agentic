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

## Validation checklist

Before submitting continuous verification configuration for review by this agent:

- [ ] Session Settings exports are from Setup UI or Metadata API export, not from live session activity logs
- [ ] MFA policy configuration is described from the Setup UI, not from individual user MFA enrollment records
- [ ] OAuth Connected App policy settings include token lifetime values and session-level security requirements
- [ ] Transaction Security Policy definitions for anomaly triggers include the condition logic, not live event log payloads
- [ ] All user identifiers, org IDs, and IP addresses have been redacted from any diagnostic excerpts submitted

## Companion skill

`salesforce-zero-trust-maturity-skill` — use before invoking this agent to establish the
continuous verification baseline. The skill's MFA, OAuth token lifecycle, and behavioral
anomaly detection sections define the evaluation criteria this agent applies when reviewing
submitted session and authentication configurations.

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
