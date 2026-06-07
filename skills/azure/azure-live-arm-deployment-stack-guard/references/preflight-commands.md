# Preflight Commands: Azure Live ARM Deployment Stack Guard

Use shell variables for examples instead of raw identifiers. Populate them from an approved change record or already configured shell context; never paste tenant, subscription, resource, or secret values into chat.

## Evidence-variable convention

Variables such as $AZURE_RESOURCE_GROUP_NAME, $APP_SERVICE_APP_NAME, or $KEY_VAULT_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

Run these before any ARM or Deployment Stack mutation. Paste sanitized output as evidence.

## 1. Confirm identity and subscription target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az group show -n $AZURE_RESOURCE_GROUP_NAME --query "{name:name, location:location, provisioningState:properties.provisioningState}"
```

## 2. Run what-if before any deployment

```bash
# ARM template what-if
az deployment group what-if \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --template-file $ARM_TEMPLATE_FILE \
  --parameters @$ARM_PARAMETERS_FILE

# Bicep what-if
az deployment group what-if \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --template-file $BICEP_TEMPLATE_FILE \
  --parameters @$BICEP_PARAMETERS_FILE
```

Review the what-if output for resource replacements (marked with `~` or `-/+`).
Any replacement of a stateful resource (database, storage, Key Vault) must be
explicitly approved before proceeding.

## 3. Inspect existing Deployment Stack state

```bash
az deployment-stack group show \
  -n $DEPLOYMENT_STACK_NAME \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --query "{provisioningState:provisioningState, denySettings:properties.denySettings, resources:properties.resources[].id}"
```

## 4. List managed resources and their protection status

```bash
az deployment-stack group show -n $DEPLOYMENT_STACK_NAME -g $AZURE_RESOURCE_GROUP_NAME \
  --query "properties.resources[].{id:id, denyStatus:denyStatus}"
```

## 5. Validate the template without deploying

```bash
az deployment group validate \
  -g $AZURE_RESOURCE_GROUP_NAME \
  --template-file $ARM_TEMPLATE_FILE \
  --parameters @$ARM_PARAMETERS_FILE
```


## Deployment Stack what-if caveat

Microsoft Learn currently documents that what-if support is not yet available for Deployment Stacks. Use ARM/Bicep what-if for the underlying deployment where available, then explicitly label any stack-level delete/detach and deny-setting risk that what-if does not prove.
