# Assessment Rubric Reference

100-point org posture scoring model across five domains for Salesforce
org-level health and security assessment.

---

## Scoring Model Overview

| Domain | Max Points | Weight |
|--------|-----------|--------|
| Access and Identity | 25 | 25% |
| Object Model and Metadata | 20 | 20% |
| Automation and Code Quality | 20 | 20% |
| Integrations and Callouts | 20 | 20% |
| Data Integrity and Compliance | 15 | 15% |
| **Total** | **100** | **100%** |

### Score Interpretation

| Score | Rating | Action |
|-------|--------|--------|
| 90-100 | Excellent | Monitor for drift; review annually |
| 75-89 | Good | Address MEDIUM findings within 90 days |
| 60-74 | Fair | Address HIGH findings within 30 days; 90-day remediation plan |
| 40-59 | Poor | Immediate remediation plan; executive escalation |
| < 40 | Critical | Emergency review; restrict access until remediated |

---

## Domain 1: Access and Identity (25 points)

### 1.1 Multi-Factor Authentication (6 points)

| Check | Points |
|-------|--------|
| MFA enforced for all active internal users | 3 |
| MFA enforced via org-wide policy (not just individual setting) | 2 |
| MFA required for API-access user accounts | 1 |

**Query:**
```sql
SELECT Id, Name, Profile.Name,
       (SELECT Id FROM UserPreferences WHERE PreferencesIsVerificationEnabled = false)
FROM User
WHERE IsActive = true
  AND UserType = 'Standard'
LIMIT 500
```

### 1.2 Admin Account Controls (7 points)

| Check | Points |
|-------|--------|
| System Administrator profile users < 5% of total active users | 2 |
| Named admin accounts (no shared/generic admin logins) | 2 |
| Admin accounts have login IP restrictions configured | 2 |
| Emergency access ("break glass") account documented and monitored | 1 |

**Query:**
```sql
SELECT COUNT(Id) adminCount
FROM User
WHERE Profile.Name = 'System Administrator'
  AND IsActive = true
```

### 1.3 Integration User Controls (5 points)

| Check | Points |
|-------|--------|
| Dedicated integration user accounts (not shared with humans) | 2 |
| Integration user profiles have only required permissions | 2 |
| Integration user credentials rotated within last 90 days | 1 |

### 1.4 Permission Model Health (7 points)

| Check | Points |
|-------|--------|
| Modify All Data users documented with business justification | 2 |
| View All Data users reviewed and minimized | 2 |
| Custom Permission Sets in use (not all-in-one profiles) | 2 |
| Guest User profile reviewed and minimal | 1 |

---

## Domain 2: Object Model and Metadata (20 points)

### 2.1 Custom Field Health (5 points)

| Check | Points |
|-------|--------|
| Account custom fields < 100 | 2 |
| Fields unused in 6+ months documented or removed | 2 |
| External ID fields present on objects synced with external systems | 1 |

### 2.2 Schema Design Quality (8 points)

| Check | Points |
|-------|--------|
| Master-detail vs lookup relationships correctly applied | 3 |
| No hardcoded IDs in custom fields' formula or default values | 2 |
| Record Types configured per distinct business process (not overloaded) | 2 |
| Custom object OWD is Private (or Controlled by Parent) | 1 |

### 2.3 Metadata Currency (7 points)

| Check | Points |
|-------|--------|
| No active Workflow Rules (all migrated to Flows) | 3 |
| No active Process Builder processes | 3 |
| No Apex on API version < 50.0 | 1 |

---

## Domain 3: Automation and Code Quality (20 points)

### 3.1 Apex Code Health (10 points)

| Check | Points |
|-------|--------|
| No SOQL in loops detected (PMD scan passing) | 3 |
| No DML in loops detected | 3 |
| All classes declare `with sharing` or `without sharing` (no implicit) | 2 |
| Test coverage > 75% org-wide | 2 |

**Query:**
```sql
SELECT PercentCovered
FROM ApexOrgWideCoverage
```

### 3.2 Flow Health (6 points)

| Check | Points |
|-------|--------|
| All record-triggered flows have fault paths on DML elements | 3 |
| No DML-in-loop patterns in active flows | 2 |
| Inactive flows cleaned up (no orphaned inactive versions in production) | 1 |

### 3.3 Governor Limit Exposure (4 points)

| Check | Points |
|-------|--------|
| No AsyncApexJob failures in the last 30 days | 2 |
| No LimitException events in debug logs in the last 7 days | 2 |

```sql
SELECT COUNT(Id)
FROM AsyncApexJob
WHERE Status = 'Failed'
  AND CreatedDate = LAST_N_DAYS:30
```

---

## Domain 4: Integrations and Callouts (20 points)

### 4.1 Named Credential Usage (8 points)

| Check | Points |
|-------|--------|
| All callouts use Named Credentials (no hardcoded endpoints) | 4 |
| No Remote Site Settings with DisableProtocolSecurity = true | 4 |

### 4.2 Connected App Security (7 points)

| Check | Points |
|-------|--------|
| OAuth Username-Password Flow disabled | 3 |
| All Connected Apps have IP Relaxation = "Enforce IP Restrictions" | 2 |
| OAuth token timeouts configured (not unlimited) | 2 |

### 4.3 Integration Resilience (5 points)

| Check | Points |
|-------|--------|
| Error handling exists on all `@AuraEnabled` callout methods | 2 |
| Platform Event or Custom Error Log used for callout failure tracking | 2 |
| Retry logic exists for transient failures | 1 |

---

## Domain 5: Data Integrity and Compliance (15 points)

### 5.1 Consent Management (5 points)

| Check | Points |
|-------|--------|
| Individual object linked to Contacts with email addresses | 2 |
| ContactPointConsent records present with CaptureDate populated | 2 |
| Opt-out synced between all connected marketing systems | 1 |

### 5.2 Data Residency and Encryption (5 points)

| Check | Points |
|-------|--------|
| Org on Hyperforce (if regulatory data residency required) | 2 |
| Shield Platform Encryption enabled for PII/PHI fields | 2 |
| Encryption key rotation schedule documented | 1 |

### 5.3 Audit and Monitoring (5 points)

| Check | Points |
|-------|--------|
| Setup Audit Trail exported to SIEM or long-term storage | 2 |
| Login History retention beyond 6 months configured | 2 |
| Automated monitoring alert for admin login from new IP | 1 |

---

## Scoring Worksheet

```
Domain 1 Score: ___ / 25
Domain 2 Score: ___ / 20
Domain 3 Score: ___ / 20
Domain 4 Score: ___ / 20
Domain 5 Score: ___ / 15

Total Score:    ___ / 100

Rating: _________________

Assessment Date: ____________
Assessor: ____________
Next Review Date: ____________
```
