# Consent Model Reference

Reference for Salesforce's consent data model covering the Individual object,
Contact Point Consent, Communication Subscription objects, and their
relationships to Contact/Lead records.

---

## Core Consent Objects

Salesforce provides a built-in consent data model in the core platform.
<!-- verify-before-merge:2026-05-21 --> This model is extended by Marketing Cloud
and the Health/Financial cloud products.

### Object Map

```
Contact / Lead / Person Account
    |
    v
Individual (1:1 linked via IndividualId field)
    |
    +-- ContactPointConsent (per contact point + purpose)
    |       |-- ContactPointEmail
    |       |-- ContactPointPhone
    |       |-- ContactPointAddress
    |
    +-- IndividualHistory (audit changes to Individual record)

Communication Subscription Type (catalog of subscription types)
    |
    v
Communication Subscription (per Individual + Type)
    |
    v
Communication Subscription Channel Type (per channel: email, SMS, push)
```

---

## Individual Object

The `Individual` sObject is the hub of the consent data model.
Every Contact or Lead that requires consent management should be linked to an
Individual record via the `IndividualId` lookup field.

### Key Fields

| Field | API Name | Purpose |
|-------|----------|---------|
| Name | `Name` | Individual identifier (typically mirrors Contact name) |
| Don't Process | `HasOptedOutOfProcessing` | Global processing opt-out |
| Don't Market | `HasOptedOutOfSolicitationn` | Global marketing opt-out |
| Don't Profile | `HasOptedOutOfProfiling` | Profiling/segmentation opt-out |
| Don't Track | `HasOptedOutOfTracking` | Web tracking opt-out |
| Forget | `ShouldForget` | Erasure request flag |
| Birth Date | `BirthDate` | Age verification for minor consent |
| Death Date | `DeathDate` | Deceased flag |

### Querying Individual-Contact Relationship
```sql
SELECT Id, IndividualId, Individual.HasOptedOutOfProcessing,
       Individual.HasOptedOutOfSolicitationn
FROM Contact
WHERE Id IN :contactIds
```

---

## ContactPointConsent Object

`ContactPointConsent` records store granular consent for a specific contact
point (email address, phone number) and a specific data use purpose.

### Key Fields

| Field | API Name | Type | Purpose |
|-------|----------|------|---------|
| Contact Point | `ContactPointId` | Polymorphic lookup | Which email/phone/address |
| Data Use Purpose | `DataUsePurposeId` | Lookup | Consent purpose |
| Capture Date | `CaptureDate` | DateTime | When consent was captured |
| Capture Source | `CaptureSource` | Text | Where consent was captured (form URL, event, etc.) |
| Privacy Consent Status | `PrivacyConsentStatus` | Picklist | OptIn / OptOut / NotSeen / Seen |
| Effective From | `EffectiveFrom` | DateTime | When consent takes effect |
| Effective To | `EffectiveTo` | DateTime | When consent expires |

### Consent Status Values

| Status | Meaning |
|--------|---------|
| `OptIn` | Explicit opt-in; can send |
| `OptOut` | Explicit opt-out; must not send |
| `NotSeen` | Individual has not been presented with consent request |
| `Seen` | Individual saw the consent request but did not act |

**Best practice:** Only send to contacts with `PrivacyConsentStatus = 'OptIn'`
for regulated channels. `NotSeen` and `Seen` are ambiguous and should be
treated as opt-out in GDPR/CASL contexts.

---

## ContactPointEmail and ContactPointPhone

These objects store individual contact points (distinct from the Email field on Contact).

```sql
-- Find all email contact points for a contact
SELECT Id, EmailAddress, Individual.Name, Individual.HasOptedOutOfSolicitationn
FROM ContactPointEmail
WHERE IndividualId IN (
    SELECT IndividualId FROM Contact WHERE Id IN :contactIds
)
```

A Contact may have multiple ContactPointEmail records (work, personal, newsletter
address). Consent should be tracked at this granularity.

---

## Communication Subscription Model

The Communication Subscription model provides a catalog-based approach to
managing preferences for named subscription types.

### Objects

| Object | Purpose |
|--------|---------|
| `CommSubscription` | Defines a named subscription type (e.g., "Monthly Newsletter") |
| `CommSubscriptionConsent` | Records an Individual's consent for a subscription |
| `CommSubscriptionChannelType` | Maps a subscription type to a channel (Email, SMS, Push) |
| `CommSubscriptionTiming` | Frequency preferences (daily, weekly, monthly) |

```sql
-- Get all active subscriptions with their channel types
SELECT Id, Name,
       (SELECT Id, ChannelType, CommChannelTypeName
        FROM CommSubscriptionChannelTypes)
FROM CommSubscription
WHERE Status = 'Active'
```

---

## Salesforce Marketing Cloud Consent Integration

<!-- verify-before-merge:2026-05-21 -->

In orgs using Marketing Cloud with a Salesforce connector (Marketing Cloud
Connect or the newer Marketing Cloud Account Engagement / MCAE):

| MC Object | Core Object Counterpart | Sync Direction |
|-----------|------------------------|----------------|
| EmailOptIn/OptOut | Individual.HasOptedOutOfSolicitationn | Bidirectional |
| Subscription status | CommSubscriptionConsent | Configurable |
| SMS opt-in | ContactPointPhone + ContactPointConsent | Core to MC |

**Key review point:** Verify that opt-out signals from Marketing Cloud propagate
to the Salesforce core Individual object and vice versa. Unidirectional sync
can result in opt-out honors on one platform but sends on another.

---

## Consent Data Queries for Review

### Find contacts without Individual records
```sql
SELECT Id, Name, Email
FROM Contact
WHERE IndividualId = null
AND Email != null
LIMIT 500
```

### Find opt-outs that may still be receiving communication
```sql
SELECT c.Id, c.Email, c.Name, i.HasOptedOutOfSolicitationn
FROM Contact c
JOIN Individual i ON c.IndividualId = i.Id
WHERE i.HasOptedOutOfSolicitationn = true
  AND c.HasOptedOutOfEmail = false
```
Mismatch between `Individual.HasOptedOutOfSolicitationn` and `Contact.HasOptedOutOfEmail`
is a data integrity finding.

### Find expired consent
```sql
SELECT Id, ContactPointId, PrivacyConsentStatus, EffectiveTo
FROM ContactPointConsent
WHERE EffectiveTo < TODAY
  AND PrivacyConsentStatus = 'OptIn'
```
Expired opt-in consent should be treated as `NotSeen` until re-confirmed.

---

## Consent Model Review Checklist

- [ ] All Contacts with email addresses have an `IndividualId` set.
- [ ] `ContactPointConsent` records exist for each ContactPointEmail.
- [ ] Consent capture date and source are recorded on every `ContactPointConsent`.
- [ ] Opt-out values are synchronized between `Individual` and `Contact` objects.
- [ ] Expired consent is not treated as active opt-in.
- [ ] Communication Subscription model in use (not just field-level opt-out).
- [ ] Consent records are immutable / append-only (history preserved, not overwritten).
- [ ] Consent withdrawal mechanism captures the withdrawal date and source.
