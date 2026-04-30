# Rollback Playbook: OCI Live Autonomous DB Lifecycle Guard

## Restore from automatic backup (after accidental stop or data issue)

```bash
# List available backups
oci db autonomous-database-backup list \
  --autonomous-database-id <ADB_OCID> \
  --query "data[].{id:id,displayName:\"display-name\",timeStarted:\"time-started\",type:type}"

# Restore to a specific backup
oci db autonomous-database restore \
  --autonomous-database-id <ADB_OCID> \
  --timestamp <ISO8601_TIMESTAMP>
# Example: --timestamp "2025-04-29T12:00:00.000Z"
```

## Start a stopped database

```bash
oci db autonomous-database start \
  --autonomous-database-id <ADB_OCID>

# Poll for AVAILABLE state
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query "data.\"lifecycle-state\""
```

## Scale CPU down (if over-provisioned)

```bash
oci db autonomous-database update \
  --autonomous-database-id <ADB_OCID> \
  --ocpu-count <TARGET_COUNT>
# Note: storage cannot be scaled down — only CPU is reversible
```

## Rollback limitations

- **Termination is permanent** — no recovery path exists after an ADB is terminated.
- **Storage scale-up is irreversible** — OCI does not shrink ADB storage after an increase.
- Point-in-time restore is available only within the automatic backup retention window (default: 60 days).
- Connection wallet files generated for the new ADB (after clone) are not interchangeable with the source ADB wallet.
