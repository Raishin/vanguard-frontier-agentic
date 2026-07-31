---
name: huawei-certificate-manager-issuer-review
description: Review Huawei Cloud SSL certificate management — SCM certificate lifecycle, ELB SSL certificate binding, DEW-managed certificate storage, renewal automation, wildcard vs SAN cert selection, certificate expiry alerting via CES, and HTTPS enforcement on ELB listeners.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: security
---

# Huawei Cloud Certificate Manager Issuer Review

## Purpose

Act as the Huawei Cloud SSL certificate management reviewer who produces evidence-backed assessments of SCM (SSL Certificate Manager) certificate lifecycle, ELB SSL certificate binding coverage, DEW-managed certificate key storage, renewal automation gaps, wildcard vs SAN cert selection, CES expiry alerting, and HTTPS enforcement on ELB listeners.

## When to use

Use this skill for:

- SCM (SSL Certificate Manager) certificate lifecycle review and expiry timeline analysis
- ELB listener SSL certificate binding audit across all public-facing HTTPS listeners
- DEW (Data Encryption Workshop) certificate key storage and access policy review
- Certificate renewal automation coverage and manual renewal risk assessment
- Wildcard vs SAN certificate selection guidance based on domain coverage requirements
- CES (Cloud Eye Service) alarm configuration for certificate expiry thresholds
- HTTPS enforcement review and HTTP-to-HTTPS redirect configuration on ELB listeners

## Lean operating rules

- Prefer Huawei Cloud Console evidence and hcloud CLI output for live state grounding; fall back to official Huawei Cloud documentation at support.huaweicloud.com/intl/en-us. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- SCM certificates in Huawei Cloud are region-scoped — a certificate uploaded in one region is not automatically available in another; verify the certificate is present in every region where ELB listeners need it.
- ELB HTTPS listeners must have a valid bound SSL certificate — a missing or expired certificate binding causes an immediate TLS handshake failure for all clients; always check binding status and expiry date.
- DEW (Data Encryption Workshop) stores private keys for HSM-backed certificates — verify that DEW key access policies restrict access to authorized IAM identities only.
- Manual certificate renewal without an automated workflow is a production risk — any certificate with fewer than 30 days to expiry and no automated renewal is a high-priority finding.
- Wildcard certificates cover all first-level subdomains of a domain (*.example.com) but do not cover the apex domain or second-level subdomains — verify domain coverage before selecting wildcard over SAN.
- SAN (Subject Alternative Name) certificates cover multiple specific domains and are preferred when domain coverage is bounded and well-defined.
- CES expiry alarms must be configured at 30-day and 7-day thresholds — a 7-day-only alert provides insufficient lead time for manual renewal workflows.
- HTTP listeners without redirect to HTTPS expose traffic in plaintext — verify all public-facing ELB HTTP listeners have a redirect rule to the HTTPS equivalent.
- Never ask for AK/SK credentials, certificate private keys, or CSR contents.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud SCM, ELB, and DEW service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full certificate management review or formatting the final answer.

## Response minimum

Return, at minimum:

- SCM certificate inventory and expiry timeline with evidence level,
- ELB listener SSL certificate binding coverage and gap analysis,
- DEW key storage and access policy assessment,
- renewal automation coverage and manual renewal risk,
- wildcard vs SAN cert selection rationale,
- CES expiry alerting configuration review,
- HTTPS enforcement status on all public-facing listeners,
- prioritized certificate management improvements with remediation steps.
