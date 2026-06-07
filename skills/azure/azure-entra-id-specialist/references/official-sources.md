# Official sources for Azure Entra ID Specialist

Use Microsoft Learn documentation through the user's configured documentation MCP before identity guidance. Documentation proves Microsoft-published behavior; it does not prove the user's tenant posture, licenses, policy state, exclusions, sign-in risk, or break-glass readiness.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Security defaults in Microsoft Entra ID](https://learn.microsoft.com/en-us/entra/fundamentals/security-defaults) | Use for MFA registration, administrator MFA, legacy auth blocking, device code flow blocking, and Conditional Access migration caveats. |
| [Build a Conditional Access policy](https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-conditional-access-policies) | Use for assignments, target resources, network, conditions, grant/session controls, and token-evaluation caveats. |
| [Common Conditional Access policies](https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-conditional-access-policy-common) | Use for secure-foundation templates and baseline policy strategy. |
| [Best practices for Microsoft Entra roles](https://learn.microsoft.com/en-us/entra/identity/role-based-access-control/best-practices) | Ground least privilege, PIM, admin MFA, and privileged-role hygiene. |
| [Privileged Identity Management](https://learn.microsoft.com/en-us/entra/id-governance/privileged-identity-management/pim-configure) | Use for eligible roles, just-in-time activation, approval, alerts, and role settings. |
| [Best practices for securely deploying Microsoft Entra ID Governance](https://learn.microsoft.com/en-us/entra/id-governance/best-practices-secure-id-governance) | Use for least privilege, backup/recovery, monitoring, access reviews, and governance operations. |
| [Workload identities overview](https://learn.microsoft.com/en-us/entra/workload-id/workload-identities-overview) | Use for app/service principal/workload identity review. |
| [Emergency access accounts](https://learn.microsoft.com/en-us/entra/identity/role-based-access-control/security-emergency-access) | Use for break-glass account design and monitoring. |

## Source-grounding rules

- Do not disable security defaults unless replacement Conditional Access coverage is ready.
- Do not recommend broad exclusions; require explicit named rationale and compensating controls.
- Do not approve privileged-role changes without PIM, MFA, alerting, and emergency access review.
- Treat tenant evidence as sensitive; request only sanitized summaries.

## Current Microsoft Learn deltas checked on 2026-06-05

- Conditional Access planning depends on Microsoft Entra licensing; risk-based policies require stronger entitlement evidence than baseline policy design.
- Emergency access accounts and service principals/service accounts require explicit exclusion or separate control design; do not assume user-scoped Conditional Access safely governs every identity type.
- Conditional Access for workload identities is scoped to single-tenant service principals in the tenant; do not claim managed identities or multi-tenant SaaS apps are covered without current documentation evidence.
- Microsoft Entra roles and Azure RBAC roles are separate control planes; never use one evidence set to prove the other.

