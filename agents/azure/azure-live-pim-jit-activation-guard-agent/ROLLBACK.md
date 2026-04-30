# PIM JIT Activation — Rollback Playbook

## Option 1: Self-deactivate an active role early

```bash
SCHED_ID="<ROLE_ASSIGNMENT_SCHEDULE_ID>"
SUB_ID=$(az account show --query id -o tsv)
REQUEST_ID=$(uuidgen)

az rest \
  --method PUT \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleAssignmentScheduleRequests/${REQUEST_ID}?api-version=2020-10-01" \
  --body "{
    \"properties\": {
      \"requestType\": \"SelfDeactivate\",
      \"linkedRoleEligibilityScheduleId\": \"${SCHED_ID}\",
      \"scheduleInfo\": {
        \"expiration\": { \"type\": \"AfterDuration\", \"duration\": \"PT0S\" }
      }
    }
  }"
```

## Option 2: Cancel a pending activation request (before approval)

```bash
az rest \
  --method DELETE \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleAssignmentScheduleRequests/<REQUEST_ID>?api-version=2020-10-01"
```

## Option 3: Deny a pending approval request (approver action)

```bash
az rest \
  --method POST \
  --url "https://management.azure.com/providers/Microsoft.Authorization/roleAssignmentApprovals/<APPROVAL_ID>/stages/<STAGE_ID>?api-version=2021-01-01-preview" \
  --body "{\"reviewResult\": \"Deny\", \"justification\": \"<REASON>\"}"
```

## Verify deactivation

```bash
az rest \
  --method GET \
  --url "https://management.azure.com/subscriptions/${SUB_ID}/providers/Microsoft.Authorization/roleAssignmentSchedules?\$filter=principalId+eq+'${PRINCIPAL_OID}'&api-version=2020-10-01" \
  --query "value[?properties.status=='Active'].{role:properties.expandedProperties.roleDefinition.displayName}"
```
