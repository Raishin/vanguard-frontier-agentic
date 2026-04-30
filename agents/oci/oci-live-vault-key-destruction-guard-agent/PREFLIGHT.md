# Vault Key Destruction — Preflight Commands

## 1. Get key metadata and protection mode

```bash
oci kms management key get \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query 'data.{name:"display-name", state:"lifecycle-state", protection:"protection-mode", algo:"key-shape".algorithm, scheduledDeletion:"time-of-deletion"}'
```

**STOP** if `protection-mode = HSM` — HSM key destruction is irreversible.
SOFTWARE keys can be re-imported; HSM keys cannot be recovered after destruction.

## 2. List all key versions (identify active and retired)

```bash
oci kms management key-version list \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --all \
  --query 'data[].{version:"key-version-id", state:"lifecycle-state", created:"time-created"}' \
  --output table
```

## 3. Audit data associations (resources encrypted by this key)

```bash
# Note: OCI does not always provide a complete list via API.
# Supplement with a resource search:
oci resource search search-resources \
  --query-text 'query all resources where freeformTags.EncryptionKeyId = '"'"'<KEY_OCID>'"'"'' \
  --query 'data.items[].{type:"resource-type", name:"display-name", compartment:"compartment-id"}'
```

If the association list is incomplete, perform a manual audit via tags before proceeding.

## 4. Check vault type (Virtual Private vs Shared HSM)

```bash
oci kms vault get \
  --vault-id <VAULT_OCID> \
  --query 'data.{type:"vault-type", state:"lifecycle-state", endpoint:"management-endpoint"}'
```

## 5. Confirm the Lifecycle.Deletable tag is set (required by our IAM policy)

```bash
oci kms management key get \
  --key-id <KEY_OCID> \
  --endpoint <VAULT_MANAGEMENT_ENDPOINT> \
  --query 'data."defined-tags"'
```
