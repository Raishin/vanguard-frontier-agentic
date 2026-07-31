---
name: netsuite-enterprise-architecture-skill
description: "Reviews NetSuite enterprise architecture decisions — SuiteCloud platform design, integration topology (REST/RESTlet/SOAP migration), OneWorld multi-subsidiary layout, SDF project structure, SuiteScript governance, and AI Connector MCP integration patterns — against Oracle best practices and the SOAP removal timeline. Produces risk-rated findings and structured architecture decision records. T0 static review — no org connection required. TRIGGER when: user requests an architecture review, asks for integration protocol selection advice, asks about OneWorld or multi-subsidiary design, needs SDF project structure guidance, needs SuiteScript version strategy, asks about AI Connector MCP design, or needs cross-domain design arbitration. Trigger phrases: NetSuite architecture, should I use REST or SOAP, OneWorld design, SuiteCloud architecture, SDF project structure, customization strategy, integration topology, architecture decision record, ADR, multi-subsidiary. DO NOT TRIGGER when: the request is a live deployment (use netsuite-live-org-mutation-guard-agent); request is specifically about SOX audit controls (use netsuite-audit-controls-sox-agent); request is specifically about role and permission SoD (use netsuite-identity-access-role-permission-agent); request is a SOAP-to-REST migration program rather than architecture advice (use netsuite-integration-migration-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
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

# NetSuite Enterprise Architecture Skill

## Purpose

Evaluate NetSuite architectural decisions against Oracle best practices, zero-trust boundaries, least-privilege design, and the SOAP-to-REST migration timeline. Produce opinionated architecture assessments with risk-rated findings and safe next actions for large-scale implementations spanning multiple subsidiaries, integration suites, and development lifecycle stages. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User presents a NetSuite architecture diagram or design document for review
- Integration protocol selection (REST vs. RESTlet vs. SOAP vs. SuiteAnalytics Connect) must be decided for a new or existing integration
- OneWorld multi-subsidiary design requires topology review including intercompany and consolidated reporting structure
- SDF project structure, environment promotion pipeline, or SuiteScript governance model needs architectural assessment
- Cross-domain design conflict between specialist agents requires arbitration and a structured ADR

## Recommended Workflow

1. Step 1 — Architecture intake: extract design elements (integration methods, subsidiary count, SuiteScript version, SDF adoption, compliance scope, scale indicators)
2. Step 2 — Risk signal identification: flag SOAP dependencies against 2026.1/2027.1/2028.2 timeline; flag Administrator-role automation; flag new-TBA-for-SOAP after 2027.1
3. Step 3 — Least-privilege review: verify that all integration roles and script run-as accounts follow the custom-role-from-standard pattern; flag any Administrator-role automation
4. Step 4 — Pattern matching: compare proposed architecture against known Oracle best-practice patterns for the relevant scale and compliance context
5. Step 5 — ADR drafting: for material decisions, produce a structured architecture decision record with rationale, alternatives considered, tradeoffs, and risk rating
6. Step 6 — Finding output: emit risk-rated findings table (Critical/High/Medium/Low/Unknown) with evidence citations and safe next actions

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets are referenced in the architecture materials
- All new integration designs specify REST web services with OAuth 2.0 (not new SOAP post-2026.1)
- No Administrator-role automation is recommended or approved
- All custom roles are confirmed as copies of standard roles per evidence item 7a
- OAuth 2.0 + SOAP is never recommended (SOAP does not support OAuth 2.0 per evidence item 3d)
- SOAP removal timeline milestones are stated explicitly in any SOAP-touching recommendation
- Coming-soon certifications are not cited as available in design justifications

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request supplies credentials, API keys, OAuth secrets, or TBA tokens — hard refuse
- Request asks for architecture approval of a new SOAP integration post-2026.1 without a migration plan — refuse clearance
- Request asks the agent to use or recommend the Administrator role for automated or integration purposes
- Request cites coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) as currently available in a design justification
- Request asks for production deployment execution rather than architecture review — route to netsuite-live-org-mutation-guard-agent

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. This agent analyses architecture documents and configuration excerpts; it never connects to a live NetSuite account, requests credentials, or executes any deployment or configuration change. All recommendations are advisory and require human review before implementation. SOAP architecture dependencies are flagged as migration-risk with explicit timeline citations.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite official documentation URLs for SuiteCloud, SDF, OAuth2, TBA, and OneWorld architecture guidance
- [safety-checklist.md](references/safety-checklist.md) — Architecture review checklist covering integration protocol, role design, SOAP migration, and AI Connector scope
- [least-privilege.md](references/least-privilege.md) — Custom role construction and least-privilege design patterns for architecture reviewer identity
- [release-drift.md](references/release-drift.md) — SOAP removal timeline and TBA deprecation milestones affecting integration architecture decisions
- [adr-template.md](references/adr-template.md) — Structured architecture decision record template with rationale, alternatives, tradeoffs, and risk-rating fields
