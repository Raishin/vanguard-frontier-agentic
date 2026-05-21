# Apex Test Failure Diagnosis Reference

Adapted from forcedotcom/sf-skills running-apex-tests references (Apache-2.0).

## Common Failure Categories

### 1. DML Not Allowed on Setup Objects

**Error message:** `System.TestException: DML operation on setup object type 'User' is not permitted after you have updated a non-setup object type`

**Root cause:** Test method is trying to insert/update both setup objects (User, Group, GroupMember) and non-setup objects (Account, Contact, etc.) in the same transaction.

**Fix:**
```apex
// BAD — User and Account in same transaction
@isTest
static void testWithUserAndAccount() {
    User u = TestDataFactory.createUser(true);   // setup object
    Account acc = TestDataFactory.createAccount(true); // non-setup object — ERROR
}

// GOOD — use @TestSetup to pre-create User, then create Account in the test method
@TestSetup
static void setup() {
    TestDataFactory.createUser(true); // setup object in @TestSetup is isolated
}

@isTest
static void testWithUserAndAccount() {
    User u = [SELECT Id FROM User WHERE Username = 'test@example.com' LIMIT 1];
    Account acc = TestDataFactory.createAccount(true); // safe — no setup objects here
}
```

---

### 2. Governor Limit: Too Many SOQL Queries

**Error message:** `System.LimitException: Too many SOQL queries: 101`

**Root cause:** SOQL query inside a loop in production code; bulk test crosses the trigger
batch boundary (200+ records) and exposes the N+1 pattern.

**Fix in production code:**
```apex
// BAD — SOQL inside loop
for (Opportunity opp : opportunities) {
    Account acc = [SELECT Id FROM Account WHERE Id = :opp.AccountId]; // N+1
}

// GOOD — bulk pattern
Set<Id> accountIds = new Set<Id>();
for (Opportunity opp : opportunities) { accountIds.add(opp.AccountId); }
Map<Id, Account> accMap = new Map<Id, Account>(
    [SELECT Id, Name FROM Account WHERE Id IN :accountIds]
);
```

**Diagnosis from stack trace:** The stack trace points to the exact line with the SOQL
inside a loop. Route to `salesforce-apex-generator-skill` for the refactor.

---

### 3. Governor Limit: Too Many DML Statements

**Error message:** `System.LimitException: Too many DML statements: 151`

**Root cause:** DML operation inside a loop.

**Fix pattern:** Collect all records to DML outside the loop (same as SOQL pattern above).

---

### 4. CalloutException in Test Context

**Error message:** `System.CalloutException: Callouts not permitted from triggers, Schedulable.execute, batch start/execute/finish, standard approval process, or user interface.` OR `You have uncommitted work pending. Please commit or rollback before calling out.`

**Root cause:** Production code makes an HTTP callout but no callout mock is registered.

**Fix:**
```apex
// Register mock before Test.startTest()
Test.setMock(HttpCalloutMock.class, new MockHttpCallout(200, '{"status":"ok"}'));

Test.startTest();
MyService.callExternalSystem();
Test.stopTest();
```

See `references/async-testing.md` for full mock implementation patterns.

---

### 5. Async Result Not Visible (Missing Test.startTest/stopTest)

**Symptom:** Test passes but assertions on async job results are empty/null — no failures
during the test, but the job's side effects are not visible.

**Root cause:** The Queueable or Batch job was enqueued outside the `Test.startTest()` /
`Test.stopTest()` boundary, so the async queue was never flushed.

**Fix:**
```apex
// BAD — enqueue before startTest, never flushed
System.enqueueJob(new MyQueueable(ids));
// assertions here see no results

// GOOD
Test.startTest();
System.enqueueJob(new MyQueueable(ids));
Test.stopTest(); // flushes the async queue
// assertions here see completed results
```

---

### 6. QueryException: List has no rows for assignment

**Error message:** `System.QueryException: List has no rows for assignment to SObject`

**Root cause:** Test uses SOQL with `=` (single-SObject assignment) and the record does
not exist — often because `@TestSetup` did not create the expected data, or the test is
relying on org data with `SeeAllData=true` that is not present in CI.

**Fix:**
```apex
// BAD — single-row assignment
Account acc = [SELECT Id FROM Account WHERE Name = 'My Company'];

// GOOD — safe list + check
List<Account> accs = [SELECT Id FROM Account WHERE Name = 'My Company' LIMIT 1];
Assert.isFalse(accs.isEmpty(), 'Expected Account named My Company from setup');
Account acc = accs[0];
```

---

### 7. NullPointerException in Test

**Error message:** `System.NullPointerException: Attempt to de-reference a null object`

**Common causes:**
- Factory method returned null because `doInsert=false` and the caller expected an inserted record with an Id
- A relationship field was not queried but is accessed in the assertion
- `@TestSetup` data was not queried in the test method

**Fix:**
- Verify factory method creates and inserts records correctly
- Add the required relationship field to the SOQL query in setup
- Re-query the record by Id before asserting on fields populated by trigger/flow

---

### 8. MIXED_DML_OPERATION

**Error message:** `System.DmlException: MIXED_DML_OPERATION, DML operation on setup object type User is not allowed after you have updated a non-setup object type`

Same root cause as category 1. Use `@TestSetup` to separate setup-object DML
from non-setup-object DML.

---

### 9. Limit: CPU Time Exceeded

**Error message:** `System.LimitException: Apex CPU time limit exceeded`

**Root cause:** Test creates a very large data set or triggers a recursive loop in
production code that consumes more than 10,000ms CPU synchronously (or 60,000ms async).

**Diagnosis:** Identify the expensive operations in the stack trace. Common sources:
- String concatenation in a loop (use `List<String>` + `String.join()`)
- Nested loops over large collections
- Recursive trigger logic

Route to `salesforce-apex-generator-skill` with the stack trace for refactoring.

---

### 10. Heap Size Exceeded

**Error message:** `System.LimitException: Apex heap size too large: N`

**Root cause:** Test creates too many records in memory, or the production class is
accumulating large collections (List of all records, string buffers, etc.).

**Fix in production:** Chunk processing into smaller batches; avoid retaining full result
sets in instance variables; prefer lazy loading over eager loading.
