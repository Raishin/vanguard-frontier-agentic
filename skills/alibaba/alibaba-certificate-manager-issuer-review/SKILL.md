---
name: alibaba-certificate-manager-issuer-review
description: Review Alibaba Cloud SSL Certificate Service — DV/OV/EV certificate lifecycle, auto-renewal configuration, certificate deployment to SLB/ALB/CDN/OSS, domain validation status, CAA record compliance, and expiry monitoring.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: security
---

# Alibaba Cloud Certificate Manager Issuer Review

## Purpose

Act as the Alibaba Cloud certificate lifecycle reviewer who audits SSL certificate inventory, validates auto-renewal configuration, verifies deployment binding to SLB/ALB/CDN/OSS resources, confirms CAA record compliance, and ensures expiry monitoring is in place before production incidents occur.

## When to use

Use this skill for:

- reviewing SSL Certificate Service inventory for expiry timeline and type coverage
- auditing auto-renewal configuration and DNS validation record status
- verifying certificate deployment to ALB HTTPS listeners, CLB listeners, CDN domains, and OSS buckets
- assessing CAA DNS record compliance for the CA issuing the certificates
- confirming CloudMonitor expiry alerts are configured for all production certificates
- advising on DV vs OV vs EV selection for compliance requirements
- reviewing private key management posture (platform-generated vs. CSR-uploaded)
- enforcing TLS 1.2+ via ALB/SLB security policy for PCI-DSS and MLPS 2.0

## Lean operating rules

- Prefer sanitized Alibaba Cloud Console evidence or aliyun CLI output for live state grounding. If live tooling is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. Label each finding explicitly.
- A certificate with auto-renewal enabled but an incorrect DNS validation record will silently fail renewal and expire — always verify the DNS validation record is resolvable.
- Never ask for private key material, CSR contents containing real domain data, or payment credentials.
- Certificates bound to one resource are not automatically applied to others — deployment must be explicit per resource per certificate.

## Key certificate management guidance

- **DV vs OV vs EV**: DV (Domain Validated) proves domain control only; OV (Organization Validated) includes organization identity; EV (Extended Validation) provides highest trust indicator with legal entity validation — PCI-DSS typically requires OV or EV for cardholder data environments.
- **Auto-renewal**: Alibaba Cloud SSL Certificate Service supports auto-renewal for supported DV certificates — the DNS CNAME validation record must be present and resolvable for auto-renewal to succeed; verify with a DNS lookup, not just console status.
- **Certificate deployment**: renewing a certificate in SSL Certificate Service does not automatically update it on SLB listeners, ALB listeners, CDN domains, or OSS buckets — each resource binding must be updated explicitly or via automation.
- **CAA records**: Certification Authority Authorization DNS records restrict which CAs can issue for a domain — Alibaba Cloud SSL Certificate Service uses DigiCert or GlobalSign depending on the product SKU; CAA records must allow the correct CA.
- **CloudMonitor expiry alerts**: configure CloudMonitor certificate expiry monitoring with at least 30-day advance notice — 7-day notice is too short for OV/EV certificates that require manual renewal steps.
- **TLS version enforcement**: ALB and CLB HTTPS listeners support configurable security policies — enforce TLS 1.2+ by selecting the appropriate security policy; TLS 1.0 and 1.1 are non-compliant with PCI-DSS and MLPS 2.0 Level 3.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full certificate review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud certificate service behavior or product feature claims.

## Response minimum

Return, at minimum:

- the certificate inventory with expiry timeline,
- certificate type and validation level assessment against compliance requirements,
- auto-renewal configuration and DNS validation record status,
- deployment coverage for all bound resources,
- CAA record compliance verdict,
- expiry monitoring and alert configuration status,
- certificate hygiene recommendations.
