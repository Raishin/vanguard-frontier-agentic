# Rollback Playbook: Azure Live PIM JIT Activation Guard

## Deactivate an active PIM role assignment immediately

```bash
# Find the active role assignment schedule instance to cancel
az role assignment schedule list \
  --scope "<approved-subscription-scope>" \
  --query "[?assignedTo=='<PRINCIPAL_ID>'].{id:name, role:roleDefinitionDisplayName, endDateTime:endDateTime}"

# Submit a deactivation request
az role assignment schedule request create \
  --scope "<approved-subscription-scope>" \
  --role-definition-id <ROLE_DEF_ID> \
  --principal-id <PRINCIPAL_ID> \
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
  --assignee <PRINCIPAL_ID> \
  --role <ROLE_NAME> \
  --scope "<approved-subscription-scope>"
```

After revoking, immediately review Azure Monitor activity log for actions taken
during the activation window and file an incident report.

## Rollback limitations

- Actions taken during an active PIM session cannot be undone by deactivating the role.
- Azure Activity Log retains actions for 90 days — preserve a log export for security review.
- PIM activation logs in Entra ID are retained per your Entra ID log retention settings.
