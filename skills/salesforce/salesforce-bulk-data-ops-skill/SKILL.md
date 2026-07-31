---
name: salesforce-bulk-data-ops-skill
description: "Generates scripts for Salesforce bulk data operations: mass owner reassignment, record deduplication (Apex MergeOpportunities pattern), mass field update, batch close stale records, contact deactivation, and lead conversion. Outputs both Data Loader CSV transformation templates and Anonymous Apex scripts ready for sf apex run. T0 generation only — T2 dry-run execution requires salesforce-deployment-validator-skill. TRIGGER when: user says reassign owners, mass update field, deduplicate records, bulk close opportunities, mass deactivate contacts, data loader script, batch update, bulk reassign, clean up stale records. DO NOT TRIGGER when: single record updates (use salesforce-soql-explorer-skill for lookup then handoff), permanent hard-deletes (T3 prohibited, route to salesforce-live-guard-agent), schema or field changes (use salesforce-metadata-fetcher-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-21"
  category: generation
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# salesforce-bulk-data-ops-skill

Pure-generation T0 skill that produces production-safe scripts for bulk
Salesforce data operations. Outputs both:

1. **Data Loader CSV transformation templates** — ready-to-use column
   mappings and example CSV structures for the Salesforce Data Loader
2. **Anonymous Apex scripts** — safe for `sf apex run --file` in sandbox,
   with bulkification, error queues, partial success handling, and audit
   logging built in

Nothing is executed by this skill. Execution requires human review, sandbox
dry-run via `salesforce-deployment-validator-skill`, and production approval
via `salesforce-live-guard-agent`.

## When This Skill Owns the Task

Use `salesforce-bulk-data-ops-skill` when the work is to **generate the
script or template** for a bulk data operation:

- "Generate an Apex script to reassign all Opportunities from user A to user B"
- "Write a Data Loader CSV template to bulk update the Account rating field"
- "Create an Anonymous Apex script to close stale Opportunities older than 90 days"
- "Generate a deduplication script using the MergeOpportunities pattern"
- "Write a batch script to deactivate inactive Contacts"
- "Build a lead conversion script for marketing-qualified leads"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Single record lookup or spot update | `salesforce-soql-explorer-skill` → handoff |
| Permanent hard-delete of records | T3 prohibited — route to `salesforce-live-guard-agent` |
| Schema changes (add/remove fields) | `salesforce-metadata-fetcher-skill` |
| Sandbox dry-run execution of generated scripts | `salesforce-deployment-validator-skill` |
| Production approval for execution | `salesforce-live-guard-agent` |
| Field mapping for migration CSV | `salesforce-field-mapping-skill` |

---

## Required Context to Gather First

Before generating any bulk script, confirm:

1. **Operation type** — update, upsert, merge/deduplicate, or soft-delete.
   Never hard-delete (T3 prohibited).
2. **Target object** — standard (`Account`, `Opportunity`, `Contact`, `Lead`)
   or custom (`Custom_Object__c`).
3. **Filter criteria** — which records should be affected? Be specific
   (date range, owner, status field value, etc.)
4. **Volume estimate** — approximately how many records? Determines batching
   strategy (Apex batch classes vs. one-shot anonymous Apex).
5. **Key field for targeting** — what uniquely identifies the records?
   (Owner Id, Status, Age, External ID)
6. **Rollback plan** — has the user confirmed they have a backup or can
   use the Recycle Bin? Field History Tracking status?
7. **Sandbox first** — confirm the user will run in sandbox before
   production. Explicit warning required for production-bound scripts.
8. **Audit logging requirement** — should the script log changes to a
   custom audit object?

If any critical context is missing for the specific operation, ask before
generating.

---

## Recommended Workflow

### Step 1 — Classify the operation and select the right template

| Operation | Recommended approach |
|---|---|
| Owner reassignment (< 10k records) | Anonymous Apex with Database.update + allOrNone=false |
| Owner reassignment (> 10k records) | Apex Batch class |
| Mass field update (< 10k records) | Anonymous Apex OR Data Loader Update |
| Mass field update (> 10k records) | Data Loader Update with CSV OR Apex Batch |
| Deduplication / merge | Anonymous Apex using Database.merge (limit 3 records per merge call) |
| Bulk close stale records | Anonymous Apex with date filter and Database.update |
| Contact deactivation | Anonymous Apex (no delete — soft-deactivate via HasOptedOutOfEmail and custom status field) |
| Lead conversion | Anonymous Apex using Database.convertLead |

### Step 2 — Generate the backup strategy

Before generating the execution script, generate the backup query:

```apex
// Backup: export records before modification
List<Opportunity> backupRecords = [
    SELECT Id, OwnerId, StageName, CloseDate, Amount
    FROM Opportunity
    WHERE <your_filter>
    LIMIT 10000
];
// Export this list to CSV before running the update script
System.debug(JSON.serialize(backupRecords));
```

Include a Data Loader SELECT template to export the affected records
before modification.

### Step 3 — Generate the Apex script with safety patterns

Apply these mandatory patterns in every Apex script:
- Chunk records into batches of 200 (Apex DML limit)
- Use `Database.update(records, false)` (allOrNone=false for partial success)
- Collect and log errors from `Database.SaveResult[]`
- Do not perform DML inside loops
- Use governor-limit-safe chunking for large volumes

### Step 4 — Add audit logging

For every script that mutates records, add an audit log:

```apex
// Audit log pattern
List<Bulk_Op_Log__c> auditEntries = new List<Bulk_Op_Log__c>;
for (Database.SaveResult sr : results) {
    if (!sr.isSuccess) {
        auditEntries.add(new Bulk_Op_Log__c(
            Operation__c = 'MassOwnerReassign',
            Record_Id__c = sr.getId,
            Error_Message__c = sr.getErrors[0].getMessage,
            Timestamp__c = DateTime.now
        ));
    }
}
if (!auditEntries.isEmpty) {
    Database.insert(auditEntries, false);
}
```

### Step 5 — Generate Data Loader template if requested

Produce a Data Loader–ready CSV header row and example data row for the
operation. Include the `Id` column and the fields being updated.

### Step 6 — Emit structured output with T2/T3 caveats

Include explicit execution pathway guidance in the output.

---

## Quality Scoring Rubric (100-point)

Score every output before emitting. Threshold: 85+ ship, 70–84 ship with
caveat, below 70 reject and revise.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Bulkification** | 25 | No DML inside loops; records chunked to 200 per batch; collection-based update pattern used |
| **Error queue / partial success handling** | 20 | Database.update with allOrNone=false; SaveResult[] iterated; failed records collected and logged; script continues after partial failure |
| **Idempotency** | 15 | Script can be safely re-run without double-applying changes; uses a status field or processed flag; duplicate detection included |
| **Null handling** | 15 | All field accesses null-guarded; SOQL WHERE clause handles null fields; no NullPointerException risk |
| **Audit logging** | 15 | Every failure logged with record ID and error message; success count logged; custom audit object or System.debug used |
| **Rollback strategy** | 10 | Backup SOQL query provided; Recycle Bin / Field History notes included; hard-delete refused with T3 note |

**Scoring penalties:**
- DML inside a loop: -30 (automatic reject and revision required)
- allOrNone=true on a bulk operation: -20
- No backup query provided: -15
- Hard-delete operation generated: automatic reject (T3 prohibited)
- Script will fail on null field access: -15 per unguarded access

---

## T0 Contract / T2 Execution Note

This skill is **static-review (T0)** only:

- No Salesforce org connection required or permitted.
- No CLI commands are executed.
- Generated scripts are text only — not executed by this skill.

### T2 Dry-Run Execution Path

1. Review generated script with your admin team.
2. Run in sandbox using `sf apex run --file <script.apex> --target-org <sandbox-alias>`
   via `salesforce-deployment-validator-skill`.
3. Verify results in sandbox before proceeding.
4. For production execution: route to `salesforce-live-guard-agent` for
   human-approval workflow.

### T3 Prohibition

Hard-deletes (`Database.delete` followed by `Database.emptyRecycleBin`,
or `DELETELIST` in Data Loader) are **T3 prohibited**. This skill will
not generate hard-delete scripts. All delete patterns use Salesforce
native soft-delete (Recycle Bin).

---

## Refusal Triggers

Stop and decline if:

- The request is to generate a hard-delete script (`emptyRecycleBin`,
  `DELETELIST` without recycle bin). Explain T3 prohibition.
- The request is to execute the script directly on a production org
  (route to `salesforce-live-guard-agent`).
- The operation involves `ModifyAllData` bypass for a specific user
  context (flag as security concern).
- The target volume exceeds 1M records in an anonymous Apex context
  (recommend Apex Batch class or Data Loader scheduled job instead).
- The operation would permanently destroy personally identifiable
  information without a documented retention policy (requires compliance
  review before generating the script).

---

## Output Format

```yaml
verdict: "ship | ship-with-caveat | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

operation_summary:
  operation_type: "<owner_reassign|mass_field_update|deduplication|bulk_close|deactivate|lead_conversion>"
  target_object: "<SObjectApiName>"
  estimated_volume: "<user-provided estimate>"
  approach: "<anonymous_apex|apex_batch|data_loader>"

backup_query:
  description: "Export these records before running the script"
  soql: "<SELECT fields FROM Object WHERE filter>"
  data_loader_export_template: "<comma-separated API names for Data Loader export>"

apex_script:
  description: "<what the script does>"
  execution_command: "sf apex run --file <script.apex> --target-org <sandbox-alias>"
  code: |
    <apex script text>

data_loader_template:
  description: "<what the template is for>"
  operation: "<update|upsert|insert>"
  csv_header_row: "<comma-separated API names>"
  example_data_row: "<example values>"
  notes: "<any special Data Loader config notes>"

audit_pattern:
  included_in_script: <true|false>
  audit_object: "<Bulk_Op_Log__c or System.debug>"
  captures: ["record_id", "error_message", "timestamp"]

rollback_strategy:
  recycle_bin_recoverable: <true|false>
  field_history_tracked_fields: "<list if known>"
  manual_rollback_steps: "<step-by-step if recycle bin is insufficient>"
  backup_query_included: <true|false>

execution_pathway:
  step_1: "Review generated script with admin team"
  step_2: "Run in sandbox via salesforce-deployment-validator-skill"
  step_3: "Verify results in sandbox"
  step_4: "Route to salesforce-live-guard-agent for production approval"

t2_t3_notes:
  t2_sandbox_required: true
  t3_prohibited: "<hard-delete prohibited; this script uses soft-delete only>"
  production_approval_required: true

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

This is a T0 generation skill with no live org connection. Apply:

1. Never emit actual Salesforce record IDs in generated scripts — use
   placeholder IDs (e.g., `'005Xx000001REPLACE'`) and label them clearly
   as placeholders.
2. Do not embed real User IDs, Profile IDs, or Role IDs in scripts — use
   a SOQL sub-query to look them up dynamically:
   ```apex
   Id targetOwnerId = [SELECT Id FROM User WHERE Username = :targetUsername LIMIT 1].Id;
   ```
3. If the user pastes sample data containing real record IDs, do not
   reproduce them in the generated script or output.

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| Sandbox dry-run execution | `salesforce-deployment-validator-skill` |
| Production approval for execution | `salesforce-live-guard-agent` |
| Field mapping for the CSV template | `salesforce-field-mapping-skill` |
| Query to identify target records first | `salesforce-soql-explorer-skill` |
| Apex code review before execution | `salesforce-apex-lwc-code-review-skill` |

---

## Stop Conditions

- Request includes hard-delete (`emptyRecycleBin`) — stop, explain T3
  prohibition, suggest soft-delete with a custom `Is_Deleted__c` flag.
- Generated script would execute more than 10,000 DML operations in a
  single anonymous Apex run — stop, require switch to Apex Batch class
  pattern.
- Operation involves Schema changes (add/remove fields, change field
  type) — stop, route to `salesforce-metadata-fetcher-skill` then
  `salesforce-data-architecture-agent`.
- User requests that the script be run directly on production org without
  sandbox dry-run — stop and require sandbox validation first.

---

## Security Notes

- **T0 static generation only:** No org connection, no CLI execution, no
  MCP calls.
- **Soft-delete only:** Hard-deletes are T3 prohibited. All delete
  patterns produce Recycle Bin–recoverable deletes.
- **allOrNone=false required:** All bulk DML uses partial success mode to
  prevent all-or-nothing failures on large datasets.
- **Sandbox-first mandatory:** Every generated script includes an explicit
  note that sandbox dry-run is required before production execution.
- **No hardcoded IDs in scripts:** User IDs and record IDs are always
  looked up dynamically via SOQL to avoid stale ID references across
  sandbox/production environments.
- **Audit logging included:** Every script includes error logging to
  either a custom audit object or System.debug output.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/data-loader-templates.md` | Data Loader operation templates with CSV patterns |
| `references/anonymous-apex-patterns.md` | Database.update with allOrNone=false, error collection, batching |
| `references/rollback-strategy.md` | Backup patterns, soft-delete vs. hard-delete, Field History, Recycle Bin |
