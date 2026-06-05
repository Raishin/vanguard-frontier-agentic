# Rollback Playbook: Azure Live Cost Budget Action Guard

## Evidence-variable convention

Shell variables in examples are local operator placeholders from an approved change record or already configured shell context. Do not commit real values, and redact them from shared evidence unless disclosure is explicitly approved.

## Budget update rollback

```bash
# Inspect current state before revert
az consumption budget show -n <BUDGET_NAME>

# Re-apply the original budget values without deleting the budget when possible
az consumption budget create \
  -n <BUDGET_NAME> \
  --amount <ORIGINAL_AMOUNT> \
  --time-grain <Monthly|Quarterly|Annually> \
  --start-date <YYYY-MM-01> \
  --end-date <YYYY-MM-01> \
  --notification $ORIGINAL_NOTIFICATION_FIELDS_JSON
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


## Cost-data latency caveat

Microsoft Learn documents that cost and usage data is typically available within 8-24 hours and budget evaluation runs every 24 hours. A rollback or threshold reduction does not undo spend that already occurred and might not immediately reflect current consumption.
