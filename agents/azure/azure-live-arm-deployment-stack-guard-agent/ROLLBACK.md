# ARM Deployment Stack — Rollback Playbook

## Option 1: Re-deploy previous known-good template (incremental)

```bash
az deployment group create \
  --resource-group <TARGET_RG> \
  --template-file main.prev.bicep \
  --parameters @params.prod.prev.json \
  --mode Incremental \
  --name "rollback-$(date +%Y%m%dT%H%M%S)"
```

## Option 2: Detach stack management without deleting resources

```bash
az deployment-stack group delete \
  --name <STACK_NAME> \
  --resource-group <TARGET_RG> \
  --action-on-unmanage detachAll \
  --yes
```

Use when the stack definition is wrong but the deployed resources are still healthy.

## Option 3: Full stack deletion (nuclear — bypass denySettings first)

```bash
# REQUIRES: denySettings override or prior denyDelete removal
az deployment-stack group delete \
  --name <STACK_NAME> \
  --resource-group <TARGET_RG> \
  --action-on-unmanage deleteAll \
  --bypass-stack-out-of-sync-error \
  --yes
```

WARNING: If `denySettings.mode = denyDelete`, this command fails by design.
That failure is correct behavior — escalate to a Principal with deny-assignment write rights.

## Verify

```bash
az deployment group show \
  --resource-group <TARGET_RG> \
  --name <DEPLOYMENT_NAME> \
  --query "{state:properties.provisioningState, timestamp:properties.timestamp}"
```
