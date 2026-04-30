# Cost Budget Action — Rollback Playbook

## Restore a lowered budget to its previous threshold

```bash
az consumption budget create \
  --budget-name <BUDGET_NAME> \
  --amount <PREVIOUS_AMOUNT> \
  --time-grain Monthly \
  --start-date <START_DATE> \
  --end-date <END_DATE>
```

## Emergency: deallocate a runaway GPU VM (requires VM operator rights — escalate if needed)

```bash
az vm deallocate \
  --resource-group <RG> \
  --name <VM_NAME> \
  --no-wait
# Verify deallocation
az vm show --resource-group <RG> --name <VM_NAME> --query "powerState" -d
```

## Scale VMSS to zero during a runaway cost event

```bash
az vmss scale \
  --resource-group <RG> \
  --name <VMSS_NAME> \
  --new-capacity 0
```

## Revert a quota increase (reduce back to previous limit)

```bash
az quota update \
  --resource-name "cores" \
  --scope "/subscriptions/<SUBSCRIPTION_ID>/providers/Microsoft.Compute/locations/<REGION>" \
  --limit-object value=<PREVIOUS_LIMIT> value-type=Individual
```

## Verify budget alert is re-active

```bash
az consumption budget show \
  --budget-name <BUDGET_NAME> \
  --query "{amount:amount, currentSpend:currentSpend.amount, notifications:notifications}"
```
