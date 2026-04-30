# Permission Model: Azure Live Cost Budget Action Guard

## Custom role — budget read/write, quota read, no VM creation

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

`Microsoft.Quota/quotas/write` is excluded: quota increase requests carry spending risk
and must go through a separate approval workflow, not this role. VM creation is
explicitly excluded to prevent the cost guard from becoming a provisioning path.

`Microsoft.Consumption/budgets/delete` and `Microsoft.CostManagement/budgets/delete`
are excluded: deleting a budget silently removes the only cross-region financial
guardrail and disables every threshold alert on the subscription. Cleanup of stale or
test budgets must go through a separate PIM-eligible role with MFA + justification gates.

## Azure Policy guardrail (deploy alongside the role)

Deny GPU VM SKU provisioning without an approved budget tag:

```json
{
  "if": {
    "allOf": [
      {"field": "type", "equals": "Microsoft.Compute/virtualMachines"},
      {"field": "Microsoft.Compute/virtualMachines/sku.name", "in": [
        "Standard_ND96asr_v4", "Standard_NC24rs_v3", "Standard_ND40rs_v2"
      ]},
      {"field": "tags.BudgetApproval", "exists": "false"}
    ]
  },
  "then": {"effect": "Deny"}
}
```

## Do not assign

- `Cost Management Contributor` at management-group scope
- `Billing Account Contributor`
- `Microsoft.Compute/virtualMachines/write` to this role
