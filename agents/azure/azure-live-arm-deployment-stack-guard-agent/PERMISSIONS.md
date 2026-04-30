# Permissions: Azure Live ARM Deployment Stack Guard

# Least-privilege RBAC guidance

## Identity model preference

1. PIM-eligible Contributor scoped to **target resource group only** — activated JIT for deploy windows
2. Service principal with scoped Contributor for CI/CD pipelines — no standing access
3. Do not use subscription-level Owner or Contributor for routine deployments

## Required Microsoft.* actions

What-if and Deployment Stacks share the same permission boundary. There is no read-only what-if role;
the operator must hold write permissions on the resources being deployed.

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
    "Microsoft.Resources/subscriptions/resourceGroups/read"
  ],
  "NotActions": [
    "Microsoft.Resources/deploymentStacks/delete"
  ],
  "DataActions": [],
  "NotDataActions": [],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<TARGET_RG>"
  ]
}
```

`deploymentStacks/delete` is in `NotActions` above. Stack deletion requires a **separate
PIM-eligible role** (see below) activated only for confirmed decommission windows.

### PIM-elevated delete role (activate only for planned decommission)

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
    "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<TARGET_RG>"
  ]
}
```

Assign this role as **PIM-eligible** (not permanent active). Require manager approval
and a maximum 2-hour activation window. Never combine with `deploymentStacks/write` in
the same PIM activation unless you are replacing a stack.

For each resource type touched by the template, add the matching write action, e.g.
`Microsoft.Compute/virtualMachines/write` for VMs. This is unavoidable — what-if requires it.

## Deployment Stacks denySettings

Recommended default for production stacks:

```bash
az deployment-stack group create \
  --deny-settings-mode denyDelete \
  --deny-settings-apply-to-child-scopes \
  ...
```

`denyDelete` generates a platform-enforced `denyAssignment` on all managed resources.
`denyWriteAndDelete` is stricter — use for compliance-mandated immutable resources.

## Do not assign

- `Owner` at subscription scope
- `Contributor` at management-group scope
- Broad `Microsoft.Resources/*` wildcards
- `Microsoft.Authorization/roleAssignments/write` (privilege escalation risk)

