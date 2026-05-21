# Least-privilege Salesforce posture for Salesforce Adaptive Access Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent operates
entirely on sanitized configuration excerpts provided in the conversation. It has no MCP server
binding, no OAuth flow, and no live-org access path. Blast radius is zero by construction.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — Transaction
Security Policy XML, Shield event monitoring subscription configuration exports, Dynamic Forms
condition definitions, permission set policy fragments, Context-Aware Access configuration
documentation, anomaly scoring threshold definitions, high-assurance session enforcement
settings, and Einstein Trust Layer boundary documentation.

It never initiates an OAuth flow, never receives a session token, and never establishes a
connection to a Salesforce org. If a caller attempts to supply org credentials, session tokens,
or Connected App secrets, the agent must refuse and return those inputs unprocessed.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client is established for this
agent under any circumstances.

The zero blast-radius guarantee is structural, not policy-based. There is no permission set to
constrain because there is no identity to constrain. Any proposal to establish an identity for
this agent requires a formal tier-upgrade review and explicit declaration of a new
`execution_tier` in `metadata.json`.

## MCP server binding

None. No MCP server is permitted for T0 agents. Any harness configuration that wires an MCP
server to this agent — including read-only Salesforce MCP servers — violates the tier contract
and must be rejected at the validation gate.

## Blast-radius bound

This agent cannot modify records, deploy metadata, invoke SOQL, read live event streams,
change Transaction Security Policy enforcement modes, alter permission set policy assignments,
enable or disable Einstein Trust Layer boundaries, adjust high-assurance session requirements,
or affect any org configuration whatsoever. Even if an attacker gained full control of the
agent's output channel, the worst-case outcome is misleading review commentary. No org state
can change as a direct result of this agent's execution. The zero-trust controls it reviews
remain fully in force regardless of what the agent outputs.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org or retrieve data from a running Shield
      event monitoring stream or Transaction Security Policy enforcement log
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      Connected App client secrets, or personally identifiable information
- [ ] Any request to approve, deploy, or recommend execution of any Transaction Security
      Policy change, permission set policy modification, or Context-Aware Access update
- [ ] Any request to disable, downgrade, or bypass a zero-trust access control without
      documented evidence from a qualified human reviewer and a named change owner
- [ ] Any request to evaluate Shield Platform Encryption key material or inspect encrypted
      field values from a live org
- [ ] Any org-type confirmation that cannot be verified from documentary evidence — treat
      unverifiable org types as production and apply the most restrictive stance

## Escalation path

All mutation requests — enabling or disabling Transaction Security Policies, modifying
permission set policies, changing high-assurance session requirements, adjusting Einstein Trust
Layer boundaries, or any other live-org operation — must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner and a fully populated
change envelope before any action is taken.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting configuration excerpts for review by this agent:

- [ ] All org IDs, user IDs, and session identifiers have been redacted from exports
- [ ] Transaction Security Policy definitions include only rule logic, not enforcement logs
- [ ] Shield event monitoring configuration exports contain subscription settings, not event payloads
- [ ] Permission set policy fragments contain permission names, not assignment lists with user IDs
- [ ] Context-Aware Access configuration is described from Setup UI screenshots or exported XML, not live API responses

## Companion skill

`salesforce-zero-trust-maturity-skill` — use before invoking this agent to establish the
current zero-trust maturity baseline. The skill output provides the comparison framework
this agent needs to evaluate gap severity in Transaction Security Policies and adaptive
access configurations.
