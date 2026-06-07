# Safety checklist for Azure Governance Policy Guardrails

## Non-negotiable gates

- Never ask for tenant identifiers, subscription identifiers, customer data, raw resource inventories, or policy exports containing sensitive names without sanitization.
- Do not recommend broad-scope deny, modify, deployIfNotExists, or remediation without canary scope, exemption plan, owner, and rollback.
- Do not assign remediation identities broad permissions without least-privilege review.
- Do not use Azure Policy to deploy full workloads; use it for governance and compliance controls.
- Require explicit approval before assignment, enforcement-mode change, remediation task, exemption change, initiative update, or deny effect rollout.

## High-risk assumptions to kill

- "Audit passed, so enforcement is safe." Enforcement can still break deployment pipelines.
- "Deny is cleaner than audit." Deny can block urgent fixes and existing automation.
- "Remediation is automatic for everything." Existing resources need tasks; identity permissions matter.
- "Exemptions are harmless." They need reason, expiration, owner, and review.
- "Management group scope is always best." Inherited deny can have wide blast radius and explicit-deny behavior.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `policy_review`: definition, initiative, assignment, or exemption reviewed statically.
- `compliance_sample`: sanitized compliance state or policy insights were sampled.
- `canary_proven`: staged scope tested without unexpected impact.
- `mutation_ready`: approval, scope, rollback, and identity permissions are documented.

## Minimum safe evidence

- Target scope, inheritance path, notScopes, exemptions, and affected resource types.
- Policy effect, mode, parameters, initiative membership, and assignment enforcement mode.
- Compliance sample, noncompliance causes, and deployment pipeline impact review.
- Managed identity permissions for DINE/Modify and remediation task plan.
- Canary scope, rollback plan, exception process, and owner.
