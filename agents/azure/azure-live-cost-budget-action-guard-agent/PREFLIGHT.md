# Cost Budget Action — Preflight Commands

## 1. List all budgets and current spend

```bash
az consumption budget list \
  --query "[].{name:name, amount:amount, currentSpend:currentSpend.amount, forecastSpend:forecastSpend.amount, timeGrain:timeGrain}" \
  --output table
```

## 2. Check GPU/HPC quota usage in target region

```bash
az vm list-usage \
  --location <REGION> \
  --query "[?contains(name.value, 'GPU') || contains(name.value, 'NC') || contains(name.value, 'ND') || contains(name.value, 'NV')].{name:name.localizedValue, used:currentValue, limit:limit}" \
  --output table
```

## 3. Inventory running GPU/HPC VMs across subscription

```bash
az vm list \
  --query "[?contains(storageProfile.imageReference.sku, 'gpu') || starts_with(hardwareProfile.vmSize, 'Standard_NC') || starts_with(hardwareProfile.vmSize, 'Standard_ND')].{name:name, size:hardwareProfile.vmSize, rg:resourceGroup, state:powerState}" \
  --show-details \
  --output table
```

## 4. Show active budget alert thresholds

```bash
az consumption budget show \
  --budget-name <BUDGET_NAME> \
  --query "{notifications:notifications, amount:amount, filter:filter, startDate:timePeriod.startDate}"
```

## 5. Check quota request history

```bash
az quota request status list \
  --scope "/subscriptions/<SUBSCRIPTION_ID>/providers/Microsoft.Compute/locations/<REGION>" \
  --query "[].{name:name, status:properties.provisioningState, value:properties.value.limit}" \
  --output table
```
