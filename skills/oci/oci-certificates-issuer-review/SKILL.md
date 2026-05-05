---
name: oci-certificates-issuer-review
description: Use this skill when reviewing OCI Certificates Service issuer configurations for cert-manager on OKE. Trigger on any request to audit OCI CA hierarchy, issuance rules, OKE Workload Identity vs Instance Principal auth, IAM policy scope, OCSP reachability, or certificate version management.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# OCI Certificates Issuer Review

## Purpose

Review Oracle Cloud Infrastructure (OCI) Certificates Service configurations used as cert-manager issuers on OKE (Oracle Kubernetes Engine). Identify CA hierarchy misconfigurations (root vs subordinate), missing issuance rules, overly broad IAM policies, Instance Principal authentication scope risks, OCSP reachability gaps, and certificate version accumulation. Output severity-labeled findings with evidence and remediation steps.

## Lean operating rules

- Flag any OCI issuer that references a ROOT CA directly as CRITICAL — only a SUBORDINATE CA should be used for cert-manager issuance. The ROOT CA must be offline (disabled after subordinate creation) or kept entirely out of the Certificates Service.
- Check whether OCI issuance rules are configured on the subordinate CA: flag missing validity caps (>90d) and missing key algorithm restrictions (RSA <2048 or EC <P-256) as MEDIUM.
- Identify the authentication method used by cert-manager to call OCI APIs: flag Instance Principal auth as HIGH — any pod on the OKE node can call the OCI Certificates API via instance metadata. Correct method is OKE Workload Identity (SA-bound, pod-level).
- Review the OCI IAM policy for cert-manager: flag `manage certificate-authorities` (grants delete/update CA) as HIGH. Minimum required: `use certificate-authorities` with `request.permission='CREATE_CERTIFICATE_REQUEST'`.
- Check OCSP reachability from OKE worker nodes to `ocsp.pki.oraclecloud.com`. Flag unreachable OCSP endpoint as MEDIUM (soft-fail revocation = revoked certs accepted by most TLS stacks).
- Review certificate version count; flag high version accumulation (> 10 versions per cert) as LOW (storage cost and management overhead).
- Label all findings as live evidence, documentation-based, or inference.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

- Severity-labeled findings list (CRITICAL / HIGH / MEDIUM / LOW)
- Evidence source for each finding
- Specific resource name, CA OCID, or IAM policy statement that caused the finding
- Recommended remediation with example OCI CLI command or IAM policy snippet
- Overall OCI PKI trust posture verdict
