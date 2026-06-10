---
name: netsuite-data-governance-privacy-skill
description: "Flashlight skill for auditing PII exposure paths, data retention and purge policies, field-level access restrictions, privacy controls, and export configurations in NetSuite. T0 static review — no live account connection or actual personal data required. TRIGGER when: user asks to review PII field access, audit data retention settings, check field-level security on sensitive records, assess privacy controls, identify PII in saved searches, review export control permissions, or evaluate GDPR/CCPA readiness of a NetSuite configuration. Trigger phrases: PII exposure, field-level security, data retention policy, GDPR compliance, personal data access, export controls, consent tracking, sensitive field access. DO NOT TRIGGER when: the user needs role and permission architecture review beyond PII fields (use netsuite-identity-access-role-permission-skill), SOX audit trail review (use netsuite-audit-controls-sox-skill), integration data-flow security (use netsuite-integration-migration-skill), subsidiary data segregation (use netsuite-oneworld-multisubsidiary-skill), or SuiteScript code PII handling review (use netsuite-suitescript-secure-code-review-skill)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: compliance
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Data Governance & Privacy Skill

## Purpose

Audits NetSuite configurations for PII field exposure, data retention and purge policy coverage, field-level access restrictions on sensitive records, export control enforcement for cross-border data flows, and privacy-relevant saved search and report scoping. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- Auditing which roles and saved searches expose PII fields on employee, customer, contact, or vendor records
- Reviewing data retention and purge policy coverage for GDPR, CCPA, or other regulatory requirements
- Assessing field-level access restrictions on sensitive fields such as SSN, bank account numbers, and credit card data
- Identifying PII exposed in scheduled reports or saved searches distributed to external partners or vendor-center roles
- Reviewing export control configurations to assess mass-export and CSV-export permission scoping on PII records

## Recommended Workflow

1. Step 1 — Gather inputs: request role configuration excerpts for PII-bearing records, saved search audience configs, data retention policy, and export control permission settings
2. Step 2 — Map PII fields: identify all PII-bearing fields on employee, customer, contact, and vendor records based on provided configuration; flag any field with no field-level security as a finding
3. Step 3 — Review field-level access: for each PII field, assess which roles have View access and whether that access is operationally justified; flag over-broad access as High
4. Step 4 — Audit saved searches and reports: identify any search or report including PII fields distributed to roles or audiences beyond operational need; flag as High or Critical
5. Step 5 — Assess data retention coverage: map configured retention periods to regulatory requirements; flag missing or zero retention configuration as Critical
6. Step 6 — Review export controls: assess mass-update, CSV-export, and file-cabinet-access permissions on PII records; flag roles with export capability and no documented justification as High
7. Step 7 — Emit structured findings report: verdict, Critical/High/Medium/Low findings table, safe next actions, and escalation triggers

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No actual personal data (real names, SSNs, emails, phone numbers, bank data) accepted — reject and ask for sanitized or synthetic examples
- No live NetSuite credentials, tokens, or session cookies accepted
- View Unencrypted Credit Cards and View Unencrypted ACH Account Numbers permissions are never recommended for any reviewer role
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference
- Any PII exposure to roles with no operational need rated High minimum; exposure to external parties rated Critical

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request provides actual personal data (real names, SSNs, email addresses, phone numbers, bank account numbers, or healthcare data) — refuse immediately, do not log or echo, ask for sanitized version
- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete field-security configurations, retention policies, or consent records in a live account
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. This agent never accepts, stores, echoes, or processes actual personal data. All inputs containing real PII are refused. No live NetSuite credentials, OAuth tokens, TBA tokens, or session cookies are accepted. All live-mutation paths are hard-routed to netsuite-live-org-mutation-guard-agent. No org connection is established at any point.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle NetSuite roles, permissions, and field-security documentation URLs
- [safety-checklist.md](references/safety-checklist.md) — Pre-review sanitization requirements for PII-bearing configuration exports
- [least-privilege.md](references/least-privilege.md) — Custom reviewer role specification for data governance review
- [release-drift.md](references/release-drift.md) — NetSuite privacy and data retention feature changes by release
- [pii-field-catalog.md](references/pii-field-catalog.md) — Reference catalog of standard NetSuite PII-bearing fields across employee, customer, contact, and vendor record types
