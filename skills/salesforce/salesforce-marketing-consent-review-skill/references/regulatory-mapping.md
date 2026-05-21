# Regulatory Mapping Reference

Mapping of major privacy and marketing regulations to Salesforce consent model
fields and compliance requirements.

---

## Regulation Overview Matrix

| Regulation | Jurisdiction | Opt-In Required | Opt-Out Required | Right to Erasure | Consent Record Required |
|-----------|-------------|----------------|-----------------|-----------------|------------------------|
| GDPR | EU/EEA | Art 6(1)(a): explicit for marketing | Art 21: right to object | Art 17: yes | Yes |
| UK GDPR | United Kingdom | Same as EU GDPR | Same as EU GDPR | Same as EU GDPR | Yes |
| CCPA/CPRA | California, USA | No (opt-out model) | Yes (right to opt out of sale/sharing) | Yes (deletion right) | Yes |
| CASL | Canada | Express consent required | Unsubscribe required | No erasure right | Yes (3 years) |
| CAN-SPAM | United States | No (opt-out model) | Yes (unsubscribe mechanism) | No | Recommended |
| ePrivacy Directive | EU/EEA | Yes (cookies and tracking) | Yes | No | Yes |
| LGPD | Brazil | Yes (similar to GDPR) | Yes | Yes | Yes |
| PDPA | Thailand | Yes | Yes | Yes | Yes |
| POPIA | South Africa | Yes | Yes | Yes | Yes |

Regulations are frequently amended. Verify
current requirements with qualified legal counsel for the specific jurisdictions
in scope.

---

## GDPR Article 6 Lawful Basis

GDPR requires a lawful basis for every processing activity. Marketing
communications typically rely on one of:

| Lawful Basis | Art 6 Reference | When Applicable | Consent Record Needed |
|-------------|----------------|----------------|----------------------|
| Consent | Art 6(1)(a) | Marketing emails, profiling, tracking | Yes — must be specific, informed, freely given |
| Legitimate Interest | Art 6(1)(f) | B2B marketing to existing contacts (with opt-out) | LIA documentation; opt-out mechanism |
| Contract | Art 6(1)(b) | Service communications (order confirmations) | No specific consent form; document lawful basis |
| Legal Obligation | Art 6(1)(c) | Compliance notifications | No |
| Vital Interest | Art 6(1)(d) | Emergency safety information | No |
| Public Task | Art 6(1)(e) | Public authority processing | No |

**Important:** Consent under GDPR must be:
- Freely given (not bundled with terms of service acceptance).
- Specific (separate consent for each purpose).
- Informed (clear description of what is being consented to).
- Unambiguous (affirmative action; pre-ticked boxes are not valid).

### Salesforce Mapping

| GDPR Requirement | Salesforce Field |
|-----------------|-----------------|
| Consent captured | `ContactPointConsent.CaptureDate` |
| Consent source | `ContactPointConsent.CaptureSource` |
| Consent status | `ContactPointConsent.PrivacyConsentStatus` |
| Lawful basis | Custom field on `ContactPointConsent` or `Individual` |
| Processing opt-out | `Individual.HasOptedOutOfProcessing` |
| Right to erasure flag | `Individual.ShouldForget` |

---

## CCPA / CPRA (California)

The California Consumer Privacy Act and its amendment (CPRA) apply to
businesses meeting volume/revenue thresholds doing business in California.

### Key Rights and Obligations

| Right | Obligation | Salesforce Mechanism |
|-------|-----------|---------------------|
| Right to Know | Disclose categories and specific data | Data export / DSAR workflow |
| Right to Delete | Delete PI within 45 days of verified request | Individual.ShouldForget + deletion batch |
| Right to Opt Out of Sale/Sharing | Provide "Do Not Sell or Share My PI" mechanism | Custom consent field or ContactPointConsent |
| Right to Correct | Correct inaccurate PI | Standard Salesforce update flow |
| Right to Limit Sensitive PI Use | Restrict sensitive PI to primary purposes | Field-level consent + sensitive data classification |

**CPRA additions** (effective 2023):
- Sensitive personal information (SPI) category requires additional disclosure
  and opt-out mechanism.
- Data minimization and purpose limitation principles.
- Retention period requirements (no longer than reasonably necessary).

### CCPA vs GDPR Key Difference

CCPA is primarily an **opt-out model**: businesses can market to California
residents without prior consent, but must provide a clear opt-out mechanism
and honor opt-out requests within 15 business days.

GDPR is primarily an **opt-in model** for marketing: affirmative consent is
required before most marketing communications.

---

## CASL (Canada's Anti-Spam Legislation)

CASL applies to all commercial electronic messages (CEMs) sent to or from
Canada.

### Consent Types

| Type | Description | Expiry |
|------|-------------|--------|
| Express Consent | Explicit opt-in (form checkbox, signature) | No expiry unless withdrawn |
| Implied Consent — existing business relationship | Purchase or inquiry in past 2 years | 2 years from last transaction |
| Implied Consent — conspicuously published address | B2B to published work addresses, relevant to role | Must offer opt-out |

**Implied consent expires.** Track the transaction date that creates the implied
consent relationship.

```sql
-- Find contacts with implied CASL consent nearing expiry (< 3 months)
SELECT Id, Name, Email, LastPurchaseDate__c
FROM Contact
WHERE ConsentType__c = 'CASL_Implied'
  AND LastPurchaseDate__c < LAST_N_DAYS:639  -- (730 - 91 days warning)
  AND LastPurchaseDate__c != null
```

### CASL Consent Record Requirements

Per CASL, consent records must be retained for the duration of the relationship
plus 3 years after opt-out or expiry. Records must contain:
- Who gave consent (name and contact details).
- When consent was given.
- How consent was given (form URL, script, checkbox label).
- What they consented to (description of CEM types).

---

## CAN-SPAM (United States)

CAN-SPAM applies to commercial emails in the United States. It is an opt-out
model with no pre-consent requirement for initial contact.

### Requirements

| Requirement | Detail |
|-------------|--------|
| Accurate header information | From, To, Reply-To must be accurate |
| Non-deceptive subject lines | Subject must not mislead about content |
| Physical postal address | Must include sender's valid physical address |
| Clear opt-out mechanism | Must provide clear and conspicuous opt-out |
| Honor opt-out within 10 business days | Process opt-out within 10 business days |
| No sharing of opted-out addresses | Cannot sell/transfer opted-out addresses |

### Salesforce Mapping for CAN-SPAM

- `Contact.HasOptedOutOfEmail = true` honors CAN-SPAM opt-out.
- Marketing Cloud Unsubscribe events should sync to `Contact.HasOptedOutOfEmail`.
- Physical address stored in sender's org settings and included in email templates.

---

## ePrivacy Directive (EU) / Cookie Consent

The ePrivacy Directive (and forthcoming ePrivacy Regulation) governs electronic
marketing and cookies. In the Salesforce context:

- **Tracking pixels and web analytics:** Require consent before firing in EU.
- **Marketing Cloud email tracking:** Open and click tracking requires consent
  under ePrivacy if IP-linked.
- **ExactTarget/SFMC web tracking (Collect.js):** Requires cookie consent banner
  before activation.

### Salesforce Tracking Consent Implementation

```javascript
// Example: Only load SFMC tracking after consent given
if (userConsentStatus === 'OptIn') {
    // Load Collect.js for web tracking
    var _etmc = _etmc || [];
    _etmc.push(['setOrgId', 'YOUR_ORG_ID']);
    // ... load script
} else {
    // Do not load tracking script
    console.log('Tracking disabled: user has not opted in.');
}
```

---

## Multi-Regulation Compliance Checklist

- [ ] Regulatory map documented for each jurisdiction where customers reside.
- [ ] Lawful basis documented for each processing purpose (not just "consent").
- [ ] Express consent captured with date, source, and purpose for GDPR/CASL.
- [ ] Opt-out honored within: GDPR (promptly), CCPA (15 business days), CAN-SPAM (10 business days), CASL (10 business days).
- [ ] Implied CASL consent relationships tracked with expiry dates.
- [ ] Consent records retained for minimum periods (CASL: relationship + 3 years).
- [ ] Erasure requests trigger data deletion across all connected systems
  (Marketing Cloud, Data Cloud, CRM, backups).
- [ ] Minor consent handled separately (parental consent where applicable).
- [ ] Privacy notice linked at point of consent capture.
