# Least-privilege Salesforce posture for Salesforce Security Identity Access Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
profiles, permission sets, permission set groups, role hierarchies, OWD sharing, SSO
configurations, MFA settings, Connected Apps, OAuth scope assignments, session policies, and
privileged access patterns from sanitized permission exports. It never modifies any security
policy and never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — profile XML
exports, permission set metadata exports, role hierarchy definitions, OWD sharing settings
documentation, Connected App OAuth configuration exports, SSO metadata XML, and session policy
configuration descriptions. It never initiates an OAuth flow and never establishes a connection
to any Salesforce org.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

This agent is specifically designed to review Connected App OAuth scope assignments. It must
flag any Connected App that includes `full`, `web`, `chatbot_api`, or `sfap_api` scopes as a
HIGH RISK finding requiring immediate remediation, regardless of the stated business purpose.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot modify profiles, assign or revoke permission sets, alter OWD sharing,
configure SSO, enable or disable MFA, change Connected App OAuth scopes, or affect any
identity and access control in any org. Even if an attacker fully controlled the agent's
output, no permission assignment, no sharing rule, and no identity policy can change as a
direct result of this agent's execution. This is especially significant given this agent's
domain: a compromised IAM review agent that cannot mutate any permission is fundamentally
safer than one with write access.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, invoke Salesforce APIs, or run the
      sf CLI to fetch live permission data or session activity
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      SSO assertion secrets, or user authentication logs with personal identifiers
- [ ] Any request to approve a security policy decision, authorize a permission set assignment,
      or certify a sharing model as compliant
- [ ] Any Connected App configuration that includes `full`, `web`, `chatbot_api`, or
      `sfap_api` OAuth scopes — these must be flagged HIGH RISK, not approved
- [ ] Any permission review that approves toxic permission combinations (e.g., ModifyAllData
      plus ViewEncryptedData in the same profile) without documented compensating controls
- [ ] Any SSO or MFA review request where disabling a control is under consideration without
      a fully documented compensating control reviewed by a qualified security engineer

## Escalation path

All requests to modify permissions, change OAuth scope assignments, alter SSO configuration,
disable MFA enforcement, or make any live-org identity and access change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner and a complete change
envelope. Security configuration changes must additionally receive dual-control approval from
a second named approver with documented authority before the change envelope is submitted.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)
