# Azure Security Posture Hardening Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling a workload hardened because it has a Key Vault.
- Using stored service-principal secrets when managed identities are supported.
- Keeping Key Vault public access or legacy access policies without justification.
- Treating Defender recommendations as optional noise instead of risk signals to triage.
- Skipping diagnostics and audit logs until after an incident.

## Officially grounded service shape

- Microsoft Learn Key Vault guidance implements Zero Trust principles: verify explicitly, use least privilege, and assume breach.
- Key Vault hardening includes one vault per application/region/environment where appropriate, private access or firewall controls, Azure RBAC over legacy access policies for critical workloads, PIM for privileged operations, soft delete, purge protection, rotation, logging, Defender, policy enforcement, and backup/recovery testing.
- Security baselines call out managed identities, Azure Policy audit/deny/deploy-if-not-exists effects, Defender for Cloud monitoring, conditional access where supported, and secure storage of credentials in Key Vault.
- Documentation-based security recommendations do not prove the user has enabled these controls. Current-state posture needs sampled read-only evidence.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Prefer managed identities over stored secrets where the service supports them.
- Use Azure RBAC, least privilege, and PIM/time-bound elevation for privileged operations.
- Disable public exposure or restrict network access where workload requirements allow it.
- Require diagnostic logs, alerts, and policy compliance evidence before calling posture audit-ready.
- Stage remediation and avoid broad production changes without rollback and owner approval.

## Minimal safe implementation flow

- Scope workload, data sensitivity, identities, secrets, public exposure, policies, logging, and production impact.
- Collect documentation requirements and sampled posture evidence when available.
- Classify gaps by identity, network, secret lifecycle, policy, monitoring, and backup/recovery risk.
- Prioritize reversible least-privilege remediations before disruptive controls.
- Return hardened target state, blockers, safe rollout sequence, verification checks, and residual risk.

## Safe verification targets

- Managed identities or approved credential model are used for service access.
- Key Vault RBAC, network controls, soft delete, purge protection, rotation, and diagnostics match sensitivity.
- Policy assignments or Defender recommendations cover required controls and exemptions are justified.
- Audit logs and alerts reach an owned destination.
- Remediation has rollback or staged deployment plan.

## When to push back

- The user wants public access because private networking is inconvenient.
- The plan stores credentials in repo, pipeline files, or app settings without a secure reference.
- The request skips logging or Defender/Policy evidence.
- The remediation would break production without staged validation.
