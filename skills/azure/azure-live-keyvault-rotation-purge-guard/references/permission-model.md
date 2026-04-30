# Permission Model: Azure Live Key Vault Rotation Purge Guard

## Rotation operator role — no delete, no purge

```json
{
  "Name": "Key Vault Rotation Guard",
  "IsCustom": true,
  "Description": "Rotate keys and update rotation policies. Cannot delete, purge, or disable soft-delete.",
  "Actions": [
    "Microsoft.KeyVault/vaults/read",
    "Microsoft.KeyVault/vaults/keys/read",
    "Microsoft.KeyVault/vaults/secrets/read"
  ],
  "DataActions": [
    "Microsoft.KeyVault/vaults/keys/read",
    "Microsoft.KeyVault/vaults/keys/rotate/action",
    "Microsoft.KeyVault/vaults/keys/rotationpolicy/read",
    "Microsoft.KeyVault/vaults/keys/rotationpolicy/write",
    "Microsoft.KeyVault/vaults/secrets/getSecret/action"
  ],
  "NotDataActions": [
    "Microsoft.KeyVault/vaults/keys/delete",
    "Microsoft.KeyVault/vaults/keys/purge/action",
    "Microsoft.KeyVault/vaults/secrets/delete",
    "Microsoft.KeyVault/vaults/secrets/purge/action"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/<TARGET_RG>/providers/Microsoft.KeyVault/vaults/<VAULT_NAME>"
  ]
}
```

Nearest built-in roles: `Key Vault Crypto Officer` (keys), `Key Vault Secrets Officer` (secrets).
Both include delete — prefer the custom role above for rotation-only scenarios.

## Purge-protection enablement (separate, PIM-gated operation)

Requires `Microsoft.KeyVault/vaults/write` on the vault resource.
Assign via PIM with justification. Maximum 1-hour activation window.

**IRREVERSIBILITY WARNING**: Once `enablePurgeProtection: true` is set on a vault,
it cannot be unset. All soft-deleted objects are protected from permanent deletion
until the retention period (7–90 days) expires. This is a one-way door.

## Do not assign

- `Key Vault Administrator` standing (includes purge rights)
- `Microsoft.KeyVault/vaults/purge/action` to rotation operators
- `Microsoft.KeyVault/vaults/accessPolicies/write` to non-admins (legacy access policy model)
