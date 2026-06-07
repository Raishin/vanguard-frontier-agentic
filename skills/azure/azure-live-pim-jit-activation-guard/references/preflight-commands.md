# Preflight Commands: Azure Live PIM JIT Activation Guard

Run these before initiating or approving a PIM activation.

## Evidence-variable convention

Shell variables in examples are local operator placeholders from an approved change record or already configured shell context. Do not commit real values, and redact them from shared evidence unless disclosure is explicitly approved.

## 1. Confirm current principal identity

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az ad signed-in-user show --query "{displayName:displayName, id:id, userPrincipalName:userPrincipalName}"
```

## 2. List current PIM eligible assignments for a principal

```bash
# Using Azure CLI
az role eligibility-schedule list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[?principalId=='$PRINCIPAL_OBJECT_ID'].{roleName:roleDefinitionDisplayName, scope:scope, endDateTime:endDateTime}"
```

## 3. List active role assignments (not eligible — currently active)

```bash
az role assignment list \
  --assignee $PRINCIPAL_LOOKUP_VALUE \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[].{role:roleDefinitionName, scope:scope, principalType:principalType}"
```

## 4. Check pending activation requests

```bash
az role assignment schedule request list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[?status=='PendingApproval' || status=='PendingAdminDecision'].{requestId:name, role:roleDefinitionDisplayName, requestor:requestorId, justification:justification}"
```

## 5. Verify audit log for recent activations

```bash
# Requires Entra ID audit log access
az monitor activity-log list \
  --resource-provider Microsoft.Authorization \
  --start-time $(date -u -d '24 hours ago' +%Y-%m-%dT%H:%M:%SZ) \
  --query "[?operationName.value contains 'roleAssignmentScheduleRequests'].{caller:caller, time:eventTimestamp, status:status.value}"
```
