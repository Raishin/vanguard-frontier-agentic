---
name: netsuite-ai-foundations-skill
description: "Flashlight skill for reviewing NetSuite AI feature enablement and AI Connector Service configuration posture, aligned to the AI Foundations Associate certification (N16765GC10, available). T0 static review — no live account connection required. NOTE: AI Specialist and AI Professional certifications are COMING SOON and are not yet available; this skill does not cover those levels. TRIGGER when: user asks to review NetSuite AI feature enablement (bill matching, anomaly detection, text enhancement, predicted risk), AI Connector Service configuration, MCP Server Connection permission setup, OAuth 2.0 Access Tokens permission for AI roles, HIPAA/BAA restriction for healthcare accounts, or AI governance and PII exposure controls. Trigger phrases: AI Foundations review, AI Connector configuration, MCP Server Connection permission, NetSuite AI features, AI bill matching, AI anomaly detection, HIPAA AI restriction, AI governance review, Log in using OAuth 2.0 Access Tokens. DO NOT TRIGGER when: request is about AI Connector MCP tool execution or SuiteQL query safety (use netsuite-ai-connector-mcp-agent); OAuth 2.0 authentication setup beyond the AI role (use netsuite-sso-oauth-tba-agent); SuiteScript code security (use netsuite-suitescript-secure-code-review-agent); or live account mutation is required (use netsuite-live-org-mutation-guard-agent). Never assert that AI Specialist or AI Professional certifications are available — they are COMING SOON."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: ai
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite AI Foundations Skill

## Purpose

Validates that NetSuite AI feature configurations and AI Connector setup follow least-privilege, zero-trust, and data-governance principles aligned to the AI Foundations Associate certification (N16765GC10, available). AI Specialist and AI Professional certifications are COMING SOON and are not available; this agent does not claim alignment to those levels. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User submits AI feature enablement screenshots for governance review
- Implementation team needs AI Connector custom role validated against least-privilege requirements (not Administrator, MCP Server Connection + Log in using OAuth 2.0 Access Tokens)
- Healthcare customer needs HIPAA/BAA restriction check before enabling AI Connector
- CoE architect needs AI foundations governance posture reviewed for enterprise deployment

## Recommended Workflow

1. Step 1 — Collect sanitized inputs: request AI feature enablement screenshot, AI Connector custom role permission export, healthcare/BAA status confirmation, and Server SuiteScript/OAuth 2.0 feature flag status
2. Step 2 — HIPAA/BAA gate: if the account is a healthcare customer with a signed BAA, immediately flag AI Connector activation as blocked (Critical finding); do not proceed with activation advice
3. Step 3 — AI feature enablement review: validate which AI features are enabled (bill matching, anomaly detection, text enhancement, predicted risk); flag any feature enabled without a corresponding governance control
4. Step 4 — AI Connector role validation: confirm the custom role is not Administrator, holds MCP Server Connection and Log in using OAuth 2.0 Access Tokens permissions (not 'Log in using Access Tokens'), and has 2FA designation
5. Step 5 — Feature flag verification: confirm Server SuiteScript, OAuth 2.0, and (if applicable) REST Web Services are enabled before AI Connector can operate
6. Step 6 — PII exposure review: identify which record types and fields are accessible via AI features; flag any sensitive fields (SSN, bank account, credit card) exposed without masking
7. Step 7 — Emit findings report: rated Critical / High / Medium / Low with [FACT] / [INFERENCE] / [ASSUMPTION] labels and safe-next-actions

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Never claim AI Specialist or AI Professional certification availability — both are COMING SOON
- AI Connector role is never Administrator and never holds full module permissions
- HIPAA/BAA restriction is checked before any AI Connector enablement advice is given
- Log in using OAuth 2.0 Access Tokens permission is distinguished from Log in using Access Tokens (evidence-matrix row 6c)

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, activating AI features, or modifying role permissions in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Request to assert AI Specialist or AI Professional certification as available — those are COMING SOON; refuse with explicit citation of evidence-matrix row 1b
- Claim that the Administrator role can be used for AI Connector — refuse; evidence-matrix row 6a explicitly prohibits Administrator or full-permissions roles for AI Connector

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only — works exclusively from sanitized configuration excerpts; never requests or accepts credentials, tokens, consumer keys, client secrets, or any authentication material. Does not connect to, query, or mutate any NetSuite account. AI Connector role must never be Administrator; required permissions are MCP Server Connection and Log in using OAuth 2.0 Access Tokens only. HIPAA/BAA restriction for healthcare customers is a hard gate. AI Specialist and AI Professional certifications are COMING SOON — never claimed as available.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite AI Foundations Associate exam URL and AI Connector documentation URLs verified in evidence-matrix
- [safety-checklist.md](references/safety-checklist.md) — Pre-submission sanitization checklist for AI feature and AI Connector configuration exports
- [least-privilege.md](references/least-privilege.md) — AI Connector custom role construction guidance — MCP Server Connection + Log in using OAuth 2.0 Access Tokens, never Administrator
- [release-drift.md](references/release-drift.md) — NetSuite release cadence notes for AI feature changes and AI Connector updates
- [ai-foundations-cert-status.md](references/ai-foundations-cert-status.md) — Certification availability status — AI Foundations Associate available; AI Specialist and AI Professional COMING SOON
