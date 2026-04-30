# Preflight Commands: OCI Live Vault Key Destruction Guard

Run these before any Vault key operation. Paste sanitized output as evidence.

## 1. Confirm identity and vault target

```bash
oci kms management vault list \
  --compartment-id <COMPARTMENT_OCID> \
  --query "data[].{displayName:\"display-name\",id:id,lifecycleState:\"lifecycle-state\",managementEndpoint:\"management-endpoint\"}"
```

## 2. Get key details and verify protection tag

```bash
oci kms management key get \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query "data.{displayName:\"display-name\",lifecycleState:\"lifecycle-state\",algorithm:\"algorithm\",definedTags:\"defined-tags\"}"
# Verify definedTags.Lifecycle.Deletable is NOT 'approved' on production keys
```

## 3. List all key versions and identify active one

```bash
oci kms management key-version list \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query "data[].{id:id,keyVersionId:\"key-version-id\",lifecycleState:\"lifecycle-state\",timeCreated:\"time-created\"}"
```

## 4. Check for scheduled deletions (pending destruction)

```bash
oci kms management key-version list \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query "data[?\"lifecycle-state\"=='PENDING_DELETION'].{id:id,timeOfDeletion:\"time-of-deletion\"}"
```

## 5. Identify dependent resources using this key (impact analysis)

```bash
# Check object storage buckets with SSE-KMS
oci os bucket list --compartment-id <COMPARTMENT_OCID> \
  --query "data[?kmsKeyId!=null].{name:name,kmsKeyId:\"kms-key-id\"}"

# Check block volumes with CMK
oci bv volume list --compartment-id <COMPARTMENT_OCID> \
  --query "data[?kmsKeyId!=null].{displayName:\"display-name\",kmsKeyId:\"kms-key-id\"}"
```

## 6. Export key backup before scheduling deletion

```bash
# For software-protected keys only (HSM-protected keys cannot be exported)
oci kms management key-version download \
  --key-id <KEY_OCID> \
  --key-version-id <KEY_VERSION_OCID> \
  --public-key-info @public-key.pem \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT>
```
