# Rollback Playbook: Azure Live ARM Deployment Stack Guard

## Cancel an in-progress deployment

```bash
# List recent deployments to find the in-flight one
az deployment group list -g <RESOURCE_GROUP> \
  --query "[?properties.provisioningState=='Running'].{name:name, timestamp:properties.timestamp}"

# Cancel by name
az deployment group cancel -g <RESOURCE_GROUP> -n <DEPLOYMENT_NAME>
```

Cancellation is best-effort. Resources already provisioned before cancel are NOT torn down.

## Redeploy the last known-good template version

```bash
# List deployment history to find the target
az deployment group list -g <RESOURCE_GROUP> \
  --query "[].{name:name, state:properties.provisioningState, timestamp:properties.timestamp}" \
  --output table

# Export the template from a prior successful deployment
az deployment group export -g <RESOURCE_GROUP> -n <GOOD_DEPLOYMENT_NAME> \
  --output json > rollback-template.json

# Redeploy
az deployment group create \
  -g <RESOURCE_GROUP> \
  --template-file rollback-template.json \
  --parameters @<PARAMS.json>
```

## Deployment Stack — update back to previous config

```bash
# Re-apply the previous stack config (update, not recreate)
az deployment-stack group create \
  -n <STACK_NAME> \
  -g <RESOURCE_GROUP> \
  --template-file rollback-template.json \
  --parameters @<PARAMS.json> \
  --action-on-unmanage deleteResources \
  --deny-settings-mode denyDelete
```

## Rollback limitations

- ARM deployments are additive by default — they do not auto-delete resources added in the failed run.
- Deployment Stack `deleteResources` on unmanage will delete resources removed from the template.
- Stateful resources (databases, storage accounts, Key Vaults) cannot be "rolled back" — only re-provisioned from backup.
- If a resource was replaced (`~` in what-if), the original resource may already be deleted.
