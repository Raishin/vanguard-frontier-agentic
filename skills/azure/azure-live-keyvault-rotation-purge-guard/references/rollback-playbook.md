# Rollback Playbook: Azure Live Key Vault Rotation Purge Guard

## Evidence-variable convention

Variables such as $APPROVED_AZURE_SCOPE, $ASSIGNEE_LOOKUP_VALUE, $ROLE_DEFINITION_NAME, $KEY_VAULT_NAME, and $KEY_VAULT_KEY_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

## Restore a key from backup

```bash
# Restore the key backup created during preflight
az keyvault key restore --vault-name $KEY_VAULT_NAME -f $KEY_VAULT_KEY_NAME-backup.json
```

Note: key backup/restore only works within the same geography and subscription security boundary.

## Re-enable a disabled key version

```bash
az keyvault key set-attributes --vault-name $KEY_VAULT_NAME -n $KEY_VAULT_KEY_NAME \
  --version <VERSION_ID> --enabled true
```

## Recover a soft-deleted key (before purge window expires)

```bash
# List soft-deleted keys
az keyvault key list-deleted --vault-name $KEY_VAULT_NAME

# Recover
az keyvault key recover --vault-name $KEY_VAULT_NAME -n $KEY_VAULT_KEY_NAME
```

## Revert rotation policy to previous settings

```bash
# Update rotation policy with restored values
az keyvault key rotation-policy update \
  --vault-name $KEY_VAULT_NAME \
  -n $KEY_VAULT_KEY_NAME \
  --value @previous-rotation-policy.json
```

## Rollback limitations

- **Purge is permanent and irreversible.** Once a key is purged, it cannot be recovered by any path.
- Purge protection prevents purge until the retention window expires — this is intentional and cannot be bypassed.
- Data encrypted with a deleted/rotated key becomes unreadable if the old key version is permanently deleted.
- Services using this key (disk encryption sets, CMK storage) must be re-keyed if the key version changes.
