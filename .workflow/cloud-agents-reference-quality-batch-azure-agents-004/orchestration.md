# Azure agents batch 004 orchestration

## Batch
Provider: Azure
Asset type: agents
Stable sorted batch after batch 003:
1. azure-live-app-service-slot-swap-guard-agent
2. azure-live-arm-deployment-stack-guard-agent
3. azure-live-cost-budget-action-guard-agent
4. azure-live-entra-role-assignment-guard-agent
5. azure-live-keyvault-rotation-purge-guard-agent

## Evidence sources
Microsoft Learn documentation through the user configured documentation MCP was used for App Service slots, Deployment Stacks, Cost Management budgets and alerts, Quotas, Azure RBAC, Microsoft Entra PIM, and Key Vault recovery/rotation behavior.

## Safety constraints
No AWS assets. No commits. No credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data requested or embedded.
