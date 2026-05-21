# Governor Limits Reference

Adapted from forcedotcom/sf-skills generating-apex references (Apache-2.0).

## Key Per-Transaction Limits

| Limit | Synchronous | Asynchronous |
|---|---|---|
| SOQL queries | 100 | 200 |
| SOQL query rows returned | 50,000 | 50,000 |
| DML statements | 150 | 150 |
| DML rows processed | 10,000 | 10,000 |
| CPU time (ms) | 10,000 | 60,000 |
| Heap size (MB) | 6 | 12 |
| Callouts (HTTP/SOAP) | 100 | 100 |
| Queueable jobs enqueued | 50 | 1 (from execute) |
| Future method calls | 50 | 0 |
| Batch jobs submitted | 100 | — |
| Email invocations | 10 | 10 |

---

## Bulkification Patterns

### The Core Pattern: Collect First, Execute Once

**Anti-pattern (SOQL in loop):**
```apex
// BAD — hits SOQL limit after 100 iterations
for (Opportunity opp : opportunities) {
    Account acc = [SELECT Id, Name FROM Account WHERE Id = :opp.AccountId];
    // ...
}
```

**Correct pattern:**
```apex
// GOOD — single SOQL regardless of collection size
Set<Id> accountIds = new Set<Id>;
for (Opportunity opp : opportunities) {
    accountIds.add(opp.AccountId);
}
Map<Id, Account> accountMap = new Map<Id, Account>([
    SELECT Id, Name FROM Account WHERE Id IN :accountIds
]);
for (Opportunity opp : opportunities) {
    Account acc = accountMap.get(opp.AccountId);
    // ...
}
```

### DML Outside Loops

**Anti-pattern:**
```apex
// BAD — each insert is a separate DML statement
for (Contact c : contacts) {
    insert c; // hits DML limit after 150
}
```

**Correct pattern:**
```apex
// GOOD — one DML statement for all records
List<Contact> toInsert = new List<Contact>;
for (Account acc : accounts) {
    toInsert.add(new Contact(LastName = 'Test', AccountId = acc.Id));
}
insert toInsert;
```

---

## Async Fallback Strategies

When a synchronous operation would exceed governor limits, use async patterns:

### Pattern 1: Queueable for single async unit

```apex
// In trigger/service: enqueue instead of executing inline
if (Limits.getQueueableJobs < Limits.getLimitQueueableJobs) {
    System.enqueueJob(new ProcessLargeDataSetQueueable(recordIds));
}
```

### Pattern 2: Batch for large-volume processing

Threshold: use Batch when processing > 10,000 records or when multiple related
SOQL/DML operations per record would compound limit usage.

```apex
Database.executeBatch(new MyDataProcessingBatch, 200);
// batch size 200 is the default trigger batch size
```

### Pattern 3: Chunking with chunked Queueables

For recursive or chained processing:

```apex
public class ChunkedProcessorQueueable implements Queueable {
    private final List<Id> remaining;
    private static final Integer CHUNK_SIZE = 100;

    public ChunkedProcessorQueueable(List<Id> ids) {
        this.remaining = ids;
    }

    public void execute(QueueableContext ctx) {
        List<Id> chunk = new List<Id>;
        for (Integer i = 0; i < Math.min(CHUNK_SIZE, remaining.size); i++) {
            chunk.add(remaining[i]);
        }
        // process chunk
        List<Id> next = remaining.subList(chunk.size, remaining.size);
        if (!next.isEmpty) {
            System.enqueueJob(new ChunkedProcessorQueueable(next));
        }
    }
}
```

---

## Limit Awareness in Code

Always check limits before triggering async or bulk operations in high-frequency paths:

```apex
// Check remaining SOQL before proceeding
if (Limits.getQueries >= 95) {
    // near limit — log and enqueue for async processing
    System.enqueueJob(new DeferredProcessingQueueable(ids));
    return;
}
```

Use `Limits` class methods: `Limits.getQueries`, `Limits.getDMLStatements`,
`Limits.getCpuTime`, `Limits.getHeapSize` for runtime limit introspection.

---

## Large Collection Anti-Patterns

### Avoid Map-of-Lists when Map is sufficient

```apex
// Wasteful: Map<Id, List<Contact>> when only one Contact per Account expected
// Better: Map<Id, Contact> with null-check on duplicate
```

### Avoid SOQL aggregates when a Map covers the use case

```apex
// Expensive: COUNT SOQL for simple existence checks
// Better: check map.containsKey(id) after a single IN-query
```

### Avoid String concatenation in loops (CPU + heap pressure)

```apex
// BAD
String result = '';
for (String s : items) {
    result += s + ','; // O(n²) string allocation
}

// GOOD
List<String> parts = new List<String>;
for (String s : items) {
    parts.add(s);
}
String result = String.join(parts, ',');
```
