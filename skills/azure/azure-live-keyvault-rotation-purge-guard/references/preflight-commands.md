# Preflight Commands: Azure Live Key Vault Rotation Purge Guard

Use shell variables for examples instead of raw identifiers. Populate them from an approved change record or already configured shell context; never paste tenant, subscription, resource, or secret values into chat.

## Evidence-variable convention

Variables such as $AZURE_RESOURCE_GROUP_NAME, $APP_SERVICE_APP_NAME, or $KEY_VAULT_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

Run these before any Key Vault rotation or purge operation.

## 1. Confirm identity and vault target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az keyvault show -n $KEY_VAULT_NAME -g $AZURE_RESOURCE_GROUP_NAME \
  --query "{name:name, enableSoftDelete:properties.enableSoftDelete, enablePurgeProtection:properties.enablePurgeProtection, softDeleteRetentionInDays:properties.softDeleteRetentionInDays}"
```

## 2. List key versions and identify current/active

```bash
az keyvault key list-versions --vault-name $KEY_VAULT_NAME -n $KEY_VAULT_KEY_NAME \
  --query "[].{kid:kid, enabled:attributes.enabled, created:attributes.created, expires:attributes.expires}"
```

## 3. Check rotation policy

```bash
az keyvault key rotation-policy show --vault-name $KEY_VAULT_NAME -n $KEY_VAULT_KEY_NAME
```

## 4. List soft-deleted keys (purge risk check)

```bash
az keyvault key list-deleted --vault-name $KEY_VAULT_NAME \
  --query "[].{name:name, deletedDate:attributes.deletedDate, scheduledPurgeDate:attributes.scheduledPurgeDate}"
```

## 5. Verify which services use this key (impact analysis)

```bash
# Check disk encryption sets using this vault
az disk-encryption-set list --query \
  "[?activeKey.sourceVault.id contains '$KEY_VAULT_NAME'].{name:name, id:id}"
# Check Storage accounts with CMK
az storage account list --query \
  "[?encryption.keyVaultProperties.keyVaultUri contains '$KEY_VAULT_NAME'].{name:name}"
```

## 6. Confirm backup exists before any key version operation

```bash
az keyvault key backup --vault-name $KEY_VAULT_NAME -n $KEY_VAULT_KEY_NAME -f $KEY_VAULT_KEY_NAME-backup.json
```


## Read-only configured evidence labels

Treat key, secret, certificate, and Managed HSM metadata reads as sampled read-only Azure evidence. Do not print secret values. A successful metadata read proves only the sampled vault/object state, not tenant-wide Key Vault posture.
