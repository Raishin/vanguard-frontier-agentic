# Azure Security Posture Hardening operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

A security score is not a security posture. It is a prioritized signal that still needs scope, owner, exception, and remediation evidence.

## Officially grounded service shape

Microsoft Defender for Cloud and the Microsoft Cloud Security Benchmark provide posture recommendations, policy initiatives, secure score, inventory, and regulatory/compliance views. Key Vault and private-link controls show why identity, network, and data protection cannot be reviewed in isolation. That is the key insight: posture hardening is evidence triage plus safe remediation, not a checklist dump.

## Non-negotiable design rules

### 1. Separate recommendation evidence from actual deployed-state proof.
### 2. Map findings to owner, scope, severity, compensating control, and safe remediation path.
### 3. Treat broad standing privilege, public management exposure, weak Key Vault access, and missing logging as high-risk.
### 4. Prefer policy-driven guardrails over manual one-off fixes.
### 5. Do not recommend remediation that can break production without rollback and approval.

## Minimal safe implementation flow

1. Classify posture domain: identity, network, data, compute, policy, logging, or compliance.
2. Ground documented Azure behavior in Microsoft Learn.
3. Review sanitized secure-score, policy, Defender, inventory, and configuration evidence if available.
4. Separate quick wins, risky remediations, exceptions, and unknowns.
5. Return verdict with blockers, safe next actions, and evidence gaps.

## High-risk assumptions to kill

- A high secure score proves the environment is secure.
- A policy assignment proves every resource is compliant.
- Public access is acceptable because authentication exists.
- Key Vault is safe because it stores secrets.

## Safe command/code verification targets

- Defender for Cloud recommendation status, secure-score controls, policy compliance, exemptions, and owners.
- RBAC/PIM, managed identities, Key Vault RBAC, purge protection, network access, and logging settings.
- Remediation impact, rollback, exception expiry, and post-change validation.

## When to push back

- The request asks to auto-fix security findings without blast-radius review.
- Evidence is only a screenshot with no scope or timestamp.
- The remediation weakens availability or operations without risk acceptance.
- The user asks for secrets, keys, or raw identity exports.
