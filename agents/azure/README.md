# 🟦 Azure Agents

<p align="center">
  <img src="../../assets/logos/cloud/azure/azure.png" alt="Azure logo" width="140" />
</p>

Azure agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Azure mutation |
|---|---|---|---|
| Role / advisory agents | Review, design, diagnose, coordinate | read-only | not allowed by default |
| Guarded live operators | Work in repos or shells that may target real Azure environments | workspace-write | approval-gated and target-confirmed only |

## 🚦 Guarded live-Azure operators

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `azure-live-aks-rollout-guard-agent` | live AKS rollout actions | PDB audit + health evidence + rollback required | rollout safety signals are weak or contradictory |
| `azure-live-arm-deployment-stack-guard-agent` | live ARM/Bicep deployment stacks | what-if evidence + denySettings + PIM-gated delete | deploying without what-if preview |
| `azure-live-app-service-slot-swap-guard-agent` | live App Service slot swaps | sticky-setting audit + traffic shift + swap-back path | slot health or sticky settings are ambiguous |
| `azure-live-keyvault-rotation-purge-guard-agent` | live Key Vault key rotation and purge | rotation policy + soft-delete + purge-protection check | purge-protection is disabled or key is in active use |
| `azure-live-pim-jit-activation-guard-agent` | live PIM JIT role activations | eligible assignment audit + MFA gate + JIT scope | activation scope or justification is missing |
| `azure-live-cost-budget-action-guard-agent` | live budget and action group mutations | budget baseline + alert threshold + quota read-only | budget action would disable cost controls |
| `azure-live-entra-role-assignment-guard-agent` | live permanent Entra ID and Azure RBAC role assignments | scope + principal-type + dangerous-role audit + PIM-preference | Owner/Contributor/UAA at subscription scope without CISO sign-off |

## 👀 Read-only advisory examples

| Agent | Focus |
|---|---|
| `azure-rbac-review-agent` | RBAC assignment scope, custom roles, dangerous permissions |
| `azure-identity-governance-review-agent` | access reviews, lifecycle workflows, entitlement management |
| `azure-security-posture-hardening-agent` | Defender for Cloud posture, secure score, misconfiguration |
| `azure-landing-zone-architect-agent` | enterprise-scale landing zone design and review |
| `azure-network-topology-review-agent` | hub-spoke topology, peering, Private Endpoints, NSGs |
| `azure-observability-investigator-agent` | Azure Monitor, Log Analytics, App Insights investigation |
| `azure-cost-optimization-governor-agent` | Azure Cost Management, savings plans, reservation coverage |

## 🛡️ Operating note

- 😄 advisory agents stay read-only by default
- 🚦 guarded live operators must confirm subscription, resource group, principal, approval, rollback, and verification before mutation
- 🔐 never treat a vague "deploy to prod" intent as permission
- 🧾 all live-guard agents produce a structured verdict response — see [`docs/evidence-output-spec.md`](../../docs/evidence-output-spec.md)
