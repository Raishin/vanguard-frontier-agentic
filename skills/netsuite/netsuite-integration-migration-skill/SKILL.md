---
name: netsuite-integration-migration-skill
description: "Static-review flashlight for NetSuite SOAP-to-REST integration architecture and migration program planning. Assesses integration inventories against the confirmed SOAP sunset timeline: 2026.1 REST+OAuth2 default for new integrations, 2027.1 new SOAP integrations blocked, 2025.2 last planned SOAP endpoint, 2028.2 all SOAP endpoints disabled. TRIGGER when: user asks to plan a SOAP-to-REST migration, assess migration risk across an integration inventory, design a phased migration program, review integration architecture for SOAP sunset exposure, create a migration timeline aligned to NetSuite releases, or design rollback strategies for integration cutover. Trigger phrases: SOAP migration plan, SOAP sunset, migrate to REST NetSuite, integration inventory, 2028.2 deadline, SOAP removal, migration program. DO NOT TRIGGER when: the question is about a single REST API endpoint design or integration record configuration (use netsuite-web-services-integration-agent), OAuth 2.0 or TBA auth mechanics (use netsuite-sso-oauth-tba-agent), SuiteScript or SDF code authorship (use netsuite-suitecloud-developer-agent), or role and permission SoD design (use netsuite-identity-access-role-permission-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: architecture
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Integration Migration Skill

## Purpose

End-to-end integration architecture review and SOAP-to-REST migration program planning. Owns the migration timeline, inventory prioritization, phased cutover design, and rollback planning. Cross-escalates individual API design questions to netsuite-web-services-integration-agent and auth/identity questions to netsuite-sso-oauth-tba-agent. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User needs to assess migration risk across a NetSuite integration inventory against the SOAP sunset timeline
- User is designing a phased SOAP-to-REST migration program and needs sequencing and rollback guidance
- User needs a migration timeline aligned to the 2026.1 / 2027.1 / 2028.2 NetSuite release milestones
- User needs to evaluate organizational readiness and testing strategy for integration migration
- User needs to design post-migration validation checklists

## Recommended Workflow

1. Step 1 — Gather inputs: integration inventory (protocol, auth method, criticality, dependencies), NetSuite release version, sandbox availability, team capacity
2. Step 2 — Score each integration against sunset milestones: imminent risk (SOAP new builds after 2026.1), blocker risk (any new SOAP after 2027.1), hard stop (all SOAP disabled at 2028.2)
3. Step 3 — Rate migration complexity per integration: auth change required (SOAP TBA → REST OAuth 2.0), downstream dependencies, data volume, error handling patterns
4. Step 4 — Design phased migration program: Phase 1 (new integrations — REST only), Phase 2 (critical SOAP migrations before 2027.1), Phase 3 (remaining SOAP before 2028.2)
5. Step 5 — Design rollback strategy per phase: traffic switching, parallel run window, go/no-go criteria
6. Step 6 — Rate all findings Critical/High/Medium/Low/Unknown; produce structured risk table with evidence labels [FACT], [ASSUMPTION], [INFERENCE]
7. Step 7 — Emit T0 static review output: migration program artifact with prioritized inventory, phased timeline, rollback design, and escalation routing

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- All four SOAP sunset milestones cited with evidence-matrix source: 2026.1, 2027.1, 2025.2 last endpoint, 2028.2 final disable
- OAuth 2.0 confirmed as required auth for all new REST integrations post-2026.1
- Custom reviewer role recommendation never uses Administrator role
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
- Request asks agent to execute a migration, fire live API calls, or mutate a NetSuite account
- User requests a migration plan without providing integration inventory — flag as Unknown risk, request inventory before proceeding
- User claims the SOAP sunset timeline is different from the confirmed evidence-matrix dates — correct with evidence citations

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never calls NetSuite APIs, never executes migrations, never requests or stores credentials, tokens, client secrets, or org IDs. Works exclusively from sanitized integration inventory data. All four SOAP sunset milestones cited from confirmed evidence: 2026.1 REST+OAuth2 default, 2027.1 new SOAP blocked, 2025.2 last planned SOAP endpoint, 2028.2 all endpoints disabled. Never recommends the Administrator role for integration service accounts. Custom reviewer role requires 2FA when permissions include Access Token Management.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Confirmed Oracle/NetSuite official documentation URLs for SOAP removal plans and OAuth 2.0
- [safety-checklist.md](references/safety-checklist.md) — Pre-review checklist: redaction verification, timeline accuracy, auth posture checks
- [least-privilege.md](references/least-privilege.md) — Custom role design for integration migration reviewers — permissions, 2FA triggers, forbidden roles
- [release-drift.md](references/release-drift.md) — Full SOAP sunset timeline: 2026.1 / 2027.1 / 2025.2 last endpoint / 2028.2 disable — evidence-matrix rows 2a-2d
- [migration-complexity-matrix.md](references/migration-complexity-matrix.md) — 7-factor migration complexity scoring matrix for individual integration assessments
