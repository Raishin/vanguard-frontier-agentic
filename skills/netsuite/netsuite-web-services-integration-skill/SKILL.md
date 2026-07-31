---
name: netsuite-web-services-integration-skill
description: "Static-review flashlight for NetSuite SuiteTalk REST/SOAP API design, integration record configuration, and OAuth 2.0 authentication posture. Reviews REST record endpoints, RESTlet definitions, integration record settings, and authentication method selection against Oracle's documented posture. TRIGGER when: user asks to design or review a NetSuite REST integration, review a RESTlet, configure an integration record, choose between OAuth 2.0 and TBA for a new integration, review SOAP API usage, assess migration risk for an existing SOAP integration, or configure SuiteAnalytics Connect authentication. Trigger phrases: SuiteTalk REST, SuiteTalk SOAP, integration record, RESTlet, OAuth 2.0 NetSuite, REST API design NetSuite, SOAP migration risk. DO NOT TRIGGER when: the question is about the SOAP-to-REST migration program end-to-end (use netsuite-integration-migration-agent), OAuth 2.0 / TBA / SSO / SAML deep auth mechanics (use netsuite-sso-oauth-tba-agent), SuiteScript code authorship or SDF deployment (use netsuite-suitecloud-developer-agent), or role and permission SoD design (use netsuite-identity-access-role-permission-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: platform
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Web Services Integration Skill

## Purpose

SuiteTalk REST/SOAP API design and integration record configuration review. Flags SOAP usage as migration risk, validates OAuth 2.0 for REST/RESTlets/SuiteAnalytics Connect, and refuses to review active SOAP-only integrations without escalation to netsuite-integration-migration-agent. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User needs to design or review a new NetSuite REST web services integration
- User is reviewing an existing SOAP integration and needs migration risk assessment
- User needs to configure OAuth 2.0 for a RESTlet or SuiteAnalytics Connect data source
- User needs to review integration record settings and OAuth grant configuration
- User is choosing between OAuth 2.0 and TBA for a new or existing integration

## Recommended Workflow

1. Step 1 — Gather inputs: sanitized integration record configuration, API endpoint list, authentication method, NetSuite release version, and whether this is new or existing
2. Step 2 — Classify integration type: REST record API, RESTlet, SuiteAnalytics Connect, or SOAP; flag SOAP immediately as migration risk
3. Step 3 — Review authentication posture: confirm OAuth 2.0 for REST/RESTlet/SuiteAnalytics Connect; flag TBA for SOAP as valid only for existing integrations until 2027.1; refuse user credentials for RESTlets (deprecated 2021) and SOAP 2020.2+
4. Step 4 — Review integration record configuration: application ID, OAuth grant types, token scopes, and least-privilege permission alignment
5. Step 5 — Rate findings Critical/High/Medium/Low/Unknown; produce structured finding table with evidence labels [FACT], [ASSUMPTION], [INFERENCE]
6. Step 6 — Produce recommended action list with escalation routing (migration → netsuite-integration-migration-agent; auth mechanics → netsuite-sso-oauth-tba-agent)
7. Step 7 — Emit T0 static review output: no live API calls, no org credentials, human review required before any configuration change

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- SOAP usage flagged as migration risk with confirmed timeline cited (2026.1 / 2027.1 / 2028.2)
- OAuth 2.0 not stated as supported for SOAP (confirmed NOT supported)
- Custom role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to fire live API calls or mutate a NetSuite account
- User claims Web Services Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires evaluating SOAP integration as a long-term strategy without flagging migration risk

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never calls NetSuite APIs, never requests or stores credentials, tokens, client secrets, or org IDs. Works exclusively from sanitized configuration excerpts. SOAP usage is flagged as a migration risk citing the confirmed sunset timeline. OAuth 2.0 is confirmed NOT supported for SOAP; only for REST, RESTlets, and SuiteAnalytics Connect. Never recommends the Administrator role. Custom reviewer role requires 2FA when permissions include Access Token Management or OAuth 2.0 Authorized Applications Management.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Confirmed Oracle/NetSuite official documentation URLs for REST, SOAP, OAuth 2.0, and TBA
- [safety-checklist.md](references/safety-checklist.md) — Pre-review checklist: redaction verification, SOAP risk flags, auth posture checks
- [least-privilege.md](references/least-privilege.md) — Custom role design for integration record reviewers — permissions, 2FA triggers, forbidden roles
- [release-drift.md](references/release-drift.md) — SOAP sunset timeline: 2026.1 REST+OAuth2 default, 2027.1 new SOAP blocked, 2028.2 all endpoints disabled
- [auth-posture-matrix.md](references/auth-posture-matrix.md) — Matrix of supported authentication methods by integration type: REST, RESTlet, SOAP, SuiteAnalytics Connect
