---
name: netsuite-sandbox-nonproduction-governance-skill
description: "Static-review flashlight for NetSuite sandbox, Release Preview, and non-production environment governance. Enforces the confirmed isolation facts: OAuth 2.0 authorized apps and client credentials flow setup in production are NOT copied to sandbox or Release Preview (and are cleared on each sandbox refresh); TBA tokens created in production are NOT copied to sandbox or Release Preview. Enforces that sandbox success does not equal production readiness without explicit re-authorization. TRIGGER when: user asks about sandbox governance, OAuth app re-authorization after sandbox refresh, TBA token management across environments, Release Preview usage policies, environment isolation between production and sandbox, sandbox-to-production promotion readiness, or sandbox refresh impact on integration testing. Trigger phrases: sandbox refresh, OAuth re-authorize sandbox, Release Preview isolation, sandbox governance, non-production environment, sandbox success production readiness, TBA token sandbox. DO NOT TRIGGER when: the question is about OAuth 2.0 or TBA auth flow mechanics (use netsuite-sso-oauth-tba-agent), integration API endpoint design (use netsuite-web-services-integration-agent), SDF release pipeline automation (use netsuite-sdf-devops-release-agent), or role and permission SoD design (use netsuite-identity-access-role-permission-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: operational
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Sandbox and Non-Production Governance Skill

## Purpose

Sandbox and non-production environment separation, OAuth 2.0 app re-authorization requirements per environment, TBA token isolation, sandbox refresh cycles, and Release Preview usage governance. Enforces the principle that authorized applications and tokens are not copied between environments and must be explicitly re-authorized after each sandbox refresh. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User needs to review or design OAuth 2.0 re-authorization procedures for sandbox and Release Preview environments
- User is planning a sandbox refresh and needs to assess impact on active OAuth apps and TBA tokens
- User needs to assess whether their sandbox test results are sufficient evidence of production readiness
- User needs to establish environment separation policies between production, sandbox, and Release Preview
- User needs to design a sandbox-to-production promotion checklist that includes OAuth and TBA re-authorization steps

## Recommended Workflow

1. Step 1 — Gather inputs: environment inventory, OAuth 2.0 re-authorization processes, sandbox refresh schedule, TBA token management procedures, integration test suite scope
2. Step 2 — Verify environment separation: confirm distinct OAuth 2.0 authorized apps, client credentials flow setups, and TBA tokens per environment; flag any assumption that production config copies to sandbox
3. Step 3 — Assess post-refresh re-authorization coverage: identify which OAuth apps and TBA tokens require explicit re-authorization or re-creation after each sandbox refresh
4. Step 4 — Evaluate sandbox-to-production promotion readiness checklist: verify OAuth re-authorization step is present before smoke-test; flag its absence as High governance gap
5. Step 5 — Review Release Preview governance: confirm it is not used for production-equivalent load testing or treated as production fallback without explicit re-authorization
6. Step 6 — Rate all governance gaps Critical/High/Medium/Low/Unknown; produce structured finding table with evidence labels [FACT], [ASSUMPTION], [INFERENCE]
7. Step 7 — Emit T0 static review output: governance gap report with re-authorization checklist, sandbox refresh impact assessment, and escalation routing

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- Core isolation fact enforced: OAuth 2.0 authorized apps and client credentials flow NOT copied to sandbox/Release Preview; TBA tokens NOT copied (evidence-matrix rows 8a-8d)
- Sandbox success != production readiness principle enforced — re-authorization step required in promotion checklist
- Custom reviewer role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to access a live NetSuite account, execute environment changes, or mutate any account
- User asserts that OAuth 2.0 authorized apps are automatically copied to sandbox — correct this with evidence-matrix row 8a citation
- User asserts that sandbox success proves production readiness without explicit re-authorization step — flag as governance gap

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — never accesses live NetSuite accounts, never executes environment changes, never requests or stores credentials, tokens, client secrets, or org IDs. Works exclusively from sanitized environment documentation. Enforces confirmed isolation facts: OAuth 2.0 authorized apps and client credentials flow setup are NOT copied to sandbox or Release Preview (cleared on refresh); TBA tokens are NOT copied. Enforces that sandbox success does not equal production readiness. Never recommends Administrator role for sandbox governance roles. Custom reviewer role requires 2FA when permissions include OAuth 2.0 Authorized Applications Management or Access Token Management.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Confirmed Oracle/NetSuite official documentation URLs for OAuth 2.0 environment isolation and sandbox governance
- [safety-checklist.md](references/safety-checklist.md) — Pre-review checklist: redaction verification, environment isolation facts, re-authorization coverage checks
- [least-privilege.md](references/least-privilege.md) — Custom role design for sandbox governance reviewers — permissions, 2FA triggers, forbidden roles
- [release-drift.md](references/release-drift.md) — Environment-specific OAuth 2.0 and TBA isolation facts — evidence-matrix rows 8a-8d
- [sandbox-promotion-checklist.md](references/sandbox-promotion-checklist.md) — Sandbox-to-production promotion checklist including OAuth re-authorization and smoke-test steps
