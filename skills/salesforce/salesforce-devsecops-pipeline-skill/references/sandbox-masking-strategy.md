# Sandbox Masking Strategy Reference

Guidelines for data masking in Salesforce sandboxes to protect PII, PHI, and
regulated data when refreshing from production.

---

## Why Masking Is Required

Sandbox environments are used by developers, QA engineers, ISV partners, and
consultants who typically should not access production customer data. Without
masking:

- PII violations under GDPR, CCPA, HIPAA, and similar regulations.
- Breach notification obligations if sandbox data is exposed.
- SOC 2 Type II non-conformance in access control controls.

Salesforce Sandbox Data Masking <!-- verify-before-merge:2026-05-21 --> is a licensed add-on product
(part of Salesforce Shield or available separately) that automates masking
during sandbox refresh. Orgs without the add-on must implement a post-refresh
masking flow.

---

## Classify Data Before Masking

### Classification Tiers

| Tier | Description | Masking Requirement |
|------|-------------|---------------------|
| Public | No restriction | Not required |
| Internal | Business-sensitive, no regulatory constraint | Pseudonymization recommended |
| Confidential | PII (name + email + phone when combined) | Mask or anonymize |
| Restricted | Financial data, account numbers, SSN/NI | Must mask or delete |
| Regulated | PHI (HIPAA), SPI (CCPA), special categories (GDPR) | Must mask; prefer deletion |

### Common Salesforce Fields by Tier

| Object | Field | Classification |
|--------|-------|----------------|
| Contact | Email | Confidential |
| Contact | Phone, MobilePhone | Confidential |
| Contact | MailingStreet + MailingCity | Confidential (combined) |
| Contact | SSN__c (custom) | Restricted |
| Account | BillingAddress | Internal |
| Lead | Email, Phone | Confidential |
| Individual | IndividualId | Restricted |
| HealthCloudGA__EhrPatient__c | All fields | Regulated (PHI) |
| FinServ__FinancialAccount__c | FinServ__Balance__c | Restricted |

---

## Masking Techniques

### 1. Deterministic Pseudonymization

Replace PII with a consistent synthetic value derived from a hash. Same input
always produces the same synthetic output, allowing relational integrity to be
maintained (foreign key lookups still work).

```apex
// Example: SHA-256-based email pseudonymization
public static String pseudonymizeEmail(String realEmail, String salt) {
    Blob hash = Crypto.generateDigest('SHA-256',
        Blob.valueOf(salt + realEmail.toLowerCase()));
    String hex = EncodingUtil.convertToHex(hash).left(12);
    return hex + '@masked.example.com';
}
```

Use this when sandbox tests require consistent email across related records
(e.g., Contact.Email and Case.SuppliedEmail must match).

### 2. Randomization

Replace values with random synthetic data. Breaks relational integrity but is
simpler and sufficient for fields not used as join keys.

```apex
public static String randomEmail() {
    String chars = 'abcdefghijklmnopqrstuvwxyz0123456789';
    String localPart = '';
    for (Integer i = 0; i < 8; i++) {
        localPart += chars.charAt((Integer)(Math.random() * chars.length()));
    }
    return localPart + '@example-masked.org';
}

public static String randomPhone() {
    // Format: +1-555-RANDOM where RANDOM is 7 digits
    Integer suffix = (Integer)(Math.random() * 9000000) + 1000000;
    return '+1-555-' + String.valueOf(suffix);
}
```

### 3. Nullification (Field Blanking)

Set sensitive fields to null. Simplest approach; use for fields that are not
required by sandbox tests.

```apex
List<Contact> contacts = [SELECT Id, SSN__c, CreditCardNumber__c FROM Contact LIMIT 10000];
for (Contact c : contacts) {
    c.SSN__c = null;
    c.CreditCardNumber__c = null;
}
Database.update(contacts, false);
```

### 4. Value Substitution with Realistic Patterns

Replace with realistic-looking but clearly fake data, useful for UI testing.

```apex
// Names from a fixed list clearly labeled as fake
static final List<String> FAKE_FIRST_NAMES = new List<String>{
    'TestFirst', 'SampleFirst', 'DemoFirst', 'FakeFirst'
};
static final List<String> FAKE_LAST_NAMES = new List<String>{
    'TestLast', 'SampleUser', 'DemoContact', 'MaskedRecord'
};
```

---

## Regulated-Vertical Considerations

### Healthcare (HIPAA)

HIPAA Safe Harbor requires 18 specific identifiers to be removed or masked:
- Names, geographic data below state level, all date elements (except year),
  phone, fax, email, SSN, account numbers, license numbers, VINs, device
  identifiers, URLs, IPs, biometric identifiers, full-face photos, and any
  other unique identifying numbers.

In Salesforce Health Cloud <!-- verify-before-merge:2026-05-21 -->:
- `HealthCloudGA__EhrPatient__c` — delete all records or use fully synthetic patients.
- `HealthCloudGA__ClinicalEncounter__c` — delete or replace with synthetic.
- Custom PHI fields on standard objects — apply nullification or pseudonymization.

### Financial Services (GLBA / PCI-DSS)

- `FinServ__FinancialAccount__c`.`FinServ__Balance__c` — randomize to non-zero
  dummy values (zero balances may cause test logic failures).
- Payment card data must NEVER appear in sandbox. Salesforce org-level
  encryption and payment masking add-ons should prevent this from flowing in.

### GDPR / CCPA

Data subject rights requests apply to sandbox data if the sandbox contains
identifiable individuals. Masking eliminates this obligation for the sandbox.

---

## Post-Refresh Masking Execution Pattern

```
1. Sandbox refresh completes (new sandbox copy of production).
2. System Administrator runs masking Apex batch before granting access to sandbox.
3. Masking batch processes records in chunks of 200.
4. Masking audit record created: timestamp, operator, object counts masked.
5. Access granted to sandbox users only after masking audit record is created.
```

### Masking Sequence (dependency order)

Mask in this order to avoid foreign key constraint issues:

1. `Individual` object (master PII record)
2. `Contact` (linked to Individual)
3. `Lead`
4. `User` (non-system-admin users)
5. `Case` (SuppliedEmail, SuppliedName)
6. `CaseComment` (CommentBody may contain PII)
7. Custom regulated objects

---

## Sandbox Masking Checklist

- [ ] All production-refresh sandboxes have a masking job configured.
- [ ] Masking job runs automatically on refresh via Automation tool or manual
  post-refresh runbook.
- [ ] Sandbox access is blocked (login disabled or IP restricted) until masking
  job completes.
- [ ] Custom PII fields on standard objects are included in masking scope.
- [ ] External connected app credentials stored in Auth. Providers are rotated
  post-refresh (they are copied from production but should not be active in sandbox).
- [ ] Outbound email is disabled in sandbox (Setup > Deliverability: System Email
  Only) to prevent real emails reaching real customers.
- [ ] Named Credential endpoints are pointed to sandbox/mock counterparts.
