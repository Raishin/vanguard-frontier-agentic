# Rollback Playbook: OCI Live Cost Budget Runaway Guard

## Revert a budget threshold change

```bash
# Re-apply original budget amount
oci budgets budget update \
  --budget-id <BUDGET_OCID> \
  --amount <ORIGINAL_AMOUNT>
```

## Remove a runaway alert rule

```bash
# List current alert rules
oci budgets alert-rule list --budget-id <BUDGET_OCID> \
  --query "data[].{id:id,displayName:\"display-name\",threshold:threshold}"

# Delete a specific alert rule
oci budgets alert-rule delete \
  --budget-id <BUDGET_OCID> \
  --alert-rule-id <RULE_OCID> \
  --force
```

## Restore previous alert rule configuration

```bash
oci budgets alert-rule create \
  --budget-id <BUDGET_OCID> \
  --display-name <NAME> \
  --type <ACTUAL|FORECAST> \
  --threshold <VALUE> \
  --threshold-type <PERCENTAGE|ABSOLUTE> \
  --recipients <EMAIL> \
  --message "Budget threshold reached"
```

## Rollback limitations

- Spend that already occurred before the budget alert triggered cannot be reversed.
- Deleting a budget does NOT stop any running compute instances — it only removes the alert.
- Compartment quota reductions (setting `gpu-vm-count to 0`) take effect immediately but do not terminate existing instances.
- OCI does not auto-stop resources when budget limits are hit — only notifications are sent.
