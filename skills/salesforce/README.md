# Salesforce Skills

<p align="center">
  <img src="../../assets/logos/cloud/salesforce/salesforce.svg" alt="Salesforce logo" width="200" />
</p>

This directory contains **25** Salesforce domain skills curated for this marketplace, spanning four execution tiers and four delivery waves.

Provider: `salesforce`
Lifecycle: `experimental`
Author: `github: Raishin`

---

## Execution tiers

Skills in this portfolio are classified by the [execution-tier contract](../../docs/execution-tiers.md):

| Tier | Label | Blast radius |
|---|---|---|
| T0 | `static-review` | Zero — no network egress, operates on local files and pasted exports only |
| T1 | `read-only-runtime` | Bounded — read-only CLI access to a connected org; outputs sanitized |
| T2 | `sandbox-mutating` | Scoped — mutation allowed only in SANDBOX orgs; production hard-refused |
| T3 | `production-mutating` | **Prohibited for all agents** — routed through Live Guard and human approval |

---

## Wave 1 — Static review (T0): core advisory skills

Nine foundational review disciplines. No network access; all inputs are sanitized exports or pasted configuration.

| Skill | Tier | Summary |
|---|---|---|
| `salesforce-org-assessment-skill` | `static-review` | Org posture baseline from sanitized exports — object model, automation inventory, permission topology, integration map, technical debt |
| `salesforce-metadata-review-skill` | `static-review` | Reviews metadata (objects, fields, layouts, LWC record pages, profiles, permission sets, sharing rules) for over-customization and deprecated types |
| `salesforce-permission-model-review-skill` | `static-review` | Audits profiles, permission sets, permission set groups, sharing rules, OWD, role hierarchy, IP restrictions, and session policies; flags toxic combinations |
| `salesforce-flow-automation-review-skill` | `static-review` | Reviews Flow XML, validation rules, approval processes, and record-triggered automation for recursion, ungoverned bypass, brittle null handling, and governor-limit risk |
| `salesforce-apex-lwc-code-review-skill` | `static-review` | Reviews Apex classes, triggers, LWC, and async jobs for SOQL/DML in loops, sharing keyword omission, governor-limit risk, LWC XSS surface, and Locker Service issues |
| `salesforce-release-readiness-skill` | `static-review` | Pre-release checklist covering sandbox refresh strategy, test coverage threshold, destructiveChanges.xml review, rollback plan, and approval matrix |
| `salesforce-integration-review-skill` | `static-review` | Reviews integration designs for API choice, middleware position, retry/idempotency patterns, secret handling, OAuth scope minimization, and MuleSoft architecture |
| `salesforce-marketing-consent-review-skill` | `static-review` | Reviews Marketing Cloud, Account Engagement, and Data Cloud flows for consent capture, lawful basis, suppression list integrity, and deliverability authentication |
| `salesforce-agentforce-risk-review-skill` | `static-review` | Reviews Agentforce and Salesforce AI configurations for grounding quality, action allowlist safety, human handoff design, hallucination containment, and model-risk controls
|

---

## Wave 2 — Infrastructure and zero-trust review (T0)

Four specialized review disciplines for infrastructure security and DevSecOps posture.

| Skill | Tier | Summary |
|---|---|---|
| `salesforce-zero-trust-maturity-skill` | `static-review` | Evaluates Salesforce deployment zero-trust readiness against NIST SP 800-207 — MFA posture, network policies, least-privilege identity, continuous verification |
| `salesforce-infrastructure-audit-skill` | `static-review` | Structured audit of Salesforce infrastructure security posture including Hyperforce configuration, network policies, certificate management, and sandbox governance
|
| `salesforce-devsecops-pipeline-skill` | `static-review` | Reviews Salesforce CI/CD pipeline configurations — SFDX/Salesforce CLI usage, dependency scanning, secrets handling, deployment gating, and branch protection |
| `salesforce-soql-explorer-skill` | `static-review` | Analyzes SOQL query patterns from pasted query text for bulkification, index usage, LIMIT/OFFSET anti-patterns, and sharing enforcement |

---

## Wave 3 — Generation skills (T0)

Six code and artifact generation skills. All generate output locally from conversation context; none execute against a live org.

| Skill | Tier | Summary |
|---|---|---|
| `salesforce-soql-generator-skill` | `static-review` | Generates SOQL queries from plain-English requirements without executing against an org — includes WITH SECURITY_ENFORCED and field-level security annotations |
| `salesforce-apex-generator-skill` | `static-review` | Generates production-grade Apex classes with Service-Selector-Domain layering, CRUD/FLS enforcement, and governor-limit patterns |
| `salesforce-apex-test-generator-skill` | `static-review` | Generates Apex test classes with TestDataFactory patterns, Assert class usage, and bulkification test cases |
| `salesforce-validation-rule-writer-skill` | `static-review` | Converts plain-English business rules into deployable Salesforce Validation Rule formula syntax |
| `salesforce-field-mapping-skill` | `static-review` | Maps CSV and spreadsheet column headers to Salesforce field API names and resolves picklist values for data-load preparation |
| `salesforce-bulk-data-ops-skill` | `static-review` | Generates scripts for bulk Salesforce data operations — mass owner reassignment, archive extraction, and Data Loader job configuration |

---

## Wave 4 — Operational skills (T1 and T2)

Six skills with live connectivity, distinguishing this portfolio from pure static review. T1 skills are read-only; T2 skills require sandbox-only target confirmation and hard-refuse production org targets.

| Skill | Tier | Summary |
|---|---|---|
| `salesforce-metadata-fetcher-skill` | `read-only-runtime` | Fetches Salesforce metadata (objects, fields, flows, validation rules) from a connected org via `sf sobject describe` and metadata API reads — read-only |
| `salesforce-agentforce-stdm-observer-skill` | `read-only-runtime` | Queries Salesforce Telemetry and Data Management (STDM) and Data Cloud
event streams in read-only mode for Agentforce operational monitoring |
| `salesforce-apex-test-runner-skill` | `read-only-runtime` | Executes Apex tests against a connected sandbox org via `sf apex run test` — observes results only; does not deploy or modify org state |
| `salesforce-apex-log-analyzer-skill` | `read-only-runtime` | Retrieves Apex debug logs from a connected Salesforce org and analyzes them for governor-limit breaches, exceptions, and SOQL/DML patterns |
| `salesforce-flow-debugger-skill` | `read-only-runtime` | Diagnoses Salesforce Flow failures from pasted error messages (T0 mode) or live debug log retrieval (T1 mode) — read-only observation only |
| `salesforce-deployment-validator-skill` | `sandbox-mutating` | Runs `sf project deploy validate` against a SANDBOX org — validation only, no commit; hard-refuses production targets |

---

## Companion protocol skills

Five cross-functional protocol skills in `skills/cross-functional/` govern how Salesforce matters are classified, routed, and handed off:

| Protocol skill | Purpose |
|---|---|
| `salesforce-routing-protocol` | Classification and routing discipline for Salesforce matters |
| `salesforce-case-capsule` | Standardized cross-agent handoff structure |
| `salesforce-risk-taxonomy` | Matter types, risk tiers, and escalation gates |
| `salesforce-live-change-approval-protocol` | Refusal-by-default gate for live org mutations |
| `salesforce-data-exposure-escalation-protocol` | Immediate escalation path for data exposure events |

---

## Security and operating principles

- T0 skills are read-only, static-review disciplines — no network egress, no credentials.
- T1 skills require read-only OAuth scopes (`api refresh_token`) against a pre-authorized Connected App; outputs are sanitized before emission.
- T2 skills hard-refuse production org targets at the skill level; production targets are never agent-callable.
- No skill requests live org credentials, session IDs, OAuth tokens, or customer data in conversation context.
- All inputs must be sanitized before submission; org IDs and user IDs must be replaced with placeholders.
- Advisory findings require human authorization before any remediation action.
- Regulated-vertical findings (HIPAA, PCI, FINRA) are always escalated to qualified compliance counsel.
- Escalation gates from `salesforce-risk-taxonomy` are hard stops, not suggestions.

Run `npm run validate` after changing cataloged Salesforce skills.
