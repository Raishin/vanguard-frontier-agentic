# Flow Anti-Patterns Reference

Common mistakes in Salesforce Flow <!-- verify-before-merge:2026-05-21 --> configurations that cause
performance failures, infinite recursion, data integrity issues, or
unhandled errors.

---

## 1. DML in a Loop Element

### Description
Using a "Create Records", "Update Records", or "Delete Records" element inside
a Loop element causes one DML operation per iteration — equivalent to DML in an
Apex loop.

### Why It Fails
Flows share Apex governor limits when invoked from a trigger context. A loop
over 200 records with an Update Records inside exceeds the DML statement limit.

### Detection
In Flow Builder: navigate to each Loop element and inspect elements inside the
loop body. Any Create/Update/Delete Records element is a finding.

### Correct Pattern

Move DML outside the loop:
1. Inside the loop, use an Assignment element to build a collection variable
   (`recordCollection`).
2. After the loop ends, use a single Update Records / Create Records element
   operating on the full collection.

```
Loop (iterate over accountList)
  -> Assignment: add account to toUpdateCollection (use "Add" operation)
End Loop
-> Update Records: Source = toUpdateCollection (one DML statement for all)
```

---

## 2. Missing Fault Paths

### Description
A Flow with callout actions, DML actions, or Apex actions has no Fault connector
from those elements. If the action fails, the flow terminates with an unhandled
fault — often surfaced to the user as a generic error page.

### Why It Matters
Silent failures in critical business processes (case creation, order submission)
create data integrity gaps that are hard to diagnose.

### Detection
In Flow Builder: click each Action, Create Records, Update Records, or Delete
Records element. Look for a red Fault connector (anchor at bottom of element).
If no Fault connector exists, that element has no fault handling.

### Correct Pattern

```
Create Records: Case
  Success path -> next element
  Fault path -> Fault Connector
    -> Assignment: set errorMessage = {!$Flow.FaultMessage}
    -> Create Records: Log Error (CustomErrorLog__c object)
    -> Screen (or Subflow): "Something went wrong. Your request ID: {!caseId}"
```

Minimum viable fault handling:
- Log the fault message to a custom error log object.
- Show a user-friendly message rather than a raw error.
- For autolaunched flows triggered from Apex: throw a re-throw-able exception
  so the Apex caller can handle it.

---

## 3. Infinite Recursion

### Description
A record-triggered flow updates the same record it was triggered on (or updates
a related record that triggers another flow that updates the original record),
creating a loop until Salesforce detects recursion and throws an error.

### Why It Happens
Flows triggered on Account > After Save that update an Account field will
re-trigger the same flow. Without a guard condition, this recurses.

### Detection
For each record-triggered flow:
1. Check the trigger object.
2. Check all Update Records or Create Records elements.
3. If any element updates a record of the same object type as the trigger, check
   whether the update will re-satisfy the trigger condition.

### Correct Patterns

**Pattern A: Entry condition guards**
Add a trigger condition that only fires when the relevant field has actually changed:
```
Trigger Conditions:
  Field: Status__c    Operator: Is Changed    Value: True
  (only runs when Status__c changes, not on every save)
```

**Pattern B: Timestamp / flag field guard**
```
Update Records: Account
  Set LastFlowProcessed__c = {!$Flow.CurrentDateTime}

Entry Condition:
  LastFlowProcessed__c < {!$Flow.CurrentDateTime} - 60 seconds
  (skip if flow already ran in the last minute for this record)
```

**Pattern C: Apex-managed recursion guard**
If flow invokes Apex, use a static boolean in the handler class (see Apex
recursion guard pattern in apex-anti-patterns.md).

---

## 4. System Context vs User Context Confusion

### Description
Autolaunched Flows and Before-Save Flows run in System Context by default —
they bypass sharing rules and FLS. This can expose records to flows that should
not be able to see or modify them.

### Execution Context Quick Reference

| Flow Type | Default Context | Override Available |
|-----------|---------------|-------------------|
| Autolaunched Flow (no trigger) | System w/ Sharing OFF | Set "Run as" to System with Sharing or User |
| Record-Triggered Flow (Before Save) | System w/ Sharing | Cannot change |
| Record-Triggered Flow (After Save) | System w/ Sharing | Cannot change |
| Screen Flow (launched by user) | User Context | Can switch to System |
| Scheduled-Triggered Flow | System w/ Sharing OFF | Cannot change |
| Platform Event-Triggered Flow | System w/ Sharing OFF | Cannot change |

<!-- verify-before-merge:2026-05-21 --> "Run as" context options may vary by API version and
Salesforce release.

### Risk Scenarios

1. Autolaunched Flow queries ALL records of an object regardless of sharing, then
   exposes results in a screen or action — data over-exposure.
2. Before-Save Flow updates a field on a restricted object without checking
   whether the running user should have edit access — implicit privilege escalation.

### Remediation
- For autolaunched flows that handle user data, set the Run As context to
  "System Context With Sharing" or invoke from Apex using `with sharing`.
- For flows that must run in system context, document the business justification
  and restrict the flow's invocation to trusted Apex classes only.

---

## 5. Unguarded Get Records (No Result Check)

### Description
A Get Records element queries for a record but has no Decision element to
check whether the record was found before using its fields downstream. If the
record does not exist, downstream elements that reference `{!GetRecord}` will
use null values, causing unexpected behavior or errors.

### Correct Pattern

```
Get Records: Account (WHERE Id = {!recordId})
-> Decision: Did We Find Account?
    YES (Is Null check: {!GetAccount} Is Null = False) -> proceed with account fields
    NO  (otherwise) -> Screen: "Account not found" or Fault logging path
```

---

## 6. Hard-Coded API Names

### Description
Flow elements reference picklist values, queue names, or record type API names
as hard-coded string literals. When these values change in production, the flow
breaks silently.

### Detection
Search Flow XML for string literals in Assignment/Decision elements:
```bash
grep -rn "Status = 'Active'" force-app/main/default/flows/*.flow-meta.xml
grep -rn "Queue\.Name" force-app/main/default/flows/*.flow-meta.xml
```

### Correct Approach
- Use Picklist field values via `{!$GlobalConstant.EmptyString}` or via Get
  Records to dynamically resolve names.
- Reference Queue IDs dynamically via Get Records on Group object:
  ```
  Get Records: Group WHERE Name = 'Support Queue' AND Type = Queue
  -> use {!GetQueue.Id} as OwnerId
  ```

---

## Flow Anti-Pattern Summary

| Anti-Pattern | Impact | PMD/Equivalent Rule |
|-------------|--------|---------------------|
| DML in loop | Governor limit failure | `FlowDmlInLoop` |
| Missing fault path | Silent failures | `FlowMissingFaultPath` |
| Infinite recursion | Recursion error | Manual review + entry conditions |
| System context over-exposure | Data security | Architecture review |
| Unguarded Get Records | Null field errors | `FlowUnusedVariable` (indirect) |
| Hard-coded API names | Brittleness | Code review checklist |
| Unactivated flow left in prod | Confusion / accidental activation | `FlowInactiveFlow` |
