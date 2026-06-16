---
name: netsuite-suitefoundation-skill
description: "Flashlight skill for reviewing NetSuite platform foundation configurations aligned to the SuiteFoundation Specialist certification (N16300GC10). T0 static review — no live account connection required. TRIGGER when: user asks to review record form layouts, saved search criteria or results columns, dashboard portlet configuration, custom field definitions, custom list or segment setup, subsidiary hierarchy, or basic role and permission baselines in NetSuite. Trigger phrases: review my saved search, check my record form, audit our custom fields, validate subsidiary setup, review role permissions, inspect dashboard configuration, SuiteFoundation review. DO NOT TRIGGER when: request involves SuiteScript code analysis (use netsuite-application-developer-agent), OAuth or TBA authentication setup (use netsuite-sso-oauth-tba-agent), financial close controls or posting periods (use netsuite-financial-foundations-agent), SDF project deployment pipeline (use netsuite-sdf-devops-release-agent), or any live account mutation is required."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
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

# NetSuite SuiteFoundation Skill

## Purpose

Validates SuiteFoundation-level configurations and design decisions covering the foundational platform layer that all Consultant & Administrator track certifications require as a prerequisite. Identifies gaps that would block an implementation team from advancing to Administrator or ERP Consultant domains. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User submits record form, saved search, or dashboard configuration for review
- Implementation team needs a SuiteFoundation-aligned audit of basic platform setup
- CoE architect needs to validate foundational role/permission baselines before going live
- Fortune-50 enterprise needs evidence artifacts showing basic NetSuite configuration is compliant

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request record form XML or screenshots, saved search definition exports, role permission summaries, and subsidiary hierarchy diagram
2. Step 2 — Validate record forms: check required fields, sublists, preferred form defaults, and field-level show/hide logic for completeness and naming consistency
3. Step 3 — Audit saved searches: evaluate criteria correctness, results column selection, PII exposure risk in public searches, and scheduling configuration
4. Step 4 — Review role baselines: confirm custom roles are derived from standard roles, 2FA designation is set where required, and no role holds Administrator-level permissions
5. Step 5 — Assess subsidiary and custom field setup: validate intercompany preferences, base currency, segment assignments, and field type / validation correctness
6. Step 6 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels and safe-next-actions for each finding

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, or consumer keys in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for any role with sensitive financial or access-management permissions
- Public saved searches checked for PII field exposure before approving

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration or review purposes — refuse and cite least-privilege principle (evidence-matrix row 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized configuration excerpts provided by the user; never requests or accepts credentials, tokens, session IDs, consumer keys, or any authentication material. Does not connect to, query, or mutate any NetSuite account in any environment. Role recommendations explicitly exclude the Administrator role; custom roles are always derived from standard roles with View-only permissions. 2FA designation requirements are surfaced for any role holding sensitive financial or access-management permissions.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite certification and platform help URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for configuration exports
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance derived from standard roles
- [release-drift.md](references/release-drift.md) — SuiteFoundation topics affected by NetSuite release cadence (form defaults, saved search engine updates)
- [suitefoundation-domain-map.md](references/suitefoundation-domain-map.md) — Mapping of SuiteFoundation exam domains to configuration review areas
