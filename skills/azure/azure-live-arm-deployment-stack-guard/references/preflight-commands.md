# Preflight Commands: Azure Live ARM Deployment Stack Guard

Run these before any ARM or Deployment Stack mutation. Paste sanitized output as evidence.

## 1. Confirm identity and subscription target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az group show -n <resource-group-name> --query "{name:name, location:location, provisioningState:properties.provisioningState}"
```

## 2. Run what-if before any deployment

```bash
# ARM template what-if
az deployment group what-if \
  -g <resource-group-name> \
  --template-file <TEMPLATE.json> \
  --parameters @<PARAMS.json>

# Bicep what-if
az deployment group what-if \
  -g <resource-group-name> \
  --template-file <TEMPLATE.bicep> \
  --parameters @<PARAMS.bicepparam>
```

Review the what-if output for resource replacements (marked with `~` or `-/+`).
Any replacement of a stateful resource (database, storage, Key Vault) must be
explicitly approved before proceeding.

## 3. Inspect existing Deployment Stack state

```bash
az deployment-stack group show \
  -n <stack-name> \
  -g <resource-group-name> \
  --query "{provisioningState:provisioningState, denySettings:properties.denySettings, resources:properties.resources[].id}"
```

## 4. List managed resources and their protection status

```bash
az deployment-stack group show -n <stack-name> -g <resource-group-name> \
  --query "properties.resources[].{id:id, denyStatus:denyStatus}"
```

## 5. Validate the template without deploying

```bash
az deployment group validate \
  -g <resource-group-name> \
  --template-file <TEMPLATE.json> \
  --parameters @<PARAMS.json>
```


## Deployment Stack what-if caveat

Microsoft Learn currently documents that what-if support is not yet available for Deployment Stacks. Use ARM/Bicep what-if for the underlying deployment where available, then explicitly label any stack-level delete/detach and deny-setting risk that what-if does not prove.
