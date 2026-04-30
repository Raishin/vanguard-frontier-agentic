# Autonomous DB Lifecycle — Rollback Playbook

## Start a stopped ADB (fastest recovery from accidental stop)

```bash
oci db autonomous-database start \
  --autonomous-database-id <ADB_OCID>

# Wait for AVAILABLE state
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query 'data."lifecycle-state"'
```

## Scale CPU back to previous count (scale-down is supported)

```bash
oci db autonomous-database update \
  --autonomous-database-id <ADB_OCID> \
  --cpu-core-count <PREVIOUS_CPU_COUNT>
```

WARNING: **Storage scale-up cannot be reversed on ADB.** Verify storage size before
scaling up — there is no reduce path once committed.

## Restore from backup after data-level issue

```bash
# Point-in-time recovery
oci db autonomous-database restore \
  --autonomous-database-id <ADB_OCID> \
  --timestamp "2026-04-29T10:00:00.000Z"
```

## Clone-to-new for investigation (non-destructive)

```bash
oci db autonomous-database create-from-clone \
  --compartment-id <COMPARTMENT_OCID> \
  --db-name "<CLONE_NAME>" \
  --source-id <ADB_OCID> \
  --clone-type FULL
```

## CANNOT ROLL BACK

- **Terminated ADB**: database and all backups are permanently deleted.
  No OCI Support recovery path exists.
- **Storage scale-up**: ADB storage can only grow, never shrink.
- **Prevention**: always verify `Operations.Lifecycle = protected` tag is set on prod ADBs.
