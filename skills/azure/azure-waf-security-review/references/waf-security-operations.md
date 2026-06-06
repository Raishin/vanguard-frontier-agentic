# Azure WAF Security Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, feature availability, and recommendation semantics against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating a generic security checklist as proof of workload security.
- Calling secure score, Defender coverage, or policy compliance production readiness without owner-reviewed evidence.
- Equating encryption-at-rest defaults with complete data protection.
- Assuming private endpoints or firewalls fix weak identity, secret handling, or egress paths.
- Skipping threat modeling, secure-development controls, incident-response testing, and recovery security.

## Officially grounded service shape

- Microsoft Learn evidence says Well-Architected security starts from Zero Trust, the CIA triad, and recurring security improvement rather than one-time hardening.
- The Security checklist covers baseline/compliance, secure development lifecycle and threat modeling, data classification, segmentation, strict IAM, network isolation, encryption, resource hardening, application secrets, threat monitoring, security testing, and incident response.
- Microsoft security guidance says to verify explicitly, use least privilege for the right duration and assets, and assume breach with compensating controls that limit blast radius.
- Defender for Cloud can assess resources against security standards such as Microsoft Cloud Security Benchmark, but documentation-based recommendations do not prove the user's configured posture.
- MCSB and Defender compliance evidence can support prioritization, but preview controls, manual/shared responsibilities, exemptions, and paid-plan requirements must be labeled clearly.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Define the workload boundary, data classification, critical flows, compliance drivers, owners, and production impact before judging security.
- Require identity-first access, least privilege, time-bound privileged access, and workload identities before approving sensitive operations.
- Require intentional segmentation across identity, network, resource organization, and data paths; do not treat a single perimeter as sufficient.
- Require evidence for data protection, secret lifecycle, logging, threat monitoring, vulnerability management, and incident response.
- Treat security recommendations as staged risk reduction, not as permission to mutate policies, networking, Defender plans, or identity controls without approval.

## Minimal safe implementation flow

- Scope workload, environments, data sensitivity, critical flows, compliance obligations, owners, and current evidence sources.
- Collect Microsoft Learn documentation requirements and sampled read-only evidence when available for identity, network, data, policy, Defender, logging, secrets, and DevSecOps controls.
- Classify gaps against Security checklist areas: baseline, SDL/threat model, classification, segmentation, IAM, networking, encryption, hardening, secrets, monitoring, testing, and incident response.
- Prioritize fixes by exploitability, business impact, blast radius, reversibility, and owner readiness.
- Return verdict, evidence level, blockers, staged remediation, validation checks, and residual risk.

## High-risk assumptions to kill

- “We use Azure, so default platform security is enough.”
- “Secure score is high, so the workload is secure.”
- “Private networking means attackers cannot reach the workload.”
- “Encryption is enabled, so data protection is complete.”
- “PIM exists, so privileged access is solved.”
- “Defender/Sentinel is connected, so detection and response are ready.”
- “No incidents means no security gaps.”

Those are lazy assumptions.

## Safe command/code verification targets

- Inventory role assignments, privileged access paths, workload identities, Conditional Access dependencies, and secret usage with read-only queries.
- Inspect segmentation evidence across resource organization, network boundaries, ingress, egress, private access, and data flows.
- Review data classification, encryption configuration, key ownership, Key Vault posture, secret rotation, and audit logs without exposing secret values.
- Query Defender recommendations, secure score controls, policy compliance, exemptions, diagnostic settings, alert rules, and log destinations as sampled current-state evidence.
- Check secure-development controls such as threat models, dependency scanning, IaC scanning, image scanning, release gates, and vulnerability triage evidence.
- Verify incident-response runbooks, alert ownership, escalation paths, tabletop/test evidence, and post-incident feedback loops before claiming readiness.

## Safe verification targets

- Security baseline maps to compliance requirements, platform recommendations, and workload-specific threats.
- IAM is strict, conditional, auditable, least-privilege, and time-bound for privileged operations.
- Data is classified and protected with encryption, key ownership, access controls, and audit trails matching sensitivity.
- Network segmentation controls north-south and east-west traffic and has explicit egress handling.
- Threat monitoring, alert routing, vulnerability management, testing, and incident response are owned and exercised.

## When to push back

- The user wants production-ready approval without workload boundary, data classification, or critical-flow context.
- Broad roles, public exposure, static secrets, or unowned alerts are normalized as temporary shortcuts.
- Defender, Sentinel, or policy is cited without current-state evidence and owner follow-up.
- The recommendation would change identity, network, policy, or Defender settings without explicit approval, blast-radius review, and rollback plan.
