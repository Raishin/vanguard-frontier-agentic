# Rollback Playbook: Azure Live PIM JIT Activation Guard

## Evidence-variable convention

Shell variables in examples are local operator placeholders from an approved change record or already configured shell context. Do not commit real values, and redact them from shared evidence unless disclosure is explicitly approved.

## Deactivate an active PIM role assignment immediately

```bash
# Find the active role assignment schedule instance to cancel
az role assignment schedule list \
  --scope "$APPROVED_AZURE_SCOPE" \
  --query "[?assignedTo=='$PRINCIPAL_OBJECT_ID'].{id:name, role:roleDefinitionDisplayName, endDateTime:endDateTime}"

# Submit a deactivation request
az role assignment schedule request create \
  --scope "$APPROVED_AZURE_SCOPE" \
  --role-definition-id $ROLE_DEFINITION_ID \
  --principal-id $PRINCIPAL_OBJECT_ID \
  --request-type SelfDeactivate
```

## Deny a pending approval request

PIM approval actions are performed via Entra ID portal or the PIM API:

```
PATCH https://management.azure.com/{scope}/providers/Microsoft.Authorization/roleAssignmentScheduleRequests/{requestId}?api-version=2020-10-01
Body: { "properties": { "status": "Denied", "justification": "<reason>" } }
```

## Revoke an emergency break-glass access grant

```bash
# Remove the active role assignment
az role assignment delete \
  --assignee $PRINCIPAL_OBJECT_ID \
  --role $ROLE_DEFINITION_NAME \
  --scope "$APPROVED_AZURE_SCOPE"
```

After revoking, immediately review Azure Monitor activity log for actions taken
during the activation window and file an incident report.

## Rollback limitations

- Actions taken during an active PIM session cannot be undone by deactivating the role.
- Azure Activity Log retains actions for 90 days — preserve a log export for security review.
- PIM activation logs in Entra ID are retained per your Entra ID log retention settings.
