# Apex Anti-Patterns Reference

Patterns that cause governor limit violations, security vulnerabilities, or
maintainability failures in Apex code.

---

## 1. SOQL in a Loop

### Why It Fails
Each SOQL query inside a loop consumes one query against the 100-per-transaction
governor limit. With even 10 records, a nested SOQL loop exhausts limits fast.

### Detection
```
grep -rn "\bfor\b.*{" --include="*.cls" .
# Then inspect each loop body for SELECT keywords
```
PMD rule: `AvoidSoqlInLoops`

### Bad Pattern
```apex
for (Account acc : accounts) {
    List<Contact> contacts = [SELECT Id FROM Contact WHERE AccountId = :acc.Id];
    // SOQL inside loop: LIMIT EXCEPTION at 101st account
}
```

### Correct Pattern
```apex
Set<Id> accountIds = new Map<Id, Account>(accounts).keySet;
Map<Id, List<Contact>> contactsByAccount = new Map<Id, List<Contact>>;

for (Contact c : [SELECT Id, AccountId FROM Contact WHERE AccountId IN :accountIds]) {
    if (!contactsByAccount.containsKey(c.AccountId)) {
        contactsByAccount.put(c.AccountId, new List<Contact>);
    }
    contactsByAccount.get(c.AccountId).add(c);
}

for (Account acc : accounts) {
    List<Contact> contacts = contactsByAccount.get(acc.Id) ?? new List<Contact>;
}
```

---

## 2. DML in a Loop

### Why It Fails
Each DML statement (insert/update/delete/upsert) inside a loop consumes one
DML statement against the 150-per-transaction limit and creates excessive CPU time.

### Detection
PMD rule: `AvoidDmlStatementsInLoops`

### Bad Pattern
```apex
for (Lead l : leadsToConvert) {
    Database.LeadConvert lc = new Database.LeadConvert;
    lc.setLeadId(l.Id);
    Database.convertLead(lc); // DML in loop
}
```

### Correct Pattern
```apex
List<Database.LeadConvert> conversions = new List<Database.LeadConvert>;
for (Lead l : leadsToConvert) {
    Database.LeadConvert lc = new Database.LeadConvert;
    lc.setLeadId(l.Id);
    lc.setConvertedStatus('Closed - Converted');
    conversions.add(lc);
}
Database.LeadConvertResult[] results = Database.convertLead(conversions);
for (Database.LeadConvertResult r : results) {
    if (!r.isSuccess) {
        System.debug('Conversion failed: ' + r.getErrors[0].getMessage);
    }
}
```

---

## 3. Missing `with sharing` Declaration

### Why It Is a Security Risk
Without `with sharing`, a class runs in system context, bypassing the org's
sharing rules. An LWC that calls such an Apex method can expose records the
user has no business accessing.

### Detection
```
grep -L "with sharing\|without sharing\|inherited sharing" src/classes/*.cls
```
Any class that has an `@AuraEnabled` or `@RemoteAction` method and no sharing
declaration is a finding.

### Patterns

```apex
// WRONG — implicit system mode
public class AccountController {
    @AuraEnabled
    public static List<Account> getAccounts {
        return [SELECT Id, Name FROM Account]; // returns ALL accounts
    }
}

// CORRECT — explicit sharing enforcement
public with sharing class AccountController {
    @AuraEnabled
    public static List<Account> getAccounts {
        return [SELECT Id, Name FROM Account]; // respects user's visibility
    }
}

// INTENTIONAL system mode — document why
public without sharing class IntegrationBatchProcessor {
    // System mode required: processes inbound integration records
    // that may belong to service users without record access.
}
```

---

## 4. Hardcoded IDs

### Why It Fails
Record IDs, User IDs, Profile IDs, and RecordType IDs differ between sandboxes
and production. Hardcoded IDs cause deployment failures or silent wrong-record
references.

### Detection
```
grep -rn "00[0-9A-Za-z]\{15,17\}" --include="*.cls" .
```

### Bad Pattern
```apex
Id adminProfileId = '00e000000000001'; // Profile ID: System Administrator
Id recordTypeId = '012000000000001';   // RecordType: Enterprise Account
```

### Correct Pattern
```apex
Id adminProfileId = [SELECT Id FROM Profile WHERE Name = 'System Administrator' LIMIT 1].Id;
Id recordTypeId = Schema.SObjectType.Account
    .getRecordTypeInfosByDeveloperName
    .get('Enterprise_Account')
    .getRecordTypeId;
```

For RecordType lookups, use `getRecordTypeInfosByDeveloperName` (API name is
stable) rather than `getRecordTypeInfosByName` (label is translatable and may
change).

---

## 5. Unhandled Governor Limit Approaching

### Description
Code does not check remaining limits before high-volume operations and will
throw `LimitException` in large orgs or during scheduled/batch peaks.

### Detection
Look for missing `Limits` class usage in batch or trigger contexts processing
large volumes.

### Safeguard Pattern
```apex
public class BulkProcessor {
    public static void processRecords(List<SObject> records) {
        Integer queriesRemaining = Limits.getLimitQueries - Limits.getQueries;
        if (queriesRemaining < 10) {
            System.debug(LoggingLevel.WARN,
                'Near SOQL limit before processing. Queries used: ' + Limits.getQueries);
            // Abort or defer remainder to async context
            return;
        }
        // Proceed with processing
    }
}
```

---

## 6. Trigger Logic in Trigger Body

### Why It Is a Problem
Placing business logic directly in trigger files (rather than in handler classes)
makes testing, debugging, and future refactoring difficult. It also breaks the
single-responsibility principle.

### Correct Architecture

```apex
// AccountTrigger.trigger — thin router only
trigger AccountTrigger on Account (before insert, before update, after insert, after update) {
    AccountTriggerHandler handler = new AccountTriggerHandler;
    if (Trigger.isBefore) {
        if (Trigger.isInsert) handler.onBeforeInsert(Trigger.new);
        if (Trigger.isUpdate) handler.onBeforeUpdate(Trigger.new, Trigger.oldMap);
    }
    if (Trigger.isAfter) {
        if (Trigger.isInsert) handler.onAfterInsert(Trigger.new);
        if (Trigger.isUpdate) handler.onAfterUpdate(Trigger.new, Trigger.oldMap);
    }
}

// AccountTriggerHandler.cls — business logic
public with sharing class AccountTriggerHandler {
    public void onBeforeInsert(List<Account> newAccounts) { ... }
    public void onAfterInsert(List<Account> newAccounts) { ... }
}
```

---

## 7. Missing FLS Enforcement in Apex

### Description
SOQL queries in Apex run in system context by default. `with sharing` enforces
row-level sharing but NOT Field-Level Security (FLS). Sensitive fields can be
returned even if the user's profile lacks read permission on those fields.

### Detection
Look for `@AuraEnabled` methods that return SObject records directly without
`Schema.SObjectField.getDescribe.isAccessible` checks.

### Correct Pattern
```apex
public with sharing class SecureContactController {
    @AuraEnabled
    public static Contact getContact(Id contactId) {
        // Enforce FLS before returning
        if (!Schema.SObjectType.Contact.fields.SSN__c.isAccessible) {
            throw new AuraHandledException('Insufficient field permissions.');
        }
        return [SELECT Id, Name, SSN__c FROM Contact WHERE Id = :contactId LIMIT 1];
    }
}
```

Alternatively, use `Security.stripInaccessible` which handles FLS in bulk:
```apex
SObjectAccessDecision decision = Security.stripInaccessible(
    AccessType.READABLE,
    [SELECT Id, Name, SSN__c FROM Contact WHERE Id = :contactId]
);
List<Contact> safeContacts = (List<Contact>) decision.getRecords;
```

---

## Governor Limits Quick Reference

| Limit | Synchronous | Asynchronous (Batch/Future/Queueable) |
|-------|------------|--------------------------------------|
| SOQL queries | 100 | 200 |
| SOQL rows returned | 50,000 | 50,000 |
| DML statements | 150 | 150 |
| DML rows | 10,000 | 10,000 |
| CPU time | 10,000ms | 60,000ms |
| Heap size | 6MB | 12MB |
| Callouts | 100 | 100 |
| Future method calls | 50 | N/A |
| Queueable jobs | 50 | 1 child per job |

Limits are subject to change per Salesforce release.
