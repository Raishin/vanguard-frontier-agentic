# Resource Manager Stack — Preflight Commands

## 1. Confirm identity and region

```bash
oci iam region list --output table
oci iam user get --user-id <OPERATOR_OCID> --query 'data.name'
```

## 2. Inspect current stack state

```bash
oci resource-manager stack get \
  --stack-id <STACK_OCID> \
  --query 'data.{state:"lifecycle-state", updated:"time-updated", terraform:"terraform-version", compartment:"compartment-id"}'
```

## 3. Detect drift (always before apply or destroy)

```bash
oci resource-manager stack detect-drift \
  --stack-id <STACK_OCID>

# List drift details once job completes
oci resource-manager stack list-resource-drift-details \
  --stack-id <STACK_OCID>
```

## 4. Create a plan job and review output before any apply

```bash
oci resource-manager job create-plan-job \
  --stack-id <STACK_OCID> \
  --display-name "preflight-plan-$(date +%Y%m%dT%H%M%S)"

# Retrieve plan logs
oci resource-manager job get-job-logs \
  --job-id <PLAN_JOB_OCID> --all
```

Stop and escalate if plan output shows unexpected resource deletions or replacements.

## 5. Verify no other job is currently running

```bash
oci resource-manager job list \
  --compartment-id <COMPARTMENT_OCID> \
  --stack-id <STACK_OCID> \
  --lifecycle-state IN_PROGRESS \
  --query 'data[].{id:id, op:"operation", started:"time-created"}'
```
