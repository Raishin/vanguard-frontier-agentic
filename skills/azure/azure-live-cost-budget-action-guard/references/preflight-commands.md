# Preflight Commands: Azure Live Cost Budget Action Guard

Run these before any budget modification. Paste sanitized output as evidence.

## 1. Confirm identity and subscription

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
```

## 2. List current budgets

```bash
az consumption budget list --query \
  "[].{name:name, amount:properties.amount, timeGrain:properties.timeGrain, currentSpend:properties.currentSpend.amount}"
```

## 3. Inspect a specific budget detail

```bash
az consumption budget show -n <BUDGET_NAME> \
  --query "{amount:properties.amount, filter:properties.filter, notifications:properties.notifications}"
```

## 4. Check current spend vs. budget

```bash
az costmanagement query \
  --type ActualCost \
  --dataset-aggregation '{"totalCost":{"name":"PreTaxCost","function":"Sum"}}' \
  --timeframe MonthToDate \
  --scope "/subscriptions/<SUBSCRIPTION_ID>"
```

## 5. Check compute quota usage before action

```bash
az vm list-usage -l <LOCATION> \
  --query "[?contains(name.value,'cores') || contains(name.value,'GPU')].{name:name.localizedValue, current:currentValue, limit:limit}"
```

## 6. Verify budget action groups are configured

```bash
az consumption budget show -n <BUDGET_NAME> \
  --query "properties.notifications"
# All notification.actionGroups should point to valid Action Group resource IDs
```
