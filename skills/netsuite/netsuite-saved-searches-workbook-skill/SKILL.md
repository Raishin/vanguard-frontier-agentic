---
name: netsuite-saved-searches-workbook-skill
description: "Reviews NetSuite saved search criteria, results column configuration, SuiteAnalytics Workbook pivot and chart design, PII-in-export exposure, and cross-subsidiary data leakage risk. TRIGGER when: user asks to review or build a saved search, configure search criteria or results columns, design a SuiteAnalytics Workbook, troubleshoot search results, check for PII in exported data, or validate cross-subsidiary filtering; phrases include 'saved search criteria', 'search results columns', 'SuiteAnalytics workbook', 'pivot table in NetSuite', 'scheduled search', 'PII in search export', 'cross-subsidiary filter'. DO NOT TRIGGER when: the request is about high-level report layout or KPI meters (use netsuite-bi-reporting-skill), SuiteScript code driving the search (use netsuite-suitecloud-developer-skill), or when the user needs to execute a live query against a connected org."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: data
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Saved Searches Workbook Skill

## Purpose

Saved search and SuiteAnalytics Workbook mechanics: criteria syntax, results columns, join paths, formula fields, scheduling, and data-export risk including PII exposure and cross-subsidiary leakage. Does NOT cover high-level report layout or KPI design — route those to netsuite-bi-reporting-agent. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User asks to review or create a NetSuite saved search or SuiteAnalytics Workbook
- User needs to validate search criteria syntax, join paths, or formula fields
- User asks to check for PII fields in search results or scheduled export recipients
- User needs to verify cross-subsidiary filters are correctly scoped in a OneWorld account
- User asks about search scheduling, delivery settings, or public vs. private access controls

## Recommended Workflow

1. Step 1 — Gather the saved search or workbook configuration: type, all criteria conditions, results columns, summary type, and access setting.
2. Step 2 — Identify the record type and confirm subsidiary scope; flag if cross-subsidiary risk applies.
3. Step 3 — Scan results columns for PII fields (email, phone, address, SSN, credit card, date of birth); flag any as a default High finding.
4. Step 4 — Review criteria completeness: verify required filters (date range bounds, subsidiary, transaction status) are present and correctly joined.
5. Step 5 — Assess scheduling and delivery: confirm recipient roles, data sensitivity of output, and whether external email delivery is appropriate.
6. Step 6 — Generate findings labeled [FACT] / [ASSUMPTION] / [INFERENCE]; rate each Critical / High / Medium / Low / Unknown.
7. Step 7 — Produce a review artifact with findings, recommendations, and escalation pointers for cross-domain issues.

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection, credentials, or session tokens used at any point
- PII-in-export flagged as High by default when personal data fields appear in results columns
- Cross-subsidiary leakage flagged as High when subsidiary filter is absent in OneWorld context
- All field internal IDs from user-supplied configuration only; lookups marked [INFERENCE] if not confirmed
- Scheduling and delivery risks escalated to netsuite-data-governance-privacy-agent when external recipients involved

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to execute, run, preview, or schedule a search against a live NetSuite account
- Request to share or publish a search or workbook
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw unmasked PII fields without prior sanitization acknowledgment
- Coming-soon certification claimed as currently available for this domain

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never executes, schedules, or shares any saved search or workbook in any NetSuite account. No credentials, session tokens, or API keys are requested or processed. PII-in-export findings are treated as High severity by default and escalated to the data governance agent when external delivery is involved.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite saved search and SuiteAnalytics documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — PII-in-export and cross-subsidiary leakage refusal gates
- [least-privilege.md](references/least-privilege.md) — Custom role definition and permission rationale for search review
- [release-drift.md](references/release-drift.md) — NetSuite release notes affecting saved search engine and SuiteAnalytics Workbook
- [pii-field-catalog.md](references/pii-field-catalog.md) — Catalog of NetSuite record fields that constitute PII for export risk assessment
