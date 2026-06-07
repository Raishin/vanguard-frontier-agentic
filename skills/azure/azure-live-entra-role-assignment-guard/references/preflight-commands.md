# Preflight Commands: Azure Live Entra Role Assignment Guard

Run all of these before creating or deleting any role assignment.

## 1. Confirm caller identity and active subscription

```bash
az account show --query "{subscription:id, name:name, tenantId:tenantId, caller:user.name}"
az ad signed-in-user show --query "{displayName:displayName, id:id, userPrincipalName:userPrincipalName}"
```

## 2. Inspect existing role assignments on the target scope

```bash
# Subscription scope
az role assignment list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --include-inherited \
  --query "[].{role:roleDefinitionName, principal:principalName, principalType:principalType, scope:scope}"

# Management group scope
az role assignment list \
  --scope "$APPROVED_MANAGEMENT_GROUP_SCOPE" \
  --include-inherited \
  --query "[].{role:roleDefinitionName, principal:principalName, principalType:principalType, scope:scope}"

# Resource group scope
az role assignment list \
  --resource-group $AZURE_RESOURCE_GROUP_NAME \
  --include-inherited \
  --query "[].{role:roleDefinitionName, principal:principalName, principalType:principalType, scope:scope}"
```

## 3. Verify the assignee identity and principal type

```bash
# For a user
az ad user show --id $ASSIGNEE_LOOKUP_VALUE \
  --query "{displayName:displayName, userPrincipalName:userPrincipalName, userType:userType, accountEnabled:accountEnabled}"

# userType: "Guest" = external account, elevated risk. Always flag.

# For a service principal
az ad sp show --id $SERVICE_PRINCIPAL_LOOKUP_VALUE \
  --query "{displayName:displayName, appId:appId, servicePrincipalType:servicePrincipalType}"

# For a managed identity
az identity show --name $MANAGED_IDENTITY_NAME --resource-group $AZURE_RESOURCE_GROUP_NAME \
  --query "{name:name, principalId:principalId, tenantId:tenantId}"
```

## 4. Check for existing dangerous standing assignments (audit)

```bash
# Find Owner and UAA at subscription scope (Kusto alternative via activity log)
az role assignment list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[?roleDefinitionName=='Owner' || roleDefinitionName=='User Access Administrator'].{role:roleDefinitionName, principal:principalName, principalType:principalType}"
```

## 5. Check whether a PIM eligible assignment already exists (prefer PIM over permanent)

```bash
az role eligibility-schedule list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[?principalId=='$ASSIGNEE_OBJECT_ID'].{role:roleDefinitionDisplayName, endDateTime:endDateTime, status:status}"
```

If an eligible assignment already exists, the correct action is PIM activation, not a new permanent assignment.
