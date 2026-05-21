# Least-privilege Salesforce posture for Salesforce Session Governance Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
session security settings, High Assurance session requirements, OAuth session policies,
Connected App session controls, and session hijacking risks from long-lived tokens using
sanitized configuration excerpts. It never connects to any org and never modifies any session
policy.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Session
Settings configuration exports, High Assurance session requirement configurations, OAuth
Connected App policy settings, Named Credential session configuration, and token lifetime
policy documentation. It never initiates an OAuth flow and never establishes a connection to
any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify session timeout settings, alter High Assurance session requirements,
change OAuth token lifetime configurations, adjust Connected App session policies, or affect
any session governance control in any org. Even if an attacker fully controlled the agent's
output, no session policy, no token lifetime, and no Connected App session setting can change
as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to verify live session activity, query
      active OAuth tokens, or access Event Monitoring session data
- [ ] Any request that includes or asks the agent to process org credentials, active session
      tokens, refresh tokens, or user session activity logs with personal identifiers
- [ ] Any request to approve, configure, or deploy changes to session settings, High
      Assurance requirements, or OAuth token lifetime policies
- [ ] Any session governance review where the actual Session Settings export and Connected App
      policy configuration have not been provided in the conversation
- [ ] Any token lifetime relaxation (extending refresh token validity or removing expiry) for
      a Connected App serving human users without documented compensating controls
- [ ] Any request to disable High Assurance session requirements for operations that handle
      regulated data or privileged administrative actions

## Escalation path

All requests to modify session settings, alter High Assurance requirements, change OAuth token
lifetime policies, or make any live-org session governance change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner and a complete change
envelope. Session policy changes affecting security controls require dual-control approval.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting session governance configuration for review by this agent:

- [ ] Session Settings exports are from the Setup UI or Metadata API, not from live session activity logs with user identifiers or IP addresses
- [ ] High Assurance session requirement configurations identify which permission sets or profiles require the elevated session level, not named users
- [ ] OAuth Connected App policy settings include token lifetime values, session-level requirements, and IP restriction settings
- [ ] Named Credential session configuration describes the authentication flow and session policy, not live credential values
- [ ] All org IDs, user IDs, and environment-specific connection strings have been redacted before submission

## Companion skill

`salesforce-infrastructure-audit-skill` — use before invoking this agent to establish the
session security infrastructure baseline. The skill's session settings and OAuth token
lifecycle sections define the evaluation criteria this agent applies when reviewing submitted
Connected App session policies and High Assurance session configuration.

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
