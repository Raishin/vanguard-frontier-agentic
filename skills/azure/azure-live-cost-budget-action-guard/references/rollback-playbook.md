# Rollback Playbook: Azure Live Cost Budget Action Guard

## Revert a budget modification

```bash
# Inspect current state before revert
az consumption budget show -n <BUDGET_NAME>

# Delete and recreate with original values
az consumption budget delete -n <BUDGET_NAME>

az consumption budget create \
  -n <BUDGET_NAME> \
  --amount <ORIGINAL_AMOUNT> \
  --time-grain <Monthly|Quarterly|Annually> \
  --start-date <YYYY-MM-01> \
  --end-date <YYYY-MM-01> \
  --notification <KEY=VALUE pairs from original>
```

## Remove a runaway action group from a budget

```bash
# Show notification rules
az consumption budget show -n <BUDGET_NAME> --query "properties.notifications"

# Update budget to clear action groups on a specific notification key
az consumption budget create -n <BUDGET_NAME> \
  --amount <AMOUNT> \
  --time-grain Monthly \
  --start-date <DATE> \
  --end-date <DATE>
# Re-specify only the notification rules you want to keep
```

## Rollback limitations

- Spend that already occurred before the budget alert triggered cannot be reversed.
- Deleting a budget does NOT stop any VMs or resources — it only removes the alerting rule.
- Quota increases, once approved by Microsoft, cannot be reduced below the original limit.
