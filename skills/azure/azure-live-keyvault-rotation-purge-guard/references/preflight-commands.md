# Preflight Commands: Azure Live Key Vault Rotation Purge Guard

Run these before any Key Vault rotation or purge operation.

## 1. Confirm identity and vault target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az keyvault show -n <vault-name> -g <resource-group-name> \
  --query "{name:name, enableSoftDelete:properties.enableSoftDelete, enablePurgeProtection:properties.enablePurgeProtection, softDeleteRetentionInDays:properties.softDeleteRetentionInDays}"
```

## 2. List key versions and identify current/active

```bash
az keyvault key list-versions --vault-name <vault-name> -n <KEY_NAME> \
  --query "[].{kid:kid, enabled:attributes.enabled, created:attributes.created, expires:attributes.expires}"
```

## 3. Check rotation policy

```bash
az keyvault key rotation-policy show --vault-name <vault-name> -n <KEY_NAME>
```

## 4. List soft-deleted keys (purge risk check)

```bash
az keyvault key list-deleted --vault-name <vault-name> \
  --query "[].{name:name, deletedDate:attributes.deletedDate, scheduledPurgeDate:attributes.scheduledPurgeDate}"
```

## 5. Verify which services use this key (impact analysis)

```bash
# Check disk encryption sets using this vault
az disk-encryption-set list --query \
  "[?activeKey.sourceVault.id contains '<vault-name>'].{name:name, id:id}"
# Check Storage accounts with CMK
az storage account list --query \
  "[?encryption.keyVaultProperties.keyVaultUri contains '<vault-name>'].{name:name}"
```

## 6. Confirm backup exists before any key version operation

```bash
az keyvault key backup --vault-name <vault-name> -n <KEY_NAME> -f <KEY_NAME>-backup.json
```


## Read-only configured evidence labels

Treat key, secret, certificate, and Managed HSM metadata reads as sampled read-only Azure evidence. Do not print secret values. A successful metadata read proves only the sampled vault/object state, not tenant-wide Key Vault posture.
