# Azure WAF Security Review operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

The bad review pattern is to say Security pillar and then inventory tools. Defender, Sentinel, private endpoints, and scans are not proof of a secure workload unless they map to threat model, identity, data, network, deployment, detection, response, and recovery evidence.

## Officially grounded service shape

Microsoft Learn grounds the Azure Well-Architected Security pillar in Zero Trust and the CIA triad: verify explicitly, use least privilege, assume breach, protect confidentiality, integrity, and availability, and continuously improve posture. The checklist adds concrete review areas for baseline, secure development, data classification, segmentation, IAM, networking, encryption, and hardening. That is the key insight: Azure WAF Security is an evidence-backed workload risk review, not a tool coverage checklist.

## Non-negotiable design rules

### 1. Start from workload threat model, data classification, and business impact, not from a product inventory.
### 2. Verify Zero Trust explicitly: identity, permissions, duration, asset scope, device or workload context, and expected location.
### 3. Treat standing privileged access, shared credentials, local accounts, and service principals with secrets as high-risk until justified.
### 4. Review segmentation across networks, roles, workload identities, and resource organization; one boundary does not replace the others.
### 5. Require detection, response, and recovery evidence; prevention-only security is incomplete.
### 6. Do not recommend policy, Conditional Access, Defender, Sentinel, network, or production changes without approval and rollback.

## Minimal safe implementation flow

1. Classify workload, data sensitivity, threat model, compliance drivers, and production impact.
2. Ground Well-Architected Security behavior in Microsoft Learn and label it documentation-based.
3. Review sanitized evidence for IAM, segmentation, data protection, network controls, hardening, Defender, Sentinel, DevSecOps, and policy compliance.
4. Separate proven controls, unverified claims, compensating controls, exceptions, and risky remediations.
5. Return verdict, evidence level, blockers, safe next actions, and open questions.

## High-risk assumptions to kill

- Defender for Cloud coverage proves the workload is secure.
- A high secure score replaces threat modeling.
- Private endpoints compensate for weak identity or missing data classification.
- Sentinel enabled with default rules means detection is mature.
- SAST or secret scanning alone satisfies DevSecOps maturity.
- Documentation proves the user's current security posture.

## Safe command/code verification targets

- Threat model, security baseline, data classification, regulatory mapping, and workload owner evidence.
- RBAC/PIM/Conditional Access, managed identities, credential elimination, and access review evidence.
- Network segmentation, NSGs, firewall/WAF policies, private endpoints, DNS, ingress and egress paths.
- Encryption, Key Vault posture, key ownership, purge protection, backup/recovery, and audit logs.
- Defender recommendations, secure score controls, Sentinel analytics coverage, incident handling, and alert routing.
- Pipeline SAST, secret scanning, IaC scanning, image scanning, dependency scanning, and release gates.

## When to push back

- The user wants a security verdict from tool presence alone.
- Evidence omits scope, timestamp, owner, or sanitized source.
- A remediation weakens availability, recovery, or operations without risk acceptance.
- The request asks for secrets, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, or customer data.
