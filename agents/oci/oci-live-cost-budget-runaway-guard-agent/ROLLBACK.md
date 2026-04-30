# Cost Budget Runaway — Rollback Playbook

## Restore a raised budget threshold to previous value

```bash
oci budgets budget update \
  --budget-id <BUDGET_OCID> \
  --amount <PREVIOUS_AMOUNT>

# Verify
oci budgets budget get \
  --budget-id <BUDGET_OCID> \
  --query 'data.{amount:amount, reset:"reset-period", spent:"actual-spend"}'
```

## Emergency: stop a runaway GPU instance (requires Compute operator — escalate if needed)

```bash
# Soft stop (OCPU billing continues for stopped-but-preserved VMs until termination)
oci compute instance action \
  --instance-id <INSTANCE_OCID> \
  --action STOP

# For bare metal GPU (BM.GPU4.8) — billing stops only on TERMINATE
# Escalate to Compute operator with appropriate compartment manage rights
```

## Lower a compartment GPU quota to prevent further provisioning

```bash
oci limits quota create \
  --compartment-id <COMPARTMENT_OCID> \
  --name "emergency-gpu-cap-$(date +%Y%m%d)" \
  --statements '["set compute-core-count quota gpu-count to 0 in compartment <COMPARTMENT>"]'
```

## Revert a budget alert threshold change

```bash
oci budgets alert update \
  --budget-id <BUDGET_OCID> \
  --alert-id <ALERT_OCID> \
  --threshold <PREVIOUS_THRESHOLD> \
  --threshold-type ABSOLUTE
```

## Verify budget enforcement is restored

```bash
oci budgets budget get \
  --budget-id <BUDGET_OCID> \
  --query 'data.{amount:amount, alerts:alerts[*].threshold}'
```
