---
name: netsuite-audit-controls-sox-skill
description: "Flashlight skill for reviewing NetSuite financial governance and SOX internal control configurations. T0 static review — no live account connection required. TRIGGER when: user asks to review segregation of duties, SoD conflicts, posting period controls, period-close procedures, revenue recognition schedules, approval workflow chains for journal entries or vendor bills, audit trail completeness, or SOX walkthrough evidence in NetSuite. Trigger phrases: SOX review, segregation of duties, SoD conflict, posting period locked, period close checklist, revenue recognition schedule, journal entry approval, audit trail, SOX material weakness, internal controls review. DO NOT TRIGGER when: request is about routine AP/AR transaction processing without a SOX angle (use netsuite-financial-foundations-agent), OAuth or TBA authentication setup (use netsuite-sso-oauth-tba-agent), SuiteFlow workflow syntax or builder mechanics (use netsuite-suiteflow-automation-agent), SuiteScript code security (use netsuite-suitescript-secure-code-review-agent), or live account mutation is required (use netsuite-live-org-mutation-guard-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
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

# NetSuite Audit Controls SOX Skill

## Purpose

Validates that NetSuite financial control configurations meet SOX audit requirements: SoD conflicts across AP/AR/GL roles, posting period lock-down rules, multi-step journal entry approval chains, ASC 606 / VSOE revenue recognition setup, and audit trail integrity for all financial transactions. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User submits role permission exports or approval workflow definitions for SOX control review
- Internal audit team needs SoD conflict analysis across AP, AR, GL, and payroll roles
- External auditor requests SOX walkthrough documentation for NetSuite financial controls
- CoE architect needs to validate posting period lock procedures and close-calendar coverage
- Finance team needs revenue recognition schedule reviewed against ASC 606 / VSOE requirements

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request role permission exports, approval workflow definitions, posting period status, revenue recognition configuration, and audit trail coverage table
2. Step 2 — SoD analysis: identify permission overlaps across AP entry, AP approval, AR entry, AR approval, GL posting, and cash management functions; rate each conflict
3. Step 3 — Posting period review: verify lock/unlock sequence, confirm who holds Manage Accounting Periods, check that prior periods are locked before current period close
4. Step 4 — Approval workflow audit: validate multi-step approval chains exist for journal entries, vendor bills, purchase orders, and expense reports; check for approval bypass conditions
5. Step 5 — Revenue recognition review: confirm recognition method aligns to ASC 606 or VSOE requirements, deferral account mapping is correct, and schedule release is approval-gated
6. Step 6 — Audit trail completeness: verify system notes are enabled for all financial transaction types; flag any transaction type missing field-history tracking
7. Step 7 — Emit SOX findings report: rated Critical / High / Medium / Low with control deficiency categorization (deficiency / significant deficiency / material weakness) and safe-next-actions

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for roles with Manage Accounting Periods or Access Token Management permissions
- All SoD findings cite specific permission overlaps from submitted role exports, not from inference alone
- Approval workflow bypass conditions (e.g., auto-approve for low amounts) are flagged and rated

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, activating, or unlocking any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration, review, or period-close operations — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized configuration excerpts; never requests or accepts credentials, tokens, session IDs, consumer keys, or any authentication material. Does not connect to, query, or mutate any NetSuite account in any environment. Role recommendations explicitly exclude the Administrator role. 2FA designation requirements are surfaced for roles with Manage Accounting Periods or sensitive access-management permissions. SOX evidence artifacts are generated as draft documents for human reviewer sign-off only.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite certification and financial governance help URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for role exports and financial configuration excerpts
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance for SOX reviewer posture derived from Accountant standard role
- [release-drift.md](references/release-drift.md) — NetSuite release cadence notes for posting period engine and approval workflow changes
- [sox-control-map.md](references/sox-control-map.md) — Mapping of SOX Section 302/404 control objectives to NetSuite configuration review areas
