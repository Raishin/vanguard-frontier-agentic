---
name: netsuite-sso-oauth-tba-skill
description: "Static review of NetSuite OAuth 2.0, TBA, and SSO/SAML configurations. Validates OAuth scope (REST/RESTlets only, not SOAP), TBA fallback timeline, SAML correctness, deprecated NLAuth, and sandbox re-authorization. Trigger: OAuth 2.0, TBA, SSO/SAML, token auth, RESTlet auth, SuiteAnalytics Connect auth, sandbox re-auth, SOAP auth migration. Escalate: role design (use identity-access-role-permission), SDF deploy (use sdf-devops-release), SuiteScript security (use suitescript-secure-code-review), live token ops (use live-org-mutation-guard), AI Connector auth (use ai-connector-mcp)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: security
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite SSO OAuth TBA Skill

## Purpose

Authentication mechanism design and correctness in NetSuite integrations: OAuth 2.0 applicability scope (REST/RESTlets/SuiteAnalytics Connect only; NOT SOAP), TBA use-cases and sunset timeline, SSO/SAML integration, deprecated NLAuth/Passport patterns, and per-environment re-authorization requirements for sandbox and Release Preview. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- An integration record's authentication type needs validation against OAuth 2.0 and TBA support scope
- A SOAP-based integration with TBA needs a migration-risk assessment against the 2026.1/2027.1/2028.2 deprecation timeline
- OAuth 2.0 sandbox or Release Preview re-authorization gaps need to be identified and documented
- SSO/SAML setup needs review for correct configuration and 2FA designation of required permissions
- Deprecated NLAuth or Passport credential usage needs to be detected and remediation planned

## Recommended Workflow

1. Step 1 — Collect sanitized integration record configuration; confirm no token values, client secrets, or SAML assertions are present
2. Step 2 — Identify the authentication type (OAuth 2.0, TBA, NLAuth/Passport, SSO/SAML) and the transport protocol (REST, RESTlet, SuiteAnalytics Connect, SOAP)
3. Step 3 — Apply protocol-to-auth compatibility matrix: OAuth 2.0 supported for REST/RESTlets/SuiteAnalytics Connect (evidence 3a–3c); not supported for SOAP (evidence 3d); flag mismatches as Critical
4. Step 4 — Apply SOAP deprecation timeline to any SOAP + TBA integrations: assess urgency by release version (evidence 2a–2d); flag missing migration plan
5. Step 5 — Check for deprecated credential patterns (NLAuth on RESTlets, Passport on SOAP 2020.2+); flag as Critical if found on active integrations (evidence 4b, 4c)
6. Step 6 — Verify sandbox and Release Preview re-authorization documentation; flag any assumption that OAuth 2.0 apps or TBA tokens carry over from production (evidence 8a–8d)
7. Step 7 — Rate every finding Critical / High / Medium / Low / Unknown; emit structured report with migration guidance and escalation triggers

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No access tokens, refresh tokens, client secrets, TBA token values, or SAML assertions in the submitted configuration
- All OAuth 2.0 applicability claims cite evidence rows 3a–3d
- All SOAP deprecation timeline claims cite evidence rows 2a–2d verbatim
- No live token generation or account authorization is recommended without explicit human approval and netsuite-live-org-mutation-guard-agent routing
- Administrator role is never recommended for integration authentication

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes or asks for access tokens, refresh tokens, client secrets, TBA token values, SAML assertions, or session cookies
- Request asks the agent to generate OAuth 2.0 authorization codes, client credentials, or TBA token pairs
- Request asks the agent to perform a live sandbox refresh, authorize an OAuth application in a live account, or create TBA tokens
- Request asks to act as or use Administrator role
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for authentication context
- Scope creep: role and permission questions route to netsuite-identity-access-role-permission-agent

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works from sanitized configuration excerpts and never requests or handles credentials, access tokens, refresh tokens, client secrets, TBA token pairs, SAML assertions, or session cookies. Does not perform live authorizations, token generations, or sandbox refreshes. Every authentication-mechanism claim cites official Oracle documentation evidence.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite official documentation URLs for OAuth 2.0, TBA, SSO, and deprecation timeline
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission checklist for sanitizing integration configuration before analysis
- [least-privilege.md](references/least-privilege.md) — Auth reviewer role design: minimal permissions for configuration review without credential exposure
- [release-drift.md](references/release-drift.md) — SOAP and TBA deprecation milestone tracker: 2026.1, 2027.1, 2028.2 key dates
- [auth-compatibility-matrix.md](references/auth-compatibility-matrix.md) — Protocol-to-auth-method compatibility matrix (REST/RESTlet/SuiteAnalytics/SOAP vs OAuth 2.0/TBA/NLAuth)
