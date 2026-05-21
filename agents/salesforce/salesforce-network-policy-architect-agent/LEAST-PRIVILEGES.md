# Least-privilege Salesforce posture for Salesforce Network Policy Architect Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Salesforce org-level network security policies, IP allowlist configurations, session timeout
settings, and CSP Trusted Sites definitions from sanitized configuration excerpts. It never
connects to any org and never modifies any network policy.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Network Access
configuration exports, Trusted IP Range definitions, Session Settings configuration screenshots
or XML, CSP Trusted Sites configuration exports, and My Domain settings documentation. It never
initiates an OAuth flow and never establishes a connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify IP allowlist entries, alter session timeout values, add or remove
CSP Trusted Sites, change My Domain HTTPS settings, or affect any network security policy in
any org. Even if an attacker fully controlled the agent's output, no network policy, no IP
range, and no session setting can change as a direct result of this agent's execution.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org to fetch live network configuration or
      test IP allowlist enforcement
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      or API keys
- [ ] Any request to approve, configure, or deploy changes to IP allowlists, session settings,
      or CSP Trusted Sites
- [ ] Any network policy review where the actual Network Access configuration export or
      session settings screenshots have not been provided in the conversation
- [ ] Any CSP Trusted Sites change that adds an `unsafe-inline` or wildcard origin without
      documented security justification reviewed by a qualified engineer
- [ ] Any session timeout relaxation (increase beyond org default) without documented
      compensating controls

## Escalation path

All requests to modify IP allowlists, change session settings, alter CSP Trusted Sites, or
make any live-org network policy change must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner and a complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
