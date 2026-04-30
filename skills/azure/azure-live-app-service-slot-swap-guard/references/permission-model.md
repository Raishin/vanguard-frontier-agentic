# Permission Model: Azure Live App Service Slot Swap Guard

## Custom role — slot swap only, no config writes

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

## Nearest built-in alternative

`Website Contributor` includes swap rights but also allows config writes.
Use only when custom role scope is impractical — and scope it to the single App Service, not the resource group.

## Do not assign

- `Owner` on the App Service — allows deletion
- `Microsoft.Web/sites/config/write` without a change-management gate
- `Microsoft.Web/sites/slots/delete` — slot deletion is irreversible and must not be in the swap role
- Subscription-level `Website Contributor` for routine swap operations
