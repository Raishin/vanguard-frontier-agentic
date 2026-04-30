# Key Vault Rotation & Purge — Preflight Commands

## 1. Check vault recovery state

```bash
az keyvault show \
  --name <VAULT_NAME> \
  --query "{softDeleteEnabled:properties.enableSoftDelete, purgeProtectionEnabled:properties.enablePurgeProtection, retentionDays:properties.softDeleteRetentionInDays, sku:sku.name}"
```

**STOP** if `purgeProtectionEnabled` is `null` or `false` and you are about to enable it.
Enabling purge-protection is **irreversible**. Get explicit written approval.

## 2. List key versions and active version

```bash
az keyvault key list-versions \
  --vault-name <VAULT_NAME> \
  --name <KEY_NAME> \
  --query "[].{version:kid, enabled:attributes.enabled, expires:attributes.expires, created:attributes.created}" \
  --output table
```

## 3. Show current rotation policy

```bash
az keyvault key rotation-policy show \
  --vault-name <VAULT_NAME> \
  --name <KEY_NAME>
```

## 4. List secrets with expiry audit

```bash
az keyvault secret list \
  --vault-name <VAULT_NAME> \
  --query "[].{name:name, expires:attributes.expires, enabled:attributes.enabled}" \
  --output table
```

## 5. Check for soft-deleted objects awaiting recovery or purge decision

```bash
az keyvault key list-deleted --vault-name <VAULT_NAME> --output table
az keyvault secret list-deleted --vault-name <VAULT_NAME> --output table
```
