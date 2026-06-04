# Rollback Playbook: Azure Live Entra Role Assignment Guard

Permanent role assignments do not expire automatically. Rollback means explicit deletion. Always capture the assignment details before write so deletion is unambiguous.

## Before any assignment write — capture the full assignment for rollback

```bash
# Save the exact object ID, role definition ID, and scope
az role assignment list \
  --assignee <PRINCIPAL_OBJECT_ID_OR_UPN> \
  --scope <SCOPE> \
  --query "[].{name:name, roleDefinitionId:roleDefinitionId, principalId:principalId, scope:scope}"
```

## Remove a role assignment by name (most precise)

```bash
az role assignment delete \
  --ids /subscriptions/<SUBSCRIPTION_ID>/providers/Microsoft.Authorization/roleAssignments/<ASSIGNMENT_NAME>
```

## Remove by role + assignee + scope (if name not captured)

```bash
az role assignment delete \
  --assignee <PRINCIPAL_OBJECT_ID_OR_UPN> \
  --role "<ROLE_NAME_OR_ID>" \
  --scope <SCOPE>
```

## Verify deletion took effect

```bash
az role assignment list \
  --assignee <PRINCIPAL_OBJECT_ID_OR_UPN> \
  --scope <SCOPE> \
  --query "[].{role:roleDefinitionName, scope:scope}"
# Should return empty or not include the deleted assignment
```

## Caveats

- Token caching: deleted assignments may still appear valid for up to 10 minutes due to Azure Resource Manager caching; managed identity group membership can have longer cache behavior. Wait before declaring rollback complete.
- Inherited assignments: if the assignment was at a parent scope (subscription or management group), removing it at the child scope is not possible — you must delete from the parent scope where it was created.
- Guest accounts: if the principal is a guest and the assignment was their only entitlement, removal may trigger MFA re-enrollment on next access. Communicate with the affected user.
- Audit log: the deletion will appear in Azure Activity Log under `Microsoft.Authorization/roleAssignments/delete`. Retain the activity log entry as evidence.

## What cannot be rolled back automatically

- Access exercised during the window the assignment was active (data accessed, operations performed) cannot be undone via role removal.
- Any resources created or deleted by the principal during the assignment window must be remediated separately.
