# Key Vault Rotation & Purge — Rollback Playbook

## Recover a soft-deleted key (within retention window)

```bash
az keyvault key recover \
  --vault-name <VAULT_NAME> \
  --name <KEY_NAME>
```

## Recover a soft-deleted secret

```bash
az keyvault secret recover \
  --vault-name <VAULT_NAME> \
  --name <SECRET_NAME>
```

## Re-enable a previous key version (roll back to prior version as active)

```bash
az keyvault key set-attributes \
  --vault-name <VAULT_NAME> \
  --name <KEY_NAME> \
  --version <PREVIOUS_VERSION_ID> \
  --enabled true
```

## Restore rotation policy to previous settings

```bash
az keyvault key rotation-policy update \
  --vault-name <VAULT_NAME> \
  --name <KEY_NAME> \
  --value @rotation-policy-backup.json
```

## CANNOT ROLL BACK

- **Purge-protection enable**: once set, cannot be disabled on the vault.
- **Hard-purged key**: permanently destroyed. Data encrypted exclusively by this
  key version is unrecoverable. Escalate to incident response immediately.
- **Expired soft-delete retention + no purge-protection**: objects auto-purged
  after retention window expires with no recovery option.
