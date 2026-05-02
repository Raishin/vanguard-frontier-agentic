# Official Sources

Load these only when needed:

## Kubernetes core

- [Configure ServiceAccounts for Pods](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/) — use for `automountServiceAccountToken`, projected token volumes, and dedicated SA patterns.
- [ServiceAccount admin guide](https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/) — use for bound ServiceAccount tokens, audience binding, and migration from legacy auto-mounted tokens.
- [TokenRequest API](https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/token-request-v1/) — use when reviewing custom code that calls `TokenRequest` for bespoke token issuance.
- [OIDC issuer discovery](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#openid-connect-tokens) — use when the cluster's own OIDC issuer is consumed by external trust policies.

## AWS IRSA

- [IAM Roles for Service Accounts overview](https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts.html) — use as the entry point for IRSA.
- [IRSA technical deep dive](https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts-technical-overview.html) — use for the OIDC trust policy structure (`Federated`, `Condition.StringEquals` on `aud` and `sub`) and the AssumeRoleWithWebIdentity flow.
- [Configuring IRSA pod identity](https://docs.aws.amazon.com/eks/latest/userguide/pod-configuration.html) — use for the ServiceAccount annotation, env injection, and credential-chain interaction.
- [EKS Pod Identity (the newer alternative)](https://docs.aws.amazon.com/eks/latest/userguide/pod-identities.html) — use when the cluster has migrated to EKS Pod Identity instead of IRSA; the trust model is different.

## Azure Workload Identity

- [Azure Workload Identity overview](https://learn.microsoft.com/en-us/azure/aks/workload-identity-overview) — use for the federated identity credential model and AKS-specific configuration.
- [Workload Identity deploy and configure](https://learn.microsoft.com/en-us/azure/aks/workload-identity-deploy-cluster) — use for OIDC issuer enablement, webhook installation, and the ServiceAccount/Pod label/annotation set.
- [Federated identity credentials on a user-assigned managed identity](https://learn.microsoft.com/en-us/entra/workload-id/workload-identity-federation) — use for the issuer/subject/audience trust scope.

## GCP Workload Identity Federation

- [GKE Workload Identity overview](https://cloud.google.com/kubernetes-engine/docs/concepts/workload-identity) — use for KSA → GSA mapping via `iam.gke.io/gcp-service-account` annotation and the `roles/iam.workloadIdentityUser` IAM binding.
- [GKE Workload Identity setup](https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity) — use for cluster-level config and migration steps.
- [Workload Identity Federation (non-GKE)](https://cloud.google.com/iam/docs/workload-identity-federation) — use when the workload runs outside GKE but federates to GCP IAM.

## Specifications

- [OpenID Connect Core 1.0](https://openid.net/specs/openid-connect-core-1_0.html) — use for the standard claims (`iss`, `sub`, `aud`, `exp`, `nbf`, `iat`) that all three providers verify.
- [JSON Web Token (JWT) — RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519) — use for token structure and validation.

## Grounded insights worth carrying into the skill

- Workload identity replaces long-lived static credentials with short-lived federated tokens issued by the cluster's OIDC issuer. The cloud's IAM trusts the cluster's OIDC issuer URL and the trust policy narrows on `iss`, `aud`, `sub` claims.
- The most-cited trust-policy mistake across all three providers is a wildcard in `sub` (AWS), `subject` (Azure), or member set (GCP). Wildcards mean any ServiceAccount in scope can assume the cloud identity.
- Cloud SDK credential chains explain why workloads frequently keep using static credentials after a workload identity migration. The SDK searches for credentials in a fixed order (env vars → file → instance metadata → web identity); whichever is found first wins. Leaving a static credential anywhere in the chain defeats the migration.
- The Kubernetes-native primitive under all three flavors is the **projected ServiceAccount token volume** with `audience` and `expirationSeconds`. The cloud webhook (AWS Pod Identity Webhook, Azure Workload Identity admission webhook) automates the projection setup.
- AWS IRSA injects `AWS_ROLE_ARN` and `AWS_WEB_IDENTITY_TOKEN_FILE` env vars into pods whose ServiceAccount carries the `eks.amazonaws.com/role-arn` annotation. The AWS SDK then calls `sts:AssumeRoleWithWebIdentity` to exchange the projected JWT for IAM credentials.
- Azure Workload Identity requires both a label on the ServiceAccount AND a label on the Pod (`azure.workload.identity/use: "true"`). Forgetting the Pod label is a frequent silent failure — the SDK falls back to other auth modes.
- GKE Workload Identity uses a metadata-server proxy on each node. SDK calls to `metadata.google.internal` are intercepted and federated to the bound GSA. There is no token file mounted into the pod.
- Projected ServiceAccount tokens are auto-rotated by the kubelet at ~50% of `expirationSeconds`. Long-running SDK clients must read the token file dynamically, not cache it.
- EKS Pod Identity is AWS's newer alternative to IRSA. It uses a node-level agent and a different trust model (no OIDC trust policy on the IAM role; instead a Pod Identity Association resource). Reviews must distinguish which model is in use because the controls are different.
- Setting `automountServiceAccountToken: false` on the ServiceAccount is the correct safer default for workloads that do not call the Kubernetes API. Pod spec overrides this; the override is the failure mode.
