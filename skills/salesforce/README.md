# Salesforce Domain Skills

This directory contains 9 domain skills for Salesforce platform review and assessment.
All skills in this portfolio operate as read-only, static-review disciplines:
they accept sanitized exports and design documents, never request live org
credentials or API access, and produce advisory findings that require human
authorization before action.

Provider: `salesforce`
Lifecycle: `experimental`
Author: `github: Raishin`

---

## Skills

### 1. salesforce-org-assessment-skill
**Category:** platform

Structured workflow for assessing a Salesforce org's overall posture from
sanitized exports. Covers object model, automation inventory, permission
topology, integration map, and technical debt indicators. Produces a risk
register and remediation backlog.

Use when an org needs a health baseline, is being evaluated for acquisition,
or requires a documented posture assessment before a major release or migration.

---

### 2. salesforce-metadata-review-skill
**Category:** platform

Reviews pasted or exported Salesforce metadata (objects, fields, layouts,
Lightning record pages, profiles, permission sets, sharing rules) for
over-customization, unused fields, hardcoded IDs, and deprecated metadata types.

Use when a metadata export needs review before a release or as part of a
technical debt assessment.

---

### 3. salesforce-permission-model-review-skill
**Category:** security

Reviews profiles, permission sets, permission set groups, permission set
licenses, muting permission sets, sharing rules, OWD, role hierarchy, IP
restrictions, and session policies. Flags toxic combinations including
ModifyAllData with broad assignment, ViewAllData on PII, API Enabled without
IP restriction, and Customize Application outside admin profiles.

Use when a permission audit is required, a new permission model is being
designed, or guest-user access patterns need security review.

---

### 4. salesforce-flow-automation-review-skill
**Category:** platform

Reviews Flow XML, validation rules, approval processes, and record-triggered
automation for recursion, ungoverned bypass flags, brittle null handling,
missing fault paths, hardcoded recipients, before-save vs after-save misuse,
and mixed Process Builder plus Flow plus Apex on the same object.

Use when automation is being reviewed before production activation or when
unexpected automation behavior needs investigation.

---

### 5. salesforce-apex-lwc-code-review-skill
**Category:** security

Reviews Apex classes, triggers, Lightning Web Components (LWC), and async
jobs (Queueable, Batch, Future, Schedulable) for SOQL and DML inside loops,
missing test coverage patterns, WITH SECURITY_ENFORCED and stripInaccessible
usage, sharing keyword omission, governor-limit risk, LWC XSS surface, and
Locker Service issues.

Use when Apex or LWC code must be reviewed before a release or when a
security concern has been raised about code.

---

### 6. salesforce-release-readiness-skill
**Category:** delivery

Pre-release checklist assessment covering sandbox refresh strategy, source
tracking state, package version diff, destructiveChanges.xml review, test
coverage threshold, post-deploy steps, rollback plan, communications plan,
and approval matrix.

Use when a release is being prepared and must be evaluated for deployment
readiness before handing off to the live-change approval gate.

---

### 7. salesforce-integration-review-skill
**Category:** architecture

Reviews integration designs for API choice (REST, SOAP, Bulk, Streaming,
Platform Events, CDC), middleware position, retry and idempotency patterns,
error queue design, observability, secret handling, OAuth scope minimization,
named credential vs callout patterns, and MuleSoft vs point-to-point
architecture.

Use when an integration is being designed or audited for security and
reliability.

---

### 8. salesforce-marketing-consent-review-skill
**Category:** compliance

Reviews marketing data flows (Marketing Cloud, Account Engagement, Data Cloud)
for consent capture, lawful basis, purpose limitation, preference center
coverage, suppression list integrity, subscriber-key collision risk,
deliverability authentication (SPF, DKIM, DMARC), and unsubscribe link
integrity.

Use when a marketing data flow must be reviewed for privacy compliance or
when a deliverability issue may be related to authentication configuration.

---

### 9. salesforce-agentforce-risk-review-skill
**Category:** ai

Reviews Agentforce and Salesforce AI agent configurations for grounding
quality, retrieval scope, action allowlist safety, human handoff design,
hallucination containment, prompt injection surface, autonomous action
boundary, audit logging, and model-risk controls. All Agentforce and Einstein
feature names carry verify-before-merge annotations.

Use when an AI agent configuration is being reviewed before deployment or
when an AI agent has taken an unexpected action.

---

## Companion protocol skills

These 5 cross-functional protocol skills (in `skills/cross-functional/`) govern
how Salesforce matters are classified, routed, and handed off:

| Protocol skill | Purpose |
|---|---|
| `salesforce-routing-protocol` | Classification and routing discipline for Salesforce matters |
| `salesforce-case-capsule` | Standardized cross-agent handoff structure |
| `salesforce-risk-taxonomy` | Matter types, risk tiers, and escalation gates |
| `salesforce-live-change-approval-protocol` | Refusal-by-default gate for live org mutations |
| `salesforce-data-exposure-escalation-protocol` | Immediate escalation path for data exposure events |

---

## Security and operating principles

- All skills are read-only, static-review disciplines.
- No skill requests live org credentials, session IDs, OAuth tokens, or
  customer data.
- All inputs must be sanitized before submission; org IDs and user IDs must
  be replaced with placeholders.
- Advisory findings require human authorization before any remediation action.
- Regulated-vertical findings (HIPAA, PCI, FINRA) are always escalated to
  qualified compliance counsel.
- Escalation gates from `salesforce-risk-taxonomy` are hard stops, not
  suggestions.
