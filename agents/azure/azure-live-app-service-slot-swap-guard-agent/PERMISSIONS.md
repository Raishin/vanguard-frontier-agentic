# Permissions: Azure Live App Service Slot Swap Guard

# Least-privilege RBAC guidance for App Service slot swaps

## Custom role (slot swap only, one App Service)

```json
{
  "Name": "App Service Slot Swap Guard",
  "IsCustom": true,
  "Description": "Read App Service slot config and perform staged swap. No write to app settings or deployment config.",
  "Actions": [
    "Microsoft.Web/sites/read",
    "Microsoft.Web/sites/slots/read",
    "Microsoft.Web/sites/slots/config/read",
    "Microsoft.Web/sites/slots/slotsswap/action",
    "Microsoft.Web/sites/slotsswap/action",
    "Microsoft.Web/sites/config/read"
  ],
  "NotActions": [
    "Microsoft.Web/sites/config/write",
    "Microsoft.Web/sites/slots/config/write",
    "Microsoft.Web/sites/delete",
    "Microsoft.Web/sites/slots/delete"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<TARGET_RG>/providers/Microsoft.Web/sites/<APP_NAME>"
  ]
}
```

## Nearest built-in role (broader than needed — prefer the custom role above)

`Website Contributor` includes swap rights but also allows config writes.
Use it only when the custom role assignment scope is too difficult to manage.

## Do not assign

- `Owner` on the App Service — allows deletion
- `Microsoft.Web/sites/config/write` without change-management gate
- `Microsoft.Web/sites/slots/delete` — slot deletion is irreversible and excluded from the swap role
- Subscription-level `Website Contributor` for routine swap operations

