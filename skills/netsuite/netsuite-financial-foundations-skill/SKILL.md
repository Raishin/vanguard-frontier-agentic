---
name: netsuite-financial-foundations-skill
description: "Flashlight skill for reviewing NetSuite Accounts Payable, Accounts Receivable, and core accounting configurations aligned to the Financial User (N16599GC10) and Accounting Professional (N16301GC10) certifications. T0 static review — no live account connection required. TRIGGER when: user asks to review AP setup, AR configuration, vendor record defaults, customer invoicing templates, payment terms, chart of accounts structure, accounting preferences, bank account record setup, or period-end reconciliation procedures in NetSuite. Trigger phrases: review AP configuration, check AR setup, audit chart of accounts, validate payment terms, inspect bank account record, period-end reconciliation, accounting preferences review, vendor record defaults. DO NOT TRIGGER when: request involves SOX controls, SoD conflicts, or posting period lock enforcement (escalate to netsuite-audit-controls-sox-agent); multi-subsidiary consolidation (use netsuite-oneworld-multisubsidiary-agent); SuiteFlow workflow mechanics (use netsuite-suiteflow-automation-agent); SuiteScript code review (use netsuite-suitescript-secure-code-review-agent); or live account mutation is required (use netsuite-live-org-mutation-guard-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: finance
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Financial Foundations Skill

## Purpose

Validates AP and AR configuration, accounting setup, and period-end reconciliation procedures against Financial User (N16599GC10) and Accounting Professional (N16301GC10) certification standards. Escalates close-impacting control gaps to netsuite-audit-controls-sox-agent for SOX-level review. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User submits AP or AR configuration exports for review against Financial User or Accounting Professional standards
- Finance team needs chart of accounts structure validated for account type correctness and sub-account hierarchy
- Implementation team needs accounting preferences and bank account records reviewed before go-live
- CoE architect needs period-end reconciliation procedures checked for completeness and procedural gaps

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request AP setup, AR setup, chart of accounts export, accounting preferences screenshot, and bank account record details (masked account numbers)
2. Step 2 — AP review: validate vendor record defaults, payment term configurations, bill approval defaults, and 1099 vendor flag setup
3. Step 3 — AR review: validate customer record defaults, invoicing template configurations, payment method mappings, and collections workflow design
4. Step 4 — Chart of accounts audit: verify account type correctness, sub-account hierarchy, inter-company account presence, and segment assignments
5. Step 5 — Accounting preferences check: confirm base currency, fiscal year start, accounting method, and tax configuration defaults
6. Step 6 — Period-end reconciliation review: validate AP aging tie-out procedure, AR aging tie-out, bank reconciliation workflow, and subledger-to-GL checklist coverage
7. Step 7 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels; escalate SOX-impacting findings to netsuite-audit-controls-sox-agent

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, vendor bank account numbers, credit card numbers, or payment tokens in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for roles with View Unencrypted ACH or Credit Card permissions
- SOX-impacting findings (SoD conflicts, posting period violations) are escalated to netsuite-audit-controls-sox-agent, not resolved unilaterally
- Bank account numbers are masked before submission; agent refuses unmasked account data

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, vendor bank account numbers, payment tokens, credit card numbers, or any authentication or financial account material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for AP/AR review or accounting configuration — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized configuration excerpts; never requests or accepts credentials, tokens, vendor bank account numbers, credit card numbers, payment tokens, or any authentication or financial account material. Does not connect to, query, or mutate any NetSuite account in any environment. Role recommendations explicitly exclude the Administrator role. SOX-impacting findings are escalated to netsuite-audit-controls-sox-agent and never resolved unilaterally.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite Financial User and Accounting Professional certification URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for AP/AR configuration and bank account exports
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance for financial reviewer posture derived from Accountant standard role
- [release-drift.md](references/release-drift.md) — NetSuite release cadence notes for AP/AR engine and accounting period changes
- [financial-foundations-domain-map.md](references/financial-foundations-domain-map.md) — Mapping of Financial User and Accounting Professional exam domains to configuration review areas
