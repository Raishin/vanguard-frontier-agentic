# Rollback Playbook: Azure Live ARM Deployment Stack Guard

## Evidence-variable convention

Variables such as $AZURE_RESOURCE_GROUP_NAME, $APP_SERVICE_APP_NAME, $DEPLOYMENT_NAME, or $DEPLOYMENT_STACK_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

## Cancel an in-progress deployment

```bash
# List recent deployments to find the in-flight one
az deployment group list -g $AZURE_RESOURCE_GROUP_NAME \
  --query "[?properties.provisioningState=='Running'].{name:name, timestamp:properties.timestamp}"

# Cancel by name
az deployment group cancel -g $AZURE_RESOURCE_GROUP_NAME -n $DEPLOYMENT_NAME
```

Cancellation is best-effort. Resources already provisioned before cancel are NOT torn down.

## Redeploy the last known-good template version

```bash
# List deployment history to find the target
az deployment group list -g $AZURE_RESOURCE_GROUP_NAME \
  --query "[].{name:name, state:properties.provisioningState, timestamp:properties.timestamp}" \
  --output table

# Export the template from a prior successful deployment
az deployment group export -g $AZURE_RESOURCE_GROUP_NAME -n $KNOWN_GOOD_DEPLOYMENT_NAME \
  --output json > rollback-template.json

# Redeploy
az deployment group create \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --template-file rollback-template.json \
  --parameters @$ARM_PARAMETERS_FILE
```

## Deployment Stack — update back to previous config

```bash
# Re-apply the previous stack config (update, not recreate)
az deployment-stack group create \
  -n $DEPLOYMENT_STACK_NAME \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --template-file rollback-template.json \
  --parameters @$ARM_PARAMETERS_FILE \
  --action-on-unmanage deleteResources \
  --deny-settings-mode denyDelete
```

## Rollback limitations

- ARM deployments are additive by default — they do not auto-delete resources added in the failed run.
- Deployment Stack `deleteResources` on unmanage will delete resources removed from the template.
- Stateful resources (databases, storage accounts, Key Vaults) cannot be "rolled back" — only re-provisioned from backup.
- If a resource was replaced (`~` in what-if), the original resource may already be deleted.
