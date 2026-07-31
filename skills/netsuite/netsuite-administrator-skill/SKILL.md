---
name: netsuite-administrator-skill
description: "Flashlight skill for reviewing NetSuite account administration configurations aligned to the Administrator Professional certification (N16291GC10). T0 static review — no live account connection required, never Administrator role. TRIGGER when: user asks to review accounting preferences, tax nexus setup, currency management, user provisioning policy, email template defaults, sandbox refresh procedures, release preview planning, or account-level system preferences in NetSuite. Trigger phrases: review account setup, audit user provisioning, check accounting preferences, validate tax configuration, sandbox refresh checklist, release preview prep, administrator review. DO NOT TRIGGER when: request concerns OAuth 2.0 or TBA authentication flows (use netsuite-sso-oauth-tba-agent), role permission and SoD matrix design (use netsuite-identity-access-role-permission-agent), financial close controls or posting periods (use netsuite-financial-foundations-agent), SuiteScript code review (use netsuite-application-developer-agent), or any live mutation in a production account is requested."
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

# NetSuite Administrator Skill

## Purpose

Validates enterprise-grade NetSuite account administration decisions and settings that require Administrator Professional-level depth (N16291GC10) but are executed through least-privilege custom roles, never via the Administrator role itself. Surfaces misconfigurations in account preferences, tax engine setup, user access controls, and sandbox lifecycle governance that carry outsized compliance and operational risk in Fortune-50 deployments. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Enterprise implementation team needs account-level administration configurations reviewed before go-live
- CoE or IT governance team submits accounting preferences, tax setup, or user provisioning policies for audit
- Fortune-50 administrator needs sandbox refresh governance documentation reviewed for OAuth 2.0 re-authorization compliance
- Release preview impact assessment is needed for an upcoming NetSuite version upgrade

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: accounting preferences export, tax nexus summary, currency settings, user provisioning template, and sandbox refresh runbook
2. Step 2 — Review accounting preferences: fiscal year alignment, period management defaults, accounting impact settings, and GL preferences
3. Step 3 — Audit tax and currency: validate nexus completeness, tax engine selection rationality, base currency correctness, and exchange rate source configuration
4. Step 4 — Evaluate user provisioning: role assignment patterns, 2FA designation for sensitive roles, separation of concerns in provisioning workflow
5. Step 5 — Assess sandbox and release posture: verify OAuth 2.0 re-authorization procedures exist for post-refresh environments; identify SOAP integration deprecation risks against 2026.1–2028.2 timeline
6. Step 6 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels, explicit Administrator-role prohibition reminders, and safe-next-actions for each finding

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts only
- No credentials, tokens, passwords, or consumer keys in submitted inputs
- Administrator role never recommended under any circumstances
- 2FA designation verified for all roles holding sensitive administrative permissions
- Sandbox refresh runbook includes OAuth 2.0 re-authorization checklist (evidence-matrix rows 8a, 8b)
- SOAP deprecation risk surfaced if any integration is identified as SOAP-based

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, consumer keys, client secrets, passwords, or any authentication material — stop and require sanitization before resubmitting
- Request involves executing, deploying, or activating any configuration change in a live or production account
- Request to use or recommend the Administrator role for any purpose — an absolute refusal; cite evidence-matrix rows 7a and 7b
- Request to connect, authenticate, or log in to any NetSuite environment
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production-environment changes without documented sandbox validation evidence

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized configuration exports; never requests or accepts credentials, tokens, session IDs, consumer keys, or any authentication material. Does not connect to, query, or mutate any NetSuite account. The Administrator role is absolutely prohibited — custom roles are always derived from standard roles with View-only permissions. OAuth 2.0 sandbox isolation requirements (re-authorization after each refresh) are surfaced in every sandbox governance review. SOAP deprecation risks (2026.1 / 2027.1 / 2028.2 milestones) are flagged for any integration posture identified during review.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite Administrator Professional certification and help URLs from evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization and least-privilege custom role construction checklist
- [least-privilege.md](references/least-privilege.md) — Custom role derivation pattern for administrative reviewer — never Administrator role
- [release-drift.md](references/release-drift.md) — SOAP removal timeline (2026.1 / 2027.1 / 2028.2) and OAuth 2.0 default migration impact
- [sandbox-oauth-isolation.md](references/sandbox-oauth-isolation.md) — OAuth 2.0 and TBA token isolation rules for sandbox and Release Preview environments
