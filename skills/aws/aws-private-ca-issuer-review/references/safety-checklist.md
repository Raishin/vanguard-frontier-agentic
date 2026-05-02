# Safety checklist

Use this reference before any recommendations on cert-manager PKI configuration, IRSA policy changes, CA hierarchy decisions, or CRL distribution point design.

## Non-negotiables

- Never ask users to paste secrets, access keys, private keys, CA passwords, or PKCS#12 bundles into chat.
- Prefer official AWS MCP tools or sanitized `kubectl get` / `aws acm-pca` CLI output for current-state evidence. Label the evidence level.
- Do not invent CA ARNs, certificate template ARNs, IRSA role ARNs, or RAM resource share IDs.
- Require explicit platform-team sign-off before any change that modifies a CA hierarchy, revokes a CA, or deletes a PCA CRL S3 bucket.
- Keep IRSA permissions scoped to the minimum: `acm-pca:IssueCertificate`, `acm-pca:GetCertificate`, `acm-pca:DescribeCertificateAuthority`.

## PKI attack vector: required awareness

cert-manager with an `AWSPCAClusterIssuer` that has `acm-pca:IssueCertificate` via IRSA can issue certificates for any DNS name trusted by your internal PKI. A compromised cert-manager pod is equivalent to a compromised subordinate CA. Always review:
- Which namespaces can request from this ClusterIssuer (CertificateRequestPolicy coverage)
- Whether the CA certificate template allows sub-CA issuance (SubordinateCACertificate templates are CRITICAL)
- Whether the certificate SAN validation enforces DNS name scope

## Stress checks

- Which workloads trust this CA chain — blast radius of CA compromise?
- Can cert-manager request certificates for arbitrary SANs without CertificateRequestPolicy guard?
- Is the CA ROOT or SUBORDINATE? (ROOT issuance is CRITICAL)
- Is the CRL S3 bucket reachable from all pods that verify TLS using this CA?
- Is cross-account RAM share scoped to specific organizational units?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live AWS state.
