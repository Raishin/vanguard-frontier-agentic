# Private CA Issuer Trust Boundaries Guide

Use this reference for AWS ACM Private CA issuer reviews involving cert-manager `AWSPCAIssuer`, `AWSPCAClusterIssuer`, IRSA, certificate templates, subordinate CA usage, CRL reachability, cross-account CA sharing, and Kubernetes certificate issuance.

## What people get wrong

The lazy story is:

> cert-manager can issue certificates, so the issuer is configured correctly.

Wrong. A working issuer can still mint the wrong certificate class, use the wrong CA tier, bypass namespace boundaries, break revocation, or grant cert-manager CA-administration permissions.

Common bad assumptions:

- A ROOT CA ARN is acceptable if issuance works.
- Any ACM PCA template is safe for workloads.
- IRSA permissions can be broad because cert-manager is trusted infrastructure.
- CRL distribution is optional for private workloads.
- Cross-account PCA sharing removes the need for tight security-account controls.
- Long workload certificate duration is operationally safer.

## Private-CA failure modes

- `AWSPCAIssuer` or `AWSPCAClusterIssuer` references a root CA instead of subordinate CA.
- Certificate template allows subordinate CA issuance instead of end-entity certificates.
- IRSA role includes `CreateCertificateAuthority`, `DeleteCertificateAuthority`, broad resource `*`, or unrelated PCA actions.
- Cluster-scoped issuer allows namespaces to request certificates outside their trust boundary.
- CRL S3 distribution point is unreachable from clients, disabling practical revocation.
- Cross-account RAM/shared CA policy grants issuance without namespace/cluster/account guardrails.

## Minimum safe workflow

1. Identify issuer kind, namespace scope, CA ARN, CA type, template ARN, Kubernetes service account, and IRSA role.
2. Verify CA hierarchy: workload issuers use subordinate CA, not root CA; root remains offline/highly restricted.
3. Verify certificate template is end-entity only unless there is explicit approved subordinate-CA issuance use case.
4. Review IAM/IRSA for minimum issuance actions: IssueCertificate, GetCertificate, DescribeCertificateAuthority.
5. Check certificate duration, renewBefore, key usages, SANs, namespace boundaries, and approval policy.
6. Verify CRL/OCSP/revocation distribution reachability and audit logging.
7. Require explicit approval before changing CA, issuer, template, or IRSA policy.

## Verification targets

- cert-manager `Certificate`, `Issuer`, `ClusterIssuer`, `AWSPCAIssuer`, and `AWSPCAClusterIssuer` YAML fields
- ACM Private CA type, state, ARN, certificate template ARN, CRL configuration, audit report, and CloudTrail issuance events
- IRSA role trust policy, service account annotation, IAM actions/resources, and permissions boundary/SCP effects
- Kubernetes namespace/RBAC boundaries, approver policy, cert-manager logs/events, and certificate request status
- CRL S3 bucket policy, KMS policy, network reachability, client trust stores, and revocation test evidence
- cross-account RAM/shared CA permissions and security-account ownership

## When to push back

Push back if the user asks to:

- issue workload certificates directly from a root CA
- allow cert-manager to mint subordinate CAs
- grant broad ACM PCA admin actions to IRSA
- ignore CRL reachability or revocation evidence
- approve long-lived workload certificates without risk acceptance
- treat successful issuance as proof of safe PKI posture
