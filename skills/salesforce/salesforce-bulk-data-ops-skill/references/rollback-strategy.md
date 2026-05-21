# Rollback Strategy Reference

## Rollback Options by Operation Type

| Operation | Rollback option | Complexity | Time limit |
|---|---|---|---|
| Update (field change) | Field History Tracking → manual revert CSV | Medium | Depends on FHT retention |
| Soft delete | Recycle Bin restore | Low | 15 days (default) |
| Merge | No native rollback | High | N/A — irreversible |
| Lead conversion | Partial — uncovert via API | High | No time limit but complex |
| Owner reassignment | Re-run script with reversed owner mapping | Low | No time limit |

---

## Backup Table Pattern

Before every bulk operation, export the affected records to a backup CSV.
This is the primary rollback mechanism.

### Step 1: Export before modification

```apex
// Use sf data query or Data Loader Export to capture current state
// Example query for before-state backup
SELECT Id, OwnerId, StageName, CloseDate, Amount, LastModifiedDate
FROM Opportunity
WHERE OwnerId = '<source_user_id>'
AND IsClosed = false
ORDER BY Id
```

**Data Loader Export steps:**
1. Open Data Loader → Export
2. Paste the backup SOQL query
3. Save to a timestamped file: `backup_opp_reassign_20260521_before.csv`
4. Keep the file until operation is verified complete

### Step 2: Store backup securely

- Store backup CSV in a change management ticket or SharePoint folder
- Name format: `backup_<object>_<operation>_<YYYYMMDD>_before.csv`
- Retain for minimum 30 days after operation

### Step 3: Export after modification for comparison

```
backup_opp_reassign_20260521_after.csv
```

Compare row counts and key field values to verify the operation succeeded
as expected.

---

## Salesforce Recycle Bin

Salesforce maintains a Recycle Bin for soft-deleted records.

| Property | Value |
|---|---|
| Retention period | 15 days (default) |
| Storage limit | 25× org storage limit |
| Bulk restore | Via Data Loader: Export All → filter IsDeleted=true |
| API restore | `Database.undelete` in Apex |
| Hard delete | `Database.emptyRecycleBin` — T3 prohibited |

### Restore deleted records via Apex

```apex
// Restore recently deleted Opportunities
List<Opportunity> deletedOpps = [
    SELECT Id FROM Opportunity WHERE IsDeleted = true
    AND CreatedDate = TODAY
    ALL ROWS  // Required to query deleted records
    LIMIT 1000
];

Database.undelete(deletedOpps, false);
System.debug('Restored: ' + deletedOpps.size + ' records');
```

### Restore deleted records via Data Loader

1. Export → Enable "Include deleted and archived records"
2. Query: `SELECT Id FROM Opportunity WHERE IsDeleted = true`
3. Save the ID list as CSV
4. Data Loader → Undelete operation (if available) or contact Salesforce Support

---

## Field History Tracking

Salesforce Field History Tracking records changes to specified fields for
up to 18 months (standard) or indefinitely with Salesforce Shield Field Audit Trail.

### How to use Field History for rollback

1. Before the bulk operation, verify which fields have Field History Tracking enabled
2. After the operation, query `OpportunityFieldHistory` (or `<Object>History`) to
   see field-by-field change records

```apex
// Query field history for a specific record after bulk update
List<OpportunityFieldHistory> history = [
    SELECT Field, OldValue, NewValue, CreatedDate, CreatedById
    FROM OpportunityFieldHistory
    WHERE OpportunityId = '<record_id>'
    AND CreatedDate = TODAY
    ORDER BY CreatedDate DESC
    LIMIT 100
];

for (OpportunityFieldHistory h : history) {
    System.debug(h.Field + ': ' + h.OldValue + ' → ' + h.NewValue);
}
```

### Rollback from Field History

To reverse a field change using Field History data:

1. Export `OpportunityFieldHistory` for affected records before operation
2. Extract `OldValue` for the changed field
3. Build a rollback CSV with `Id` and the old value
4. Run a Data Loader Update to restore the old values

**Limitation:** Field History `OldValue` is stored as text — type conversion
may be needed for Date, Number, and Lookup fields.

### Fields that support Field History Tracking

Admins can enable tracking on up to 20 fields per object (standard limit).
Verify which fields are tracked in Setup → Object Manager → <Object> → Fields
and Relationships → Set History Tracking.

---

## Soft-Delete vs. Hard-Delete Comparison

| Attribute | Soft delete (Database.delete) | Hard delete (emptyRecycleBin) |
|---|---|---|
| Recycle Bin recoverable | Yes (15 days) | No — permanent |
| Tier | T2 (sandbox), T3 (production with approval) | T3 prohibited for agents |
| Undo method | `Database.undelete` | None |
| Storage impact | Counts against Recycle Bin storage | Frees storage immediately |
| Compliance risk | Lower (reversible) | High (data destruction) |
| Use case | Operational cleanup | End-of-life data destruction per retention policy |

**Agent policy:** This skill only generates soft-delete scripts.
Hard-delete (`Database.emptyRecycleBin`) requires explicit human approval
via `salesforce-live-guard-agent` and a documented data retention policy.

---

## Sandbox vs. Production Risk Matrix

| Risk | Sandbox | Production |
|---|---|---|
| Incorrect filter causes unintended updates | Low — data is disposable | HIGH — may affect real deals |
| Governor limit exception causes partial update | Low — no business impact | HIGH — inconsistent data state |
| Merge operation on wrong master record | Low — reset sandbox | CRITICAL — irreversible |
| Owner reassignment notifies wrong users | Low — sandbox emails disabled | HIGH — customers/reps notified |

**Mandatory pattern:**
1. Run in sandbox → verify row counts and spot-check records
2. If verified → escalate to `salesforce-live-guard-agent` for production approval
3. After production run → compare before/after CSV exports

---

## Apex Batch Class Pattern for Large Volumes

For operations on > 10,000 records, use an Apex Batch class instead of
anonymous Apex to avoid governor limits.

```apex
/**
 * Batch class template for mass field update
 * Deploy to sandbox, execute via: Database.executeBatch(new MassFieldUpdateBatch, 200);
 */
public class MassFieldUpdateBatch implements Database.Batchable<sObject> {

    public Database.QueryLocator start(Database.BatchableContext bc) {
        return Database.getQueryLocator([
            SELECT Id, Industry
            FROM Account
            WHERE Industry = null
            AND Type = 'Customer - Direct'
        ]);
    }

    public void execute(Database.BatchableContext bc, List<Account> scope) {
        for (Account a : scope) {
            a.Industry = 'Other';  // CONFIGURE
        }
        Database.update(scope, false);
    }

    public void finish(Database.BatchableContext bc) {
        System.debug('Batch complete: ' + bc.getJobId);
    }
}

// Execute in Anonymous Apex:
// Database.executeBatch(new MassFieldUpdateBatch, 200);
```

Batch classes process 200 records per batch with independent governor limits,
allowing operations on millions of records safely.
