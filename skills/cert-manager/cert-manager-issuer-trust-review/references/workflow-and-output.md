# Workflow and Output Contract

## Workflow

### Step 1 — Identify scope and collect raw evidence

1. Confirm the review target: a ClusterIssuer, a namespace-scoped Issuer, a Certificate resource, a CertificateRequestPolicy, or a trust-manager Bundle.
2. List all issuers and their types:
   ```bash
   kubectl get clusterissuer -o yaml
   kubectl get issuer -A -o yaml
   ```
   For each issuer, note the `spec` type: `acme`, `ca`, `selfSigned`, `vault`, `venafi`, `acmepca` (AWS), `azureKeyVault`.
3. List all CertificateRequestPolicy resources (approver-policy CRD):
   ```bash
   kubectl get certificaterequestpolicy -o yaml
   ```
   If the CRD does not exist, approver-policy is not installed — all cert requests are auto-approved. Record this as a critical gap.
4. List certificates with their issuers and SAN content:
   ```bash
   kubectl get certificate -A -o custom-columns=\
   "NS:.metadata.namespace,NAME:.metadata.name,ISSUER:.spec.issuerRef.name,\
   KIND:.spec.issuerRef.kind,DURATION:.spec.duration,DNS:.spec.dnsNames"
   ```

### Step 2 — Audit ClusterIssuer vs Issuer scope

1. For every ClusterIssuer, determine what namespaces can reference it:
   - A `ClusterIssuer` has no namespace — any Certificate in any namespace can reference it.
   - An `Issuer` is namespace-scoped — only Certificates in the same namespace can reference it.
2. For cloud-backed ClusterIssuers (AWS PCA, Azure Key Vault, Vault), check the authentication method:
   ```bash
   # AWS PCA ClusterIssuer — check for IRSA annotation
   kubectl get clusterissuer <name> -o jsonpath='{.spec.acmepca}' 2>/dev/null
   kubectl get serviceaccount -n cert-manager cert-manager -o jsonpath='{.metadata.annotations}'
   ```
   Flag as **HIGH** if the ClusterIssuer authenticates to a cloud CA using static credentials (AWS access key, Azure client secret) instead of workload identity (IRSA, Azure Workload Identity).
3. Example of a safely scoped setup vs a risky setup:
   ```yaml
   # SAFE: Namespace-scoped Issuer, only one namespace can use it
   apiVersion: cert-manager.io/v1
   kind: Issuer
   metadata:
     name: internal-ca
     namespace: payments
   spec:
     ca:
       secretName: payments-ca-secret

   # RISKY: ClusterIssuer for corporate CA with no request policy
   apiVersion: cert-manager.io/v1
   kind: ClusterIssuer
   metadata:
     name: corp-private-ca
   spec:
     acmepca:
       arn: arn:aws:acm-pca:us-east-1:123456789:certificate-authority/abc
   ```

### Step 3 — Audit CertificateRequestPolicy coverage

CertificateRequestPolicy is the RBAC layer for PKI. Without it, any Certificate resource is auto-approved.

1. Verify approver-policy is installed:
   ```bash
   kubectl get crd certificaterequestpolicies.policy.cert-manager.io
   ```
   If not found, record as **CRITICAL**: all certificate requests are auto-approved.
2. For each CertificateRequestPolicy, inspect the subject constraints:
   ```bash
   kubectl get certificaterequestpolicy <name> -o yaml
   ```
   Check:
   - `spec.allowed.dnsNames.values` — which DNS names the policy permits
   - `spec.allowed.dnsNames.validations` — regex constraints on allowed names
   - `spec.allowed.subject` — allowed subject distinguished names
   - `spec.selector.issuerRef` — which issuers this policy covers
   - `spec.selector.namespace` — which namespaces this policy governs
3. Example of a correctly constrained CertificateRequestPolicy:
   ```yaml
   apiVersion: policy.cert-manager.io/v1alpha1
   kind: CertificateRequestPolicy
   metadata:
     name: payments-internal-certs
   spec:
     allowed:
       dnsNames:
         values:
           - "*.payments.svc.cluster.local"
         validations:
           - rule: self.endsWith('.payments.svc.cluster.local')
             message: "DNS name must be in payments namespace service domain"
       subject:
         organizations:
           values: ["payments-team"]
       usages:
         - "digital signature"
         - "key encipherment"
         - "server auth"
         - "client auth"
     selector:
       issuerRef:
         name: corp-private-ca
         kind: ClusterIssuer
         group: cert-manager.io
       namespace:
         matchLabels:
           team: payments
   ```
4. Flag as **CRITICAL** if no CertificateRequestPolicy restricts a ClusterIssuer backed by a corporate or cloud CA.
5. Flag as **HIGH** if a CertificateRequestPolicy allows `dnsNames` with a wildcard that covers high-value internal FQDNs (e.g., `*.internal.company.com`).

### Step 4 — Audit Certificate SAN and duration

1. For each Certificate, review `spec.dnsNames` for excessive scope:
   ```bash
   kubectl get certificate -A -o yaml | grep -A 5 "dnsNames"
   ```
2. Flag as **HIGH** any Certificate where a single microservice's cert includes:
   - `*.internal.company.com` (covers all internal services)
   - `*.svc.cluster.local` (covers all cluster services)
3. Review certificate duration and renewal:
   ```bash
   kubectl get certificate -A -o custom-columns=\
   "NAME:.metadata.name,DURATION:.spec.duration,RENEW:.spec.renewBefore,READY:.status.conditions[0].status"
   ```
   - Flag as **HIGH** if `duration` exceeds `8760h` (1 year) for workload certs.
   - Flag as **CRITICAL** if `duration` is `87600h` (10 years) or similar for workload certs.
   - Flag as **MEDIUM** if `renewBefore` is not set or is less than 1/3 of `duration`.
4. Verify certificate readiness:
   ```bash
   kubectl get certificate -A | grep -v "True"
   ```
   Any certificate not in `Ready=True` state that is approaching expiry is a **HIGH** finding.

### Step 5 — Audit cert-manager webhook health

A failing cert-manager webhook blocks all new certificate issuance and renewals.

1. Check webhook pod health:
   ```bash
   kubectl get pods -n cert-manager
   kubectl describe deployment cert-manager-webhook -n cert-manager
   ```
2. Check webhook configuration:
   ```bash
   kubectl get validatingwebhookconfiguration cert-manager-webhook -o yaml | grep -A 5 "failurePolicy"
   ```
   `failurePolicy: Fail` means a webhook outage blocks all cert operations. `failurePolicy: Ignore` means webhook failures are skipped — cert validation is bypassed.
3. Check for recent CertificateRequest failures:
   ```bash
   kubectl get certificaterequest -A | grep -v "True"
   kubectl describe certificaterequest -A | grep -A 5 "Reason:"
   ```
4. Flag as **HIGH** if the cert-manager-webhook deployment has unavailable replicas and any certificates are approaching expiry within 30 days.

### Step 6 — Audit trust-manager Bundle distribution

1. List trust-manager Bundles:
   ```bash
   kubectl get bundle -o yaml
   kubectl get configmapbundle -o yaml 2>/dev/null
   ```
2. For each Bundle, check the target namespace selector:
   ```yaml
   # RISKY: no namespaceSelector distributes to all namespaces
   spec:
     target:
       configMap:
         key: "bundle.pem"
       namespaceSelector: {}   # matches all namespaces

   # SAFE: explicit namespace label selector
   spec:
     target:
       configMap:
         key: "bundle.pem"
       namespaceSelector:
         matchLabels:
           cert-manager.io/trust-bundle: "enabled"
   ```
3. Flag as **MEDIUM** if a Bundle distributes a corporate or cloud CA bundle to all namespaces without a restrictive namespace selector — untrusted workloads receive the CA and can potentially use it for internal service impersonation if combined with a cert issuance gap.

### Step 7 — Audit SPIFFE / service mesh CA integration

1. Check if cert-manager is serving as the Istio CA via istio-csr:
   ```bash
   kubectl get pods -n istio-system | grep cert-manager
   kubectl get cm istio -n istio-system -o yaml | grep caAddress
   ```
2. If cert-manager feeds the mesh trust domain, the ClusterIssuer it references is the root of trust for all SPIFFE SVIDs in the mesh.
   - A compromised ClusterIssuer in this scenario allows forging any SPIFFE SVID for any mesh workload.
   - Flag as **HIGH** if the mesh CA ClusterIssuer uses a shared corporate private CA without CertificateRequestPolicy constraints on the istio-csr service account.
3. For Linkerd:
   ```bash
   kubectl get secret linkerd-identity-issuer -n linkerd -o yaml | grep -v "^  tls"
   ```
   Verify the issuer cert expiry is managed by cert-manager and has a `renewBefore` set.

## Output

Return:

- **target**: ClusterIssuer/Issuer names, Certificate references, or CertificateRequestPolicy names, with evidence source,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **issuer scope**: namespace-scoped Issuer or cluster-wide ClusterIssuer, backing CA type, authentication method (workload identity vs static credentials),
- **CertificateRequestPolicy coverage**: present/absent, constrained issuers, allowed DNS names scope, namespace selector,
- **certificate SAN and duration audit**: wildcard SAN findings, duration exceeding recommended thresholds, renewBefore settings,
- **webhook health**: cert-manager-webhook pod state, failurePolicy, any CertificateRequest failures,
- **trust-manager posture**: Bundle distribution scope, namespace selector presence,
- **mesh integration**: whether cert-manager feeds a mesh CA and the blast radius of that issuer,
- **risk findings** (with severity: critical / high / medium / low),
- **safest next actions** with sample YAML,
- **assumptions and missing facts**.

## Security notes

- Never recommend removing CertificateRequestPolicy to unblock a blocked cert request — the correct path is to add an appropriate policy.
- Never request or print CA private key contents, PKCS#12 bundles, Vault tokens, or AWS credentials.
- A ClusterIssuer backed by a corporate Private CA with no CertificateRequestPolicy is equivalent to an open PKI endpoint — any namespace can issue trusted certs for any FQDN.
- Always confirm approver-policy CRD presence before concluding that cert requests are constrained.
- cert-manager `failurePolicy: Ignore` on the webhook means the webhook can be bypassed — verify this is not used in production cert issuance paths for sensitive CAs.
