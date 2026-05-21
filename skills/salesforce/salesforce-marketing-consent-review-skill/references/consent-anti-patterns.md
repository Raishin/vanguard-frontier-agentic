# Consent Anti-Patterns Reference

Common consent management mistakes that create regulatory violations,
trust failures, or data integrity problems.

---

## 1. Soft Opt-In Misuse

### Description
"Soft opt-in" is a specific exception under the UK's Privacy and Electronic
Communications Regulations (PECR) and similar rules. It permits sending
commercial messages to existing customers about similar products WITHOUT
explicit prior consent, provided an opt-out was offered at the time of data
collection.

### What Constitutes Valid Soft Opt-In

All conditions must be satisfied:
1. Contact details obtained during a **sale or negotiation of a sale**.
2. Marketing is **only for similar products or services** to what was purchased.
3. The individual was **given a clear opportunity to opt out** at collection.
4. An **opt-out mechanism exists in every message**.

### Common Misuses

| Misuse | Why It Fails |
|--------|-------------|
| Applying soft opt-in to contacts obtained from bought lists | Not obtained during a sale |
| Marketing unrelated products using soft opt-in basis | Must be "similar products" only |
| Using soft opt-in for B2C contacts under GDPR | GDPR requires explicit consent; soft opt-in is a PECR concept |
| Failing to provide opt-out at collection point | Invalidates the soft opt-in basis |
| Treating "gave us their card at a conference" as soft opt-in | No sale/negotiation occurred |

### Salesforce Data Model Check

```sql
-- Find contacts with soft-opt-in consent basis where lawful basis is unverified
SELECT Id, Name, Email, ConsentBasis__c, ConsentCaptureSource__c,
       ConsentCaptureDate__c
FROM Contact
WHERE ConsentBasis__c = 'SoftOptIn'
  AND (ConsentCaptureSource__c = null OR ConsentCaptureDate__c = null)
```

---

## 2. Pre-Checked Consent Boxes

### Description
Consent forms that preselect the marketing consent checkbox, requiring
users to actively uncheck to opt out rather than actively check to opt in.

### Why It Is Invalid

Under GDPR Article 7 and Recital 32, consent must be given by a "clear
affirmative act". Pre-ticked boxes do not constitute affirmative action and
are explicitly prohibited.

Under CASL, consent obtained via a pre-checked box is **not valid express consent**.

### Detection

Review all forms, landing pages, and registration flows:
- Web-to-Lead forms (Setup > Web-to-Lead)
- Marketing Cloud CloudPages and Smart Capture forms
- Custom LWC or VisualForce forms

Checklist item: every marketing consent checkbox must have `checked="false"`
(or no `checked` attribute) by default.

### Correct HTML Pattern
```html
<!-- WRONG: pre-checked -->
<input type="checkbox" name="marketingConsent" checked="checked">
<label>Send me marketing communications</label>

<!-- CORRECT: unchecked by default -->
<input type="checkbox" name="marketingConsent">
<label>Yes, I would like to receive marketing communications about [specific products].</label>
```

---

## 3. Missing Consent Withdrawal Mechanism

### Description
Users cannot easily opt out of marketing communications, or the opt-out
mechanism is obscured, dysfunctional, or not honored consistently across
all channels.

### Regulatory Requirements

| Regulation | Opt-Out Deadline | Mechanism Requirement |
|-----------|-----------------|----------------------|
| GDPR | Without undue delay | Must be as easy as giving consent |
| CAN-SPAM | 10 business days | Clear opt-out in every email |
| CASL | 10 business days | Unsubscribe link in every CEM |
| CCPA | 15 business days | Conspicuous "Do Not Sell/Share" link |

### Anti-Patterns

1. **Opt-out link buried in 6pt font at bottom of email:** Fails "as easy to withdraw as to give" requirement.
2. **Opt-out requires creating an account:** Illegal friction under GDPR.
3. **Opt-out confirmation page has re-subscribe push:** Acceptable if not coercive; review wording.
4. **Opt-out updates only Marketing Cloud but not Salesforce CRM:** Cross-system sync gap means the contact may be re-added to future lists.
5. **Opt-out requires customer service call or email:** Must be automated; human-in-the-loop opt-out processes are too slow.

### Cross-System Sync Verification

```sql
-- Find contacts opted out in MC but still active in CRM (sync gap)
-- This query assumes a custom field tracking MC opt-out sync
SELECT Id, Email, HasOptedOutOfEmail, MCOptOutSyncStatus__c, LastMCOptOutDate__c
FROM Contact
WHERE HasOptedOutOfEmail = false
  AND MCOptOutSyncStatus__c = 'OptedOut'
```

---

## 4. Stale Consent

### Description
Consent captured years ago is treated as perpetually valid without considering
whether the individual's circumstances or the processing purpose has changed.

### Why Stale Consent Is a Risk

Under GDPR Recital 171, when the original basis for consent changes, new consent
is required. Additionally, inactive contacts who consented years ago and have
not engaged may no longer reasonably expect to receive marketing.

### CASL-Specific Rule

Implied consent under CASL expires after 2 years from the last transaction.
Express consent does not expire unless withdrawn.

### Identifying Stale Consent

```sql
-- GDPR context: consent older than 2 years with no engagement
SELECT cp.Id, cp.EmailAddress, cpc.CaptureDate, cpc.PrivacyConsentStatus,
       c.LastActivityDate
FROM ContactPointConsent cpc
JOIN ContactPointEmail cp ON cpc.ContactPointId = cp.Id
JOIN Contact c ON cp.IndividualId = c.IndividualId
WHERE cpc.PrivacyConsentStatus = 'OptIn'
  AND cpc.CaptureDate < LAST_N_YEARS:2
  AND c.LastActivityDate < LAST_N_YEARS:2
ORDER BY cpc.CaptureDate ASC
LIMIT 500
```

### Remediation: Consent Re-Engagement Campaign

Before bulk opt-out of stale contacts:
1. Send a single re-confirmation email ("Are you still interested in hearing from us?").
2. Only contacts who click re-confirm are retained as opt-in.
3. Contacts who do not respond within 30 days are moved to `NotSeen` status.
4. Document the re-engagement process for compliance record.

---

## 5. Bundle Consent (All or Nothing)

### Description
Consent for marketing is bundled with consent for terms of service, product
use, or other processing. Users cannot selectively consent to marketing while
accepting the terms.

### Why It Is Invalid

GDPR Article 7(2): "If the data subject's consent is given in the context of a
written declaration which also concerns other matters, the request for consent
shall be presented in a manner which is clearly distinguishable from the other
matters."

Bundled consent is not "freely given" because refusal of marketing consent would
prevent acceptance of the service.

### Correct Form Design
```
[  ] I accept the Terms of Service and Privacy Policy (required)

--- Marketing Communications ---
[  ] I would like to receive product updates by email (optional)
[  ] I would like to receive SMS offers (optional)
[  ] I would like to receive personalized recommendations (optional)
```

Each marketing option must be independently selectable and optional.

---

## 6. Consent Without Purpose Specification

### Description
Consent is recorded as "marketing consent" without specifying which types
of marketing, which channels, and which sender entity.

### Why It Is a Problem

Under GDPR, consent must be **specific** to a purpose. Blanket "marketing consent"
is too vague if the same consent is used for:
- Product newsletters (via email).
- SMS promotions (different channel).
- Profiling for personalization (different processing activity).
- Third-party data sharing for joint campaigns (different legal entity).

### Correct Approach

Separate `ContactPointConsent` records for each purpose + channel combination:

```
ContactPointConsent record 1:
  ContactPointId = [email address]
  DataUsePurpose = "Monthly Newsletter"
  PrivacyConsentStatus = OptIn

ContactPointConsent record 2:
  ContactPointId = [email address]
  DataUsePurpose = "Product Promotional Offers"
  PrivacyConsentStatus = OptOut

ContactPointConsent record 3:
  ContactPointId = [phone number]
  DataUsePurpose = "SMS Marketing"
  PrivacyConsentStatus = OptIn
```

---

## Consent Anti-Pattern Summary

| Anti-Pattern | Regulation Violated | Severity |
|-------------|--------------------|---------| 
| Pre-checked boxes | GDPR, CASL | CRITICAL |
| Bundle consent with T&C | GDPR | HIGH |
| Soft opt-in misapplied | PECR/UK GDPR, CASL | HIGH |
| Stale consent treated as valid | GDPR, CASL implied expiry | HIGH |
| Missing opt-out mechanism | CAN-SPAM, CASL, GDPR | CRITICAL |
| Opt-out not honored in all systems | All regulations | CRITICAL |
| No consent purpose specification | GDPR | HIGH |
| Consent obtained via coercion | GDPR | CRITICAL |
| No consent withdrawal record | GDPR Art 7(1), CASL | HIGH |
| Withdrawal mechanism requires account creation | GDPR | HIGH |
