# Workflow and Output Contract

## Review Workflow

### Step 1 — Identify the cert-manager issuer configuration

Locate the cert-manager issuer resource that references Azure Key Vault:

```bash
kubectl get issuer -A -o yaml | grep -A10 "azureKeyVault\|keyVault"
kubectl get clusterissuer -o yaml | grep -A10 "azureKeyVault\|keyVault"
```

Extract the Key Vault name and vault URI from the issuer spec. The exact fields depend on the cert-manager Azure issuer plugin in use (e.g., `cert-manager-webhook-azure` or CAPZ-style issuers).

### Step 2 — Check Managed Identity role assignment

Identify the Managed Identity or Service Principal used by cert-manager on AKS:

```bash
# Get the cert-manager pod's managed identity annotation
kubectl get pod -n cert-manager -l app=cert-manager -o jsonpath='{.items[0].metadata.annotations}'

# Or check the ServiceAccount for workload identity annotation
kubectl get serviceaccount cert-manager -n cert-manager -o jsonpath='{.metadata.annotations}'
```

Retrieve role assignments on the Key Vault:

```bash
KV_ID=$(az keyvault show --name <vault-name> --query id -o tsv)
az role assignment list --scope "$KV_ID" --output table
```

**Correct role:** `Key Vault Certificate Officer` (data plane only)

Role comparison:

| Role | Plane | Grants | Risk |
|------|-------|--------|------|
| `Key Vault Certificate Officer` | Data | Create, update, import, delete certificates | Correct |
| `Key Vault Certificates Officer` | Data | Same as above (alias) | Correct |
| `Key Vault Contributor` | Management | Manage vault config, delete vault, change policies | HIGH — management plane access |
| `Key Vault Administrator` | Data + Management | Full control including purge | HIGH |
| `Owner` / `Contributor` at subscription | All | Everything | CRITICAL |

### Step 3 — Check RBAC mode vs legacy access policies

```bash
az keyvault show --name <vault-name> --query properties.enableRbacAuthorization
```

- `true` — RBAC mode (preferred, auditable via Azure RBAC)
- `false` or `null` — legacy access policies (harder to audit)

If legacy access policies are in use, check the policy:

```bash
az keyvault show --name <vault-name> --query properties.accessPolicies
```

The cert-manager identity should only have `certificates: ["get", "create", "import", "update", "list"]` — not `all` and not management operations.

### Step 4 — Review certificate policy and exportability

```bash
az keyvault certificate get-default-policy
az keyvault certificate show --vault-name <vault-name> --name <cert-name>
```

Key fields in the certificate policy:

```json
{
  "x509CertificateProperties": {
    "subject": "CN=myapp.internal",
    "validityInMonths": 3,
    "keyUsage": ["digitalSignature", "keyEncipherment"]
  },
  "keyProperties": {
    "exportable": false,
    "keyType": "RSA",
    "keySize": 2048,
    "reuseKey": false
  },
  "issuerParameters": {
    "name": "Self"
  }
}
```

**Flags:**
- `exportable: true` on a cert used for cluster-internal mTLS — MEDIUM (private key extractable)
- `keySize < 2048` for RSA or `keySize < 256` for EC — HIGH (weak key)
- `validityInMonths > 12` for workload certs — MEDIUM (excessive validity)

Note: Non-exportable certs require the application to use Key Vault SDK or CSI driver for key operations, not just cert retrieval. Confirm application capability before enforcing non-exportable.

### Step 5 — Review Key Vault network access

```bash
az keyvault show --name <vault-name> --query properties.networkAcls
az keyvault show --name <vault-name> --query properties.publicNetworkAccess
```

If `publicNetworkAccess: Disabled`:

```bash
# Check for private endpoint
az network private-endpoint list \
  --query "[?privateLinkServiceConnections[?groupIds[0]=='vault']].{name:name,subnet:subnet.id}" \
  --output table

# Check for private DNS zone
az network private-dns zone list --query "[?contains(name,'vaultcore')]" --output table
```

For AKS access to Key Vault:
- AKS cluster VNet must be peered with or the same as the VNet hosting the private endpoint
- Private DNS zone `privatelink.vaultcore.azure.net` must be linked to the AKS cluster VNet
- Outbound traffic from cert-manager pod must route through the private endpoint

**Flags:**
- Key Vault with public access from internet and no firewall restrictions — MEDIUM
- Key Vault with `publicNetworkAccess: Disabled` but missing private endpoint — HIGH (cert issuance will fail)
- No private DNS zone link to AKS VNet (DNS resolution fails for private endpoint) — HIGH

### Step 6 — Review integrated CA configuration (if applicable)

For DigiCert or GlobalSign integrated CAs:

```bash
az keyvault certificate issuer show --vault-name <vault-name> --issuer-name DigiCert
```

Check that the issuer credential secret is stored in Key Vault and scoped to a minimum profile:

```bash
az keyvault secret show --vault-name <vault-name> --name DigiCert-issuer-creds
```

**Flags:**
- Integrated CA credentials that have account-wide issuance scope (not single profile) — MEDIUM
- Integrated CA credentials stored outside Key Vault (e.g., in a Kubernetes Secret) — MEDIUM

### Step 7 — Review rotation race condition

cert-manager rotation schedule:
```bash
kubectl get certificate <name> -n <namespace> -o jsonpath='{.spec.duration} {.spec.renewBefore}'
```

Key Vault auto-rotation policy:
```bash
az keyvault certificate get-default-policy | jq '.lifetimeActions'
```

A `lifetimeAction` of type `AutoRenew` triggers Key Vault to request a new cert from the issuer. If cert-manager's `renewBefore` window overlaps with the Key Vault auto-renewal trigger (both fire within the same rotation window), both may attempt to renew simultaneously, causing a temporary version mismatch.

**Mitigation:** Disable Key Vault auto-rotation for certs managed by cert-manager, or ensure the Key Vault auto-renewal threshold is set beyond the cert-manager `renewBefore` window.

---

## Output Format

### Finding: `<short title>`

| Field | Value |
|-------|-------|
| Severity | CRITICAL / HIGH / MEDIUM / LOW |
| Resource | Key Vault name, role assignment, cert name, or policy field |
| Evidence | documentation-based / live evidence / inference |
| Description | What is wrong and its impact |
| Remediation | Azure CLI command, policy JSON, or configuration change |

---

### Overall Posture

| Category | Status |
|----------|--------|
| Managed Identity role (data plane only) | PASS / FAIL |
| RBAC mode (not legacy policies) | PASS / FAIL |
| Certificate exportability | PASS / FAIL |
| Key Vault network access | PASS / FAIL |
| Certificate validity periods | PASS / FAIL |
| Integrated CA credential scope | PASS / N/A / FAIL |
| Rotation policy alignment | PASS / FAIL |

**Verdict:** TRUSTED / UNTRUSTED / CONDITIONAL (list conditions)
