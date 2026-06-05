# Permission Model: Azure Live ARM Deployment Stack Guard

## Custom role — what-if and stack write, stack deletion excluded

```json
{
  "Name": "ARM Deployment Stack Guard",
  "IsCustom": true,
  "Description": "Minimum rights for guarded ARM what-if and Deployment Stack changes in one target resource group. Stack deletion is EXCLUDED — it requires a separate PIM-elevated role.",
  "Actions": [
    "Microsoft.Resources/deployments/read",
    "Microsoft.Resources/deployments/write",
    "Microsoft.Resources/deployments/whatIf/action",
    "Microsoft.Resources/deploymentStacks/read",
    "Microsoft.Resources/deploymentStacks/write",
    "Microsoft.Resources/deploymentStacks/manageDenySetting/action",
    "Microsoft.Resources/subscriptions/resourceGroups/read"
  ],
  "NotActions": [
    "Microsoft.Resources/deploymentStacks/delete"
  ],
  "DataActions": [],
  "NotDataActions": [],
  "AssignableScopes": [
    "<resource-group-resource-scope>"
  ]
}
```

`deploymentStacks/delete` is in `NotActions`. Stack deletion requires a separate
PIM-eligible role activated only for confirmed decommission windows (see below).

## PIM-elevated delete role (activate only for planned decommission)

```json
{
  "Name": "ARM Deployment Stack Delete (PIM)",
  "IsCustom": true,
  "Description": "Stack deletion only. Must be PIM-activated with approval and time-bound to a decommission window.",
  "Actions": [
    "Microsoft.Resources/deploymentStacks/read",
    "Microsoft.Resources/deploymentStacks/delete"
  ],
  "NotActions": [],
  "AssignableScopes": [
    "<resource-group-resource-scope>"
  ]
}
```

Assign as **PIM-eligible only**. Require manager approval. Maximum 2-hour activation.

## Deployment Stacks denySettings recommendation

```bash
az deployment-stack group create \
  --deny-settings-mode denyDelete \
  --deny-settings-apply-to-child-scopes \
  ...
```

Use `denyWriteAndDelete` for compliance-mandated immutable resources.

## Do not assign

- `Owner` at subscription scope
- `Contributor` at management-group scope
- `Microsoft.Resources/*` wildcards
- `Microsoft.Authorization/roleAssignments/write` (privilege escalation risk)


## Deny-setting permission caveat

Microsoft Learn documents dedicated deployment stack roles and a separate `Microsoft.Resources/deploymentStacks/manageDenySetting/action` permission for managing deny settings. If the operator cannot prove this permission with sampled read-only evidence or an approved eligible role activation, do not change deny settings.

Use exact resource scopes from approved change records; do not paste raw subscription identifiers into chat.
