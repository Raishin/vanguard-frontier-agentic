# ARM Deployment Stack — Preflight Commands

Run all of these before executing any live ARM or Deployment Stack change.

## 1. Confirm identity and active subscription

```bash
az account show --query "{sub:id, tenant:tenantId, user:user.name, env:environmentName}"
```

## 2. What-if on ARM / Bicep template

```bash
az deployment group what-if \
  --resource-group <TARGET_RG> \
  --template-file main.bicep \
  --parameters @params.prod.json \
  --result-format FullResourcePayloads
```

Stop if what-if shows unexpected deletions or replacements. Deletions require separate approval.

## 3. Inspect current Deployment Stack state

```bash
az deployment-stack group show \
  --name <STACK_NAME> \
  --resource-group <TARGET_RG> \
  --query "{state:provisioningState, denySettings:denySettings, resourceCount:length(resources)}"
```

## 4. Review deny assignments on target scope

```bash
az role assignment list \
  --resource-group <TARGET_RG> \
  --include-deny \
  --query "[?type=='Microsoft.Authorization/denyAssignments'].{name:name,actions:denyAssignmentPermissions[0].actions}"
```

## 5. Validate template syntax

```bash
az deployment group validate \
  --resource-group <TARGET_RG> \
  --template-file main.bicep \
  --parameters @params.prod.json
```
