# Preflight Commands: OCI Live Autonomous DB Lifecycle Guard

Run these before any ADB lifecycle mutation. Paste sanitized output as evidence.

## 1. Confirm identity and tenancy

```bash
oci iam region list --query "data[0]" 2>/dev/null || true
oci iam user get --user-id $(oci iam user list --query "data[0].id" --raw-output) \
  --query "data.{name:name,id:id}" 2>/dev/null
# Or check OCI config profile
cat ~/.oci/config | grep -E "^(user|tenancy|region|fingerprint)"
```

## 2. Get current ADB state

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query "data.{displayName:\"display-name\",lifecycleState:\"lifecycle-state\",ocpuCount:\"ocpu-count\",dataStorageSizeInTBs:\"data-storage-size-in-tbs\",freeformTags:\"freeform-tags\",definedTags:\"defined-tags\"}"
```

## 3. Verify protection tag is set (before any manage-verb operation)

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query "data.\"defined-tags\".Operations.Lifecycle"
# Must return "protected" on production ADBs
```

## 4. Confirm recent backup exists

```bash
oci db autonomous-database-backup list \
  --autonomous-database-id <ADB_OCID> \
  --sort-by TIMECREATED \
  --sort-order DESC \
  --limit 3 \
  --query "data[].{displayName:\"display-name\",lifecycleState:\"lifecycle-state\",timeStarted:\"time-started\",isAutomatic:\"is-automatic\"}"
```

## 5. Check wallet and connection strings (before scale/clone)

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query "data.{connectionStrings:\"connection-strings\",connectionUrls:\"connection-urls\"}"
```

## 6. Verify no in-flight operations

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query "data.{lifecycleState:\"lifecycle-state\",lifecycleDetails:\"lifecycle-details\"}"
# lifecycleState must be AVAILABLE before any mutation
```
