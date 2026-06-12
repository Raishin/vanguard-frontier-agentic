---
name: netsuite-oneworld-multisubsidiary-skill
description: "Flashlight skill for reviewing NetSuite OneWorld multi-entity configurations: subsidiary hierarchies, intercompany account boundaries, cross-subsidiary visibility restrictions, multi-currency settings, and tax-jurisdiction nexus alignment. T0 static review — no live account connection required. TRIGGER when: user asks to review subsidiary structure, audit intercompany accounts, check cross-subsidiary role scoping, validate tax nexus coverage, assess consolidation configuration, or diagnose OneWorld hierarchy issues. Trigger phrases: subsidiary hierarchy, intercompany elimination, cross-subsidiary access, multi-currency consolidation, tax nexus, due-to due-from, legal entity registration, OneWorld configuration. DO NOT TRIGGER when: the user needs authentication or OAuth/TBA token review (use netsuite-sso-oauth-tba-skill), role/permission assignment analysis beyond subsidiary scoping (use netsuite-identity-access-role-permission-skill), SOX audit evidence generation (use netsuite-audit-controls-sox-skill), or SDF deployment of subsidiary changes (use netsuite-sdf-devops-release-skill)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: compliance
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite OneWorld Multi-Subsidiary Skill

## Purpose

Audits OneWorld multi-subsidiary configurations for boundary integrity, intercompany elimination correctness, currency/tax-jurisdiction alignment, and cross-subsidiary role scoping. Identifies misconfigured subsidiary access, missing intercompany accounts, and jurisdiction gaps. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Reviewing a multi-subsidiary NetSuite setup for structural completeness and intercompany correctness
- Auditing cross-subsidiary role scoping to verify users cannot see data outside their legal entity
- Validating tax nexus and jurisdiction alignment for a new country or subsidiary
- Diagnosing consolidation discrepancies caused by missing intercompany elimination accounts
- Assessing multi-currency configuration and exchange-rate type assignments across subsidiaries

## Recommended Workflow

1. Step 1 — Gather inputs: request subsidiary hierarchy export, intercompany account mapping, role restriction excerpts, tax nexus config, and currency settings
2. Step 2 — Map the hierarchy: identify parent/child subsidiary relationships, legal-entity registrations, base currencies, and countries; flag any subsidiary missing a country or currency assignment
3. Step 3 — Review intercompany boundaries: verify each intercompany transaction type has a matching due-to/due-from account pair and an elimination journal entry; flag gaps as High
4. Step 4 — Audit cross-subsidiary visibility: review role configurations for subsidiary restrictions; flag any role granting broader access than the user's legal entity as High
5. Step 5 — Assess tax jurisdiction coverage: map each subsidiary's country to its configured nexus; flag missing nexus registrations as High
6. Step 6 — Review multi-currency settings: verify exchange-rate types and revaluation rules per subsidiary; flag currency mismatches as Medium or High depending on consolidation impact
7. Step 7 — Emit structured findings report: verdict, Critical/High/Medium/Low findings table, safe next actions, and escalation triggers

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite credentials, tokens, or session cookies accepted — reject and ask for sanitized exports
- Tax registration numbers (VAT/GST IDs) must be redacted before submission
- No live mutations recommended — all changes must go through netsuite-live-org-mutation-guard-agent
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference
- Cross-subsidiary data exposure findings escalated as Critical minimum

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete subsidiaries, legal entities, or intercompany accounts in a live account
- Request provides unredacted tax registration numbers, VAT/GST IDs, or legal-entity bank account data — flag and ask for redacted version
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. This agent never requests, stores, echoes, or logs NetSuite credentials, OAuth tokens, TBA tokens, client secrets, or session cookies. Tax registration numbers and legal-entity bank data must be redacted before submission. All live-mutation paths are hard-routed to netsuite-live-org-mutation-guard-agent. No org connection is established at any point.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite OneWorld and multi-currency documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — Pre-review sanitization steps for subsidiary and tax configuration exports
- [least-privilege.md](references/least-privilege.md) — Custom reviewer role specification for OneWorld configurations
- [release-drift.md](references/release-drift.md) — OneWorld feature changes by release that may affect subsidiary or intercompany behavior
- [intercompany-patterns.md](references/intercompany-patterns.md) — Reference patterns for intercompany account pairing and elimination journal configurations
