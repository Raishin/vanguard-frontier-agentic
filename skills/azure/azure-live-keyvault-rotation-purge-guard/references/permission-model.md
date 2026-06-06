# Permission Model: Azure Live Key Vault Rotation Purge Guard

## Evidence-variable convention

Shell variables in examples are local operator placeholders from an approved change record or already configured shell context. Do not commit real values, and redact them from shared evidence unless disclosure is explicitly approved.

## Rotation operator role — no delete, no purge

```json
{
  "Name": "Key Vault Rotation Guard",
  "IsCustom": true,
  "Description": "Rotate keys and update rotation policies. Cannot delete or purge keys/secrets/certificates. Cannot purge the vault itself. Cannot disable soft-delete.",
  "Actions": [
    "Microsoft.KeyVault/vaults/read",
    "Microsoft.KeyVault/vaults/keys/read",
    "Microsoft.KeyVault/vaults/secrets/read"
  ],
  "NotActions": [
    "Microsoft.KeyVault/vaults/purge/action",
    "Microsoft.KeyVault/vaults/delete",
    "Microsoft.KeyVault/vaults/write",
    "Microsoft.KeyVault/vaults/accessPolicies/write"
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
    "Microsoft.KeyVault/vaults/secrets/purge/action",
    "Microsoft.KeyVault/vaults/certificates/delete",
    "Microsoft.KeyVault/vaults/certificates/purge/action"
  ],
  "AssignableScopes": [
    "$APPROVED_KEY_VAULT_SCOPE"
  ]
}
```

Nearest built-in roles: `Key Vault Crypto Officer` (keys), `Key Vault Secrets Officer` (secrets).
Both include delete — prefer the custom role above for rotation-only scenarios.

**Action vs DataAction distinction (security-critical)**:
`Microsoft.KeyVault/vaults/purge/action` is a **control-plane Action** that purges the
soft-deleted **vault** itself (irreversible). It is **not** a DataAction and is not blocked
by `NotDataActions`. It must be in `NotActions`. Certificate operations exist on both planes;
this role blocks both. Do not assume `NotDataActions` covers all destructive Key Vault paths.

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

Use exact resource scopes from approved change records; do not paste raw subscription identifiers into chat.
