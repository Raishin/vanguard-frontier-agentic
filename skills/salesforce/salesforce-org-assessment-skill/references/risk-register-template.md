# Risk Register Template Reference

Template for documenting findings during Salesforce org assessments with
severity tiering, matter taxonomy, and remediation priority guidance.

---

## Finding Record Structure

Each finding should be recorded with the following fields:

| Field | Type | Description |
|-------|------|-------------|
| Finding ID | Text | Sequential identifier (e.g., `SF-2026-001`) |
| Title | Text | Short descriptive title (< 80 characters) |
| Matter Type | Picklist | Taxonomy category (see below) |
| Domain | Picklist | Assessment domain |
| Severity | Picklist | CRITICAL / HIGH / MEDIUM / LOW / INFO |
| CVSS-Like Score | Number 1-10 | Risk scoring |
| Affected Component | Text | Object, field, class, or feature name |
| Evidence | Long Text | How the finding was identified |
| Impact | Long Text | What could happen if not remediated |
| Recommendation | Long Text | Specific remediation steps |
| Effort to Remediate | Picklist | Days / Weeks / Months |
| Remediation Owner | Text | Team or individual responsible |
| Target Resolution Date | Date | Based on severity SLA |
| Status | Picklist | Open / In Progress / Remediated / Accepted Risk |
| Accepted Risk Justification | Long Text | Required if Status = Accepted Risk |

---

## Matter Type Taxonomy

Consistent classification enables trend analysis across multiple assessments.

| Code | Matter Type | Description |
|------|-------------|-------------|
| ACC | Access Control | Permissions, profiles, permission sets, sharing |
| AUTH | Authentication | MFA, session security, login controls |
| DATA | Data Exposure | Sensitive data accessible to unauthorized users |
| CODE | Code Quality | Apex/LWC security or performance issues |
| INT | Integration Security | Connected app, OAuth, Named Credential issues |
| FLOW | Automation | Flow, Process Builder, Workflow Rule issues |
| COMP | Compliance | GDPR, CCPA, HIPAA, CASL, SOC 2 alignment |
| CONF | Configuration | Security settings, network access, CSP |
| DEPR | Deprecated Technology | Legacy automation, end-of-life APIs |
| DEBT | Technical Debt | Code quality, field bloat, unused metadata |
| MON | Monitoring | Audit trail, alerting, observability |
| RESIL | Resilience | Error handling, retry, fault paths |

---

## Severity Definitions

### CRITICAL

Immediate exploitation risk or active data exposure. Requires emergency response.

Examples:
- `ModifyAllData` permission granted to integration user with no IP restriction.
- SOQL injection vulnerability in public-facing REST endpoint.
- PII field accessible via Guest User profile.
- OAuth Username-Password Flow enabled and in use in production.

**Remediation SLA:** Within 48 hours of identification.

### HIGH

Significant security risk or compliance gap. Requires priority remediation.

Examples:
- System Administrator profile assigned to > 10% of active users.
- No MFA enforcement for internal users.
- Callout credentials hardcoded in Apex source.
- Shield Encryption not configured for regulated PII fields.
- Active Workflow Rules with no migration plan.

**Remediation SLA:** Within 14 business days.

### MEDIUM

Notable risk or quality concern. Should be remediated in next planning cycle.

Examples:
- Custom object OWD set to Public Read/Write without documented rationale.
- Named Credential exists without credential rotation schedule.
- Flow missing fault path on a non-critical email action.
- Custom field count approaching 100 on Account.

**Remediation SLA:** Within 90 days.

### LOW

Minor hygiene or best-practice deviation. Address in next cleanup sprint.

Examples:
- Custom fields lacking Description text.
- Apex classes missing ApexDoc comments on public methods.
- Inactive flow versions not cleaned up.

**Remediation SLA:** Next scheduled maintenance window.

### INFO

Informational observation. No immediate action required.

Examples:
- Org is running Salesforce Classic for 5% of users (monitoring recommended).
- Third-party managed package 1 minor version behind (monitor for update).

**Remediation SLA:** Note for next assessment.

---

## Risk Scoring Matrix (CVSS-Like)

Score = (Impact × 0.6) + (Likelihood × 0.4)

Where:
- **Impact:** 1 (minimal) to 5 (catastrophic)
- **Likelihood:** 1 (theoretical) to 5 (actively exploited)

| Score | Severity |
|-------|---------|
| 9-10 | CRITICAL |
| 7-8.9 | HIGH |
| 4-6.9 | MEDIUM |
| 1-3.9 | LOW |

---

## Finding Record Template

```
FINDING ID: SF-YYYY-NNN
TITLE: [Short descriptive title]
MATTER TYPE: [ACC / AUTH / DATA / CODE / INT / FLOW / COMP / CONF / DEPR / DEBT / MON / RESIL]
DOMAIN: [Access and Identity / Object Model / Automation / Integrations / Data Compliance]
SEVERITY: [CRITICAL / HIGH / MEDIUM / LOW / INFO]
RISK SCORE: [1-10]

AFFECTED COMPONENT:
[Object name, field API name, Apex class, Flow name, setting path]

EVIDENCE:
[How was this identified? Query results, screenshot reference, file path]

Example query result:
  SELECT COUNT(Id) FROM User WHERE Profile.Name = 'System Administrator'
    AND IsActive = true
  Result: 47 (expected: < 5)

IMPACT:
[What risk does this create? Data exposure, compliance gap, reliability issue]

RECOMMENDATION:
[Specific steps to remediate. Reference official documentation or this skill's
references/ files where relevant]

Step 1: [Action]
Step 2: [Action]
Step 3: [Verification]

EFFORT TO REMEDIATE: [< 1 day / 1-3 days / 1-2 weeks / > 2 weeks]
REMEDIATION OWNER: [Team or role]
TARGET RESOLUTION DATE: [Based on severity SLA from identification date]
STATUS: Open
```

---

## Sample Completed Finding

```
FINDING ID: SF-2026-001
TITLE: OAuth Username-Password Flow Enabled in Production
MATTER TYPE: AUTH
DOMAIN: Integrations and Callouts
SEVERITY: CRITICAL
RISK SCORE: 9.2

AFFECTED COMPONENT:
Setup > Apps > Connected Apps > [App Name] > Edit OAuth Policies
Setting: Permitted Users = "All users may self-authorize"
OAuth Flow: Username-Password enabled

EVIDENCE:
Reviewed Connected App OAuth policy configuration.
Username-Password Flow was confirmed active.
4 active integrations were identified using this flow pattern.

IMPACT:
The Username-Password OAuth flow transmits credentials in the API request body.
This bypasses MFA enforcement. A credential leak exposes production org access
without any additional authentication factor. This also violates OAuth 2.1
deprecation of the resource owner password flow.

RECOMMENDATION:
Step 1: Audit all integrations using Username-Password flow (grep integration
        code for grant_type=password).
Step 2: Migrate each integration to JWT Bearer Flow (server-to-server) or
        Authorization Code Flow with PKCE (user-facing).
Step 3: Disable Username-Password Flow in the Connected App OAuth policy.
Step 4: Rotate all credentials that were used in the Username-Password flow.
Step 5: Verify no new logins via Username-Password flow in Login History.

EFFORT TO REMEDIATE: 1-2 weeks (per integration)
REMEDIATION OWNER: Integration Platform Team
TARGET RESOLUTION DATE: [Identification date + 14 business days]
STATUS: Open
```
