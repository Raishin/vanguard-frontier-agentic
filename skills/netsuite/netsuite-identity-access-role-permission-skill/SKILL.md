---
name: netsuite-identity-access-role-permission-skill
description: "Static review flashlight for NetSuite role configurations, permission assignments, and Segregation-of-Duties design. Validates custom roles against standard baselines, resolves permission codes from the 684-code SDF catalog, and flags SoD conflicts and over-permissioned roles. TRIGGER when: user asks to review a NetSuite role, check permissions on a role, audit segregation of duties, validate a custom role, analyze SDF customrole XML, check who has Administrator access, review run-as configuration for a script or integration, map permissions to least privilege, or assess 2FA role designations. Trigger phrases: review netsuite role, check role permissions, segregation of duties netsuite, custom role from standard, sdf customrole xml, least privilege role, who has administrator, run-as role, 2fa role designation. DO NOT TRIGGER when: the question is about OAuth 2.0, TBA, SSO, or SAML configuration (use netsuite-sso-oauth-tba-skill); when SDF project structure or deployment pipeline is the subject (use netsuite-sdf-devops-release-skill); when the request is to write SuiteScript or review code security (use netsuite-suitescript-secure-code-review-skill); or when the user needs a live role assignment executed in a NetSuite account (escalate to netsuite-live-org-mutation-guard-agent)."
license: UPL-1.0
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: security
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Identity Access Role Permission Skill

## Purpose

Role structure, permission levels, and SoD conflict detection in NetSuite. Covers standard role baselines, custom role derivation, permission catalog lookup against the 684-code SDF catalog, and multi-role SoD conflict matrices. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User needs a role configuration reviewed for over-permission or SoD conflicts
- SDF customrole XML export needs permission-level validation against the 684-code catalog
- Custom role derivation from a standard role must be verified
- Integration record or script run-as role needs least-privilege assessment
- 2FA designation coverage for privileged roles needs an audit

## Recommended Workflow

1. Step 1 — Collect sanitized role export or SDF customrole XML; confirm no credentials or token values are present
2. Step 2 — Identify the standard role baseline the custom role was copied from; flag if copied from Administrator or created blank
3. Step 3 — Resolve each permkey against the netsuite-sdf-roles-and-permissions catalog (684 codes); label unknowns [UNVERIFIED]
4. Step 4 — Apply SoD conflict matrix: flag any role combining initiating and approving functions on the same transaction type
5. Step 5 — Map permissions triggering mandatory 2FA (evidence 5c); flag any such role missing 2FA designation
6. Step 6 — Rate every finding Critical / High / Medium / Low / Unknown; emit structured report with remediation guidance and escalation triggers

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or client secrets in the submitted configuration excerpt
- Role analysis is read-only — no account changes are recommended without human review
- Every permission recommendation cites an evidence row or the Oracle SDF permission catalog
- Administrator role is never recommended for any purpose
- SoD findings are rated and routed to a named human decision owner before remediation

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes or asks for user passwords, access tokens, TBA token values, OAuth client secrets, or session cookies
- Request asks the agent to act as or assume Administrator role
- Request asks to perform a live role assignment, permission edit, or user account modification — escalate to netsuite-live-org-mutation-guard-agent
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for role alignment context
- Request asks to generate TBA tokens, OAuth authorization codes, or integration credentials
- Scope creep: authentication mechanism design questions belong to netsuite-sso-oauth-tba-agent

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works from sanitized configuration excerpts and never requests credentials, tokens, client secrets, or user PII. Never assumes or recommends Administrator role. Every permission recommendation cites official evidence. Does not perform live role assignments or account mutations.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite official documentation URLs for roles, permissions, and 2FA requirements
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission checklist for sanitizing role exports before analysis
- [least-privilege.md](references/least-privilege.md) — Custom role design guide: standard role baselines, permkey conventions, SoD matrix
- [release-drift.md](references/release-drift.md) — Tracks SOAP/TBA deprecation milestones relevant to integration-record role design
- [sod-conflict-matrix.md](references/sod-conflict-matrix.md) — Reference conflict pairs for common NetSuite financial and administrative function combinations
