---
name: azure-waf-security-review
description: "Review Azure workload security posture against the Well-Architected Framework Security pillar: identity and access, network boundaries, data protection, threat detection, DevSecOps maturity, and policy compliance."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: security
---

# Azure WAF Security Review

The Azure Well-Architected Framework Security pillar provides guidance for protecting workload data, systems, and assets while maintaining confidentiality and integrity.

## 8 Security Design Principles

1. **Plan your security readiness** — understand security requirements, threats, and compliance obligations before building
2. **Design to protect confidentiality** — prevent exposure of sensitive data through encryption, access controls, and data classification
3. **Design to protect integrity** — detect and prevent unauthorized access to systems and data; use identity-based access
4. **Design to protect availability** — defend against attacks that deny access to workloads; use DDoS protection and redundancy
5. **Sustain and evolve your security posture** — continuously monitor, learn from incidents, and improve defenses
6. **Defense in depth** — layer multiple security controls so no single failure exposes the workload
7. **Zero trust** — verify explicitly, use least privilege access, assume breach
8. **Minimize the blast radius** — limit the scope of impact from security incidents

## Azure Security Service Areas

### Identity
- Azure Active Directory (Entra ID), Conditional Access, MFA, Privileged Identity Management (PIM), Managed Identity

### Network Security
- Azure Firewall Premium, Azure Front Door + WAF, DDoS Protection Standard, Network Security Groups (NSG), Azure Private Link, Virtual Network Service Endpoints

### Data Protection
- Azure Key Vault (keys, secrets, certificates), Azure Disk Encryption, Storage Service Encryption, Always Encrypted (SQL), Microsoft Purview (data governance, sensitivity labels)

### Threat Detection
- Microsoft Defender for Cloud, Microsoft Sentinel (SIEM/SOAR), Azure Monitor, Defender for Servers/Containers/SQL/Storage/App Service

### Governance
- Azure Policy, Management Groups, Blueprints (deprecated → use Deployment Stacks), Microsoft Cloud Security Benchmark (MCSB)

### DevSecOps
- GitHub Advanced Security (CodeQL SAST, secret scanning), Microsoft Defender for DevOps, Container image scanning in ACR, Infrastructure scanning (PSRule for Azure, Checkov)

## Assessment Questions

- How do you manage identity and access to your workload resources?
- How do you protect your network boundaries?
- How do you classify and protect your data?
- How do you detect and respond to security threats?
- How do you ensure your workload and supply chain are free of vulnerabilities?
- How do you enforce and validate compliance with security policies?
- How do you segment access and contain blast radius?

## Validation Checklist

- [ ] All human access uses Entra ID — no local accounts or shared credentials on Azure resources
- [ ] Managed Identity used for all Azure service-to-service access — no client secrets or certificates in code
- [ ] MFA enforced via Conditional Access for all users with Azure RBAC assignments
- [ ] PIM used for privileged roles (Owner, Contributor, User Access Administrator) with approval workflow and just-in-time access
- [ ] Azure Policy assignments block public storage accounts, unapproved VM SKUs, non-compliant regions
- [ ] Microsoft Defender for Cloud Standard tier enabled across all subscriptions; security score tracked
- [ ] All storage accounts and Key Vaults use Private Endpoints — no public network access
- [ ] Key Vault soft-delete and purge protection enabled for all key vaults
- [ ] Azure Monitor diagnostic settings enabled for all critical resources; logs sent to Log Analytics
- [ ] Microsoft Sentinel connected to Log Analytics workspace with MCSB analytics rules enabled
- [ ] NSG Flow Logs enabled for all production VNets
- [ ] Container images scanned in ACR; Defender for Containers enabled for AKS

## Response Shape

Identity and access posture → network security assessment → data protection → threat detection coverage → DevSecOps maturity → policy compliance → prioritized recommendations → open risks

## Official Documentation

- https://learn.microsoft.com/azure/well-architected/security/
- https://learn.microsoft.com/security/benchmark/azure/

## Security Notes

Read-only advisory. Do not modify Entra ID policies, Conditional Access rules, Azure Policy, or Defender configurations without explicit approval.
