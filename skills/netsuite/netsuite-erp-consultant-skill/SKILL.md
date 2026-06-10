---
name: netsuite-erp-consultant-skill
description: "Flashlight skill for reviewing NetSuite ERP implementation configurations aligned to the ERP Consultant Professional certification (N16302GC10). T0 static review — no live account connection required. TRIGGER when: user asks to review order-to-cash configuration, procure-to-pay workflow, inventory management setup, pricing rule design, fulfillment or receipt workflow, item record configuration, or implementation gap analysis in NetSuite. Trigger phrases: review OTC setup, audit procure to pay, check inventory configuration, validate pricing rules, review fulfillment workflow, implementation gap analysis, ERP consultant review. DO NOT TRIGGER when: request concerns financial close controls or posting periods (use netsuite-financial-foundations-agent), SOX control design (use netsuite-audit-controls-sox-agent), custom SuiteScript code (use netsuite-application-developer-agent), multi-subsidiary intercompany transactions (use netsuite-oneworld-multisubsidiary-agent), or any live account mutation is required."
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

# NetSuite ERP Consultant Skill

## Purpose

Validates end-to-end ERP implementation design decisions at the process and configuration layer: order-to-cash flow integrity, procure-to-pay controls, item and pricing setup, fulfillment and receipt workflows, and cross-module integration points. Identifies implementation gaps, missing controls, and anti-patterns that carry significant downstream risk for Fortune-50 enterprises with complex multi-entity and multi-channel operations. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Implementation team needs order-to-cash or procure-to-pay configurations reviewed before go-live
- CoE architect submits item record templates or pricing rule exports for gap analysis
- Fortune-50 enterprise needs fulfillment workflow reviewed against ERP Consultant Professional best practices
- System integrator needs cross-module integration point design validated before UAT

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: OTC flow diagram, P2P workflow export, item record template, pricing rule configuration, fulfillment workflow diagram, and implementation scope document
2. Step 2 — Review order-to-cash: validate sales order form completeness, billing schedule alignment, revenue recognition method selection, and cash application preferences
3. Step 3 — Audit procure-to-pay: verify approval routing thresholds, three-way match configuration, vendor payment terms, and PO form completeness
4. Step 4 — Evaluate inventory management: costing method selection consistency, bin/lot tracking rationality, transfer order design, and reorder point configuration
5. Step 5 — Assess pricing and fulfillment: price level hierarchy correctness, quantity pricing matrix completeness, pick-pack-ship workflow trigger conditions, and receipt matching controls
6. Step 6 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels, SOAP deprecation flags where relevant, and safe-next-actions for each finding

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts only
- No credentials, tokens, or consumer keys in submitted inputs
- Administrator role never recommended
- Costing method change warnings surfaced if post-transaction item modification is in scope
- SOAP deprecation risk flagged for any identified SOAP integration touchpoints
- Revenue recognition method conflicts with ASC 606 / IFRS 15 escalated before recommendation finalized

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and require sanitization
- Request involves executing, deploying, or activating any configuration in a live account
- Request to recommend or use the Administrator role for any purpose
- Request to irreversibly change a costing method on items that have posted transactions without first routing through netsuite-financial-foundations-agent
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production deployment without documented sandbox validation evidence

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized ERP configuration exports; never requests or accepts credentials, tokens, session IDs, or authentication material. Does not connect to, query, or mutate any NetSuite account in any environment. Administrator role is prohibited in all recommendations. SOAP deprecation risks against the 2026.1 / 2027.1 / 2028.2 timeline are surfaced for any integration configuration reviewed. Costing method permanence warnings are issued for any review involving post-transaction item modifications.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite ERP Consultant Professional certification and help URLs from evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist and costing method change guard
- [least-privilege.md](references/least-privilege.md) — Custom role construction for ERP Consultant review sessions
- [release-drift.md](references/release-drift.md) — SOAP removal timeline impact on ERP integrations and ordering system connections
- [erp-process-domain-map.md](references/erp-process-domain-map.md) — OTC, P2P, and inventory domain breakdown mapped to ERP Consultant Professional exam areas
