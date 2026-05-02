# Workflow and Output Contract

## Review Workflow

### Step 1 — Identify the issuer resource type

Determine whether the configuration uses `AWSPCAIssuer` (namespace-scoped) or `AWSPCAClusterIssuer` (cluster-scoped):

```bash
kubectl get awspcaissuer -A
kubectl get awspcaclusterissuer
```

Retrieve the issuer spec:

```bash
kubectl get awspcaissuer <name> -n <namespace> -o yaml
kubectl get awspcaclusterissuer <name> -o yaml
```

Key fields to extract:
- `spec.arn` — the CA ARN (must be a SUBORDINATE CA, not ROOT)
- `spec.region` — AWS region of the CA
- `spec.signingAlgorithm` — signing algorithm
- `spec.template.arn` — certificate template ARN (controls what types of certs can be issued)

### Step 2 — Validate CA ARN type

Use the AWS CLI to confirm the CA type:

```bash
aws acm-pca describe-certificate-authority \
  --certificate-authority-arn <arn> \
  --query 'CertificateAuthority.Type' \
  --output text
```

Expected output: `SUBORDINATE`

If output is `ROOT` — this is a CRITICAL finding. cert-manager is directly wired to the root of trust.

Also check CA status:
```bash
aws acm-pca describe-certificate-authority \
  --certificate-authority-arn <arn> \
  --query 'CertificateAuthority.Status' \
  --output text
```

Expected: `ACTIVE`. If `DISABLED` or `DELETED`, the issuer will fail silently until the CA is restored.

### Step 3 — Validate certificate template ARN

The template ARN controls what type of certificate ACM PCA will issue. Common template ARNs:

| Template ARN Suffix | Purpose | Risk |
|---------------------|---------|------|
| `EndEntityCertificate/V1` | Standard workload cert | Safe — correct choice |
| `EndEntityClientAuthCertificate/V1` | Client auth cert | Safe for mTLS |
| `SubordinateCACertificate_PathLen0/V1` | Subordinate CA cert | CRITICAL — allows sub-CA issuance |
| `SubordinateCACertificate_PathLen1/V1` | Subordinate CA with chain | CRITICAL |
| `RootCACertificate/V1` | Root CA cert | CRITICAL |

Full ARN format:
```
arn:aws:acm-pca:::template/EndEntityCertificate/V1
```

If no template is specified in the issuer, PCA defaults to `EndEntityCertificate/V1` — verify this assumption against the actual ACM PCA issuance policy.

### Step 4 — Review IRSA IAM role policy

Retrieve the IAM role attached to the cert-manager ServiceAccount:

```bash
kubectl get serviceaccount cert-manager -n cert-manager -o jsonpath='{.metadata.annotations.eks\.amazonaws\.com/role-arn}'
```

Retrieve and review the role policy:

```bash
aws iam list-role-policies --role-name <role-name>
aws iam get-role-policy --role-name <role-name> --policy-name <policy-name>
```

Minimum required IAM policy:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "acm-pca:IssueCertificate",
        "acm-pca:GetCertificate",
        "acm-pca:DescribeCertificateAuthority"
      ],
      "Resource": "arn:aws:acm-pca:<region>:<account>:certificate-authority/<ca-id>"
    }
  ]
}
```

**Flag as HIGH if the policy includes any of:**
- `acm-pca:DeleteCertificateAuthority`
- `acm-pca:CreateCertificateAuthority`
- `acm-pca:UpdateCertificateAuthority`
- `acm-pca:RestoreCertificateAuthority`
- `acm-pca:*` (wildcard)
- Resource set to `*` instead of scoped CA ARN

### Step 5 — Review Certificate validity periods

List all cert-manager Certificate resources and their durations:

```bash
kubectl get certificate -A -o custom-columns=\
NAMESPACE:.metadata.namespace,\
NAME:.metadata.name,\
DURATION:.spec.duration,\
RENEW_BEFORE:.spec.renewBefore,\
ISSUER:.spec.issuerRef.name
```

Validity guidelines:
- Workload certs: <= 90d (best practice), <= 365d (acceptable)
- Internal service mesh mTLS: <= 24h (optimal)
- Long-lived infrastructure certs: <= 2y (acceptable with documented justification)

Note: ACM PCA silently caps certificate validity at the CA's own remaining validity. A cert with `duration: 87600h` (10 years) issued by a CA expiring in 2 years will be capped at 2 years without error. Always verify the CA's own expiration date:

```bash
aws acm-pca describe-certificate-authority \
  --certificate-authority-arn <arn> \
  --query 'CertificateAuthority.NotAfter' \
  --output text
```

### Step 6 — Review CRL configuration and reachability

Check the CRL configuration on the CA:

```bash
aws acm-pca describe-certificate-authority \
  --certificate-authority-arn <arn> \
  --query 'CertificateAuthority.RevocationConfiguration'
```

Verify the CRL S3 bucket name from the output. Then check reachability from within the VPC:

- Does the VPC have an S3 Gateway VPC endpoint for the CRL bucket's region?
- Is the CRL S3 bucket policy allowing access from the VPC?
- Is the CRL distribution point URL embedded in issued certs accessible?

```bash
# Check for S3 gateway VPC endpoint
aws ec2 describe-vpc-endpoints \
  --filters "Name=service-name,Values=com.amazonaws.<region>.s3" \
             "Name=vpc-id,Values=<vpc-id>"
```

If the CRL S3 bucket requires a VPC endpoint and none exists, revocation checking is effectively disabled (most TLS clients soft-fail on CRL/OCSP unreachability).

### Step 7 — Cross-account PCA review (if applicable)

Identify if the CA ARN belongs to a different AWS account than the EKS cluster:

```bash
# Extract account ID from CA ARN
echo "arn:aws:acm-pca:<region>:<account-id>:certificate-authority/<id>"
# Compare with current account
aws sts get-caller-identity --query Account --output text
```

For cross-account configurations:

1. Verify the RAM share exists in the security account:
```bash
aws ram list-resources --resource-owner SELF --resource-type acm-pca:CertificateAuthority
```

2. Verify the workload-account IRSA role trust policy references the correct EKS OIDC provider.

3. Confirm the cross-account IAM permissions follow least-privilege (issuance only, not management).

---

## Output Format

### Finding: `<short title>`

| Field | Value |
|-------|-------|
| Severity | CRITICAL / HIGH / MEDIUM / LOW |
| Resource | AWSPCAIssuer name, CA ARN, IAM role, or cert name |
| Evidence | documentation-based / live evidence / inference |
| Description | What is wrong and why it matters for PKI trust |
| Remediation | IAM policy snippet, ARN change, or configuration fix |

---

### Overall PKI Trust Posture

| Category | Status |
|----------|--------|
| CA hierarchy (subordinate only) | PASS / FAIL |
| Certificate template scope | PASS / FAIL |
| IRSA permissions (least-privilege) | PASS / FAIL |
| Certificate validity periods | PASS / FAIL |
| CRL reachability | PASS / FAIL |
| Cross-account configuration | PASS / N/A / FAIL |

**Verdict:** TRUSTED / UNTRUSTED / CONDITIONAL (list conditions)
