# Governor Limit Signatures and Remediation Reference

Adapted from forcedotcom/sf-skills debugging-apex-logs references (Apache-2.0).

## How to Find Limit Hits in a Log

Search for these patterns in log output:

```bash
# Find all limit-related entries
grep -i "LIMIT_USAGE\|LimitException\|FATAL_ERROR" apex-debug.log

# Find SOQL count entries
grep "Number of SOQL" apex-debug.log

# Find the fatal error line
grep -A 10 "FATAL_ERROR" apex-debug.log
```

---

## SOQL Query Limit (100 sync / 200 async)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Number of SOQL queries: 101 out of 100
FATAL_ERROR|System.LimitException: Too many SOQL queries: 101
Class.AccountSelector.getAccountsByOwnerIds: line 12, column 1
Class.AccountService.processOwnerChange: line 34, column 1
Trigger.AccountTrigger: line 8, column 1
```

**Diagnosis:** SOQL inside a loop. Stack trace points to the SOQL call site.
Count the `SOQL_EXECUTE_BEGIN` entries — if N entries exist for the same query
text with the same calling class/method, the query is in a loop.

**Remediation:**
```apex
// Before: SOQL in loop
for (Opportunity opp : opps) {
    Account acc = [SELECT Id FROM Account WHERE Id = :opp.AccountId];
}

// After: collect IDs, single SOQL, Map lookup
Set<Id> accIds = new Set<Id>();
for (Opportunity opp : opps) { accIds.add(opp.AccountId); }
Map<Id, Account> accMap = new Map<Id, Account>([SELECT Id FROM Account WHERE Id IN :accIds]);
for (Opportunity opp : opps) {
    Account acc = accMap.get(opp.AccountId);
}
```

---

## DML Statement Limit (150)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Number of DML statements: 151 out of 150
FATAL_ERROR|System.LimitException: Too many DML statements: 151
```

**Diagnosis:** DML (`insert`, `update`, `delete`) inside a loop.
Count `DML_BEGIN` entries with `Rows:1` — if many exist for the same object type,
DML is in a loop.

**Remediation:** Collect all records in a List before the loop, then DML the full list
outside the loop.

---

## CPU Time Limit (10,000ms sync / 60,000ms async)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|CPU time (ms): 10001 out of 10000
FATAL_ERROR|System.LimitException: Apex CPU time limit exceeded
```

**Diagnosis:** Identify CPU hotspots from cumulative elapsed time between METHOD_ENTRY
and METHOD_EXIT. Common causes:
- Nested loops over large collections
- String concatenation in loops (use `String.join()`)
- Excessive `JSON.serialize()` on large object graphs
- Regex operations on large strings
- Recursive trigger logic with deep call stacks

**Remediation:**
- Replace string concatenation loops with `List<String>` + `String.join()`
- Break nested loops into Map lookups
- Move heavy computation to Batch/Queueable async context
- Cache `JSON.serialize()` results if called repeatedly

---

## Heap Size Limit (6MB sync / 12MB async)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Maximum heap size (MB): 6.1 out of 6
FATAL_ERROR|System.LimitException: Apex heap size too large: 6291457
```

**Diagnosis:** Look for large collection allocations. Common causes:
- Querying all records without LIMIT (large result set held in memory)
- Accumulating records into a List in `Database.Stateful` batch without clearing
- Large string buffers
- Nested object graphs from `JSON.deserialize()`

**Remediation:**
- Add `LIMIT` to queries and chunk processing
- Clear intermediate collections when no longer needed (`myList.clear()`)
- Process in smaller batches in Batch Apex
- Avoid retaining full query result sets in instance variables across DML operations

---

## SOQL Row Limit (50,000)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Number of query rows: 50001 out of 50000
FATAL_ERROR|System.QueryException: Too many query rows: 50001
```

**Diagnosis:** Query returns too many rows. Usually a missing or insufficient WHERE clause.

**Remediation:**
- Add selective WHERE clause with indexed fields
- Add LIMIT for interactive/UI contexts
- Use Batch Apex with `Database.QueryLocator` for bulk processing — Batch bypasses the
  50,000 row query limit

---

## DML Row Limit (10,000)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Number of DML rows: 10001 out of 10000
FATAL_ERROR|System.LimitException: Too many DML rows: 10001
```

**Remediation:** Use Batch Apex to process records in chunks of ≤ 2,000 DML rows per
execute call. Default batch size of 200 stays well within limit.

---

## Callout Limit (100)

**Log signature:**
```
LIMIT_USAGE_FOR_NS|(default)|Number of callouts: 101 out of 100
FATAL_ERROR|System.LimitException: Too many callouts: 101
```

**Diagnosis:** HTTP/SOAP callouts inside a loop. Check `CALLOUT_REQUEST` entries.

**Remediation:**
- Batch API calls into a single request where the external API supports it
- Queue individual records and process in Queueable with a single bulk callout

---

## Near-Limit Warnings (75% threshold)

Even without a `FATAL_ERROR`, warn when `LIMIT_USAGE_FOR_NS` shows:
- SOQL queries > 75 (sync) or > 150 (async)
- DML statements > 112 (sync)
- CPU time > 7,500ms (sync)
- Heap > 4.5MB (sync)

These near-limit conditions are not failures today but are risks for future growth
as data volume increases.
