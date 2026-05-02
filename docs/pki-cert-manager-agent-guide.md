# PKI & cert-manager Agent Selection Guide

This guide explains when to use each PKI specialist agent, how to run them together, and how the Maestro routers trigger and route to them.

---

## The integration seam

cert-manager is a certificate lifecycle controller, not a CA. The audit surface spans three distinct layers — and each layer has a dedicated specialist:

```
K8s workload
    ↓  requests cert via
cert-manager Certificate CRD
    ↓  submitted as
CertificateRequest → CertificateRequestPolicy (approver-policy)
    ↓  if approved, forwarded to
External Issuer plugin (aws-privateca-issuer / azure-keyvault-issuer / oci-certificates-issuer)
    ↓  authenticates via
Workload identity (IRSA / Azure Workload Identity / OCI Workload Identity)
    ↓  calls
Cloud Private CA API (AWS PCA / Azure Key Vault / OCI Certificates)
    ↓  CA issues cert based on
Certificate Template / Issuance Policy (key usage, validity, SAN constraints)
    ↓  cert returned, distributed into
trust-manager ConfigMapBundle / mesh SPIFFE trust domain / app trust stores
```

Every arrow is an audit surface. No single agent covers the full chain — that is intentional. Each layer has different evidence, different risk profile, and different remediation authority.

---

## Agent map

| Agent | Layer | Provider | Use when |
|---|---|---|---|
| `cert-manager-issuer-trust-review-agent` | K8s | Any | Reviewing cert-manager CRDs, issuer scope, CertificateRequestPolicy, trust distribution |
| `aws-private-ca-issuer-review-agent` | Cloud CA | AWS | Reviewing AWS ACM PCA configuration, IRSA scope, template ARN, CRL reachability |
| `azure-keyvault-certificate-issuer-review-agent` | Cloud CA | Azure | Reviewing Azure Key Vault certificate policy, Managed Identity role, rotation race conditions |
| `oci-certificates-issuer-review-agent` | Cloud CA | OCI | Reviewing OCI Certificates issuance rules, Workload Identity vs Instance Principal, OCSP |

---

## Agent 1: `cert-manager-issuer-trust-review-agent`

**Layer:** Kubernetes — cloud-provider-agnostic  
**Skill:** `skills/cert-manager/cert-manager-issuer-trust-review/SKILL.md`

### What it reviews

- `ClusterIssuer` vs `Issuer` scope and whether backing CA is ROOT or SUBORDINATE
- `CertificateRequestPolicy` (approver-policy) coverage — **absence is CRITICAL: all cert requests are auto-approved**
- `Certificate` spec risks: wildcard `dnsNames`, excessive `duration`, short `renewBefore`
- `trust-manager` Bundle / ConfigMapBundle distribution blast radius
- cert-manager webhook health and CSR controller status
- SPIFFE trust domain integration (Istio `cacerts` vs external CA chain)

### When to use — concrete triggers

| Scenario | Why this agent |
|---|---|
| "Review our cert-manager ClusterIssuer setup before onboarding a new team" | Checks issuer scope — a single unguarded ClusterIssuer backing a corporate CA grants the entire cluster cert issuance capability |
| "Cert renewal failed overnight and pods are getting TLS errors" | Checks webhook health, `renewBefore` alignment with `duration`, CertificateRequest status, and whether approver-policy rejected the request |
| "We want to issue certs for `*.internal.corp.com` to our payments service" | Wildcard SAN blast-radius audit — `*.internal.corp.com` issued to a single workload covers the entire internal domain |
| "Add cert-manager to a new namespace that connects to prod" | Reviews whether a namespace-scoped `Issuer` or a `ClusterIssuer` is appropriate, and whether `CertificateRequestPolicy` constrains the new namespace |
| "Set up trust-manager to distribute the internal CA cert cluster-wide" | Reviews ConfigMapBundle scope, which namespaces receive the trust bundle, and whether the bundle rotation is automated |
| "Migrate from Vault PKI secrets engine to AWS PCA" | Reviews ClusterIssuer reconfiguration, any in-flight Certificates, and whether CertificateRequestPolicy needs updating for the new issuer |

### What it does NOT cover

- The CA hierarchy itself (ROOT vs SUBORDINATE configuration inside AWS/Azure/OCI)
- The IRSA / Managed Identity permissions used by the external issuer plugin
- Certificate template ARN scope at the cloud CA level

Route to the cloud CA specialist in parallel when any external issuer is in scope.

---

## Agent 2: `aws-private-ca-issuer-review-agent`

**Layer:** Cloud CA — AWS ACM Private Certificate Authority  
**Skill:** `skills/aws/aws-private-ca-issuer-review/SKILL.md`

### What it reviews

- CA ARN type: ROOT CA backing an active issuer is **CRITICAL** — only SUBORDINATE CAs should be online for cert-manager issuance
- Certificate template ARN: `SubordinateCACertificate/V1` allows cert-manager to mint intermediate CAs — **CRITICAL privilege escalation**; correct template is `EndEntityCertificate/V1`
- IRSA role policy: required actions are `acm-pca:IssueCertificate`, `acm-pca:GetCertificate`, `acm-pca:DescribeCertificateAuthority`; `acm-pca:DeleteCertificateAuthority` or `acm-pca:*` are **HIGH**
- Certificate validity: durations > 365d for workload certs are **MEDIUM**; best practice ≤ 90d
- CRL S3 bucket reachability from the cluster VPC: unreachable = revocation disabled = **HIGH**
- Cross-account RAM-shared CA: verifies minimum issuance-only permissions in the security account

### When to use — concrete triggers

| Scenario | Why this agent |
|---|---|
| "Set up aws-privateca-issuer for our EKS cluster" | Full pre-flight: CA ARN type, template ARN, IRSA scope, CRL reachability |
| "Our cert-manager IRSA role has `acm-pca:*` — is that a problem?" | IRSA scope audit: `acm-pca:*` includes `DeleteCertificateAuthority` and `CreateCertificateAuthority` — that is the ability to destroy or create CAs from a compromised pod |
| "Security team flagged our cert-manager pod can issue certs for any SAN" | Template ARN audit: SubordinateCACertificate template is the enabler; also checks whether CertificateRequestPolicy (K8s layer) provides any guard |
| "We share our Private CA across three AWS accounts via RAM" | Cross-account scope: verifies RAM resource share is scoped to the right OUs, and that the foreign account has issuance-only permissions |
| "Audit whether a compromised cert-manager pod could escalate via our PKI" | Full blast-radius audit: CA type, template, IRSA scope, CRL reachability, validity periods |
| "Our CRL endpoint is a public S3 URL — should it be private?" | CRL reachability: public vs VPC-endpoint-accessible S3 bucket; also checks whether revocation is actually functioning |

### Attack vector this agent exists to catch

A compromised pod with access to the cert-manager IRSA role can call `acm-pca:IssueCertificate` directly, request a cert for `*.internal.corp.com`, and receive a CA-signed credential trusted by every internal service. No admission webhook fires. No PSA flag. The cert is valid until expiry. This agent reviews whether the template ARN, IRSA scope, and validity configuration close that path.

### What it does NOT cover

- The `CertificateRequestPolicy` gate (K8s layer) — route `cert-manager-issuer-trust-review-agent` in parallel
- The IRSA annotation on the cert-manager ServiceAccount — route `kubernetes-workload-identity-review-agent` if that is the starting evidence

---

## Agent 3: `azure-keyvault-certificate-issuer-review-agent`

**Layer:** Cloud CA — Azure Key Vault  
**Skill:** `skills/azure/azure-keyvault-certificate-issuer-review/SKILL.md`

### What it reviews

- Managed Identity role assignment: `Key Vault Certificate Officer` (data-plane, certificates only) vs `Key Vault Contributor` (management-plane, includes secrets and keys) — Contributor is a **HIGH** finding
- RBAC mode vs legacy access policies on the Key Vault
- Certificate policy: key type, key size, validity, exportability, SAN constraints — cloud policy wins when more permissive than cert-manager spec
- Private endpoint requirement: Key Vault without private endpoint reachable from cluster = dependency on public internet path
- Integrated CA credentials (DigiCert / GlobalSign via Key Vault): external CA trust chain differs from standalone Key Vault CA; credential scoping errors are **HIGH**
- Rotation race condition: Azure auto-rotates on certificate contacts; cert-manager rotation triggered by `renewBefore` — simultaneous rotation = cert not found at mount time

### When to use — concrete triggers

| Scenario | Why this agent |
|---|---|
| "Configure cert-manager to use our Azure Key Vault CA on AKS" | Pre-flight: Managed Identity role, RBAC mode, certificate policy, private endpoint, issuer config |
| "Cert-manager pod has `Key Vault Contributor` role — is that least privilege?" | Contributor grants full management-plane access including secrets and keys in the vault — not just certificates |
| "Certs are expiring despite Azure saying they auto-renew" | Rotation race: Azure Key Vault auto-renewal via certificate contacts conflicts with cert-manager renewal triggered by `renewBefore` |
| "We use DigiCert through Azure Key Vault for our internal CA" | Integrated CA path: Managed Identity must reach the DigiCert endpoint through the Azure plane; trust chain is distinct from self-signed Key Vault CA |
| "Key Vault is on a private endpoint — does cert-manager need to be in the same VNet?" | Network path audit: cert-manager pod must reach the private endpoint; reviews AKS network config, subnet delegation, and DNS resolution for the vault |
| "Our cert policy allows exportable private keys — is that a risk?" | Exportable keys allow the private key to be extracted from Key Vault; reviews whether any identity can export the key and what the blast radius is |

### What it does NOT cover

- The `CertificateRequestPolicy` gate (K8s layer) — route `cert-manager-issuer-trust-review-agent` in parallel
- Azure Workload Identity setup on the cert-manager ServiceAccount — route `kubernetes-workload-identity-review-agent` if that is the starting evidence

---

## Agent 4: `oci-certificates-issuer-review-agent`

**Layer:** Cloud CA — OCI Certificates Service  
**Skill:** `skills/oci/oci-certificates-issuer-review/SKILL.md`

### What it reviews

- CA type: ROOT CA active for issuance is **CRITICAL**; OCI Certificates should use a SUBORDINATE CA for cert-manager
- Issuance rules: validity cap, key algorithm constraints — cert-manager requests outside issuance rules fail at **renewal**, not creation (outage on renewal day, not day one)
- Authentication method: OKE Workload Identity (mapped ServiceAccount — recommended) vs Instance Principal (node-level identity — every pod on the node has issuance capability)
- IAM policy scope: `manage certificate-authority-requests` scoped to the specific CA OCID, not compartment-wide
- OCSP endpoint reachability from OKE pod CIDR
- Certificate version lifecycle: OCI accumulates certificate versions; versions consume vault storage; no cleanup policy = storage quota breach over time

### When to use — concrete triggers

| Scenario | Why this agent |
|---|---|
| "Set up cert-manager on OKE with OCI Certificates as the issuer" | Pre-flight: CA type, issuance rules, auth method (Workload Identity vs Instance Principal), IAM policy, OCSP |
| "Cert renewals are failing in production but new certs issued fine in staging" | Issuance rule mismatch: renewal requests may differ from initial issuance in key algorithm or SAN format; rules block renewal but not original issuance in some edge cases |
| "Our cert-manager uses Instance Principal — security team flagged it" | Instance Principal = every pod on the node has `IssueCertificateRequest` capability, not just cert-manager; recommends migrating to OKE Workload Identity |
| "OCI Certificates IAM policy uses compartment-level `manage` permission" | Compartment-wide `manage certificate-authority-requests` grants issuance capability for all CAs in the compartment — scope to specific CA OCID |
| "How many certificate versions does OCI accumulate over time?" | Certificate version lifecycle audit: OCI does not auto-delete superseded versions; reviews whether a cleanup policy or lifecycle script is in place |
| "We use a BYOCA (Bring Your Own CA) with OCI Certificates" | BYOCA chain-of-trust: OCI acts as pass-through; if the external root is misconfigured, OCI issues certs that clients reject at chain validation |

### What it does NOT cover

- The `CertificateRequestPolicy` gate (K8s layer) — route `cert-manager-issuer-trust-review-agent` in parallel
- OKE Workload Identity setup on the cert-manager ServiceAccount — route `kubernetes-workload-identity-review-agent` if that is the starting evidence

---

## Multi-agent scenarios

### Full PKI posture audit (AWS)

Use when: "Review our full cert-manager and AWS PCA setup before a security audit."

```
Route: cert-manager-issuer-trust-review-agent, aws-private-ca-issuer-review-agent
Reason: Full audit spans the K8s cert-manager layer (issuer scope, CertificateRequestPolicy, trust distribution) and the AWS CA layer (CA type, template ARN, IRSA, CRL).
Mode: parallel (2)
```

`cert-manager-issuer-trust-review-agent` audits ClusterIssuer scope, CertificateRequestPolicy coverage, Certificate SAN and duration risks, and trust-manager bundle distribution. `aws-private-ca-issuer-review-agent` audits CA ARN type, certificate template ARN, IRSA permissions, validity periods, and CRL S3 reachability.

---

### PKI + workload identity review (AKS + Azure Key Vault)

Use when: "We're migrating to cert-manager with Azure Key Vault CA on AKS — review PKI and identity setup."

```
Route: cert-manager-issuer-trust-review-agent, azure-keyvault-certificate-issuer-review-agent, kubernetes-workload-identity-review-agent
Reason: Task spans three layers: cert-manager K8s config, Azure Key Vault certificate issuer policy, and Azure Workload Identity setup for the cert-manager ServiceAccount.
Mode: parallel (3)
```

---

### CertificateRequestPolicy + AWS PCA blast-radius after workload identity incident

Use when: "A cert-manager pod was compromised — assess what certificates could have been issued."

```
Route: cert-manager-issuer-trust-review-agent, aws-private-ca-issuer-review-agent
Reason: Incident response requires two concurrent reviews: CertificateRequestPolicy coverage (K8s gate before CA) and IRSA scope + template ARN (blast radius at the CA layer).
Mode: parallel (2)
```

Flag both outputs to the security team before any remediation. IRSA credentials remain valid until expiry — certificate revocation at the CRL layer is the only effective revocation path if certs were already issued.

---

### OCI PKI + admission security pre-prod review

Use when: "Review cert-manager, OCI Certificates, and our Kyverno policies before promotion to prod."

```
Route: cert-manager-issuer-trust-review-agent, oci-certificates-issuer-review-agent, kyverno-policy-review-agent
Reason: Pre-prod gate spans K8s PKI config, OCI CA issuance rules, and admission policies that govern what workloads can deploy.
Mode: parallel (3)
```

---

## Maestro routing

### Kubernetes Maestro signals

The Kubernetes Maestro routes to PKI agents on these keyword signals:

| Signal keywords | Agent | Domain | Live-guard? |
|---|---|---|---|
| cert-manager, ClusterIssuer, Issuer, CertificateRequest, CertificateRequestPolicy, approver-policy, trust-manager, Bundle, ConfigMapBundle, certificate renewal, TLS cert, mTLS cert, SPIFFE, cert-manager webhook, PKI K8s layer | `cert-manager-issuer-trust-review-agent` | PKI K8s review | No |

**Routing header example:**

```
Route: cert-manager-issuer-trust-review-agent
Reason: Task involves CertificateRequestPolicy coverage and cert-manager ClusterIssuer scope — K8s PKI domain.
Mode: single
```

**Multi-domain example with workload identity overlap:**

A request for "review our cert-manager IRSA annotation and CertificateRequestPolicy" spans two domains:

```
Route: cert-manager-issuer-trust-review-agent, kubernetes-workload-identity-review-agent
Reason: Task spans cert-manager K8s PKI config (CertificateRequestPolicy) and IRSA workload identity setup for the cert-manager ServiceAccount.
Mode: parallel (2)
```

---

### AWS Maestro signals

The AWS Maestro routes to `aws-private-ca-issuer-review-agent` on these keyword signals:

| Signal keywords | Agent | Domain |
|---|---|---|
| ACM PCA, AWS Private CA, aws-privateca-issuer, AWSPCAIssuer, AWSPCAClusterIssuer, certificate template ARN, CRL distribution point, CRL S3, IRSA cert-manager, cross-account PCA, RAM-shared CA, SubordinateCACertificate template, EndEntityCertificate template, private certificate authority | `aws-private-ca-issuer-review-agent` | PKI cloud CA — AWS |

**Routing header example:**

```
Route: aws-private-ca-issuer-review-agent
Reason: Task involves AWS ACM Private CA configuration for cert-manager — PKI cloud CA domain.
Mode: single
```

**PKI spans both layers — escalate to Kubernetes Maestro when cert-manager K8s config is also in scope.** The AWS Maestro owns the cloud CA layer only. If the request mentions `CertificateRequestPolicy`, `ClusterIssuer`, `trust-manager`, or `cert-manager CRDs`, flag that the K8s PKI layer is also in scope and recommend routing the kubernetes maestro in parallel.

---

### Azure Maestro signals

The Azure Maestro routes to `azure-keyvault-certificate-issuer-review-agent` on these keyword signals:

| Signal keywords | Agent | Domain |
|---|---|---|
| Key Vault certificate issuer, AKV issuer, azure-keyvault-issuer, Key Vault Certificate Officer, Key Vault Contributor cert-manager, cert-manager Azure, certificate contacts Azure, DigiCert Key Vault, GlobalSign Key Vault, Key Vault certificate policy, Azure private CA, exportable cert key | `azure-keyvault-certificate-issuer-review-agent` | PKI cloud CA — Azure |

**Routing header example:**

```
Route: azure-keyvault-certificate-issuer-review-agent
Reason: Task involves Azure Key Vault certificate issuer setup for cert-manager — PKI cloud CA domain.
Mode: single
```

Note: The existing `azure-key-vault-secret-lifecycle-auditor-agent` covers Key Vault **secrets** lifecycle (rotation, expiry, access logs). The `azure-keyvault-certificate-issuer-review-agent` covers Key Vault **certificate issuer** configuration for cert-manager. Route to both in parallel when the review spans secret storage and certificate issuance in the same vault.

---

### OCI Maestro signals

The OCI Maestro routes to `oci-certificates-issuer-review-agent` on these keyword signals:

| Signal keywords | Agent | Domain |
|---|---|---|
| OCI Certificates, OCI certificate authority, oci-certificates-issuer, OKE cert-manager, issuance rules OCI, Instance Principal cert-manager, OKE Workload Identity cert, OCSP OCI, BYOCA OCI, certificate version lifecycle OCI | `oci-certificates-issuer-review-agent` | PKI cloud CA — OCI |

**Routing header example:**

```
Route: oci-certificates-issuer-review-agent
Reason: Task involves OCI Certificates Service issuer configuration for cert-manager on OKE — PKI cloud CA domain.
Mode: single
```

---

## The attack class these agents jointly address

**Scenario:** cert-manager's workload identity (IRSA / Azure WI / OCI WI) is compromised via pod escape, SSRF, or metadata service credential theft.

**Attack path:**
1. Attacker calls `IssueCertificate` (AWS PCA) / `CreateCertificate` (Azure KV) / `CreateCertificateRequest` (OCI) directly — bypassing cert-manager entirely
2. Requests a cert for `*.internal.corp.com` or a specific high-value service FQDN
3. The cert is signed by the corporate Private CA
4. Every service that trusts that CA accepts mTLS connections from the attacker using that cert
5. Silent lateral movement — the cert is valid, no admission webhook fires, no PSA flag

**The gates each agent reviews:**

| Gate | Agent that reviews it |
|---|---|
| `CertificateRequestPolicy` — only gate before CA issuance at the K8s layer | `cert-manager-issuer-trust-review-agent` |
| Certificate template ARN — determines whether the issued cert is an end-entity or a sub-CA | `aws-private-ca-issuer-review-agent` |
| IRSA / Managed Identity / OCI WI scope — blast radius of the compromised credential | cloud CA specialist + `kubernetes-workload-identity-review-agent` |
| CA type (ROOT vs SUBORDINATE) — ROOT online = entire PKI trust chain at risk | cloud CA specialist |
| Cert validity and CRL reachability — determines how long a maliciously issued cert remains trusted | cloud CA specialist |

No single agent closes all gates. Run `cert-manager-issuer-trust-review-agent` and the relevant cloud CA specialist together for any PKI security review.
