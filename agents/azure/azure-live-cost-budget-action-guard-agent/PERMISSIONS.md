# Permissions: Azure Live Cost Budget Action Guard

# Least-privilege RBAC guidance for cost budget and GPU guard

## Custom role (budget read/write + quota read, NO VM creation)

```json
{
  "Name": "Cost Budget Action Guard",
  "IsCustom": true,
  "Description": "Read and modify subscription budgets and read compute quotas. Cannot create VMs.",
  "Actions": [
    "Microsoft.Consumption/budgets/read",
    "Microsoft.Consumption/budgets/write",
    "Microsoft.Consumption/budgets/delete",
    "Microsoft.CostManagement/budgets/read",
    "Microsoft.CostManagement/budgets/write",
    "Microsoft.CostManagement/query/action",
    "Microsoft.Compute/locations/usages/read",
    "Microsoft.Compute/locations/vmSizes/read",
    "Microsoft.Quota/quotas/read",
    "Microsoft.Quota/usages/read"
  ],
  "NotActions": [
    "Microsoft.Compute/virtualMachines/write",
    "Microsoft.Compute/virtualMachineScaleSets/write",
    "Microsoft.Quota/quotas/write"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>"
  ]
}
```

VM creation is explicitly excluded. `Microsoft.Quota/quotas/write` is also excluded:
quota increase requests carry spending risk and must go through a separate approval
workflow (e.g., Azure Support or an IT-ops request process), not through this role.
GPU SKU approval flows through budget-action alerts only — not through quota write.

## Azure Policy guardrail (deploy alongside the custom role)

Deny GPU VM SKU provisioning without an approved budget tag:

```json
{
  "if": {
    "allOf": [
      {"field": "type", "equals": "Microsoft.Compute/virtualMachines"},
      {"field": "Microsoft.Compute/virtualMachines/sku.name", "in": [
        "Standard_ND96asr_v4", "Standard_NC24rs_v3", "Standard_ND40rs_v2",
        "Standard_HB120rs_v3", "Standard_HB176rs_v4"
      ]},
      {"field": "tags.BudgetApproval", "exists": "false"}
    ]
  },
  "then": {"effect": "Deny"}
}
```

## Do not assign

- `Cost Management Contributor` at management-group scope (modifies all child subscriptions)
- `Billing Account Contributor`
- `Microsoft.Compute/virtualMachines/write` to this role

