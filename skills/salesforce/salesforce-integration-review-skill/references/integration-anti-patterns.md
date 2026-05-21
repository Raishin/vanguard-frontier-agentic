# Integration Anti-Patterns Reference

Common mistakes in Salesforce integration design that cause security
vulnerabilities, reliability failures, or operational problems.

---

## 1. Hardcoded Endpoints

### Description
API base URLs stored directly in Apex code rather than in Custom Settings,
Custom Metadata, or Named Credentials.

### Why It Is a Problem
- URL changes require code deployment (change set, package) rather than
  configuration change.
- Different environments (sandbox, UAT, production) have different endpoints,
  leading to hardcoded production URL in sandbox or vice versa.
- Security — endpoints are visible in code repositories.

### Detection
```bash
grep -rn "https://" --include="*.cls" force-app/main/default/classes/ | \
  grep -v "testEndpoint\|Named Credential\|callout:"
```

### Anti-Pattern
```apex
HttpRequest req = new HttpRequest();
req.setEndpoint('https://api.prod.vendor.example.com/v2/orders');
req.setMethod('POST');
```

### Correct Pattern
```apex
// Option A: Named Credential (preferred)
req.setEndpoint('callout:VendorAPI_Prod/v2/orders');

// Option B: Custom Metadata Type with per-environment record
VendorSettings__mdt settings = [
    SELECT Endpoint__c FROM VendorSettings__mdt
    WHERE DeveloperName = 'Production' LIMIT 1
];
req.setEndpoint(settings.Endpoint__c + '/v2/orders');
```

---

## 2. Missing Error Queues

### Description
Callouts that fail are logged only to `System.debug()` or silently swallowed.
Failed messages are permanently lost with no retry mechanism.

### Why It Is a Problem
Data integrity: records that should have been sent to an external system are
never sent. No visibility into failure rate or failure cause.

### Correct Pattern: Dead-Letter Queue via Platform Event

```apex
// On callout failure, publish to an error queue event
public static void sendToVendor(String payload) {
    HttpRequest req = buildRequest(payload);
    try {
        HttpResponse res = new Http().send(req);
        if (res.getStatusCode() >= 400) {
            publishToErrorQueue(payload, 'HTTP ' + res.getStatusCode(), 0);
        }
    } catch (System.CalloutException ex) {
        publishToErrorQueue(payload, ex.getMessage(), 0);
    }
}

private static void publishToErrorQueue(String payload, String error, Integer attempts) {
    Integration_Error__e errorEvent = new Integration_Error__e(
        Payload__c = payload,
        ErrorMessage__c = error,
        AttemptCount__c = attempts,
        SourceSystem__c = 'VendorAPI',
        OccurredAt__c = DateTime.now()
    );
    EventBus.publish(errorEvent);
}
```

A separate Platform Event-triggered flow or Apex subscriber picks up the
error event for monitoring and retry.

---

## 3. No Retry / No Backoff

### Description
Transient failures (HTTP 503, network timeout, rate limit 429) are not retried.
The integration gives up after the first failure.

### Why It Is a Problem
Transient errors are normal in distributed systems. Without retry, a 30-second
vendor outage causes permanent data loss.

### Retry Pattern with Exponential Backoff

```apex
public class RetryableIntegration implements Queueable, Database.AllowsCallouts {
    private final String payload;
    private final Integer attempt;
    private static final Integer MAX_ATTEMPTS = 5;
    private static final List<Integer> BACKOFF_SECONDS = new List<Integer>{
        2, 4, 8, 16, 32
    };

    public RetryableIntegration(String payload, Integer attempt) {
        this.payload = payload;
        this.attempt = attempt;
    }

    public void execute(QueueableContext ctx) {
        HttpRequest req = buildRequest(payload);
        HttpResponse res;
        try {
            res = new Http().send(req);
        } catch (System.CalloutException ex) {
            scheduleRetry(ex.getMessage());
            return;
        }

        if (res.getStatusCode() == 200 || res.getStatusCode() == 201) {
            // Success
            return;
        }

        if (isRetryable(res.getStatusCode())) {
            scheduleRetry('HTTP ' + res.getStatusCode());
        } else {
            // Non-retryable (400, 401, 403) — log and stop
            logPermanentFailure(payload, 'HTTP ' + res.getStatusCode());
        }
    }

    private Boolean isRetryable(Integer statusCode) {
        return statusCode == 429 || statusCode == 503 || statusCode == 504 || statusCode == 0;
    }

    private void scheduleRetry(String reason) {
        if (attempt >= MAX_ATTEMPTS) {
            logPermanentFailure(payload, reason);
            return;
        }
        // Enqueue next attempt (backoff is simulated via Scheduled Apex delay)
        System.enqueueJob(new RetryableIntegration(payload, attempt + 1));
    }

    private static void logPermanentFailure(String payload, String reason) {
        insert new Integration_Failure__c(
            Payload__c = payload.left(32000),
            Reason__c = reason,
            FailedAt__c = DateTime.now()
        );
    }
}
```

---

## 4. Blocking Callouts in Trigger Context

### Description
Making synchronous HTTP callouts directly inside an Apex trigger is not
allowed when the trigger is invoked by DML from another Apex context.
It also blocks the database transaction for the duration of the callout.

### Why It Fails
`System.CalloutException: You have uncommitted work pending`

Callouts are not allowed after DML has been issued in the same transaction.
Triggers always have DML pending (the record being saved).

### Anti-Pattern
```apex
trigger AccountTrigger on Account (after insert) {
    for (Account acc : Trigger.new) {
        // FAILS: callout in trigger context
        HttpRequest req = new HttpRequest();
        req.setEndpoint('callout:ExternalSystem/accounts');
        HttpResponse res = new Http().send(req); // throws CalloutException
    }
}
```

### Correct Pattern: Async via @future
```apex
trigger AccountTrigger on Account (after insert) {
    Set<Id> newAccountIds = Trigger.newMap.keySet();
    ExternalSystemSync.syncAccounts(newAccountIds);
}

public class ExternalSystemSync {
    @future(callout=true)
    public static void syncAccounts(Set<Id> accountIds) {
        List<Account> accounts = [SELECT Id, Name, BillingAddress FROM Account
                                   WHERE Id IN :accountIds];
        for (Account acc : accounts) {
            // Now safe to callout
            sendToExternalSystem(acc);
        }
    }
}
```

Or use Queueable for better chaining control.

---

## 5. Exposing Internal Error Details to External Systems

### Description
Error responses from Apex REST endpoints or integration handlers include
internal Salesforce error messages, stack traces, or org IDs in the
response body returned to external callers.

### Why It Is a Security Risk
Internal error messages reveal:
- Apex class names and methods (aids targeted exploitation).
- Object and field API names (aids SOQL injection attempts).
- Record IDs and count information (aids enumeration attacks).

### Anti-Pattern
```apex
@RestResource(urlMapping='/api/v1/orders/*')
global class OrderService {
    @HttpPost
    global static void createOrder() {
        try {
            // process order
        } catch (Exception e) {
            // VULNERABLE: exposes internal details
            RestContext.response.statusCode = 500;
            RestContext.response.responseBody = Blob.valueOf(e.getMessage() + '\n' + e.getStackTraceString());
        }
    }
}
```

### Correct Pattern
```apex
@RestResource(urlMapping='/api/v1/orders/*')
global class OrderService {
    @HttpPost
    global static void createOrder() {
        try {
            // process order
            RestContext.response.statusCode = 201;
        } catch (Exception e) {
            // Log internally; return generic message externally
            System.debug(LoggingLevel.ERROR, 'Order creation failed: ' + e.getStackTraceString());
            insert new Integration_Error__c(ErrorMessage__c = e.getMessage(), OccurredAt__c = DateTime.now());
            RestContext.response.statusCode = 500;
            RestContext.response.responseBody = Blob.valueOf('{"error":"Internal error. Contact support."}');
        }
    }
}
```

---

## Integration Anti-Pattern Summary

| Anti-Pattern | Finding Severity | Detection Method |
|-------------|-----------------|-----------------|
| Hardcoded endpoints in Apex | MEDIUM | grep for https:// in .cls files |
| No error queue / silent failures | HIGH | Review callout error handling |
| No retry for transient errors | MEDIUM | Review exception handling blocks |
| Synchronous callout in trigger | HIGH | Compiler error / code review |
| Internal error details in response | HIGH | Code review of REST resource classes |
| Credentials in code | CRITICAL | grep for password/token literals |
| Missing timeout on callout | MEDIUM | Check req.setTimeout() |
| Unbounded response size handling | MEDIUM | Check response body size before processing |
| No idempotency key on POST | MEDIUM | Review for duplicate-safe design |
| Missing rate limit handling | MEDIUM | Check for 429 response handling |
