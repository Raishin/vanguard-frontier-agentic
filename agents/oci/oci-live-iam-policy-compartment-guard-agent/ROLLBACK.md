# IAM Policy Compartment — Rollback Playbook

## Restore previous policy statements

```bash
# Read backup statements from file saved in preflight step
PREV_STATEMENTS=$(cat /tmp/policy-backup-<TIMESTAMP>.json)

oci iam policy update \
  --policy-id <POLICY_OCID> \
  --statements "${PREV_STATEMENTS}" \
  --version-date $(date +%Y-%m-%d) \
  --force
```

## Verify policy restored correctly

```bash
oci iam policy get \
  --policy-id <POLICY_OCID> \
  --query 'data.{name:name, statements:statements, version:"version-date"}'
```

## Delete a newly-created incorrect policy immediately

```bash
oci iam policy delete \
  --policy-id <POLICY_OCID> \
  --force
```

WARNING: policy delete is **immediate and total** — all access granted by the policy
is revoked the moment the delete completes. This can cause service outages if the policy
granted runtime access to compute or database resources. Confirm blast radius before delete.

## Disable a dynamic group (remove matching rule to prevent new matches)

```bash
oci iam dynamic-group update \
  --dynamic-group-id <DG_OCID> \
  --matching-rule "None {instance.id = 'ocid1.instance.oc1.PLACEHOLDER'}"
```

This effectively empties the group without deleting it.
