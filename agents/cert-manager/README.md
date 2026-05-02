# 🔐 cert-manager Agents

<p align="center">
  <span style="font-size:3.5em">🔐</span>
</p>

cert-manager agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit cert-manager PKI configuration, Issuer/ClusterIssuer scope, trust-manager bundles, and cloud CA authentication | read-only | not allowed |

## 📋 Issuer trust review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `cert-manager-issuer-trust-review-agent` | Review Issuer and ClusterIssuer scope, CertificateRequestPolicy auto-approval gap, certificate SAN wildcards and duration risks, trust-manager CA bundle blast radius, SPIFFE mesh CA integration, and cert-manager webhook health | read-only | — |

## ☁️ Cloud CA issuer review agents

Cloud-backed CA review agents are housed in their respective provider directories:

| Agent | Provider | Primary use |
|---|---|---|
| `aws-private-ca-issuer-review-agent` | AWS | Review `aws-privateca-issuer` plugin: IRSA trust chain, PCA hierarchy, certificate template scope, CRL/OCSP distribution, and cross-account PCA access |
| `azure-keyvault-certificate-issuer-review-agent` | Azure | Review Azure Key Vault certificate issuer: Managed Identity auth, CA chain trust, soft-delete retention, and rotation trigger alignment |
| `oci-certificates-issuer-review-agent` | OCI | Review OCI Certificates Service issuer: instance principal auth, validity duration, revocation policy, and compartment CA hierarchy |

## 🛡️ Operating note

- **Auto-approval gap (CRITICAL):** Without `CertificateRequestPolicy` (cert-manager approver-policy), any namespace can request a certificate for any DNS name from a shared `ClusterIssuer`. The cert-manager controller approves all requests automatically.
- **cert-manager PKI attack vector:** cert-manager service account with cloud CA permissions (e.g., IRSA → `acm-pca:IssueCertificate`) can be used to issue wildcard certificates trusted enterprise-wide. A compromised cert-manager pod is equivalent to a compromised private CA.
- `trust-manager` bundles distributed to all namespaces create an implicit trust dependency — a compromised CA bundle propagates automatically.
- cert-manager webhook unavailability blocks all new certificate issuance and renewals across the cluster.

## 📦 Install

```bash
# Install cert-manager PKI review agent (K8s layer)
npx vfa-export-agents --platform claude-code --agents cert-manager-issuer-trust-review-agent --repo .

# Install cert-manager + cloud CA PKI agents
npx vfa-export-agents --platform claude-code --role kubernetes-pki-engineer --repo .
```
