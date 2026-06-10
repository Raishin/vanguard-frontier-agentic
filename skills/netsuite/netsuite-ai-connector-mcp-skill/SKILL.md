---
name: netsuite-ai-connector-mcp-skill
description: "Flashlight skill for reviewing the security and governance posture of NetSuite AI Connector Service (MCP) deployments. Verifies exact required permissions ('MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens' — not 'Log in using Access Tokens'), confirms the connecting role is never Administrator, validates explicit tool allowlists, checks HIPAA/BAA restrictions, and reviews prompt-injection testing coverage. T0 static review — no live account connection required. Depends on Oracle upstream skill netsuite-ai-connector-instructions (UPL-1.0) for tool-selection decision trees and SuiteQL safety patterns. TRIGGER when: user asks to review AI Connector configuration, audit MCP permissions, check AI agent access to NetSuite, validate tool allowlists, review prompt injection mitigations for NetSuite AI sessions, or assess AI Connector role setup. Trigger phrases: AI Connector, MCP Server Connection, NetSuite AI Service, MCP governance, tool allowlist, prompt injection NetSuite, AI agent permissions, OAuth 2.0 Access Tokens permission. DO NOT TRIGGER when: the user needs general OAuth/TBA authentication review beyond AI Connector permissions (use netsuite-sso-oauth-tba-skill), SuiteQL query design outside AI Connector context (use netsuite-web-services-integration-skill), broader REST/SOAP integration architecture (use netsuite-integration-migration-skill), or general role/permission assignment (use netsuite-identity-access-role-permission-skill)."
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

# NetSuite AI Connector MCP Skill

## Purpose

Audits the NetSuite AI Connector Service setup for correct role/permission configuration (NOT Administrator; exactly 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens'), explicit tool allowlists, HIPAA/BAA restriction compliance, and prompt-injection safeguards. Combines Vanguard harness governance (routing, logging, retry) with Oracle upstream AI connector guardrails. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Auditing the role and permissions used by an AI Connector Service integration to confirm it is not the Administrator role
- Verifying the two exact required permissions are present: 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens'
- Reviewing whether explicit tool allowlists are defined and scoped to the minimum operations needed by the AI session
- Checking prompt-injection testing coverage for an AI Connector integration
- Assessing whether a healthcare account with a signed BAA is attempting AI Connector activation (blocked by Oracle policy)

## Recommended Workflow

1. Step 1 — Load Oracle upstream skill: read and follow netsuite-ai-connector-instructions (DEPENDENCY, UPL-1.0) for tool-selection decision trees, SuiteQL safety checklist, and SafeWords principles before proceeding
2. Step 2 — Gather inputs: request AI Connector role permission list, feature enablement status, tool allowlist config, prompt-injection test plan, and account type indicator
3. Step 3 — Role check: verify the AI Connector role is not the Administrator role and does not have full account permissions (evidence row 6a); any violation is Critical
4. Step 4 — Permission check: confirm exactly 'MCP Server Connection' (6b) and 'Log in using OAuth 2.0 Access Tokens' (6c) are present; flag confusion with 'Log in using Access Tokens' as a finding
5. Step 5 — Feature check: confirm Server SuiteScript, OAuth 2.0 enabled; confirm REST Web Services enabled if MCP Standard Tools SuiteApp is used (6d)
6. Step 6 — Tool allowlist and prompt-injection review: assess explicitness of tool allowlist; assess presence and coverage of prompt-injection test cases; flag gaps as High
7. Step 7 — HIPAA/BAA check: if account is healthcare with signed BAA, flag as Critical (6e); emit structured findings report with verdict, findings table, safe next actions, and escalation triggers

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite credentials, tokens, or session cookies accepted
- AI Connector role must never be Administrator or full-permission role — Critical if present
- Exact permission names must match verbatim: 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens'
- HIPAA/BAA healthcare accounts must not activate AI Connector — Critical if attempted
- Tool allowlist must be explicit — implicit full access is a High finding
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full permissions to access NetSuite features for AI Connector configuration (evidence row 6a)
- Request asks the agent to directly activate, modify, or disable the AI Connector Service in a live account
- Request uses 'Log in using Access Tokens' instead of 'Log in using OAuth 2.0 Access Tokens' and asserts they are equivalent — they are NOT equivalent (evidence row 6c); flag and correct
- Request claims AI Specialist or AI Professional certifications are currently available — they are COMING SOON only (evidence rows 1b, AI track)
- Request attempts to configure the AI Connector for a healthcare account with a signed BAA — blocked by Oracle policy (evidence row 6e)

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. This agent never requests, stores, echoes, or logs NetSuite credentials, OAuth tokens, TBA tokens, client secrets, or session cookies. The AI Connector role reviewed must never be the Administrator role (evidence row 6a). Exact permission names are critical: 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens' (evidence rows 6b, 6c). HIPAA/BAA healthcare accounts cannot use the AI Connector (evidence row 6e). All live-mutation paths are hard-routed to netsuite-live-org-mutation-guard-agent. No org connection is established at any point.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite AI Connector Required Features and Permissions page URLs (evidence rows 6a-6e)
- [safety-checklist.md](references/safety-checklist.md) — Pre-review checklist for AI Connector role, permissions, features, and HIPAA/BAA status
- [least-privilege.md](references/least-privilege.md) — Custom reviewer role and AI Connector role minimum-permission specifications
- [release-drift.md](references/release-drift.md) — AI Connector feature changes by NetSuite release that may affect MCP governance posture
- [prompt-injection-patterns.md](references/prompt-injection-patterns.md) — Reference patterns for prompt-injection testing and SafeWords mitigations in NetSuite AI Connector sessions
