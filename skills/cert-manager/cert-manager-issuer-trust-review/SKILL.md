---
name: cert-manager-issuer-trust-review
description: Use this skill when reviewing cert-manager PKI configuration for Kubernetes clusters. Trigger when the user asks about Issuer or ClusterIssuer scope, CertificateRequestPolicy coverage, certificate SAN or duration risks, trust-manager bundle distribution, SPIFFE mesh CA integration, cert-manager webhook health, or cloud CA authentication method.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# cert-manager Issuer Trust Review

## Purpose

Review cert-manager Issuer and ClusterIssuer scope, CertificateRequestPolicy (approver-policy) authorization coverage, certificate SAN wildcard and duration risks, trust-manager CA bundle distribution blast radius, SPIFFE/service-mesh CA integration, and cloud-backed CA authentication method. cert-manager's security posture depends on whether namespace-scoped request authorization exists — without CertificateRequestPolicy, any namespace can issue a certificate for any DNS name from a shared ClusterIssuer.

## Lean operating rules

- Prefer live evidence (`kubectl get clusterissuer,issuer -A -o yaml`, `kubectl get certificaterequestpolicy -o yaml`, `kubectl get certificate -A -o yaml`) when the active client exposes it; otherwise fall back to official cert-manager documentation and sanitized YAML from the user.
- Separate confirmed facts from inference. If CertificateRequestPolicy deployment, certificate health, or trust-manager bundle scope was not directly queried, say so.
- Treat no CertificateRequestPolicy deployed cluster-wide as a critical finding — any cert request in any namespace is auto-approved against any ClusterIssuer.
- Treat a ClusterIssuer backed by a corporate private CA with no namespace restriction via CertificateRequestPolicy as a high finding — any namespace can request corp-trusted certs.
- Treat Certificate `spec.dnsNames` containing wildcards like `*.internal.company.com` for a single microservice as a high finding — overly broad trust grants.
- Treat `spec.duration` exceeding 90 days for workload certs as a high finding; certs with `duration: 87600h` (10 years) are critical.
- Treat cert-manager-webhook in a degraded or failing state as a high finding — no new cert renewals can complete.
- Treat a trust-manager Bundle with no namespace selector distributing CA bundles to all namespaces as a medium finding unless intentionally cluster-wide.
- Keep the answer scoped, evidence-labeled, and explicit about what was not queried.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

Return, at minimum:
- the scoped target (ClusterIssuer, Issuer, Certificate, CertificateRequestPolicy, or trust-manager Bundle) and evidence level,
- the issuer type and backing CA (self-signed, ACME, AWS PCA, Azure Key Vault, Vault, etc.) and whether it is namespace-scoped or cluster-scoped,
- CertificateRequestPolicy presence and subject/issuer constraint coverage,
- certificate SAN scope and duration for any reviewed Certificate resources,
- trust-manager Bundle distribution scope,
- the safest next actions and any assumptions or blockers.
