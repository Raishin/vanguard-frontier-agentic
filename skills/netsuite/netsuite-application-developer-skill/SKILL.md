---
name: netsuite-application-developer-skill
description: "Reviews NetSuite SuiteScript 2.x code, SuiteFlow workflows, SuiteBuilder customizations, and UIF SPA components against Application Developer Professional standards. Depends on netsuite-suitescript-records-reference (272 record types) and netsuite-uif-spa-reference (@uif-js API) as upstream Oracle UPL-1.0 reference contexts. TRIGGER when: user asks to review or write SuiteScript, design a SuiteFlow workflow, configure a custom record or field, build a UIF SPA component, or review a script deployment; phrases include 'client script', 'user event script', 'MapReduce script', 'scheduled script', 'Suitelet', 'RESTlet', 'SuiteFlow', 'custom record type', 'UIF component', '@uif-js', 'governance limits', 'script entry point'. DO NOT TRIGGER when: the request is about SDF project deployment or SuiteScript 1.0 migration (use netsuite-suitecloud-developer-skill), security code review for injection or XSS (use netsuite-suitescript-secure-code-review-skill), REST or SOAP API integration design (use netsuite-web-services-integration-skill), or live deployment execution."
license: UPL-1.0
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

# NetSuite Application Developer Skill

## Purpose

SuiteScript 2.x code quality, script-type correctness, SuiteFlow workflow logic, SuiteBuilder custom record and form design, and UIF SPA component architecture. Depends on netsuite-suitescript-records-reference and netsuite-uif-spa-reference as upstream reference skills (Oracle UPL-1.0). T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User asks to review or write a SuiteScript 2.x client script, user event script, scheduled script, map/reduce script, Suitelet, or RESTlet
- User needs to review a SuiteFlow workflow configuration including trigger, states, conditions, and actions
- User asks to design or validate a SuiteBuilder custom record type, custom field, or custom form layout
- User needs to review a UIF SPA component including DataGrid, Form, StackPanel, or store/state patterns
- User asks about governance limits, entry-point selection, or script deployment run-as configuration

## Recommended Workflow

1. Step 1 — Load netsuite-suitescript-records-reference as dependency context for field ID and record type validation; load netsuite-uif-spa-reference if UIF component review is in scope.
2. Step 2 — Gather the SuiteScript file(s) with NS annotations, the deployment record configuration, and the record type binding.
3. Step 3 — Validate script type selection, entry-point declarations, and module require() calls for correctness and least-privilege posture.
4. Step 4 — Assess governance limit exposure: identify synchronous paths that risk exceeding account limits; flag unbounded search loops or excessive record loads.
5. Step 5 — Review SuiteFlow or UIF component configuration if in scope; validate API correctness against the loaded reference contexts.
6. Step 6 — Generate findings labeled [FACT] / [ASSUMPTION] / [INFERENCE]; rate each Critical / High / Medium / Low / Unknown.
7. Step 7 — Produce a review artifact with findings, escalation pointers, and recommended next actions.

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection, credentials, or session tokens used at any point
- netsuite-suitescript-records-reference loaded before asserting field ID or record type compatibility
- netsuite-uif-spa-reference loaded before asserting @uif-js API correctness
- Governance limit violations rated Critical when synchronous path can exhaust account limits
- SuiteScript 1.0 patterns escalated to netsuite-suitecloud-developer-agent, not handled here

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to deploy, activate, schedule, or execute any script or workflow in a live or sandbox account
- Request to assume Administrator role or any role granting full account access
- Request to run security penetration tests or exploit discovery — use netsuite-suitescript-secure-code-review-agent
- Request to perform SDF project deployment or SuiteScript 1.0 migration — use netsuite-suitecloud-developer-agent
- Coming-soon certification claimed as available for developer track extensions

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never deploys, activates, or modifies any script, workflow, or customization in any NetSuite account. No credentials, session tokens, or API keys are requested or processed. Script run-as accounts reviewed must follow least-privilege posture with Administrator role explicitly forbidden.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle Application Developer Professional exam and SuiteScript documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — Governance limit, run-as role, and static-review safety gates
- [least-privilege.md](references/least-privilege.md) — Custom role definition and permission rationale for developer review
- [release-drift.md](references/release-drift.md) — NetSuite release notes affecting SuiteScript APIs, SuiteFlow, and UIF components
- [script-type-reference.md](references/script-type-reference.md) — SuiteScript 2.x script type entry-point and governance limit quick reference
