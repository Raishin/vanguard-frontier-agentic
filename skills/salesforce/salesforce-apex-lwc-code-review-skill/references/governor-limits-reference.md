# Governor Limits Reference

Comprehensive reference for Salesforce Apex governor limits organized by
execution context, with bulkification strategies and monitoring patterns.

<!-- verify-before-merge:2026-05-21 --> Verify all limit values against current Salesforce documentation
before citing these in audit findings — values may change between releases.

---

## Execution Contexts

| Context | How Triggered |
|---------|--------------|
| Synchronous Apex | Triggers, VF controllers, LWC Apex imperative calls, REST API |
| Asynchronous Apex | `@future`, `Queueable`, `Schedulable`, `Batchable` |
| Batch Apex execute() | Each `execute()` call in a `Database.Batchable` job |

---

## Per-Transaction Limit Table

| Resource | Synchronous | Asynchronous | Batch execute() |
|----------|------------|--------------|----------------|
| SOQL queries | 100 | 200 | 200 |
| SOQL rows returned total | 50,000 | 50,000 | 50,000 |
| DML statements | 150 | 150 | 150 |
| DML rows processed | 10,000 | 10,000 | 10,000 |
| CPU time (ms) | 10,000 | 60,000 | 60,000 |
| Heap size | 6 MB | 12 MB | 12 MB |
| Callouts (HTTP/SOAP) | 100 | 100 | 100 |
| Email invocations | 10 | 10 | 10 |
| Push notification calls | 10 | 10 | 10 |
| Future method calls | 50 | N/A | N/A |
| Queueable jobs enqueued | 50 | 1 (child) | 1 (child) |
| `System.schedule()` calls | 100 | 100 | 100 |
| Describe calls | 100 | 100 | 100 |
| Characters in dynamic SOQL | 20,000 | 20,000 | 20,000 |

---

## Daily Limits (Org-Wide)

| Resource | Limit |
|----------|-------|
| Async Apex executions | 250,000 per license (varies by edition) |
| `@future` calls from triggers | 200 per transaction; daily per org varies |
| Scheduled Apex concurrent jobs | 100 at any time |
| Batch jobs in Apex Flex Queue | 100 |
| Active Scheduled Jobs | 100 |

---

## Checking Limits Programmatically

```apex
// In Apex — available Limits class methods
System.debug('SOQL queries used: ' + Limits.getQueries());
System.debug('SOQL queries limit: ' + Limits.getLimitQueries());
System.debug('DML statements used: ' + Limits.getDMLStatements());
System.debug('DML rows used: ' + Limits.getDMLRows());
System.debug('CPU time used (ms): ' + Limits.getCpuTime());
System.debug('Heap size used (bytes): ' + Limits.getHeapSize());
System.debug('Callouts used: ' + Limits.getCallouts());

// Safety check pattern
public static Boolean isSafeToQuery(Integer reserveCount) {
    return (Limits.getLimitQueries() - Limits.getQueries()) > reserveCount;
}
```

---

## Bulkification Rules

### Rule 1: Collect, then DML
Never DML inside a loop. Collect all records to modify in a list, then DML once.

```apex
List<Contact> toUpdate = new List<Contact>();
for (Contact c : Trigger.new) {
    if (c.Email != null && !c.Email.contains('@')) {
        c.addError('Invalid email format');
    } else {
        toUpdate.add(new Contact(Id = c.Id, EmailVerified__c = true));
    }
}
if (!toUpdate.isEmpty()) {
    Database.update(toUpdate, false); // allOrNone = false for resilience
}
```

### Rule 2: Batch at 200 for DML
The DML per-batch row limit aligns with trigger batch size (200 by default).
For manual batching in anonymous Apex:

```apex
Integer BATCH_SIZE = 200;
for (Integer i = 0; i < records.size(); i += BATCH_SIZE) {
    Integer endIdx = Math.min(i + BATCH_SIZE, records.size());
    Database.update(records.subList(i, endIdx), false);
}
```

### Rule 3: Query Once, Map Results
```apex
// Query all related records in one SOQL
Map<Id, List<OpportunityLineItem>> linesByOpp = new Map<Id, List<OpportunityLineItem>>();
for (OpportunityLineItem oli : [
    SELECT Id, OpportunityId, Quantity, TotalPrice
    FROM OpportunityLineItem
    WHERE OpportunityId IN :oppIds
]) {
    if (!linesByOpp.containsKey(oli.OpportunityId)) {
        linesByOpp.put(oli.OpportunityId, new List<OpportunityLineItem>());
    }
    linesByOpp.get(oli.OpportunityId).add(oli);
}
```

---

## Batch Apex Sizing Guide

| Volume | Recommended Batch Size | Notes |
|--------|----------------------|-------|
| < 10,000 records | 200 (default) | Standard SOQL-queried batch |
| 10,000 – 1M records | 200 | Increase if processing is CPU-light |
| 1M+ records | 2,000 (Bulk API mode) | Use `Database.QueryLocator` with Bulk API |
| Heavy callouts per record | 1–10 | Callout limit is per transaction |
| Complex transformations | 50–100 | Watch CPU time |

```apex
Database.executeBatch(new MyBatchClass(), 200); // override default batch size
```

The `Database.QueryLocator` can return up to 50 million rows (vs. 50,000 for
standard SOQL in non-batch context).

---

## `@future` vs `Queueable` vs `Batch` Decision Guide

| Need | Best Choice |
|------|------------|
| Fire-and-forget, no result needed | `@future` |
| Chaining jobs, need result | `Queueable` |
| Process > 10,000 records | `Batchable` |
| Scheduled recurring | `Schedulable` |
| Callout from trigger | `@future(callout=true)` |
| Callout + chaining | `Queueable` implementing `Database.AllowsCallouts` |

---

## Heap Size Management

Heap limit is 6 MB synchronous, 12 MB asynchronous. Common causes of heap overflow:

- Large `String` or `Blob` values stored in variables.
- Accumulating records in a list when only a subset is needed.
- Deserializing large JSON payloads without streaming.

Mitigation:
```apex
// Release memory by nulling unused collections
List<SObject> bigList = fetchLargeDataset();
process(bigList);
bigList = null; // eligible for GC — reduces heap pressure

// Parse only needed JSON fields instead of full deserialization
Map<String, Object> parsed = (Map<String, Object>) JSON.deserializeUntyped(rawJson);
String neededField = (String) parsed.get('fieldName');
parsed = null;
```

---

## Error Handling with `Database.SaveResult`

Always use `allOrNone=false` in batch contexts to ensure partial success:

```apex
Database.SaveResult[] results = Database.insert(records, false);
List<String> errors = new List<String>();
for (Integer i = 0; i < results.size(); i++) {
    if (!results[i].isSuccess()) {
        for (Database.Error err : results[i].getErrors()) {
            errors.add('Record index ' + i + ': ' + err.getStatusCode() +
                ' - ' + err.getMessage() +
                ' Fields: ' + String.join(err.getFields(), ', '));
        }
    }
}
if (!errors.isEmpty()) {
    // Log to custom object, Platform Event, or System.debug
    System.debug(LoggingLevel.ERROR, String.join(errors, '\n'));
}
```
