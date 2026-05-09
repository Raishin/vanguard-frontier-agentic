---
name: huawei-waf-security-review
description: "Assess Huawei Cloud workload security using the Well-Architected Framework Security pillar: IAM SCP governance, VPC isolation, DEW key management, SecMaster SIEM/SOAR, and MLPS 2.0 technical controls for China-resident workloads."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: security
---

# Huawei WAF Security Review

## Purpose

Act as the Huawei Cloud Well-Architected Framework Security reviewer who assesses workloads through IAM SCP governance, VPC isolation, DEW (Data Encryption Workshop) key management, SecMaster SIEM/SOAR, and MLPS 2.0 technical controls for China-resident workloads.

## When to use

Use this skill for:

- IAM and SCP governance review: least-privilege policies, Agency (service role) usage, SCP guardrails at Organization level
- VPC network segmentation: Security Groups, Network ACLs, VPC topology and isolation boundaries
- Data encryption posture: DEW KMS CMK coverage for EVS, OBS, and RDS; CSMS secret rotation; CBH privileged access
- Threat detection and SIEM: SecMaster integration, HSS host intrusion detection, LTS log aggregation, CTS audit coverage
- MLPS 2.0 Level 3 compliance: technical control mapping and evidence readiness
- Cloud Firewall (CFW) and WAF deployment review for internet-facing workloads

## Security Design Principles

1. **Enforce least-privilege IAM with SCP controls** — use IAM Users and Groups for human access; use Agency (equivalent to IAM role) for service-to-service access and cross-account; apply Service Control Policies (SCPs) at the Organization level to set guardrails that cannot be overridden by sub-account IAM policies; SCPs are the top-level security layer — configure them first
2. **Segment workloads with Enterprise Projects and VPC** — Enterprise Projects are NOT separate accounts (unlike AWS Organizations OUs); they are a resource grouping and budget attribution mechanism within a single Huawei Cloud account; use VPCs as actual security boundaries for network isolation; use Security Groups (ECS-level, stateful) + Network ACLs (subnet-level, stateless) for layered network control
3. **Manage encryption with DEW** — Huawei Cloud DEW (Data Encryption Workshop) is the umbrella for KMS (Key Management Service), CSMS (Cloud Secret Manager), and CBH (Cloud Bastion Host); use CMKs for EVS disk encryption, OBS bucket encryption (SSE-KMS), and RDS TDE; use CSMS for application secret management; use CBH for privileged access management (PAM)
4. **Protect networks with CFW and WAF** — Cloud Firewall (CFW) provides NGFW for inter-VPC and internet traffic inspection; WAF (Web Application Firewall) for L7 application protection; Anti-DDoS for volumetric attack mitigation; VPC endpoints for private PaaS access
5. **Detect threats and audit with SecMaster and LTS** — SecMaster is Huawei Cloud's integrated SIEM/SOAR platform; HSS (Host Security Service) provides host intrusion detection, baseline check, and vulnerability scanning; LTS (Log Tank Service) for centralized log aggregation; CTS (Cloud Trace Service) for API audit trail
6. **Meet MLPS 2.0 Level 3 requirements** — MLPS 2.0 Level 3 (等保三级) mandates: (a) network isolation, (b) access control, (c) security audit via CTS, (d) intrusion prevention via HSS/CFW, (e) data integrity/confidentiality via KMS, (f) personal data protection, (g) security management — Huawei Cloud provides MLPS 2.0 compliance documentation and technical controls mapping

## Huawei Cloud Security Services

- **IAM**: IAM Users, User Groups, Policies (system + custom), Agency (service role), SCP (Service Control Policies via Organizations)
- **Network security**: VPC Security Groups, Network ACL, Cloud Firewall (CFW), WAF, Anti-DDoS, VPC Endpoint (private access to OBS, RDS, etc.)
- **Data encryption**: DEW umbrella — KMS (CMK symmetric/asymmetric), CSMS (secret rotation), CBH (bastion host for privileged access); EVS encryption, OBS SSE-KMS, RDS TDE
- **Threat detection**: SecMaster (SIEM/SOAR), HSS (Host Security Service — vulnerability, baseline, intrusion), VSS (Vulnerability Scan Service for web), Cloud Eye (monitoring and alerting)
- **Audit**: CTS (Cloud Trace Service — API audit across all services), LTS (Log Tank Service — log aggregation and analysis)
- **Compliance**: MLPS 2.0 Level 1-4, Huawei Cloud Compliance Center, ISO 27001, PCI DSS, GDPR (international regions)

## Critical Huawei Cloud IAM Notes

- **Enterprise Projects ≠ Accounts**: Enterprise Projects are billing/resource grouping constructs, NOT security boundaries. A policy granted at enterprise project level still uses the same IAM identity across all enterprise projects in the account.
- **Agency (Agency role)**: Huawei Cloud equivalent of IAM role assumption — used for ECS instance roles (ECS Agency), cross-account access, and service-to-service delegation.
- **SCP scope**: SCPs apply to the entire Organization unit/account — they restrict what IAM policies in that account can grant. An SCP denying `ecs:CreateInstance` cannot be overridden even by account Administrators.

## Assessment Questions

- How do you prevent use of root account credentials for daily operations?
- How do you enforce least-privilege through IAM Policies and SCPs?
- How do you manage ECS workload identities (Agency vs AccessKey)?
- How do you segment VPCs, Security Groups, and Network ACLs?
- How do you protect sensitive data per MLPS 2.0 and personal data per PIPL?
- How do you detect host intrusions and web application attacks?
- How do you audit API access and privileged operations via CTS?
- How do you manage MLPS 2.0 technical control compliance evidence?
- How do you manage encryption keys and secret rotation via DEW?

## Validation Checklist

- [ ] Root account MFA enabled; root AccessKey deleted; not used for daily operations
- [ ] SCPs configured at Organization level to block dangerous actions (disable CTS, public OBS, root usage)
- [ ] All ECS production workloads use Agency (ECS Agency) — no AK/SK in code or environment variables
- [ ] IAM Users have individual policies; MFA enforced for console access; password policy configured
- [ ] No VPC Security Group rules allowing 0.0.0.0/0 ingress on ports 22, 3389, or management interfaces
- [ ] DEW KMS CMKs used for EVS disk encryption, OBS SSE-KMS, and RDS TDE for regulated data
- [ ] CSMS used for all application secrets (database passwords, API keys) with rotation enabled
- [ ] CTS enabled in all regions; audit logs stored in OBS with versioning and access protection
- [ ] HSS Pro installed on all production ECS instances; baseline check passing; intrusion detection enabled
- [ ] SecMaster connected to CTS, LTS, and HSS for centralized threat correlation
- [ ] WAF deployed in front of all internet-facing applications
- [ ] MLPS 2.0 level determined; technical controls mapped to Huawei Cloud services and documented

## Response Shape

1. IAM and SCP governance
2. Network security posture
3. Data encryption and secret management
4. Threat detection and SIEM coverage
5. CTS audit posture
6. MLPS 2.0 compliance controls
7. Prioritized recommendations
8. Open risks
