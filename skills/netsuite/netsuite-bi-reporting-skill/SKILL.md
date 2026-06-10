---
name: netsuite-bi-reporting-skill
description: "Reviews and designs NetSuite reports, dashboards, and KPI definitions against BI & Reporting Associate/Specialist standards. Validates data-source scoping, period and subsidiary filters, KPI formula correctness, and financial narrative accuracy. TRIGGER when: user asks to review or design a NetSuite report, dashboard, KPI meter, financial narrative, chart, pivot, or executive summary; phrases include 'build a dashboard', 'review my report', 'create a KPI', 'why does this report show X', 'configure a portlet', 'financial narrative for the board', 'budget vs actual report'. DO NOT TRIGGER when: the request is about saved search criteria or column configuration (use netsuite-saved-searches-workbook-skill), SuiteAnalytics Workbook pivot/table mechanics (use netsuite-saved-searches-workbook-skill), or live execution of queries against a connected NetSuite org."
license: UPL-1.0
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
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

# NetSuite BI Reporting Skill

## Purpose

Report and dashboard architecture, KPI configuration, and data-source correctness in NetSuite. Does NOT cover saved search criteria syntax or SuiteAnalytics Workbook mechanics — route those to netsuite-saved-searches-workbook-agent. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User asks to review or create a NetSuite report, dashboard, or KPI definition
- User needs to validate data-source scoping including subsidiary, period, or currency filters
- User requests a financial narrative, variance commentary, or board-level summary
- User asks why a report shows unexpected results and provides configuration details
- User needs to review report access controls or dashboard sharing settings

## Recommended Workflow

1. Step 1 — Gather the report or dashboard configuration excerpt, including type, data source, filters, columns, and KPI formula if applicable.
2. Step 2 — Identify the intended audience and use case; confirm subsidiary and period scope are explicitly set.
3. Step 3 — Review data-source correctness: verify the report type matches analytical intent (e.g., Summary vs. Detail vs. Financial Statement).
4. Step 4 — Validate KPI definitions: formula accuracy, comparison period alignment, and threshold calibration against user-supplied benchmarks.
5. Step 5 — Assess access controls: confirm report visibility is scoped to intended roles with View-only grants for consumers.
6. Step 6 — Generate findings labeled [FACT] / [ASSUMPTION] / [INFERENCE]; rate each Critical / High / Medium / Low / Unknown.
7. Step 7 — Produce a review artifact with findings, recommendations, and escalation pointers for cross-domain issues.

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection, credentials, or session tokens used at any point
- BI & Reporting Professional certification NOT claimed as available — status is UNVERIFIED
- All KPI formulas and thresholds derived from user-supplied configuration only, never fabricated
- PII-in-report concerns escalated to netsuite-saved-searches-workbook-agent
- SOX-evidenced reporting findings escalated to netsuite-audit-controls-sox-agent

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to log in to, connect to, or execute queries against a live NetSuite account
- Request to deploy, publish, schedule, or share a report or dashboard
- Claim that BI & Reporting Professional certification is currently available — status is UNVERIFIED
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw customer PII in report data without explicit sanitization

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never connects to, queries, or mutates any NetSuite account. No credentials, session tokens, or API keys are requested or processed. All review output is a draft artifact requiring human validation before any dashboard or report is published or shared.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite BI & Reporting certification and documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — Pre-review safety gates and refusal conditions
- [least-privilege.md](references/least-privilege.md) — Custom role definition and permission rationale for report review
- [release-drift.md](references/release-drift.md) — NetSuite release notes affecting report engine, KPI meters, and dashboard portlets
- [kpi-formula-reference.md](references/kpi-formula-reference.md) — Validated KPI formula patterns and common misconfiguration catalog
