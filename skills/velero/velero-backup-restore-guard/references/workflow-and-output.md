# Workflow and Output Contract

## Pre-Operation Workflow

### Step 1 — Confirm cluster context

```bash
kubectl config current-context
kubectl config view --minify -o jsonpath='{.clusters[0].cluster.server}'
```

Never proceed if context is ambiguous, stale, or unconfirmed. This is a HARD STOP.

### Step 2 — Inventory the target

For restore operations:
```bash
velero backup get
velero backup describe <backup-name> --details
velero backup logs <backup-name>
```

For schedule review:
```bash
velero schedule get
kubectl get schedule <schedule-name> -n velero -o yaml
```

For BSL review:
```bash
velero backup-location get
kubectl get backupstoragelocation -n velero -o yaml
```

For volume snapshot locations:
```bash
velero snapshot-location get
kubectl get volumesnapshotlocation -n velero -o yaml
```

### Step 3 — Capture current state

Before any mutation, export the current state as a rollback artifact:

```bash
# Backup the Schedule
kubectl get schedule <schedule-name> -n velero -o yaml > schedule-backup-$(date +%Y%m%d%H%M%S).yaml

# Backup the BSL
kubectl get backupstoragelocation <bsl-name> -n velero -o yaml > bsl-backup-$(date +%Y%m%d%H%M%S).yaml
```

### Step 4 — Scope assessment for restores

Review the proposed Restore manifest or CLI flags:

```yaml
apiVersion: velero.io/v1
kind: Restore
metadata:
  name: myapp-restore-20260502
  namespace: velero
spec:
  backupName: myapp-backup-20260501
  includedNamespaces:
    - myapp-production        # MUST be explicitly scoped; [] = cluster-wide = HARD STOP
  excludedResources:
    - nodes
    - events
    - events.events.k8s.io
    - backups.velero.io
    - restores.velero.io
    - resticrepositories.velero.io
  existingResourcePolicy: none  # "update" overwrites live resources — requires sign-off
  restorePVs: true
```

### Step 5 — Pre-restore validation (mandatory in non-emergency scenarios)

> **Velero has no `--dry-run` flag on `velero restore create`.** Earlier guidance that suggested one was incorrect. Validate via `velero backup describe`, then trial-restore on a non-production cluster.

```bash
# 1. Inspect what the backup actually contains, including resource and PV counts
velero backup describe myapp-backup-20260501 --details

# 2. Trial-restore on a non-production cluster, then inspect the result
velero restore create myapp-restore-trial \
  --from-backup myapp-backup-20260501 \
  --include-namespaces myapp-production \
  --wait
velero restore describe myapp-restore-trial --details
velero restore logs myapp-restore-trial
```

Review the validation output for:
- Unexpected resource counts (compare `velero backup describe` against last-known production state).
- Resources that would be overwritten if `existingResourcePolicy: update`.
- Missing PV restore entries (check `restorePVs` and the volume snapshot list).
- Any partially-failed items in the trial-restore logs.

### Step 6 — Sign-off gate

Document sign-off clearly before proceeding. Required fields:
- Approver name and role
- Ticket or incident reference
- Recovery point objective confirmation (is this the correct backup timestamp?)
- Cluster context and namespace scope confirmation

### Step 7 — Execute restore

```bash
velero restore create myapp-restore-20260502 \
  --from-backup myapp-backup-20260501 \
  --include-namespaces myapp-production \
  --existing-resource-policy none
```

### Step 8 — Monitor restore progress

```bash
velero restore describe myapp-restore-20260502 --details
velero restore logs myapp-restore-20260502
kubectl get pods -n myapp-production -w
```

---

## Backup Schedule Review Workflow

### Reviewing schedule cadence against RPO

```bash
kubectl get schedule -n velero -o custom-columns=\
NAME:.metadata.name,\
CRON:.spec.schedule,\
TTL:.spec.template.ttl,\
LOCATION:.spec.template.storageLocation,\
NAMESPACES:.spec.template.includedNamespaces
```

Validation checklist:
- `spec.schedule` cron expression matches RPO requirement (e.g., hourly = `0 * * * *`)
- `spec.template.ttl` is not shorter than the retention SLA (default `720h` = 30d)
- `spec.template.includedNamespaces` does not omit stateful production namespaces
- `spec.template.storageLocation` references a BSL in the same region as the cluster for DR validity
- `spec.template.volumeSnapshotLocations` is set if PVs need snapshot coverage

### Reviewing hook coverage on stateful workloads

```yaml
# Example pre-backup hook to quiesce PostgreSQL
metadata:
  annotations:
    pre.hook.backup.velero.io/command: '["/bin/bash", "-c", "psql -U postgres -c CHECKPOINT;"]'
    pre.hook.backup.velero.io/container: postgres
    pre.hook.backup.velero.io/on-error: Fail
    pre.hook.backup.velero.io/timeout: 30s
    post.hook.backup.velero.io/command: '["/bin/bash", "-c", "echo backup complete"]'
    post.hook.backup.velero.io/container: postgres
```

Missing hooks on StatefulSets running PostgreSQL, MySQL, MongoDB, or Kafka = inconsistent backup. Flag as HIGH.

---

## BackupStorageLocation Change Workflow

Before changing a BSL:

```bash
# List all active backups and their storage location
velero backup get -o yaml | grep -E 'storageLocation|name:'

# Check for in-progress backups
velero backup get | grep InProgress

# Check which schedules reference this BSL
kubectl get schedule -n velero -o json | jq '.items[] | select(.spec.template.storageLocation == "<bsl-name>") | .metadata.name'
```

BSL credential review (IRSA/Workload Identity):
```bash
kubectl get backupstoragelocation <bsl-name> -n velero -o jsonpath='{.spec.credential}'
```

Expected: `credential.secretRef` using IRSA annotations. Flag if IAM user static credentials are used.

---

## Volume Snapshot TTL vs Backup TTL Alignment

```bash
velero backup describe <backup-name> --details | grep -A5 "Volume Snapshots"
```

Verify: volume snapshot TTL >= backup TTL. If backup TTL is 30d but snapshot TTL is 7d, restore from snapshot after day 7 will fail silently (snapshot gone, backup metadata present).

---

## Output Format

Return:

1. **Target and scope** — backup name, namespace scope, cluster context, BSL, timestamp
2. **Hard-stop assessment** — is this operation blocked? List exact rule triggered.
3. **Evidence level** — live evidence, documentation-based, or inference
4. **Approval status** — confirmed sign-off or pending
5. **Recommended command** — `velero backup describe --details` and a trial restore on a non-production cluster first, then execute on production
6. **Rollback posture** — saved state file, re-apply command
7. **Verification steps** — post-restore pod health, resource counts, PV binding status
8. **Open risks** — hook coverage gaps, snapshot TTL mismatches, BSL credential posture
