# Workflow and Output Contract

## Workflow

### Step 1 — Identify the workload identity flavor

Three production flavors plus the underlying primitive:

1. **AWS IRSA (IAM Roles for Service Accounts)** — ServiceAccount annotated with `eks.amazonaws.com/role-arn: arn:aws:iam::<account>:role/<role>`. Pod identity webhook injects `AWS_WEB_IDENTITY_TOKEN_FILE` and `AWS_ROLE_ARN`. AWS SDK calls `sts:AssumeRoleWithWebIdentity`.
2. **Azure Workload Identity** — ServiceAccount labeled `azure.workload.identity/use: "true"` and annotated with `azure.workload.identity/client-id: <client-id>`. Pod labeled `azure.workload.identity/use: "true"`. Webhook injects projected token at `/var/run/secrets/azure/tokens/azure-identity-token`. Azure SDK exchanges via federated identity credential.
3. **GCP Workload Identity Federation (GKE)** — ServiceAccount annotated `iam.gke.io/gcp-service-account: <gsa>@<project>.iam.gserviceaccount.com`. GKE metadata server proxies SDK calls; ServiceAccount → GSA mapping via IAM policy binding (`roles/iam.workloadIdentityUser`).
4. **Generic projected token + OIDC** — Kubernetes-native primitive. ServiceAccount projected token volume with explicit `audience` and `expirationSeconds`. Trust-policy verification at the cloud / external service.

Reference: [Configure ServiceAccounts for Pods](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/) and [ServiceAccount admin](https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/).

### Step 2 — Audit the OIDC trust policy scope

This is the most under-reviewed control in workload identity migrations.

**AWS IRSA** trust policy structure:

```json
{
  "Effect": "Allow",
  "Principal": {
    "Federated": "arn:aws:iam::<account>:oidc-provider/oidc.eks.<region>.amazonaws.com/id/<id>"
  },
  "Action": "sts:AssumeRoleWithWebIdentity",
  "Condition": {
    "StringEquals": {
      "oidc.eks.<region>.amazonaws.com/id/<id>:aud": "sts.amazonaws.com",
      "oidc.eks.<region>.amazonaws.com/id/<id>:sub": "system:serviceaccount:<namespace>:<sa-name>"
    }
  }
}
```

Critical findings:

- `StringLike` on `:sub` with a wildcard (`system:serviceaccount:*:*` or `system:serviceaccount:<ns>:*`) — any ServiceAccount in scope can assume the role.
- `:aud` not constrained to `sts.amazonaws.com` — token reusable for non-AWS audiences.
- Multiple OIDC providers trusted from one role — broader trust surface than necessary.

**Azure Workload Identity** uses federated identity credentials on a user-assigned managed identity:

```text
issuer: https://<region>.oic.prod-aks.azure.com/<tenant>/<id>/
subject: system:serviceaccount:<namespace>:<sa-name>
audience: api://AzureADTokenExchange
```

Critical findings:

- `subject` with wildcards — Azure rejects most wildcards but pre-validation is required; mistakes get caught only at first token exchange.
- Multiple federated identity credentials on one managed identity, each from different clusters — each is a separate cluster trust; rotate / remove unused ones.

**GCP Workload Identity** uses IAM policy on the GSA:

```text
role: roles/iam.workloadIdentityUser
member: serviceAccount:<project>.svc.id.goog[<namespace>/<ksa-name>]
```

Critical findings:

- Members listing `[*/*]` — any ServiceAccount in any namespace can act as the GSA.
- Member with wildcards `[<ns>/*]` — any ServiceAccount in the namespace.

Reference: [AWS IRSA technical overview](https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts-technical-overview.html), [Azure Workload Identity overview](https://learn.microsoft.com/en-us/azure/aks/workload-identity-overview), [GKE Workload Identity](https://cloud.google.com/kubernetes-engine/docs/concepts/workload-identity).

### Step 3 — Confirm the pod is actually using the federated token

Workload identity migrations frequently leave a static credential alongside the new federated path. Cloud SDKs use a credential chain — if a static credential is found earlier in the chain, the federated token is never used.

Stress-tests:

- AWS SDK credential chain: env (`AWS_ACCESS_KEY_ID`) → shared credentials file → IRSA web identity → instance profile. A leftover env var defeats IRSA.
- Azure SDK chain: env → managed identity → workload identity → CLI. A leftover client secret in env defeats workload identity.
- GCP SDK chain: `GOOGLE_APPLICATION_CREDENTIALS` env → metadata server. A mounted SA key file in `GOOGLE_APPLICATION_CREDENTIALS` defeats GKE Workload Identity.

Verify with:

```shell
# AWS — confirm sts:AssumeRoleWithWebIdentity is the auth path
kubectl exec -it <pod> -n <ns> -- aws sts get-caller-identity
# Should show "AssumedRole" not "User"

# Azure — confirm token exchange
kubectl exec -it <pod> -n <ns> -- env | grep AZURE_FEDERATED_TOKEN_FILE

# GCP — confirm metadata server is reachable and used
kubectl exec -it <pod> -n <ns> -- curl -s -H "Metadata-Flavor: Google" \
  http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/email
```

> **Diagnostic only — do not embed in automation.** The instance metadata server is the same primitive that has been weaponized in cloud breaches (notably Capital One 2019). On AWS and Azure clusters the metadata IP is `169.254.169.254`; on GCP it resolves through `metadata.google.internal`. Any pod that can reach the metadata endpoint can request short-lived credentials for the node's identity. Block the metadata service at the network policy layer for workloads that should not read it — see [`cilium-network-policy-review`](../../../cilium/cilium-network-policy-review/SKILL.md) for the egress rule pattern that excludes `169.254.169.254/32`.

### Step 4 — Audit the projected token configuration

For provider webhooks, projection is automatic. For the generic projected-token primitive, the Pod spec includes:

```yaml
spec:
  serviceAccountName: <sa-name>
  volumes:
    - name: token
      projected:
        sources:
          - serviceAccountToken:
              path: token
              audience: <audience>
              expirationSeconds: 3600     # max recommended; tokens are auto-rotated
```

Critical findings:

- `expirationSeconds` longer than 1 hour — projected tokens should be short-lived.
- `audience` empty — defaults to the API server, which means the token is interchangeable with any ServiceAccount token (no narrowing).
- Multiple audiences for the same volume — the token can be replayed across audiences.

Reference: [Bound Service Account Tokens](https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/#bound-service-account-tokens) and [Token volume projection](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection).

### Step 5 — Audit `automountServiceAccountToken`

Default is `true`. Every pod gets a token mounted at `/var/run/secrets/kubernetes.io/serviceaccount/token` whether or not the workload uses the Kubernetes API. Findings:

- Pod that does not call the K8s API but has `automountServiceAccountToken: true` — token is exfiltratable on container compromise.
- ServiceAccount with `automountServiceAccountToken: false` but Pod spec overrides with `true` — Pod spec wins; the SA-level safer default is bypassed.

Recommended baseline: `automountServiceAccountToken: false` on the ServiceAccount, override only when the workload actually calls the K8s API.

### Step 6 — Audit cross-cluster / cross-account reuse

A single IAM role (AWS) or managed identity (Azure) or GSA (GCP) can be trusted from multiple clusters. Findings:

- An IAM role trusted from cluster A's OIDC provider AND cluster B's OIDC provider — compromise of cluster B grants the role's permissions.
- Federated identity credentials on a managed identity from clusters that no longer exist — stale trust; remove.

### Step 7 — Stress-test operational hygiene

- Prefer dedicated IAM identities per ServiceAccount, not shared roles across multiple SAs.
- Prefer narrow IAM policies (`Resource: arn:aws:s3:::specific-bucket/*`) over broad (`Resource: '*'`).
- Prefer `automountServiceAccountToken: false` as the default and override per workload.
- Prefer `audience` claims that match the cloud target's expected audience.
- Test token rotation by killing the projected token file and confirming the SDK refreshes.

## Output

Return:

- **target**: the workload identity flavor and the ServiceAccount → cloud identity binding,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **OIDC trust policy scope**: `aud`, `sub`, `iss`, with judgment on narrowness,
- **fallback assessment**: are static credentials still present? Is the SDK actually using the federated path?,
- **token projection assessment**: audience, expiration, automountServiceAccountToken posture,
- **risk findings** (with severity: high / medium / low),
- **safest next actions** with sample manifest and trust-policy changes,
- **rollback plan**: how to revert without locking the workload out of the cloud,
- **assumptions and missing facts**.

## Security notes

- Never recommend keeping a long-lived credential Secret "just in case" alongside workload identity.
- Never recommend wildcards in OIDC trust policy `sub` claim.
- Never recommend `audience` defaults that allow the projected token to be replayed against the K8s API.
- Do not print IAM access keys, client secrets, GCP service account JSON, or projected token JWT bodies.
