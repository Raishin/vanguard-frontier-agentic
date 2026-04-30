# Autonomous DB Lifecycle — Preflight Commands

## 1. Get ADB state and confirm target

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query 'data.{name:"display-name", state:"lifecycle-state", cpu:"cpu-core-count", storage:"data-storage-size-in-tbs", version:"db-version", workload:"db-workload"}'
```

## 2. Audit protection tags (CRITICAL — check before any lifecycle op)

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query 'data.{definedTags:"defined-tags", freeformTags:"freeform-tags"}'
```

Stop if `Operations.Lifecycle = protected` is set on a defined-tag namespace.
Do not proceed with terminate or clone without explicit tag-removal approval.

## 3. Confirm recent backup exists

```bash
oci db autonomous-database-backup list \
  --autonomous-database-id <ADB_OCID> \
  --all \
  --query 'data[0:5].{id:id, type:type, state:"lifecycle-state", ended:"time-ended"}' \
  --output table
```

Fail-fast if no ACTIVE backup exists within RPO window before scale or stop operations.

## 4. Audit connection strings and consumer groups

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query 'data."connection-strings".{high:high, medium:medium, low:low}'
```

## 5. Check data guard and APEX linkage (termination blockers)

```bash
oci db autonomous-database get \
  --autonomous-database-id <ADB_OCID> \
  --query 'data.{dataGuard:"is-data-guard-enabled", autoScaling:"is-auto-scaling-enabled", apex:"apex-details"}'
```
