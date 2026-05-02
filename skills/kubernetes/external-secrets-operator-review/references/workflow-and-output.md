# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized YAML snippets (no real ARNs with account IDs, no actual secret values, no real tenant IDs or vault addresses that identify their environment):
- `SecretStore` or `ClusterSecretStore` manifest(s)
- `ExternalSecret` manifest(s)
- `PushSecret` manifest(s), if any
- Optional: ESO operator deployment manifest (to check version and RBAC permissions)
- Optional: description of the external store provider (AWS Secrets Manager, Azure Key Vault, GCP Secret Manager, HashiCorp Vault, 1Password Connect) and the auth method in use

If the user provides only a partial set, note which resources are absent and scope findings accordingly.

### Step 2 — SecretStore vs ClusterSecretStore scope audit

For every `ClusterSecretStore` resource:
- Check whether `spec.conditions[].namespaceSelector` or `spec.conditions[].namespaces` is set
- If absent: flag as HIGH — every namespace can reference this store

```yaml
# HIGH — no namespace selector; any ExternalSecret in any namespace can use this store
apiVersion: external-secrets.io/v1beta1
kind: ClusterSecretStore
metadata:
  name: aws-global
spec:
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        jwt:
          serviceAccountRef:
            name: eso-sa
            namespace: external-secrets

# CORRECT — restrict to specific namespaces
apiVersion: external-secrets.io/v1beta1
kind: ClusterSecretStore
metadata:
  name: aws-payments
spec:
  conditions:
    - namespaces:
        - payments
        - payments-staging
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        jwt:
          serviceAccountRef:
            name: eso-payments-sa
            namespace: external-secrets
```

For `SecretStore` resources: verify the namespace matches the namespace of the ExternalSecrets that reference it. A SecretStore in namespace A cannot be referenced by an ExternalSecret in namespace B.

### Step 3 — Authentication method audit

For every store, identify the auth method:

| Auth method | Risk level | Notes |
|-------------|-----------|-------|
| IRSA (AWS) | Low | Preferred for EKS |
| Azure Workload Identity | Low | Preferred for AKS |
| GCP Workload Identity | Low | Preferred for GKE |
| Vault Kubernetes auth | Low | Preferred for Vault |
| Static credentials via `secretRef` | HIGH | Credential-in-credential anti-pattern |
| Static credentials inline in manifest | CRITICAL | Never acceptable |

**Static credentials pattern to flag:**
```yaml
# HIGH — K8s Secret holds AWS access key for the external store
spec:
  provider:
    aws:
      service: SecretsManager
      auth:
        secretRef:
          accessKeyIDSecretRef:
            name: aws-creds
            key: access-key-id
          secretAccessKeySecretRef:
            name: aws-creds
            key: secret-access-key
```

The K8s Secret `aws-creds` is itself a credential. Anyone who can read that Secret (namespace admin, over-privileged pod) gains full access to the AWS Secrets Manager path the store covers.

**Correct IRSA pattern:**
```yaml
# CORRECT — pod identity; no static credentials
spec:
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        jwt:
          serviceAccountRef:
            name: eso-payments-sa
            namespace: external-secrets
```

### Step 4 — dataFrom scope audit

Review every `ExternalSecret.spec.dataFrom` stanza:

**4a. `dataFrom.extract`**
Fetches all key-value pairs from a specific secret path. Review that the path is as narrow as possible.
```yaml
# ACCEPTABLE — extracts all keys from a single named secret
dataFrom:
  - extract:
      key: my-app/production/database
```

**4b. `dataFrom.find`**
Fetches multiple secrets matching a regex or tag filter. HIGH blast-radius risk.
```yaml
# HIGH — fetches ALL secrets in the store matching any name
dataFrom:
  - find:
      name:
        regexp: ".*"

# HIGH — fetches every secret under the /production/ path prefix
dataFrom:
  - find:
      path: /production/

# ACCEPTABLE — narrow regex scoped to a single application prefix
dataFrom:
  - find:
      name:
        regexp: "^my-app/production/[a-z-]+$"
      tags:
        app: my-app
```

Flag any `find` with a broad regex (`.*`, `^/`, or no regex at all) as HIGH — all matching secrets are merged into a single K8s Secret, and any pod that mounts it gets access to all of them.

### Step 5 — Refresh interval compliance audit

For every `ExternalSecret`, check `spec.refreshInterval`.

Default is `1h`. Review against the rotation policy of the external credential:

| Credential type | Typical rotation window | Recommended refreshInterval |
|----------------|------------------------|------------------------------|
| Database password (RDS IAM auth) | 15 minutes | `5m` or `10m` |
| API key with 24h rotation | 24 hours | `1h` |
| Long-lived service account key | 90 days | `1h` (acceptable) |
| TLS certificate (Let's Encrypt) | 90 days | `12h` |

```yaml
# MEDIUM — 48h refresh on a DB password that rotates every 15 minutes
spec:
  refreshInterval: 48h
  secretStoreRef:
    name: aws-store
    kind: ClusterSecretStore
  target:
    name: db-password
  data:
    - secretKey: password
      remoteRef:
        key: my-app/production/db
        property: password
```

Flag `refreshInterval: 0` as a separate risk — it disables automatic refresh; secrets only update on ExternalSecret resource changes.

### Step 6 — Target creation policy and template audit

**6a. creationPolicy**
```yaml
# MEDIUM — Owner means ESO owns the Secret lifecycle
target:
  name: my-app-secret
  creationPolicy: Owner
```
If the ExternalSecret is deleted (by a botched `helm uninstall`, namespace teardown, or GitOps drift), the managed K8s Secret is deleted immediately. Workloads using it crash. Recommend documenting this in runbooks and implementing deletion protection on critical ExternalSecrets.

Alternative `creationPolicy: Merge` — ESO writes keys into an existing Secret but does not own its lifecycle. Review that the existing Secret exists and has the correct structure.

**6b. Template correctness**
```yaml
# RISKY — template that silently omits a key if the remote key name changes
target:
  template:
    data:
      DB_PASS: "{{ .db_pass }}"
      DB_HOST: "{{ .db_host }}"
      # If the remote secret loses a key, the template renders as empty string, not an error
```

Recommend including `engineVersion: v2` and verifying that all template references have a corresponding remote key. Flag templates with no explicit key mapping verification as LOW (template drift risk).

### Step 7 — PushSecret audit

If `PushSecret` resources are present:

**7a. Auth scope**
PushSecret writes K8s Secret values into the external store. The auth principal for PushSecret needs write permission to the external store path. Review that:
- The IAM role / service principal / Vault policy grants write only to the specific path, not `secretsmanager:PutSecretValue` on `*`
- The auth principal is separate from the read-path principal (PushSecret auth should not be reused for ExternalSecret auth)

**7b. Selector scope**
```yaml
# HIGH — pushes ALL secrets from the namespace into the external store
spec:
  selector:
    secret:
      name: ""  # empty = all secrets
```

Flag any PushSecret with an empty or wildcard selector as HIGH — it exfiltrates all K8s Secrets from the namespace into the external store.

### Step 8 — ESO operator RBAC audit (if manifest provided)

Review the ClusterRole bound to the ESO operator ServiceAccount:
- ESO needs `get`, `list`, `watch` on Secrets (to read SecretStore auth credentials)
- ESO needs `create`, `update`, `patch`, `delete` on Secrets (to manage target Secrets)
- ESO does NOT need `get` on all Secrets cluster-wide unless ClusterSecretStore is used
- Flag `resources: ["secrets"]` with no `resourceNames` restriction on a ClusterRole as MEDIUM

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence summary: pass / needs work / critical issues found>

## Evidence level
<live evidence | user-provided sanitized config | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>
...

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never recommend using static credentials (`secretRef` pointing to a K8s Secret holding cloud credentials) as a permanent solution — always direct toward workload identity (IRSA, Azure Workload Identity, GCP Workload Identity, Vault Kubernetes auth).
- Treat any `ClusterSecretStore` with no `namespaceSelector` as a cross-namespace trust boundary violation — flag it regardless of whether the user considers it intentional.
- Do not recommend setting `refreshInterval: 0` on any ExternalSecret for a credential that participates in a rotation policy — zero disables automatic refresh.
- Flag the absence of monitoring on ExternalSecret sync status (`externalsecret_sync_calls_total`, `externalsecret_status_condition`) — a failing sync that goes unalerted means the cluster silently uses a stale or deleted credential.
- Treat `dataFrom.find` with a broad regex as equivalent to "grant this pod access to every secret in your vault that matches the regex" — make the blast radius explicit in the finding description.
