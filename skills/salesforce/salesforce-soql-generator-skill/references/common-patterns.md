# Common SOQL Patterns

Pre-built patterns for the most frequent admin and RevOps use cases. Copy,
adapt field names to your org's schema, and verify with
`salesforce-soql-explorer-skill` before use in automation.

---

## Pipeline by Stage (Sum and Count)

Aggregate deal value and count grouped by stage, filtered to open opportunities.

```soql
SELECT StageName,
       COUNT(Id) deal_count,
       SUM(Amount) total_amount,
       AVG(Amount) avg_deal_size
FROM Opportunity
WHERE IsClosed = false
  AND ForecastCategoryName != 'Omitted'
GROUP BY StageName
ORDER BY MIN(Probability) ASC
```

**Variant — by owner and stage:**

```soql
SELECT Owner.Name, StageName, COUNT(Id) deal_count, SUM(Amount) total_amount
FROM Opportunity
WHERE IsClosed = false
  AND CloseDate >= THIS_QUARTER
GROUP BY Owner.Name, StageName
ORDER BY Owner.Name ASC, MIN(Probability) ASC
```

**Governor notes:** Aggregate query returns one row per group — LIMIT rarely
needed but add it if GROUP BY cardinality could be very high.

---

## Opportunities Without Next Activity

Identify open deals where no future activity is scheduled (common pipeline
hygiene signal).

```soql
SELECT Id, Name, Owner.Name, StageName, Amount, CloseDate, LastActivityDate
FROM Opportunity
WHERE IsClosed = false
  AND CloseDate >= TODAY
  AND Id NOT IN (
    SELECT WhatId FROM Task
    WHERE ActivityDate >= TODAY
      AND Status != 'Completed'
      AND WhatId != null
  )
ORDER BY CloseDate ASC
LIMIT 200
```

**Notes:**
- The anti-join pattern (`NOT IN` subquery) uses a semi-join against Task.
- `WhatId` on Task is a polymorphic field — this query only captures Task-based
  activities, not Events. Run a parallel query against `Event.WhatId` if Events
  are tracked separately.
- Index: `CloseDate` and `IsClosed` are standard indexed fields on Opportunity.

---

## Stale Leads (No Update in N Days)

Find leads that have not been modified recently, indicating decay.

```soql
SELECT Id, FirstName, LastName, Email, Company, Status, Owner.Name,
       LastModifiedDate, CreatedDate
FROM Lead
WHERE IsConverted = false
  AND LastModifiedDate < LAST_N_DAYS:90
  AND Status NOT IN ('Closed - Not Converted', 'Unqualified')
ORDER BY LastModifiedDate ASC
LIMIT 500
```

**Variant — parameterised threshold (Apex context):**

```soql
SELECT Id, FirstName, LastName, Company, Status, LastModifiedDate
FROM Lead
WHERE IsConverted = false
  AND LastModifiedDate < :thresholdDate
ORDER BY LastModifiedDate ASC
LIMIT 1000
```

**Notes:**
- `LastModifiedDate` is indexed on Lead.
- Adjust the `LAST_N_DAYS:90` threshold to your org's sales cadence (30/60/90).
- Exclude statuses that represent legitimate terminal states in your process.

---

## Account Ownership Audit

Surface accounts assigned to inactive users or with no recent activity.

```soql
SELECT Id, Name, Owner.Name, Owner.IsActive, Owner.Profile.Name,
       LastModifiedDate, LastActivityDate, Type, BillingCountry
FROM Account
WHERE Owner.IsActive = false
   OR LastActivityDate < LAST_N_DAYS:180
ORDER BY Owner.IsActive ASC, LastActivityDate ASC NULLS FIRST
LIMIT 500
```

**Variant — find accounts with no owner (public queue owned):**

```soql
SELECT Id, Name, OwnerId, Owner.Type, LastModifiedDate
FROM Account
WHERE Owner.Type = 'Queue'
  AND LastModifiedDate < LAST_N_DAYS:30
ORDER BY LastModifiedDate ASC
LIMIT 200
```

**Notes:**
- `Owner.IsActive` traverses the User relationship — not indexed, but
  `OwnerId` is. Performance depends on the number of inactive owners.
- For large orgs, add a `Type` or `RecordType.Name` filter to narrow scope.

---

## Contacts with Bouncing Emails

Find contacts flagged as email-invalid or undeliverable (uses EmailBouncedDate
or a custom field depending on org configuration).

```soql
SELECT Id, FirstName, LastName, Email, EmailBouncedDate, EmailBouncedReason,
       Account.Name, Owner.Name
FROM Contact
WHERE EmailBouncedDate != null
  AND HasOptedOutOfEmail = false
ORDER BY EmailBouncedDate DESC
LIMIT 500
```

**Variant — contacts with no email at all:**

```soql
SELECT Id, FirstName, LastName, Account.Name, Owner.Name, CreatedDate
FROM Contact
WHERE Email = null
  AND HasOptedOutOfEmail = false
ORDER BY CreatedDate DESC
LIMIT 500
```

**Notes:**
- `EmailBouncedDate` is a standard field on Contact populated by Salesforce
  email features (not always by third-party ESPs — check your org config).
- If your org uses Marketing Cloud or Pardot, bounce data may live in a
  custom field or a related object.

---

## Field History Tracking Queries

Query the history tracking object for a standard or custom object. The history
object is named `<SObject>History` (e.g., `OpportunityFieldHistory`,
`AccountHistory`, `My_Object__History`).

**Track when a field changed on Opportunity:**

```soql
SELECT Id, OpportunityId, Field, OldValue, NewValue, CreatedDate, CreatedBy.Name
FROM OpportunityFieldHistory
WHERE Field = 'StageName'
  AND CreatedDate = LAST_N_DAYS:30
ORDER BY CreatedDate DESC
LIMIT 500
```

**Track all field changes on a specific record:**

```soql
SELECT Id, Field, OldValue, NewValue, CreatedDate, CreatedBy.Name
FROM AccountHistory
WHERE AccountId = '001Xx000001PLACEHOLDER'
ORDER BY CreatedDate DESC
LIMIT 200
```

**Notes:**
- Field history tracking must be enabled for each field in Setup. Not all
  fields support history tracking.
- History objects retain data for 18 months in most orgs (configurable with
  add-ons).
- Replace `'001Xx000001PLACEHOLDER'` with the actual record Id or a bind
  variable.

---

## Junction Object Queries

Query a many-to-many relationship via its junction object. Example: Campaign
Member (junction between Campaign and Lead/Contact).

```soql
SELECT Id, CampaignId, Campaign.Name, LeadId, Lead.FirstName, Lead.LastName,
       Lead.Email, Status, HasResponded, FirstRespondedDate
FROM CampaignMember
WHERE CampaignId = '701Xx000001PLACEHOLDER'
  AND Status = 'Responded'
ORDER BY FirstRespondedDate DESC
LIMIT 500
```

**Custom junction object pattern (e.g., Account_Product__c linking Account
and Product2):**

```soql
SELECT Id, Account__c, Account__r.Name, Product__c, Product__r.Name,
       Quantity__c, Start_Date__c
FROM Account_Product__c
WHERE Account__r.Type = 'Customer'
  AND Start_Date__c >= THIS_YEAR
ORDER BY Account__r.Name ASC
LIMIT 500
```

**Notes:**
- Use `__r` suffix to traverse a custom lookup relationship by API name.
- For standard relationships use the relationship name (e.g., `Campaign`,
  `Lead`), not a `__r` suffix.

---

## Polymorphic Owner Queries (User vs Queue)

The `OwnerId` field on most sObjects can reference either a User or a Queue.
Use `TYPEOF` (or a Type filter on a subquery) to distinguish.

```soql
SELECT Id, Name,
  TYPEOF Owner
    WHEN User THEN FirstName, LastName, Email, IsActive
    WHEN Group THEN Name, Type
  END
FROM Case
WHERE IsClosed = false
  AND CreatedDate >= THIS_MONTH
ORDER BY CreatedDate DESC
LIMIT 500
```

**Simpler pattern — split into two queries:**

```soql
-- User-owned open cases
SELECT Id, Subject, OwnerId, Owner.Name, Owner.IsActive, Priority
FROM Case
WHERE IsClosed = false
  AND Owner.Type = 'User'
LIMIT 500

-- Queue-owned open cases
SELECT Id, Subject, OwnerId, Owner.Name, Priority
FROM Case
WHERE IsClosed = false
  AND Owner.Type = 'Queue'
LIMIT 500
```

**Notes:**
- `TYPEOF` is supported in API version 26.0+ but has limited support in some
  Apex SOQL contexts. Test before embedding in Apex.
- The `Group` type in `TYPEOF` represents a Queue (Salesforce's internal type
  name for queues is `Group`).

---

## Usage Notes

- All record IDs in these patterns are placeholders (`001Xx000001PLACEHOLDER`,
  `701Xx000001PLACEHOLDER`, etc.). Replace before executing.
- Field API names may differ in your org (custom fields, custom objects, or
  managed package prefixes). Verify with `sf sobject describe` or the Object
  Manager before use.
- LIMIT values are conservative defaults. Adjust to your org's data volume.
- For live execution of any pattern, use `salesforce-soql-explorer-skill`.
