---
name: netsuite-sdf-devops-release-skill
description: "Static review flashlight for SuiteCloud Development Framework project structure, deployment controls, and environment promotion governance. Validates manifest.xml completeness, deploy.xml ordering, customrole permission XML against the 684-code SDF catalog, required documentation artifacts, and SuiteScript version gates. TRIGGER when: user asks to review an SDF project, validate a manifest.xml or deploy.xml, check deployment configuration, review environment promotion from sandbox to production, verify SuiteCloud release process, check for SuiteScript 1.0 code before deployment, confirm documentation artifacts are present, or audit SDF customrole permissions in a deployment object. Trigger phrases: sdf project structure, validate manifest xml, deploy xml review, sandbox to production netsuite, suitecloud deployment, sdf customdeploy, netsuite release pipeline, sdf environment promotion, suitescript version gate, architecture md missing. DO NOT TRIGGER when: the question is specifically about role SoD or permission design outside of a deployment context (use netsuite-identity-access-role-permission-skill); when the request is about OAuth 2.0 or TBA authentication mechanics (use netsuite-sso-oauth-tba-skill); when SuiteScript OWASP code security is the primary subject (use netsuite-suitescript-secure-code-review-skill); or when the user needs to execute a deployment in a live account (escalate to netsuite-live-org-mutation-guard-agent)."
license: UPL-1.0
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: devsecops
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite SDF DevOps Release Skill

## Purpose

SDF project structure correctness, deployment configuration review, and environment promotion governance. Validates manifest.xml completeness, deploy.xml ordering, customrole permission XML against the SDF permission catalog, and pre/post-deployment documentation requirements (README, ARCHITECTURE, CHANGELOG). Flags SuiteScript 1.0 unconverted code as a deployment blocker. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- SDF project manifest.xml or deploy.xml needs review before a promotion is attempted
- Environment promotion path from sandbox to production requires governance documentation
- Customrole permission XML in a deployment object needs least-privilege validation
- Documentation artifacts (README.md, ARCHITECTURE.md, CHANGELOG.md) need a completeness gate check
- SuiteScript version risk (1.0 files present) needs to be assessed before a release

## Recommended Workflow

1. Step 1 — Collect sanitized SDF project excerpts (manifest.xml, deploy.xml, selected object XML); confirm no credentials or token values are present
2. Step 2 — Validate manifest.xml: check project ID, publisher ID, object list completeness, and missing dependency declarations
3. Step 3 — Validate deploy.xml: check object ordering for dependency correctness; flag circular dependencies or missing prerequisite objects
4. Step 4 — Cross-reference customrole permkey/permlevel entries against the netsuite-sdf-roles-and-permissions 684-code catalog; flag Administrator-level grants as Critical
5. Step 5 — Check documentation artifact inventory: README.md, ARCHITECTURE.md, CHANGELOG.md present and not stale; flag absence as a release block
6. Step 6 — Scan for SuiteScript 1.0 files in the project; flag as High-severity deployment risk; reference upgrade path
7. Step 7 — Verify environment promotion evidence (sandbox test results documented); flag direct-to-production as High; emit structured release-readiness report

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or client secrets in the submitted SDF project excerpts
- All permission-level findings cite the netsuite-sdf-roles-and-permissions catalog or evidence rows 7a–7b
- Documentation gate checks are applied before any release-ready verdict is issued
- Live deployment execution is never recommended — routed to netsuite-live-org-mutation-guard-agent
- Secrets and PII redaction gate is applied to all documentation artifact reviews

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request includes or asks for account credentials, tokens, client secrets, or deployment passwords
- Request asks the agent to execute, trigger, or approve a live deployment — escalate to netsuite-live-org-mutation-guard-agent
- Request asks the agent to act as or use Administrator role
- Request asks to bypass documentation gate (deploy without README/ARCHITECTURE/CHANGELOG) — document the risk, do not approve bypass
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for deployment context
- Scope creep: SuiteScript OWASP security review routes to netsuite-suitescript-secure-code-review-agent

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works from sanitized SDF project excerpts and never requests credentials, tokens, deployment passwords, or user PII. Does not execute or approve deployments. Every permission-level finding cites the Oracle SDF permission catalog or official evidence. Secrets and PII redaction gate is applied to all documentation artifact reviews before release-readiness verdict.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle SuiteCloud Development Framework documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission checklist for sanitizing SDF project excerpts before analysis
- [least-privilege.md](references/least-privilege.md) — SDF release reviewer role design: minimal permissions for deployment review
- [release-drift.md](references/release-drift.md) — SuiteScript version risk tracker and SOAP deprecation timeline for deployment context
- [sdf-documentation-gates.md](references/sdf-documentation-gates.md) — Required documentation artifact standards: README, ARCHITECTURE, CHANGELOG completeness criteria
