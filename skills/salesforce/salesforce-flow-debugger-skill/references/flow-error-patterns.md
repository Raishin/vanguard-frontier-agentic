# Flow Error Patterns Reference

## Pattern 1: UNHANDLED_FAULT on Action Element

**Error signature:**
```
An unhandled fault has occurred in this flow
An unhandled fault has occurred while processing the flow with name <FlowName>.
Here's how to locate the problem:
Failed Flow Interviews section: <ElementName>
```

**Root cause:** An Action element (Send Email, Apex Action, HTTP Callout,
Create Records, Update Records, etc.) threw an exception and there is no
fault connector on that element.

**Fix:**
1. In Flow Builder, click the failing Action element
2. Open the "Add Fault Path" option (icon at the bottom of the element)
3. Connect the fault path to a Fault Handler subflow or a Screen element
   that displays `{!$Flow.FaultMessage}`
4. Deploy the updated Flow version

**Key note:** Every Action element that can fail in production MUST have a
fault connector unless the Flow is designed to surface unhandled errors to
the platform exception log intentionally.

---

## Pattern 2: NullPointerException on Assignment or Decision

**Error signature:**
```
NullPointerException: Attempt to dereference a null object
Element: <AssignmentOrDecisionName>
```

**Root cause variants:**
1. A variable used in an Assignment or Decision was never populated (loop
   ran zero iterations; GetRecords returned no records)
2. A collection variable element is accessed by index when the collection
   is empty
3. A record variable's field is accessed after a GetRecords that returned
   null

**Fix:**
1. Add a Decision element BEFORE the failing element to check if the
   variable or record is null:
   - `{!recordVar} Is Null → True` → fault path / error screen
   - `{!collectionVar} Is Empty → True` → skip the loop
2. For GetRecords: set "How Many Records to Store" to "Only the First Record"
   and enable "Automatically store all fields" — then add a null check on
   `{!recordVar}` before use
3. For Loops: ensure the loop's collection has at least one element before
   entering the loop (check collection size Decision)

---

## Pattern 3: DML Exception in Update Records / Create Records

**Error signature:**
```
FIELD_INTEGRITY_EXCEPTION: <field>: value not valid for org
REQUIRED_FIELD_MISSING: Required fields are missing: [<field>]
DUPLICATE_VALUE: duplicate value found
```

**Root cause variants:**
1. A required field on the record is null when Flow tries to update
2. A validation rule on the object is blocking the save
3. A duplicate management rule is firing
4. The record is locked (approval process lock or record lock)

**Fix:**
1. For required field: add an Assignment before Update Records to ensure
   the required field has a value; add a Decision to check first
2. For validation rule conflict: work with the admin to add a Flow-user
   bypass on the validation rule, or ensure Flow populates all required
   fields
3. For duplicate rule: check if Flow should merge or skip duplicates;
   update duplicate rule configuration
4. For record lock: add error handling and a custom notification to the
   approver

---

## Pattern 4: Governor Limit in Loop

**Error signature:**
```
CANNOT_INSERT_UPDATE_ACTIVATE_ENTITY: <object>: Too many SOQL queries: 101
CANNOT_INSERT_UPDATE_ACTIVATE_ENTITY: <object>: Too many DML statements: 151
```

**Root cause:** DML operations (Create Records, Update Records, Delete
Records) or subflows that query data are inside a Loop element. Each
iteration consumes one DML statement or SOQL query.

**Fix:**
1. Move all Get Records and Create/Update Records elements OUTSIDE the loop
2. Inside the loop: add items to a Record Collection variable
3. After the loop: use a single Create Records or Update Records element
   with "All records from collection"
4. Check if subflows called inside the loop perform DML — move DML to
   after the loop if possible

**Pattern: Bulk-safe loop structure**
```
Get Records → (populate collection)
Loop (iterate collection):
  → Assignment: add modified record to output collection
After loop:
  → Update Records: update all records in output collection (single DML)
```

---

## Pattern 5: Missing Fault Connector on Send Email

**Error signature:**
```
UNHANDLED_FAULT: Email send failed: invalid address <address>
SendEmailException: From address is not allowed in this context
```

**Root cause:** Flow attempts to send email to an invalid or restricted
address, but the Send Email element has no fault connector.

**Fix:**
1. Add a fault connector on the Send Email element
2. Log the fault message to a custom object field or platform event
3. Send a fallback admin notification using `{!$Flow.FaultMessage}`
4. Consider validating the email address in a Decision element before
   reaching Send Email (regex formula or ISBLANK check)

---

## Pattern 6: Record Not Found (GetRecords Returns Null)

**Error signature (implicit — no record, then null dereference follows):**
```
NullPointerException after GetRecords element
Variable <recordVar> has null value
```

**Root cause:** GetRecords found no matching records and returned null.
The next element attempts to use the record variable.

**Fix:**
1. After GetRecords, add a Decision element:
   - Condition: `{!recordVar} Is Null → True` → null handling branch
   - Default (record found) → continue the happy path
2. Alternatively, if GetRecords is set to collect into a collection, check
   if the collection `Is Empty` before the loop
3. For critical lookups (e.g., finding the Pricebook), add a fault screen
   with a user-friendly message

---

## Pattern 7: Type Mismatch in Assignment

**Error signature:**
```
INVALID_TYPE: Illegal assignment from <TypeA> to <TypeB>
```

**Root cause:** Attempting to assign a value of one type to a variable
of another type. Common examples:
- Assigning a Number to a Text variable without conversion
- Assigning a Date to a DateTime variable
- Assigning a record collection to a single record variable

**Fix:**
1. Use intermediate variables with the correct type
2. For Number → Text: use a Formula resource of type Text that references
   the Number variable
3. For Date → DateTime: use a Formula `DATETIMEVALUE(TEXT(dateVar))`
4. Verify the variable types in the Resources panel — check all variables
   used in Assignments

---

## Pattern 8: Recursive Flow / Loop Count Limit

**Error signature:**
```
FLOW_LOOP_COUNT_LIMIT: Maximum loop iterations exceeded
Recursive entry: Flow <FlowName> has been entered recursively
```

**Root cause:**
1. A Record-Triggered Flow updates a field on the same record, causing
   the trigger to fire again (recursion)
2. A Loop element exceeds the 2,000 iteration limit

**Fix for recursion:**
1. Add a custom checkbox field `Flow_Processed__c` to the object
2. Add an Entry Condition: only trigger the Flow when `Flow_Processed__c = False`
3. At the end of the Flow, set `Flow_Processed__c = True`
4. Alternatively, use `ISCHANGED(field)` entry conditions to limit when the
   Flow re-enters

**Fix for loop limit:**
1. Identify why the collection has > 2,000 items
2. Implement a Scheduled Flow with smaller batches
3. Move bulk processing to Apex

---

## Pattern 9: Subflow Not Found / Version Error

**Error signature:**
```
CANNOT_EXECUTE_FLOW_TRIGGER: <SubflowName>: active version not found
Flow Reference Error: <SubflowName> is not active
```

**Root cause:** The called subflow is not active in this org (may have been
deployed in a deactivated state or not deployed at all).

**Fix:**
1. In Setup → Flows, find `<SubflowName>` and verify it is Active
2. If not deployed, deploy it first (via Change Set or `sf project deploy`)
3. If there are multiple versions, verify the parent Flow references the
   correct version or "Latest Active Version"

---

## Pattern 10: Insufficient Access / FLS Error

**Error signature:**
```
INSUFFICIENT_ACCESS_ON_CROSS_REFERENCE_ENTITY: insufficient access rights on cross-reference id
FIELD_ACCESS_EXCEPTION: No access to field <FieldApiName>
```

**Root cause:** The user running the Flow (or the System Context setting)
does not have access to the object or field being read/written.

**Fix:**
1. Check the Flow's "Run As" context: is it "User" or "System Context with
   Sharing" or "System Context without Sharing"?
2. For Record-Triggered Flows that update related records: use
   "System Context without Sharing" if cross-object access is required
3. For Screen Flows that fail for specific users: check profile/permission
   set field-level security on the failing field
4. Route access investigation to `salesforce-permission-model-review-skill`
