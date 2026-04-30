# Rollback Playbook: OCI Live Vault Key Destruction Guard

## Cancel a scheduled key version deletion (within window)

```bash
# List key versions in PENDING_DELETION state
oci kms management key-version list \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query "data[?\"lifecycle-state\"=='PENDING_DELETION'].{id:id,timeOfDeletion:\"time-of-deletion\"}"

# Cancel scheduled deletion
oci kms management key-version cancel-key-version-deletion \
  --key-id <KEY_OCID> \
  --key-version-id <KEY_VERSION_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

## Cancel a scheduled key deletion

```bash
oci kms management key cancel-key-deletion \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

## Re-enable a disabled key version

```bash
oci kms management key-version enable-key-version \
  --key-id <KEY_OCID> \
  --key-version-id <KEY_VERSION_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

## Rotate back to a prior key version (for data re-keying)

New encryption will use the current key version. Previously encrypted data encrypted
with an older version can still be decrypted as long as that version is enabled.

```bash
# Enable a previous version to allow decryption to continue
oci kms management key-version enable-key-version \
  --key-id <KEY_OCID> \
  --key-version-id <OLD_KEY_VERSION_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

## Rollback limitations

- **Deletion is permanent and irreversible** after the scheduled deletion time passes.
- Cancellation of a pending deletion is only possible BEFORE the `time-of-deletion` timestamp.
- There is no OCI Support escalation path to recover a destroyed key.
- Data encrypted with a deleted key version is permanently unreadable.
- Vault deletion (not key deletion) is also permanent — a deleted vault and all its keys cannot be recovered.
