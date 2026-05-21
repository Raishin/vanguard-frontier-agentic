# Least-privilege Salesforce posture for Salesforce Experience Cloud Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
Experience Cloud portal configurations, external identity settings, guest-user access profiles,
sharing sets, and partner/customer access models from sanitized configuration excerpts. Guest
and external-user access is treated as HIGH RISK by default.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Experience
Cloud network configuration exports, guest user profile permission exports, sharing set and
sharing rule definitions, External Credentials configuration, digital experience page access
settings, and partner portal security configuration. It never initiates an OAuth flow and never
establishes a connection to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify guest user profiles, alter sharing sets, change external user license
assignments, deploy Experience Builder pages, modify CSP Trusted Sites for Experience Cloud
domains, or affect any portal access control in any org. Even if an attacker fully controlled
the agent's output, no guest access permission, no sharing set, and no community page can be
changed as a direct result of this agent's execution. The agent's HIGH RISK default on
guest-user access means any ambiguity is treated as a security concern, not a configuration
approval.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, access live portal session data, or
      query guest user activity from a running Experience Cloud site
- [ ] Any request that includes or asks the agent to process org credentials, external user
      passwords, session tokens, or end-user PII from portal records
- [ ] Any request to approve, configure, or deploy changes to guest user profiles, sharing
      sets, or external user access without documented business justification and human review
- [ ] Any unauthenticated access configuration for objects containing regulated data (PHI,
      PII, financial records) without escalation to a qualified architect
- [ ] Any Experience Cloud network configuration that enables Visualforce or Apex access for
      guest users without explicit documented justification
- [ ] Any review request where the guest user profile export and sharing model have not been
      provided in the conversation

## Escalation path

All requests to modify guest user profiles, alter sharing configurations, deploy portal pages,
or make any live Experience Cloud org change must be routed to **`salesforce-live-guard-agent`**
with a named human decision owner and a complete change envelope. Unauthenticated access to
regulated data must additionally be escalated to a qualified architect before the change
envelope is submitted.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
