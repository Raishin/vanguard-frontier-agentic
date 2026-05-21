# Anonymous Apex Patterns Reference

## Core Safety Rules

Every Anonymous Apex bulk script must follow these rules:

1. **Never DML inside a loop** — collect records, DML after loop
2. **allOrNone=false** — partial success; never block all on one failure
3. **Database.SaveResult[] iteration** — log every failure
4. **Chunk to 200** — Apex DML batch limit per transaction
5. **SOQL in query phase only** — no SOQL inside loops
6. **Governor limit estimation** — comment the expected row count

---

## Pattern 1: Mass Owner Reassignment

```apex
/**
 * Mass Owner Reassignment
 * Target: Opportunity records owned by the source user
 * Volume: Estimate <n> records
 * Run in: SANDBOX FIRST. Route to salesforce-live-guard-agent for production.
 */

// CONFIGURE THESE VALUES
String sourceUsername = 'outbound_rep@company.com';
String targetUsername = 'new_owner@company.com';
Integer BATCH_SIZE = 200;

// Resolve user IDs dynamically — do not hardcode
Id sourceOwnerId = [SELECT Id FROM User WHERE Username = :sourceUsername LIMIT 1].Id;
Id targetOwnerId = [SELECT Id FROM User WHERE Username = :targetUsername LIMIT 1].Id;

System.debug('Source Owner: ' + sourceOwnerId);
System.debug('Target Owner: ' + targetOwnerId);

// Query affected records
List<Opportunity> oppsToUpdate = [
    SELECT Id, OwnerId, Name
    FROM Opportunity
    WHERE OwnerId = :sourceOwnerId
    AND IsClosed = false
    ORDER BY Id
    LIMIT 10000
];

System.debug('Records to update: ' + oppsToUpdate.size);

// Update in chunks
Integer successCount = 0;
Integer failCount = 0;

for (Integer i = 0; i < oppsToUpdate.size; i += BATCH_SIZE) {
    Integer endIdx = Math.min(i + BATCH_SIZE, oppsToUpdate.size);
    List<Opportunity> batch = new List<Opportunity>;
    for (Integer j = i; j < endIdx; j++) {
        batch.add(new Opportunity(
            Id = oppsToUpdate[j].Id,
            OwnerId = targetOwnerId
        ));
    }
    Database.SaveResult[] results = Database.update(batch, false);
    for (Database.SaveResult sr : results) {
        if (sr.isSuccess) {
            successCount++;
        } else {
            failCount++;
            System.debug('ERROR on record ' + sr.getId +
                ': ' + sr.getErrors[0].getMessage);
        }
    }
}

System.debug('Reassignment complete. Success: ' + successCount + ' | Failed: ' + failCount);
```

---

## Pattern 2: Mass Field Update

```apex
/**
 * Mass Field Update — set a field value on a filtered set of records
 * Target: Account records missing Industry classification
 * Volume: Estimate <n> records
 * Run in: SANDBOX FIRST.
 */

Integer BATCH_SIZE = 200;

// Query affected records
List<Account> accountsToUpdate = [
    SELECT Id, Name, Industry
    FROM Account
    WHERE Industry = null
    AND BillingCountry = 'United States'
    AND Type = 'Customer - Direct'
    ORDER BY Id
    LIMIT 10000
];

System.debug('Records to update: ' + accountsToUpdate.size);

Integer successCount = 0;
Integer failCount = 0;

for (Integer i = 0; i < accountsToUpdate.size; i += BATCH_SIZE) {
    Integer endIdx = Math.min(i + BATCH_SIZE, accountsToUpdate.size);
    List<Account> batch = new List<Account>;
    for (Integer j = i; j < endIdx; j++) {
        batch.add(new Account(
            Id = accountsToUpdate[j].Id,
            Industry = 'Other'  // CONFIGURE: set correct default value
        ));
    }
    Database.SaveResult[] results = Database.update(batch, false);
    for (Database.SaveResult sr : results) {
        if (sr.isSuccess) {
            successCount++;
        } else {
            failCount++;
            System.debug('ERROR: ' + sr.getId + ' | ' + sr.getErrors[0].getMessage);
        }
    }
}

System.debug('Update complete. Success: ' + successCount + ' | Failed: ' + failCount);
```

---

## Pattern 3: Batch Close Stale Opportunities

```apex
/**
 * Batch Close Stale Opportunities
 * Target: Open Opportunities with CloseDate > 90 days ago
 * Effect: Sets Stage to Closed Lost and adds a note
 * Run in: SANDBOX FIRST.
 */

Integer BATCH_SIZE = 200;
Integer STALE_DAYS = 90;  // CONFIGURE: number of days past close date

Date staleDate = Date.today.addDays(-STALE_DAYS);

List<Opportunity> staleOpps = [
    SELECT Id, Name, StageName, CloseDate, OwnerId
    FROM Opportunity
    WHERE IsClosed = false
    AND CloseDate < :staleDate
    ORDER BY CloseDate ASC
    LIMIT 10000
];

System.debug('Stale opportunities to close: ' + staleOpps.size);

Integer successCount = 0;
Integer failCount = 0;

for (Integer i = 0; i < staleOpps.size; i += BATCH_SIZE) {
    Integer endIdx = Math.min(i + BATCH_SIZE, staleOpps.size);
    List<Opportunity> batch = new List<Opportunity>;
    for (Integer j = i; j < endIdx; j++) {
        batch.add(new Opportunity(
            Id = staleOpps[j].Id,
            StageName = 'Closed Lost',  // CONFIGURE: target stage
            Description = 'Auto-closed by bulk ops script on ' + Date.today.format +
                          '. Original close date: ' + staleOpps[j].CloseDate.format,
            CloseDate = Date.today  // Required if validation rule checks CloseDate
        ));
    }
    Database.SaveResult[] results = Database.update(batch, false);
    for (Database.SaveResult sr : results) {
        if (sr.isSuccess) {
            successCount++;
        } else {
            failCount++;
            System.debug('ERROR: ' + sr.getId + ' | ' + sr.getErrors[0].getMessage);
        }
    }
}

System.debug('Close complete. Success: ' + successCount + ' | Failed: ' + failCount);
```

---

## Pattern 4: Contact Deactivation (Soft)

```apex
/**
 * Contact Deactivation
 * Target: Contacts with no activity in the past 365 days
 * Effect: Sets HasOptedOutOfEmail=true and a custom Active__c flag to false
 * NOTE: This does NOT delete contacts. Soft deactivation only.
 * Run in: SANDBOX FIRST.
 */

Integer BATCH_SIZE = 200;
Date inactivityThreshold = Date.today.addDays(-365);

List<Contact> inactiveContacts = [
    SELECT Id, Name, Email, LastActivityDate
    FROM Contact
    WHERE LastActivityDate < :inactivityThreshold
    AND HasOptedOutOfEmail = false
    // AND Active__c = true  // CONFIGURE: uncomment if you have an Active field
    LIMIT 10000
];

System.debug('Contacts to deactivate: ' + inactiveContacts.size);

Integer successCount = 0;
Integer failCount = 0;

for (Integer i = 0; i < inactiveContacts.size; i += BATCH_SIZE) {
    Integer endIdx = Math.min(i + BATCH_SIZE, inactiveContacts.size);
    List<Contact> batch = new List<Contact>;
    for (Integer j = i; j < endIdx; j++) {
        batch.add(new Contact(
            Id = inactiveContacts[j].Id,
            HasOptedOutOfEmail = true
            // Active__c = false  // CONFIGURE: uncomment if field exists
        ));
    }
    Database.SaveResult[] results = Database.update(batch, false);
    for (Database.SaveResult sr : results) {
        if (sr.isSuccess) {
            successCount++;
        } else {
            failCount++;
            System.debug('ERROR: ' + sr.getId + ' | ' + sr.getErrors[0].getMessage);
        }
    }
}

System.debug('Deactivation complete. Success: ' + successCount + ' | Failed: ' + failCount);
```

---

## Pattern 5: Duplicate Lead Merge (MergeLeads Pattern)

```apex
/**
 * Merge Duplicate Leads by Email
 * Merges duplicate leads keeping the oldest record as master
 * Limit: Database.merge supports max 2 duplicates per master call
 * Run in: SANDBOX FIRST. Merged records are NOT recoverable from Recycle Bin.
 */

Integer MERGE_BATCH_LIMIT = 200;  // Number of merges per run

// Find duplicate emails (requires pre-queried duplicate map — see notes)
// IMPORTANT: Build duplicate map outside script from a Data Loader export
// This example uses a hardcoded map for illustration
Map<String, List<Id>> duplicateMap = new Map<String, List<Id>>{
    // 'email@example.com' => [masterLeadId, duplicateLeadId1, duplicateLeadId2]
    // POPULATE from SOQL query or Data Loader export
};

// QUERY APPROACH (use when volume is manageable):
List<AggregateResult> dupeGroups = [
    SELECT Email, COUNT(Id) cnt, MIN(Id) masterId
    FROM Lead
    WHERE IsConverted = false
    AND Email != null
    GROUP BY Email
    HAVING COUNT(Id) > 1
    LIMIT :MERGE_BATCH_LIMIT
];

Integer mergeCount = 0;
Integer errorCount = 0;

for (AggregateResult ar : dupeGroups) {
    String email = (String) ar.get('Email');
    Id masterId = (Id) ar.get('masterId');

    List<Lead> dupes = [
        SELECT Id FROM Lead
        WHERE Email = :email
        AND Id != :masterId
        AND IsConverted = false
        LIMIT 2  // Database.merge supports max 2 duplicates per call
    ];

    if (!dupes.isEmpty) {
        try {
            Database.merge(new Lead(Id = masterId), dupes, false);
            mergeCount++;
        } catch (Exception e) {
            errorCount++;
            System.debug('Merge failed for master ' + masterId + ': ' + e.getMessage);
        }
    }
}

System.debug('Merges complete. Success: ' + mergeCount + ' | Failed: ' + errorCount);
```

**Important notes on merge:**
- `Database.merge` is NOT reversible — merged records are permanently combined
- Maximum 2 duplicate records per merge call
- Always run in sandbox first and verify the master selection logic

---

## Pattern 6: Lead Conversion

```apex
/**
 * Lead Conversion
 * Converts qualified leads to Contact + Account + Opportunity
 * Run in: SANDBOX FIRST.
 */

Integer BATCH_SIZE = 50;  // Lead conversion is more expensive — use smaller batch

List<Lead> leadsToConvert = [
    SELECT Id, FirstName, LastName, Company, Email, Status
    FROM Lead
    WHERE IsConverted = false
    AND Status = 'Qualified'  // CONFIGURE: your qualified status value
    LIMIT 200
];

System.debug('Leads to convert: ' + leadsToConvert.size);

List<Database.LeadConvert> conversions = new List<Database.LeadConvert>;
LeadStatus convertedStatus = [
    SELECT MasterLabel FROM LeadStatus WHERE IsConverted = true LIMIT 1
];

for (Lead l : leadsToConvert) {
    Database.LeadConvert lc = new Database.LeadConvert;
    lc.setLeadId(l.Id);
    lc.setConvertedStatus(convertedStatus.MasterLabel);
    lc.setDoNotCreateOpportunity(false);  // CONFIGURE: set true to skip Opp creation
    lc.setOverwriteLeadSource(false);
    conversions.add(lc);
}

Database.LeadConvertResult[] results = Database.convertLead(conversions, false);
Integer successCount = 0;
Integer failCount = 0;

for (Database.LeadConvertResult lcr : results) {
    if (lcr.isSuccess) {
        successCount++;
        System.debug('Converted: Lead ' + lcr.getLeadId +
            ' → Account ' + lcr.getAccountId +
            ' Contact ' + lcr.getContactId +
            ' Opportunity ' + lcr.getOpportunityId);
    } else {
        failCount++;
        System.debug('FAILED: ' + lcr.getLeadId + ' | ' + lcr.getErrors[0].getMessage);
    }
}

System.debug('Conversion complete. Success: ' + successCount + ' | Failed: ' + failCount);
```

---

## Governor Limit Reference

| Limit | Value | Notes |
|---|---|---|
| DML statements per transaction | 150 | Each `Database.update(list)` = 1 statement regardless of list size |
| DML rows per transaction | 10,000 | Total records across all DML statements |
| SOQL queries per transaction | 100 | Each `[SELECT ...]` in code = 1 query |
| SOQL rows per transaction | 50,000 | Total rows returned across all queries |
| Heap size | 6 MB (sync), 12 MB (async) | Large collections of full records can exceed this |
| CPU time | 10,000 ms (sync), 60,000 ms (async) | Complex loops approach this limit |

**For volumes > 10,000 records:** Use Apex Batch classes (`Database.Batchable`)
which run asynchronously with 200-record batches and governor limits per batch.
