---
name: netsuite-suitecloud-developer-skill
description: "Static-review flashlight for NetSuite SuiteCloud Development Framework projects and SuiteScript 2.x code. Adapts the Oracle netsuite-suitescript-upgrade upstream skill (UPL-1.0, Copyright (c) 2019, 2023 Oracle and/or its affiliates) with Vanguard-specific CI gate thresholds and CHANGELOG conventions. Reviews SDF object XML, deployment manifests, SuiteScript entry points, custom record definitions, and SuiteApp packaging. TRIGGER when: user asks to review SDF project structure, audit SuiteScript 2.x code, assess SuiteScript 1.0 or 2.0 upgrade readiness, review a Suitelet or RESTlet design, inspect custom record definitions, review SuiteApp manifest configuration, or score SuiteScript migration complexity. Trigger phrases: SDF review, SuiteScript upgrade, SuiteScript 2.1, custom record design, Suitelet review, SuiteApp packaging, SDF manifest. DO NOT TRIGGER when: the question is about SDF DevOps release pipeline or CI/CD (use netsuite-sdf-devops-release-agent), OWASP SuiteScript security review (use netsuite-suitescript-secure-code-review-agent), OAuth 2.0 or TBA auth for Suitelets/RESTlets (use netsuite-sso-oauth-tba-agent), or role and permission SoD design for script run-as (use netsuite-identity-access-role-permission-agent)."
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

# NetSuite SuiteCloud Developer Skill

## Purpose

SDF project structure, SuiteScript 2.x code quality and upgrade posture, custom record and field design, Suitelet and RESTlet patterns, and SuiteApp packaging. Adapts Oracle's netsuite-suitescript-upgrade skill (UPL-1.0) with Vanguard-specific release gate thresholds and CHANGELOG conventions. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User needs to review or audit a SuiteCloud Development Framework project structure
- User needs SuiteScript 1.0 or 2.0 upgrade readiness assessment and complexity scoring
- User is designing or reviewing a Suitelet, RESTlet, or custom record definition
- User needs to validate SuiteApp manifest and packaging configuration
- User needs CI gate recommendations for SuiteScript upgrade enforcement

## Recommended Workflow

1. Step 1 — Gather inputs: sanitized SDF object XML or SuiteScript excerpt, API version declared, script type, NetSuite release version target
2. Step 2 — Identify SuiteScript API version in use; flag 1.0 as Critical upgrade-required, 2.0 as High upgrade-recommended, 2.1 as current baseline
3. Step 3 — Apply upstream netsuite-suitescript-upgrade 7-factor migration complexity matrix; emit complexity score and upgrade priority
4. Step 4 — Review SDF object definitions: manifest structure, deployment configurations, custom record/field schemas, run-as permission alignment
5. Step 5 — Review Suitelet/RESTlet design: entry-point patterns, authentication configuration, input validation patterns
6. Step 6 — Rate all findings Critical/High/Medium/Low/Unknown; produce structured finding table with evidence labels [FACT], [ASSUMPTION], [INFERENCE]
7. Step 7 — Emit T0 static review output with CI gate recommendations; flag unconverted 1.0 code as deployment blocker; route escalations per boundary rules

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, hardcoded org IDs, or secrets present in inputs — refuse and instruct user to redact if found
- SuiteScript 1.0 usage flagged as Critical upgrade-required finding
- Upstream attribution included when adapting netsuite-suitescript-upgrade material: Copyright (c) 2019, 2023 Oracle and/or its affiliates, UPL-1.0
- Custom run-as role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes credentials, tokens, secrets, hardcoded org IDs, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions for script execution
- Request asks agent to push SDF project, execute deployment commands, or mutate a NetSuite account
- User claims SuiteCloud Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires live execution of SuiteScript or SDF CLI commands

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never executes SDF CLI commands, never pushes to a NetSuite account, never requests or stores credentials, tokens, or org IDs. Works exclusively from sanitized SDF object XML and SuiteScript excerpts. SuiteScript 1.0 usage flagged as Critical. Adapted from oracle/netsuite-suitecloud-sdk netsuite-suitescript-upgrade skill (UPL-1.0, Copyright (c) 2019, 2023 Oracle and/or its affiliates). Never recommends Administrator role for script run-as configuration. All run-as roles must follow least-privilege and 2FA requirements.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Confirmed Oracle/NetSuite official documentation URLs for SDF, SuiteScript, and SuiteApps
- [safety-checklist.md](references/safety-checklist.md) — Pre-review checklist: redaction verification, API version flags, run-as permission checks
- [least-privilege.md](references/least-privilege.md) — Custom role design for SuiteCloud developer reviewers — permissions, 2FA triggers, forbidden roles
- [release-drift.md](references/release-drift.md) — SuiteScript version support lifecycle and upgrade timeline notes
- [sdf-object-reference.md](references/sdf-object-reference.md) — SDF object type reference and required XML field documentation
