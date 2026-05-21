# Automation Conflict Matrix Reference

Reference for understanding precedence, conflict scenarios, and interaction
patterns between Salesforce automation layers.

---

## Execution Order (Single Record Save)

The following is the standard Salesforce record save execution order:
<!-- verify-before-merge:2026-05-21 -->

```
1.  Load the original record from database (or initialize for new record)
2.  Load new field values from the incoming request
3.  Execute all Before-Save record-triggered Flows
4.  Execute all Before Apex Triggers
5.  Run system validations (page layouts, required fields, field formats)
6.  Save to database (not yet committed)
7.  Execute all After Apex Triggers
8.  Execute Assignment Rules (Leads and Cases)
9.  Execute Auto-Response Rules (Leads and Cases)
10. Execute Workflow Rules (LEGACY - end-of-life scheduled)
    - Field updates from workflow rules re-execute before/after triggers
11. Execute processes (Process Builder - LEGACY - end-of-life scheduled)
12. Execute all After-Save record-triggered Flows
13. Execute Escalation Rules
14. Execute Entitlement Rules
15. Commit to database
16. Execute post-commit logic (emails, outbound messages, async actions)
```

Workflow rule field updates cause triggers to re-fire (step 4-7 repeat). This
is a well-known source of unexpected behavior and is one reason Salesforce is
retiring Workflow Rules.

---

## Layer Comparison Matrix

| Capability | Apex Trigger | Before-Save Flow | After-Save Flow | Workflow Rule | Process Builder |
|------------|-------------|-----------------|----------------|--------------|----------------|
| Before-save field update | Yes | Yes | No | Yes (re-fires trigger) | No |
| After-save actions | Yes | No | Yes | Yes | Yes |
| Create child records | Yes | No | Yes | No | Yes |
| Cross-object update | Yes | No | Yes (via subflow) | Yes (1 hop) | Yes |
| Callouts | Yes (future) | No | No (unless async) | No | No |
| Complex logic (branching) | Yes | Yes | Yes | Limited | Yes (limited) |
| Recursion guard available | Yes (static var) | Conditional | Conditional | No | No |
| Governor limit shared | Yes | Yes | Yes | Yes | Yes |
| Testable in unit tests | Yes | Via mock | Via mock | Limited | Limited |

---

## Conflict Scenario Reference

### Scenario 1: Before-Save Flow vs Before Apex Trigger field conflict

When both a Before-Save Flow and a Before Apex Trigger update the same field,
the trigger executes AFTER the flow. The trigger's value wins.

```
Before-Save Flow: sets Account.Rating = 'Warm'
Before Apex Trigger: sets Account.Rating = 'Hot'

Result in database: Account.Rating = 'Hot'
```

Resolution: Decide which layer owns each field. Document ownership in metadata
comments. If the trigger's value should not override the flow, move field
assignment exclusively to the flow or guard the trigger logic with a field-change check.

### Scenario 2: Workflow Field Update triggers re-fire of Apex Trigger

Workflow Rules with field updates cause the entire before/after trigger cycle
to re-execute. This is the primary reason to migrate Workflow Rules to Flows.

```
Apex Trigger fires on Account update
  -> sends Platform Event (counted as one callout)
Workflow Rule field update re-fires trigger
  -> attempts to send Platform Event again
  -> if trigger lacks recursion guard, second Platform Event fires
  -> governor limit on callouts approaches
```

Resolution: Add recursion guard to trigger (see apex-anti-patterns.md). Or
migrate workflow field updates to Before-Save Flows to eliminate re-fire.

### Scenario 3: Two After-Save Flows update the same field

Multiple After-Save Flows on the same object and trigger condition will both
execute. If both update the same field, the last one to execute wins.

Execution order between multiple flows is not guaranteed and may change between
releases. Do not rely on specific ordering.

Resolution: Consolidate flows that modify the same field into one flow with
Decision elements to handle the branching logic.

### Scenario 4: Flow creates a record, Apex Trigger fires on creation

An After-Save Flow on Contact creates a related Task. The Task creation fires
the Task Apex Trigger. If the Task trigger updates the Contact, it can create
a loop.

```
Contact saved
  -> Contact After-Save Flow: Create Task
     -> Task Apex Trigger fires: updates Contact (sets ContactTaskCount__c++)
        -> Contact After-Save Flow fires again: creates another Task...
           RECURSION
```

Resolution:
- Add a trigger condition to the Contact flow: only fire if the triggering
  field (not ContactTaskCount__c) changed.
- Add a recursion guard in the Task trigger handler.

### Scenario 5: Validation Rule vs Before-Save Flow ordering

Validation Rules execute after Before-Save Flows but before database commit.
If a Before-Save Flow sets a field value that a Validation Rule then checks,
the updated value is visible to the Validation Rule.

```
Before-Save Flow: sets Status__c = 'Closed'
Validation Rule: ERROR IF Status__c = 'Closed' AND CloseDate__c = null
  -> Will fire and block save if CloseDate__c is null
```

This interaction is intentional and correct behavior. Document it in the
Validation Rule's description.

### Scenario 6: Assignment Rules vs After-Save Flow ownership

Case Assignment Rules run before After-Save Flows. If an After-Save Flow also
sets the OwnerId of a Case, the flow's OwnerId will overwrite the assignment
rule's OwnerId.

Resolution: Choose one authoritative ownership assignment mechanism. If
assignment rules are the authority, the flow must not touch OwnerId.

---

## Detecting Automation Conflicts

### CLI: List active automations on an object

```bash
# List all active Flows on Account
sf data query \
  --query "SELECT Label, ApiName, Status, TriggerType \
           FROM FlowDefinition \
           WHERE TriggerType IN ('RecordBeforeSave','RecordAfterSave') \
           AND Status = 'Active'" \
  --use-tooling-api \
  -o my-org

# List active Apex Triggers on Account
sf data query \
  --query "SELECT Name, Status, TableEnumOrId \
           FROM ApexTrigger \
           WHERE TableEnumOrId = 'Account' AND Status = 'Active'" \
  --use-tooling-api \
  -o my-org
```

### Conflict Analysis Checklist

- [ ] Identify all active automations (flows, triggers, workflow rules, processes)
  on the object being modified.
- [ ] Map which fields each automation reads and writes.
- [ ] Flag any field written by more than one automation layer (potential conflict).
- [ ] Identify any cross-object updates that could trigger secondary automations
  on related objects.
- [ ] Verify recursion guards exist for any automation that updates the same
  object or a related object that feeds back.

---

## Migration Status Reference

<!-- verify-before-merge:2026-05-21 -->

| Legacy Technology | Replacement | Salesforce Guidance |
|-------------------|------------|---------------------|
| Workflow Rules | Record-Triggered Flow | Retire by migration |
| Process Builder | Record-Triggered Flow | Retire by migration |
| Workflow Outbound Messages | Platform Events / Flow HTTP Callout | Migrate proactively |
| Approval Processes | Approval Processes (retained) | No migration needed |

Salesforce has announced retirement of Workflow Rules and Process Builder.
Check the official retirement timeline before using these in new development
or when assessing risk in existing orgs.
