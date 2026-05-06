---
name: aws-private-ca-issuer-review
description: Use this skill when reviewing AWS ACM Private CA (Private Certificate Authority) issuer configurations for cert-manager. Trigger on any request to audit AWSPCAIssuer, AWSPCAClusterIssuer, IRSA policy for cert-manager, certificate template ARNs, CRL configuration, or cross-account PCA usage.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# AWS Private CA Issuer Review

## Purpose

Review AWS ACM Private Certificate Authority configurations used by the cert-manager `aws-privateca-issuer` plugin. Identify CA hierarchy misconfigurations, overly permissive certificate templates, excessive IRSA permissions, unsafe validity periods, CRL reachability gaps, and cross-account PCA setup risks.

## Lean operating rules

- Flag any `AWSPCAIssuer` referencing a ROOT CA ARN directly as CRITICAL — only a SUBORDINATE CA should be active for cert-manager issuance.
- Check `spec.template.arn`: flag any SubordinateCACertificate template as CRITICAL (allows cert-manager to mint sub-CAs). Correct template is `EndEntityCertificate/V1`.
- Review IRSA role policy: required actions are `acm-pca:IssueCertificate`, `acm-pca:GetCertificate`, `acm-pca:DescribeCertificateAuthority`. Flag `acm-pca:DeleteCertificateAuthority` or `acm-pca:CreateCertificateAuthority` as HIGH.
- Review `spec.duration` in Certificate resources; flag durations > 365d for workload certs as MEDIUM; best practice is <= 90d.
- Check CRL S3 bucket reachability from within the VPC; flag unreachable CRL distribution points as HIGH (revocation disabled).
- For cross-account PCA (RAM-shared CA): verify minimum issuance-only permissions in the security account.
- Label all claims as live evidence, documentation-based, or inference.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md)
- [Safety checklist](references/safety-checklist.md)
- [Official sources](references/official-sources.md)

## Response minimum

- Severity-labeled findings list (CRITICAL / HIGH / MEDIUM / LOW)
- Evidence source for each finding
- Specific resource name or field path
- Recommended remediation with example policy or YAML snippet
- Overall PKI trust posture verdict
