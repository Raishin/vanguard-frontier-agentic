# Safety Checklist

## Pre-Restore Checklist (10 items)

Before executing any `velero restore create` command, confirm all 10 items. A single unchecked item is a HARD STOP unless the approver explicitly overrides with written justification.

- [ ] **1. Cluster context confirmed** — `kubectl config current-context` output has been shown and matches the intended target cluster. Do not assume the current context is correct.
- [ ] **2. Namespace scope is explicit** — `includedNamespaces` lists one or more specific namespaces. Empty list (`[]`) = cluster-wide restore = requires explicit platform-team sign-off with ticket reference.
- [ ] **3. Backup timestamp verified** — the backup name and creation timestamp have been confirmed as the correct recovery point. Do not restore from an older backup if a closer-in-time backup exists and is healthy.
- [ ] **4. Backup phase is Completed** — `velero backup describe <name>` shows `Phase: Completed`. Do not restore from a `PartiallyFailed` or `Failed` backup without explicit acknowledgment of the incomplete scope.
- [ ] **5. Pre-restore validation executed** — `velero backup describe <name> --details` has been reviewed for resource counts, namespace scope, and PV entries; for material restores, a trial restore was first executed on a non-production cluster and inspected with `velero restore describe <name> --details` and `velero restore logs <name>`. **Velero has no `--dry-run` flag on `velero restore create`** — never substitute it. (Exception: active P0 incident with explicit platform-team override and ticket reference.)
- [ ] **6. existingResourcePolicy reviewed** — if `existingResourcePolicy: update` is used, the approver understands this will overwrite live Secrets, ConfigMaps, RBAC objects, and ServiceAccounts in the target namespace.
- [ ] **7. PV restore posture confirmed** — `restorePVs: true/false` intent is explicit. If false, stateful applications will start with empty persistent volumes.
- [ ] **8. Current state captured** — target namespace resources have been exported (`kubectl get all,cm,secret,pvc -n <ns> -o yaml > pre-restore-state.yaml`) as a rollback artifact.
- [ ] **9. Explicit platform-team sign-off obtained** — approver name, role, and ticket/incident reference are documented. Not implied — must be explicit.
- [ ] **10. Post-restore verification plan exists** — the team knows which pods, endpoints, and data checks confirm successful restore before closing the incident.

---

## Pre-Schedule-Delete Checklist (5 items)

Before executing `velero schedule delete <name>` or removing a Schedule manifest:

- [ ] **1. Alternative backup source confirmed** — the namespaces covered by this Schedule are also covered by another Schedule or a manual backup strategy. Deleting the only backup Schedule for a production namespace is a HARD STOP.
- [ ] **2. Existing backups will not be deleted** — deleting a Schedule does not delete existing Backups by default. Confirm this is the intended behavior; if cascade-delete is intended, explicitly document which backups will be removed.
- [ ] **3. No in-progress backup from this schedule** — `velero backup get | grep InProgress` shows no active backup from this Schedule. Deleting a Schedule mid-backup can leave a partial backup with no retention management.
- [ ] **4. Dependent restore references reviewed** — no existing Restore objects reference backups created by this Schedule in a pending or future recovery plan.
- [ ] **5. Platform-team sign-off obtained** — explicit written approval with ticket reference. A Schedule deletion is irreversible (re-creation restores future backups but not the deleted Schedule's backup history lineage).

---

## Post-Restore Verification (5 items)

After a restore completes (`velero restore describe <name>` shows `Phase: Completed`):

- [ ] **1. Pod health confirmed** — all Deployments and StatefulSets in the restored namespace reach `Ready` state within the expected startup window. Check: `kubectl get pods -n <ns> -w`.
- [ ] **2. PVC binding confirmed** — all PersistentVolumeClaims are in `Bound` status. Unbound PVCs indicate snapshot restore failure or storage class mismatch. Check: `kubectl get pvc -n <ns>`.
- [ ] **3. Application data sampling** — spot-check application-level data integrity (e.g., query a database, verify a file, check an API endpoint). Pod running does not guarantee data consistency.
- [ ] **4. Service endpoints reachable** — Services and Ingress rules are routing traffic correctly. Check: `kubectl get svc,ingress -n <ns>` and a live probe to the application endpoint.
- [ ] **5. Restore warnings reviewed** — `velero restore logs <name>` has been scanned for warnings. Warnings about skipped resources, unresolved PV references, or hook failures must be triaged before marking the restore complete.
