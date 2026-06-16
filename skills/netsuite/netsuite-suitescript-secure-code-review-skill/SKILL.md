---
name: netsuite-suitescript-secure-code-review-skill
description: "Flashlight skill for static security review of SuiteScript 2.x code against OWASP Top 10 (2021) pitfall patterns (OSCP-001 through OSCP-048), extended with Vanguard severity taxonomy mapping and CI pipeline gate recommendations. Adapted from Oracle netsuite-owasp-secure-coding (UPL-1.0). T0 static review — no live account connection required. TRIGGER when: user submits SuiteScript 2.x code for security review, asks about SuiteQL injection prevention, output encoding in Suitelets or RESTlets, CSRF in SuiteScript, file upload security, RESTlet hardening, DOM XSS in client scripts, postMessage origin validation, or AI prompt-injection in SuiteScript. Trigger phrases: SuiteScript security review, OWASP SuiteScript, SuiteQL injection, XSS in Suitelet, RESTlet hardening, CSRF token SuiteScript, file upload SuiteScript, OSCP vulnerability, secure coding SuiteScript. DO NOT TRIGGER when: request is for SuiteScript 1.0 (recommend migration first), SuiteFlow workflow logic review (use netsuite-suiteflow-automation-agent), OAuth 2.0 authentication setup (use netsuite-sso-oauth-tba-agent), role and permission configuration (use netsuite-identity-access-role-permission-agent), or live code execution or deployment is required (use netsuite-live-org-mutation-guard-agent)."
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

# NetSuite SuiteScript Secure Code Review Skill

## Purpose

Reviews SuiteScript 2.x code for the 48 catalogued OWASP-mapped pitfalls (OSCP-001 through OSCP-048) from the Oracle netsuite-owasp-secure-coding upstream skill, extended with Vanguard severity taxonomy mapping, CI pipeline gate thresholds, and audit evidence artifact format. Covers SuiteQL parameterization, LDAP escaping, HTML context output encoding, CSP construction, file upload/download pipelines, RESTlet API hardening, and AI prompt-injection mitigations. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Developer submits SuiteScript 2.x code for pre-deployment security review
- CI pipeline gate triggers security scan on a pull request containing SuiteScript changes
- Security team needs OWASP-mapped findings report for a SuiteScript codebase audit
- Compliance team needs audit evidence artifacts for a SuiteScript change-management workflow

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request SuiteScript 2.x source files (no credentials), script type declaration, external input surface list, and custom module paths
2. Step 2 — Injection surface mapping: identify all points where external input enters SuiteQL queries, LDAP calls, or dynamic string construction; map to OSCP injection pitfall IDs
3. Step 3 — Output encoding review: check all Suitelet and RESTlet response construction for correct HTML context encoding across body, attribute, JavaScript, CSS, and URL contexts
4. Step 4 — CSP and CSRF review: verify Content-Security-Policy header presence in RESTlet/Suitelet responses; verify CSRF token presence in state-changing operations
5. Step 5 — File and API hardening: review file upload MIME validation, path traversal controls, RESTlet authentication enforcement, and error response sanitization
6. Step 6 — Client-side and AI safety: check for DOM XSS patterns (innerHTML, document.write), postMessage origin validation gaps, and AI prompt-injection mitigations
7. Step 7 — Emit findings report: each finding maps to an OSCP pitfall ID (or [VANGUARD-EXTENDED]), rated Critical / High / Medium / Low with CI gate recommendation (block / warn / allow) and remediation guidance

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized source code files
- No hardcoded credentials, API keys, consumer keys, or OAuth secrets in submitted code — refuse and instruct sanitization if found
- Administrator role is never recommended as a script run-as or deployment role
- Every finding maps to an OSCP pitfall ID or is explicitly labeled [VANGUARD-EXTENDED]
- CI gate recommendation (block / warn / allow) accompanies every finding
- AI prompt-injection risks are flagged separately and escalated to netsuite-ai-foundations-agent

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Submitted code contains hardcoded credentials, API keys, consumer keys, OAuth client secrets, or passwords — stop and instruct sanitization before resubmitting
- Request involves executing, deploying, or activating any SuiteScript in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role is an appropriate run-as or deployment role for SuiteScript — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of AI Specialist or AI Professional certifications as available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized SuiteScript source code; never requests or accepts credentials, tokens, consumer keys, client secrets, or any authentication material embedded in code. Does not execute, deploy, or connect to any NetSuite account. Refuses code submissions containing hardcoded secrets. All findings are rated with CI gate recommendations and structured as audit evidence artifacts. Administrator role is never recommended for script deployment or run-as configuration.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle netsuite-owasp-secure-coding upstream skill URL and NetSuite developer documentation URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for SuiteScript code files
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance for SuiteScript security reviewer posture derived from Developer standard role
- [release-drift.md](references/release-drift.md) — NetSuite release cadence notes for SuiteScript API changes and OWASP catalog updates
- [oscp-vanguard-severity-map.md](references/oscp-vanguard-severity-map.md) — Mapping of OSCP-001 through OSCP-048 pitfall IDs to Vanguard severity taxonomy and CI gate recommendations
