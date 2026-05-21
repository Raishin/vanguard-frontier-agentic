# Async Testing Patterns Reference

Adapted from forcedotcom/sf-skills generating-apex-test references (Apache-2.0).

## Test.startTest / Test.stopTest

Every async operation in a test must be wrapped with `Test.startTest` / `Test.stopTest`.

- `Test.startTest` — resets governor limits and marks the async boundary
- `Test.stopTest` — flushes the async queue (Queueable, future methods, batch jobs)
  and waits for all enqueued async work to complete before proceeding

```apex
@isTest
static void testQueueableJob {
    List<Account> accounts = TestDataFactory.createAccounts(5, true);

    Test.startTest;
    System.enqueueJob(new ProcessAccountsQueueable(
        new Map<Id, Account>(accounts).keySet
    ));
    Test.stopTest;

    // Assertions here run AFTER the Queueable has completed
    List<Account> updated = [SELECT Status__c FROM Account WHERE Id IN :accounts];
    for (Account acc : updated) {
        Assert.areEqual('Processed', acc.Status__c, 'Account should be processed');
    }
}
```

**Rule:** All synchronous setup (DML, SOQL for setup) goes before `Test.startTest`.
The code under test goes between `Test.startTest` and `Test.stopTest`.
Assertions go after `Test.stopTest`.

---

## Testing Queueable Jobs

```apex
@isTest
static void testOrderEventQueueable {
    List<Order__c> orders = TestDataFactory.createOrders(10, true);
    Set<Id> orderIds = new Map<Id, Order__c>(orders).keySet;

    Test.startTest;
    System.enqueueJob(new OrderEventQueueable(orderIds));
    Test.stopTest;

    // After stopTest, Queueable has executed
    List<Event_Log__c> logs = [SELECT Order_Id__c FROM Event_Log__c];
    Assert.areEqual(10, logs.size, 'Expected 10 event log entries');
}
```

---

## Testing Batch Jobs

`Test.startTest` / `Test.stopTest` also flushes batch execution:

```apex
@isTest
static void testAccountDeduplicationBatch {
    // Create duplicates
    List<Account> accs = TestDataFactory.createAccounts(200, true);

    Test.startTest;
    Database.executeBatch(new AccountDeduplicationBatch, 200);
    Test.stopTest;

    // Batch has run after stopTest
    Integer remaining = [SELECT COUNT FROM Account WHERE IsActive__c = true];
    Assert.isTrue(remaining < 200, 'Deduplication should have reduced count');
}
```

**Batch test limitations:**
- Only one batch execute cycle runs in test context (no multi-execute chunking)
- Batch size in tests can be set explicitly: `Database.executeBatch(new MyBatch, 1)` for
  single-record processing to test edge cases
- `Database.Stateful` instance variables persist across execute calls in test context

---

## Testing Schedulable Jobs

```apex
@isTest
static void testNightlyCleanupScheduler {
    Test.startTest;
    String jobId = System.schedule(
        'Test Nightly Cleanup',
        '0 0 2 * * ?',
        new NightlyCleanupScheduler
    );
    Test.stopTest;

    // Verify the job was scheduled
    CronTrigger ct = [SELECT Id, CronExpression, State FROM CronTrigger WHERE Id = :jobId];
    Assert.areEqual('WAITING', ct.State, 'Job should be in WAITING state');
    Assert.areEqual('0 0 2 * * ?', ct.CronExpression, 'Cron expression should match');
}
```

**Note:** `Test.stopTest` triggers a single execution of the Schedulable's `execute`
method. The Queueable or Batch that the Scheduler enqueues will also flush within
the `stopTest` boundary.

---

## Testing HTTP Callouts

Classes that make HTTP callouts require a mock implementation during tests.
The platform blocks real outbound callouts from test context.

**Step 1 — Create a mock class:**

```apex
@isTest
public class MockHttpCallout implements HttpCalloutMock {
    private Integer statusCode;
    private String responseBody;

    public MockHttpCallout(Integer statusCode, String responseBody) {
        this.statusCode = statusCode;
        this.responseBody = responseBody;
    }

    public HTTPResponse respond(HTTPRequest req) {
        HTTPResponse res = new HTTPResponse;
        res.setStatusCode(statusCode);
        res.setBody(responseBody);
        return res;
    }
}
```

**Step 2 — Register the mock and test:**

```apex
@isTest
static void testExternalApiCallout_Success {
    // Arrange — register mock before startTest
    String mockResponse = '{"status":"ok","id":"123"}';
    Test.setMock(HttpCalloutMock.class, new MockHttpCallout(200, mockResponse));

    Test.startTest;
    ExternalApiService.syncRecord('001Xx000001ABC');
    Test.stopTest;

    // Assert on results
    Sync_Log__c log = [SELECT Status__c FROM Sync_Log__c LIMIT 1];
    Assert.areEqual('Success', log.Status__c, 'Sync log should show Success');
}

@isTest
static void testExternalApiCallout_HandlesError {
    Test.setMock(HttpCalloutMock.class, new MockHttpCallout(503, '{"error":"Service Unavailable"}'));

    Test.startTest;
    try {
        ExternalApiService.syncRecord('001Xx000001ABC');
        Assert.fail('Expected CalloutException for 503 response');
    } catch (ExternalApiService.SyncException e) {
        Assert.isTrue(e.getMessage.contains('503'), 'Exception should mention status code');
    }
    Test.stopTest;
}
```

---

## Testing with StaticResourceCalloutMock

For complex response bodies, store them as Static Resources and use
`StaticResourceCalloutMock`:

```apex
@isTest
static void testCalloutWithStaticResource {
    StaticResourceCalloutMock mock = new StaticResourceCalloutMock;
    mock.setStaticResource('MockApiResponse'); // Static Resource name
    mock.setStatusCode(200);
    mock.setHeader('Content-Type', 'application/json');
    Test.setMock(HttpCalloutMock.class, mock);

    Test.startTest;
    MyApiService.fetchData;
    Test.stopTest;

    // assertions
}
```

---

## Multiple Async Jobs in One Test

If testing code that enqueues multiple async jobs:

```apex
@isTest
static void testMultipleQueueableJobs {
    // All jobs enqueued within Test.startTest/stopTest will execute
    Test.startTest;
    System.enqueueJob(new FirstQueueable);
    System.enqueueJob(new SecondQueueable);
    Test.stopTest;
    // Both jobs have completed here
}
```

**Governor limit in tests:** Only one Queueable job can be enqueued per `enqueueJob`
call from within test context. Chained Queueables (where Job A enqueues Job B)
may not all execute within a single `stopTest` call — test each job in isolation
when testing chains.
