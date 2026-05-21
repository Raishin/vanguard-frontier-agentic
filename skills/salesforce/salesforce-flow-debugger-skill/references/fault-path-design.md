# Fault Path Design Reference

## When to Add a Fault Connector

Add a fault connector to ANY element that can fail in a way that should
not silently crash the Flow or expose a raw error to end users:

| Element type | Add fault connector? |
|---|---|
| Send Email | Always |
| HTTP Callout | Always |
| Apex Action | Always |
| Create Records | When creating in transactions the user initiates |
| Update Records | When updating non-trivial records (risk of validation rule) |
| Delete Records | Always |
| Get Records | Only if null result would be unacceptable |
| Decision | Never (no DML — cannot fault) |
| Assignment | Never (no DML — cannot fault) |
| Loop | Never (loop itself does not fault) |
| Screen | Never (unless custom components throw exceptions) |
| Subflow | When the subflow is known to have fault-able elements |

---

## Fault Path Termination Patterns

### Pattern 1: Fault Screen (Screen Flows)

Best for: User-facing Screen Flows where a human needs to know an error occurred.

```
Fault Connector
  ↓
Assignment: Store fault message
  {!varFaultMessage} = {!$Flow.FaultMessage}
  ↓
Screen: Error Display
  Text Component: "An error occurred: {!varFaultMessage}"
  Button: "Return to Home"
```

**Note:** Keep the error message user-friendly. Consider a generic message
for production and include `{!$Flow.FaultMessage}` only in sandbox/debug mode.

---

### Pattern 2: Custom Notification (Auto-Launched / Record-Triggered Flows)

Best for: Background Flows where an admin or ops team needs to know about failures.

```
Fault Connector
  ↓
Assignment: Build fault details
  {!varFaultRecord} = Record being processed
  {!varFaultMessage} = {!$Flow.FaultMessage}
  ↓
Create Records: FlowErrorLog__c (custom object)
  Flow_Name__c = "<FlowApiName>"
  Error_Message__c = {!$Flow.FaultMessage}
  Record_Id__c = {!triggerRecord.Id}
  Timestamp__c = {!$Flow.CurrentDateTime}
```

**Custom Object Setup:** Create `FlowErrorLog__c` with fields:
- `Flow_Name__c` (Text, 255)
- `Error_Message__c` (Long Text Area)
- `Record_Id__c` (Text, 18)
- `Timestamp__c` (DateTime)

---

### Pattern 3: Send Fault Email to Admin

Best for: Simple notification when custom logging object is not yet set up.

```
Fault Connector
  ↓
Assignment: Populate email body
  {!varEmailBody} = "Flow " & FlowApiName & " failed.\n" &
                   "Error: " & {!$Flow.FaultMessage} & "\n" &
                   "Record: " & {!triggerRecord.Id}
  ↓
Send Email:
  To: admin@company.com (use a named credential or Org-wide email)
  Subject: "Flow Error: <FlowName>"
  Body: {!varEmailBody}
```

**Note:** The Send Email element on the fault path should NOT have its
own fault connector (would create circular fault logic). Use
`$Flow.FaultMessage` to capture the original error only.

---

### Pattern 4: Platform Event for Monitoring Integrations

Best for: Enterprise orgs with centralized error monitoring.

```
Fault Connector
  ↓
Create Records: Flow_Error_Event__e (Platform Event)
  Flow_Name__c = "<FlowApiName>"
  Error_Message__c = {!$Flow.FaultMessage}
  Record_Id__c = {!triggerRecord.Id}
```

Platform Event subscribers (Apex trigger or external system) consume
the event for centralized logging.

---

## $Flow.FaultMessage Variable

The `{!$Flow.FaultMessage}` global variable contains the fault message
from the most recently failed element. Properties:

- **Type:** Text
- **Available:** Only on fault paths (not on the main path)
- **Content:** Platform-generated error string including element name,
  error code, and message
- **Max length:** Can be long — truncate before storing in a Text (255) field

**To use:** Simply reference `{!$Flow.FaultMessage}` in any Assignment,
Screen text, or email body on the fault path.

---

## Retry Strategies

### Immediate Retry (Screen Flows only)

Add a "Retry" button on the fault screen that loops back to the
beginning of the flow or the beginning of the failed section.

```
Fault Screen:
  "The save failed: {!varFaultMessage}"
  Button: "Try Again" → Go to: [Element before the failing action]
  Button: "Cancel" → End
```

**Caution:** Retries can cause duplicate DML if partial saves occurred.
Use an idempotency check (record status field) before retrying.

### Manual Escalation (Background Flows)

When a background Flow fails, create a Task or Case to route the failure
to a human for manual resolution:

```
Fault Connector
  ↓
Create Records: Task
  Subject = "Manual action required: Flow " & FlowApiName & " failed"
  Description = {!$Flow.FaultMessage}
  WhatId = {!triggerRecord.Id}
  OwnerId = [Admin User ID or Queue ID]
  Status = "Open"
  Priority = "High"
```

---

## Fault Path Anti-Patterns

| Anti-pattern | Why it fails | Fix |
|---|---|---|
| Fault path that immediately ends with no action | Silent failure — no one knows the flow crashed | Always log or notify on fault path |
| Fault path that calls another DML-heavy action | May trigger a second fault if the log creation fails | Keep fault path lightweight — only Assignment + Screen or log |
| Nested fault paths (fault on fault) | The inner fault element's fault connector is ignored by the platform | Keep fault paths to 1–2 elements; no fault connectors on fault path elements |
| Exposing raw `{!$Flow.FaultMessage}` to end users in production | Leaks technical error details and record context | Use a generic user message; log the full message to a custom object |
| No fault path on external API callout | Any network failure causes UNHANDLED_FAULT | Always add fault connector on HTTP Callout actions |
