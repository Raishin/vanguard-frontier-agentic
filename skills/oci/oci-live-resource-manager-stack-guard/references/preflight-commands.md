# Preflight Commands: OCI Live Resource Manager Stack Guard

Run these before any Resource Manager stack mutation. Paste sanitized output as evidence.

## 1. Confirm identity and compartment

```bash
oci iam region-subscription list
oci resource-manager stack list \
  --compartment-id <COMPARTMENT_OCID> \
  --query "data[].{displayName:\"display-name\",id:id,lifecycleState:\"lifecycle-state\",terraformVersion:\"terraform-version\"}"
```

## 2. Inspect stack current state

```bash
oci resource-manager stack get \
  --stack-id <STACK_OCID> \
  --query "data.{displayName:\"display-name\",lifecycleState:\"lifecycle-state\",variables:variables,freeformTags:\"freeform-tags\"}"
```

## 3. List recent jobs on the stack

```bash
oci resource-manager job list \
  --stack-id <STACK_OCID> \
  --sort-by TIMECREATED \
  --sort-order DESC \
  --limit 5 \
  --query "data[].{operation:operation,lifecycleState:\"lifecycle-state\",timeCreated:\"time-created\",id:id}"
```

## 4. Run a plan job (dry-run) before apply

```bash
oci resource-manager job create-plan-job \
  --stack-id <STACK_OCID> \
  --display-name "preflight-plan-$(date +%Y%m%d%H%M)" \
  --wait-for-state SUCCEEDED \
  --max-wait-seconds 600

# Get plan output (Terraform plan log)
oci resource-manager job get-job-logs \
  --job-id <PLAN_JOB_OCID> \
  --query "data[].message"
```

## 5. Check for stack drift

```bash
oci resource-manager stack detect-drift \
  --stack-id <STACK_OCID> \
  --wait-for-state SUCCEEDED \
  --max-wait-seconds 300
oci resource-manager stack list-resource-drift-details \
  --stack-id <STACK_OCID>
```
