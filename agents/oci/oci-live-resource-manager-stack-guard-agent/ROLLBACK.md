# Resource Manager Stack — Rollback Playbook

Resource Manager auto-locks the stack during jobs — concurrent apply/destroy is
physically prevented. Rollback options depend on how far the failed apply progressed.

## Option 1: Apply previous configuration (re-upload prior config zip)

```bash
oci resource-manager stack update \
  --stack-id <STACK_OCID> \
  --config-source-zip-file previous-config.zip

oci resource-manager job create-apply-job \
  --stack-id <STACK_OCID> \
  --execution-plan-strategy FROM_PLAN_JOB_ID \
  --execution-plan-job-id <PRIOR_PLAN_JOB_OCID> \
  --display-name "rollback-apply-$(date +%Y%m%dT%H%M%S)"
```

## Option 2: Import a known-good Terraform state file

```bash
oci resource-manager job create-import-tf-state-job \
  --stack-id <STACK_OCID> \
  --tf-state-base64 "$(base64 -i previous.tfstate)"
```

## Option 3: Targeted destroy of newly-created resources only

```bash
oci resource-manager job create-destroy-job \
  --stack-id <STACK_OCID> \
  --execution-plan-strategy AUTO_APPROVED \
  --display-name "targeted-destroy-$(date +%Y%m%dT%H%M%S)"
```

Only use AUTO_APPROVED if human has already reviewed the destroy plan separately.

## Monitor rollback job

```bash
oci resource-manager job get \
  --job-id <JOB_OCID> \
  --query 'data."lifecycle-state"'
```
