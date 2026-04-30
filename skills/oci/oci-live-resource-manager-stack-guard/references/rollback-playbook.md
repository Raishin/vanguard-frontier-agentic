# Rollback Playbook: OCI Live Resource Manager Stack Guard

## Cancel an in-progress job

```bash
# Find in-progress job
oci resource-manager job list \
  --stack-id <STACK_OCID> \
  --query "data[?\"lifecycle-state\"=='IN_PROGRESS'].{id:id,operation:operation}"

# Cancel
oci resource-manager job cancel \
  --job-id <JOB_OCID> \
  --force
```

## Revert by applying a previous Terraform state (rollback apply)

```bash
# Create a new apply job targeting the rollback config
oci resource-manager job create-apply-job \
  --stack-id <STACK_OCID> \
  --display-name "rollback-apply-$(date +%Y%m%d%H%M)" \
  --execution-plan-strategy FROM_PLAN_JOB_ID \
  --execution-plan-job-id <PRIOR_PLAN_JOB_OCID> \
  --wait-for-state SUCCEEDED \
  --max-wait-seconds 1800
```

## Run a destroy job (full teardown — use with extreme caution)

```bash
# Plan the destroy first
oci resource-manager job create-plan-destroy-job \
  --stack-id <STACK_OCID> \
  --wait-for-state SUCCEEDED

# Approve and execute destroy
oci resource-manager job create-destroy-job \
  --stack-id <STACK_OCID> \
  --execution-plan-strategy FROM_PLAN_JOB_ID \
  --execution-plan-job-id <DESTROY_PLAN_JOB_OCID> \
  --wait-for-state SUCCEEDED
```

## Rollback limitations

- Resource Manager only allows one running job per stack — a new job cannot start while one is in progress.
- Cancelling a job stops future Terraform operations but does not revert resources already created/modified.
- Stateful resources (databases, block volumes, object storage buckets with data) cannot be reverted by Terraform rollback.
- Terraform state can diverge from actual resource state if a job was cancelled mid-run — run drift detection before the next apply.
