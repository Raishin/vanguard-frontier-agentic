# Workflow and Output Contract

## Review Workflow

### Step 1 — Identify the CA and issuer configuration

Retrieve the OCI cert-manager issuer resource:

```bash
kubectl get issuer -A -o yaml | grep -A20 "oci\|oracle"
kubectl get clusterissuer -o yaml | grep -A20 "oci\|oracle"
```

Extract the CA OCID from the issuer spec. Then inspect the CA in OCI:

```bash
oci certs-mgmt certificate-authority get \
  --certificate-authority-id <ca-ocid>
```

Key fields to check:
- `type` — must be `SUBORDINATE` (not `ROOT`)
- `lifecycleState` — must be `ACTIVE`
- `issuerCertificateAuthorityId` — should reference a ROOT CA that is itself INACTIVE or not used for direct issuance

### Step 2 — Validate CA type (root vs subordinate)

```bash
oci certs-mgmt certificate-authority get \
  --certificate-authority-id <ca-ocid> \
  --query data.config-type \
  --raw-output
```

Expected values:
- `SUBORDINATE_CA_ISSUED_BY_INTERNAL_CA` — correct for cert-manager usage
- `ROOT_CA_GENERATED_INTERNALLY` — CRITICAL finding; root directly exposed to cert-manager

Also check the issuer CA's status:
```bash
oci certs-mgmt certificate-authority get \
  --certificate-authority-id <ca-ocid> \
  --query data.lifecycle-state \
  --raw-output
```

### Step 3 — Review issuance rules

List issuance rules configured on the CA:

```bash
oci certs-mgmt certificate-authority get \
  --certificate-authority-id <ca-ocid> \
  --query "data.certificate-authority-rules"
```

Check for:

```json
{
  "ruleType": "CERTIFICATE_AUTHORITY_MAX_VALIDITY_RULE",
  "certificateMaxValidityDuration": "P90D"
}
```

And key algorithm restriction:

```json
{
  "ruleType": "CERTIFICATE_AUTHORITY_ISSUANCE_EXPIRY_RULE",
  "leafCertificateMaxValidityDuration": "P90D",
  "certificateAuthorityMaxValidityDuration": "P3650D"
}
```

**Flags:**
- No issuance rules configured (no validity cap) — MEDIUM (cert-manager can issue 10-year workload certs)
- Max validity > 365d for leaf certificates — MEDIUM
- No key algorithm restriction — MEDIUM (RSA-1024 issuance possible)

### Step 4 — Identify authentication method

Check the cert-manager configuration for OCI auth method:

```bash
# Check if OKE Workload Identity is configured
kubectl get serviceaccount cert-manager -n cert-manager \
  -o jsonpath='{.metadata.annotations}'
```

For OKE Workload Identity, the ServiceAccount should have OCI annotations:

```yaml
annotations:
  oci.oraclecloud.com/role-binding: "<dynamic-group-name>"
```

For Instance Principal auth, check if the cert-manager pod uses the instance metadata endpoint:

```bash
# Check the cert-manager deployment for OCI config
kubectl get deployment cert-manager -n cert-manager -o yaml | grep -i "oci\|instance\|workload"
```

**Auth method comparison:**

| Method | Scope | Risk |
|--------|-------|------|
| OKE Workload Identity | ServiceAccount-bound (pod-level) | Correct — minimum scope |
| Instance Principal | Node-level (all pods on node) | HIGH — any pod can issue certs |
| User auth (API key) | User credentials in secret | HIGH — credential rotation required |

### Step 5 — Review IAM policy

Retrieve the IAM policy for cert-manager:

```bash
oci iam policy list --compartment-id <compartment-id> --all \
  --query "data[?contains(statements[0], 'certificate-authority')]"
```

Minimum required policy statement:

```
Allow dynamic-group CertManagerDynamicGroup to use certificate-authorities
  in compartment <compartment-name>
  where request.permission='CREATE_CERTIFICATE_REQUEST'
```

**Flag as HIGH if the policy includes any of:**
- `manage certificate-authorities` (grants delete, update, disable, schedule-deletion)
- `manage certificates` without compartment scoping (affects all certs)
- Wildcard resources or compartment `tenancy` instead of scoped compartment

Additional permissions needed for cert-manager to retrieve issued certs:

```
Allow dynamic-group CertManagerDynamicGroup to read certificates
  in compartment <compartment-name>
```

### Step 6 — Check OCSP reachability

The OCI OCSP endpoint is `ocsp.pki.oraclecloud.com`. Verify reachability from OKE worker nodes:

```bash
# From within an OKE node or debug pod
curl -sv https://ocsp.pki.oraclecloud.com/
```

For OKE clusters with no internet gateway or restrictive security group rules:

```bash
# Check security list / NSG rules for outbound HTTPS to OCI OCSP
oci network security-list list --vcn-id <vcn-id> \
  --query "data[].egress-security-rules[]"
```

OCI OCSP endpoints use HTTPS (443). Ensure the OKE worker node security group allows outbound TCP/443 to OCI service endpoints. Using a Service Gateway with the `OCI Services in Oracle Services Network` service covers OCI PKI endpoints.

**Flags:**
- No Service Gateway configured and no internet gateway (OCI OCSP unreachable) — MEDIUM
- Security group blocks TCP/443 outbound to OCI service network — MEDIUM

### Step 7 — Review certificate version count

```bash
oci certs-mgmt certificate list-certificate-versions \
  --certificate-id <cert-ocid> \
  --all \
  --query "length(data)"
```

Each cert rotation by cert-manager creates a new version. Old versions should be cleaned up to avoid high version counts.

**Flags:**
- Certificate version count > 10 — LOW (storage cost and management overhead)
- No automated cleanup of old versions configured — LOW

---

## Output Format

### Finding: `<short title>`

| Field | Value |
|-------|-------|
| Severity | CRITICAL / HIGH / MEDIUM / LOW |
| Resource | CA OCID, IAM policy name, or cert name |
| Evidence | documentation-based / live evidence / inference |
| Description | What is wrong and its impact on PKI trust |
| Remediation | OCI CLI command, IAM policy statement, or configuration change |

---

### Overall OCI PKI Trust Posture

| Category | Status |
|----------|--------|
| CA hierarchy (subordinate only) | PASS / FAIL |
| Issuance rules (validity caps) | PASS / FAIL |
| Authentication method (Workload Identity) | PASS / FAIL |
| IAM policy scope (minimum permissions) | PASS / FAIL |
| OCSP reachability | PASS / FAIL |
| Certificate version lifecycle | PASS / FAIL |

**Verdict:** TRUSTED / UNTRUSTED / CONDITIONAL (list conditions)
