# PIM JIT Activation — Preflight Commands

## 1. Check eligible assignments for the current principal

```bash
PRINCIPAL_OID=$(az ad signed-in-user show --query id -o tsv)
SUB_ID=$(az account show --query id -o tsv)

az rest \
  --method GET \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleEligibilitySchedules?\$filter=principalId+eq+'${PRINCIPAL_OID}'&api-version=2020-10-01" \
  --query "value[].{role:properties.expandedProperties.roleDefinition.displayName, scope:properties.scope, status:properties.status, endTime:properties.endDateTime}"
```

## 2. Check for already-active assignments (prevent duplicate activation)

```bash
az rest \
  --method GET \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleAssignmentSchedules?\$filter=principalId+eq+'${PRINCIPAL_OID}'&api-version=2020-10-01" \
  --query "value[].{role:properties.expandedProperties.roleDefinition.displayName, status:properties.status, endTime:properties.endDateTime}"
```

## 3. Confirm Conditional Access and MFA status

```bash
# Verify the signed-in user's MFA registration
az rest \
  --method GET \
  --url "https://graph.microsoft.com/v1.0/me/authentication/methods" \
  --resource "https://graph.microsoft.com/"
```

## 4. List pending approval requests (for approvers)

```bash
az rest \
  --method GET \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleAssignmentScheduleRequests?\$filter=status+eq+'PendingApproval'&api-version=2020-10-01" \
  --query "value[].{requestor:properties.expandedProperties.principal.displayName, role:properties.expandedProperties.roleDefinition.displayName, justification:properties.justification}"
```
