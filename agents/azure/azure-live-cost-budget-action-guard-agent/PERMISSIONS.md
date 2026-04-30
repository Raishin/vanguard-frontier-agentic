# Permissions: Azure Live Cost Budget Action Guard

# Least-privilege RBAC guidance for cost budget and GPU guard

## Custom role (budget read/write + quota read, NO VM creation)

```json
{
  "Name": "Cost Budget Action Guard",
  "IsCustom": true,
  "Description": "Read and modify subscription budgets and read compute quotas. Cannot create VMs. Cannot delete budgets.",
  "Actions": [
    "Microsoft.Consumption/budgets/read",
    "Microsoft.Consumption/budgets/write",
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
    "Microsoft.Quota/quotas/write",
    "Microsoft.Consumption/budgets/delete",
    "Microsoft.CostManagement/budgets/delete"
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

**Budget deletion is excluded** (`Microsoft.Consumption/budgets/delete`,
`Microsoft.CostManagement/budgets/delete`). Deleting budgets silently removes the
only cross-region financial guardrail and disables every threshold alert on the
subscription. Cleanup of test or stale budgets must go through a separate
PIM-eligible "Cost Budget Cleanup" role, never the standing operational role.

## Separate PIM role: Cost Budget Cleanup (eligible-only)

```json
{
  "Name": "Cost Budget Cleanup (PIM-eligible)",
  "IsCustom": true,
  "Description": "PIM-only role for deleting stale or test budgets. Eligible-only. Maximum 2-hour activation. MFA + justification required.",
  "Actions": [
    "Microsoft.Consumption/budgets/read",
    "Microsoft.Consumption/budgets/delete",
    "Microsoft.CostManagement/budgets/read",
    "Microsoft.CostManagement/budgets/delete"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>"
  ]
}
```

Configure as PIM-eligible only (never standing active), MFA-gated, time-bounded.

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

