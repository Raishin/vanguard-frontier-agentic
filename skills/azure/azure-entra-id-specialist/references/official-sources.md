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
