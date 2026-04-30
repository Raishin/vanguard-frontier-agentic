# Vault Key Destruction — Rollback Playbook

## Cancel a scheduled key deletion (before time-of-deletion)

```bash
oci kms management key cancel-key-deletion \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>

# Verify cancellation
oci kms management key get \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query 'data.{state:"lifecycle-state", scheduledDeletion:"time-of-deletion"}'
```

## Re-enable the key after cancellation

```bash
oci kms management key enable \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

## Rotate to a new key version (non-destructive — old version remains available for decrypt)

```bash
oci kms management key create-key-version \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```

Old key versions remain ENABLED until explicitly disabled, allowing decryption of
data encrypted by prior versions. This is the safe rotation pattern.

## POINT OF NO RETURN

After `time-of-deletion` passes:

- HSM key: cryptographic material is wiped from the HSM. **Permanent. No recovery.**
- All data encrypted exclusively by this key version is **unrecoverable**.
- OCI Support Recovery SLA: **NONE**.
- Immediate escalation: open a P1 SR with OCI Support the moment accidental deletion is suspected.

Prevention checklist before scheduling deletion:
- [ ] All data encrypted by this key has been re-encrypted with the new key version
- [ ] All services using this key version have been updated to the new version
- [ ] A 30-day (not 7-day) deletion window was selected
- [ ] A second approver has confirmed the data-association audit
