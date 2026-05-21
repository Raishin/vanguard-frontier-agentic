# Fault Path Design Reference

Patterns for handling errors in Salesforce Flows with resilient fault paths,
escalation logic, and retry strategies.

---

## Why Fault Paths Are Required

Every Flow element that can fail — Create/Update/Delete Records, Apex Actions,
HTTP Callout Actions, Send Email elements — has a Fault connector in Flow
Builder. If no Fault connector is attached, an unhandled fault terminates the
flow with a generic error page and an email to the running user's org
admin. No audit record is created.

Required fault paths on:
- All DML elements (Create, Update, Delete, Upsert Records)
- All Apex Action elements
- All HTTP Callout actions (Salesforce Flow with External Services / MuleSoft)
- Send Email / Send Custom Notification
- Subflow elements

---

## Fault Path Levels

### Level 1: Minimum Viable Fault Path

Log the error, show a user-friendly message, do not leave the user on a broken screen.

```
DML Element: Create Records (Case)
  Fault Connector ->
    Assignment: errorMessage = {!$Flow.FaultMessage}
    Screen: "We couldn't complete your request. Please contact support."
            (show errorMessage in a non-technical format)
```

### Level 2: Fault Logging to Custom Object

Persist fault details for later diagnosis without developer log access.

```apex
// Custom object: FlowErrorLog__c
// Fields: FlowName__c, FaultMessage__c, RecordId__c, RunningUserId__c, Timestamp__c
```

Flow configuration:
```
Fault Connector ->
  Assignment: Set variables
    flowErrorName = "Case Creation Flow"
    faultMsg = {!$Flow.FaultMessage}
    faultUserId = {!$User.Id}
    faultTimestamp = {!$Flow.CurrentDateTime}
  Create Records: FlowErrorLog__c
    FlowName__c = {!flowErrorName}
    FaultMessage__c = {!faultMsg}
    RunningUserId__c = {!faultUserId}
    Timestamp__c = {!faultTimestamp}
  Screen: User-friendly error message
```

### Level 3: Escalation Flow

For critical business processes, route the failed transaction to a human queue.

```
Fault Connector ->
  Assignment: Set error variables
  Create Records: FlowErrorLog__c (persist fault)
  Create Records: Case
    Subject = "Automation Failure: {!flowErrorName}"
    Description = {!$Flow.FaultMessage}
    OwnerId = [Error_Handling_Queue Id]
    Priority = High
  Send Notification: Alert on-call team via Custom Notification
  Screen: "Your request has been escalated to our support team (Ticket: {!caseNumber})."
```

### Level 4: Retry Pattern (Platform Event-based)

For transient failures (network timeouts, external API 503), retry without
blocking the user.

```
Fault Connector ->
  Decision: Is this a retryable error?
    YES (fault message contains "timeout" or "503") ->
      Create Records: RetryQueue__e (Platform Event)
        FlowName__c = "Case Creation Flow"
        InputData__c = JSON.serialize(inputVariables)
        AttemptNumber__c = {!currentAttempt}
      Screen: "We're processing your request. You'll receive a notification when complete."
    NO ->
      Create Records: FlowErrorLog__c
      Screen: "Unrecoverable error. Please contact support."
```

A Platform Event-triggered flow or Apex subscriber processes the RetryQueue event
with exponential backoff.

---

## Retry Backoff Strategy

```apex
// Apex implementation of exponential backoff for Flow-invoked callouts
public class RetryableCalloutHandler implements Queueable, Database.AllowsCallouts {
    private Integer attemptNumber;
    private String payload;
    private String endpoint;
    private Integer maxAttempts = 5;

    public void execute(QueueableContext ctx) {
        try {
            HttpRequest req = new HttpRequest;
            req.setEndpoint(endpoint);
            req.setMethod('POST');
            req.setBody(payload);
            req.setTimeout(30000);
            HttpResponse res = new Http.send(req);

            if (res.getStatusCode >= 500) {
                throw new CalloutException('Server error: ' + res.getStatusCode);
            }
            // Success: process response
        } catch (Exception e) {
            attemptNumber++;
            if (attemptNumber < maxAttempts) {
                // Backoff: 2^attemptNumber * 1000ms (via Platform Event delay)
                // Enqueue next retry (Queueable cannot sleep, use Scheduled Apex or PE)
                System.enqueueJob(new RetryableCalloutHandler(
                    attemptNumber, payload, endpoint, maxAttempts
                ));
            } else {
                // Max retries reached - log and alert
                insert new FlowErrorLog__c(
                    FlowName__c = 'RetryableCallout',
                    FaultMessage__c = 'Max retries reached: ' + e.getMessage,
                    Timestamp__c = DateTime.now
                );
            }
        }
    }
}
```

Backoff schedule:
| Attempt | Wait Before Retry |
|---------|------------------|
| 1 | 2 seconds |
| 2 | 4 seconds |
| 3 | 8 seconds |
| 4 | 16 seconds |
| 5 | Fail permanently |

---

## Fault Message Parsing

`{!$Flow.FaultMessage}` contains the Salesforce error string. Parse common
patterns to determine error category:

| Fault Message Pattern | Category | Recommended Action |
|----------------------|----------|-------------------|
| `UNABLE_TO_LOCK_ROW` | Database lock contention | Retry after backoff |
| `FIELD_CUSTOM_VALIDATION_EXCEPTION` | Validation rule fired | Do not retry; show validation message to user |
| `INSUFFICIENT_ACCESS` | Sharing/FLS violation | Do not retry; escalate to admin |
| `DUPLICATE_VALUE` | Duplicate rule fired | Do not retry; show duplicate management UI |
| `REQUEST_RUNNING_TOO_LONG` | CPU timeout | Break into smaller batches |
| `System.CalloutException` | Network issue | Retry with backoff |
| `FIELD_INTEGRITY_EXCEPTION` | Foreign key violation | Check lookup target exists |

---

## Fault Path Design Checklist

- [ ] Every DML element has a Fault connector.
- [ ] Every Apex Action element has a Fault connector.
- [ ] Every External Service / HTTP Callout element has a Fault connector.
- [ ] Fault paths log `{!$Flow.FaultMessage}` to a persistent store (not just `System.debug`).
- [ ] Fault paths for screen flows show a user-friendly message (not raw error text).
- [ ] Fault paths for autolaunched flows create an error log record and/or send
  a notification to an operations team.
- [ ] Critical process fault paths create a support case or alert ticket.
- [ ] Transient error types trigger a retry mechanism; permanent errors are logged.
- [ ] Fault handling does not itself perform DML that could fail (double-fault
  scenario creates silent failure).
