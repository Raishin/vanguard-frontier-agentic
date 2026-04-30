# Permission Model: OCI Live Vault Key Destruction Guard

## 3-tier separation

| Tier | Group | Verb | Scope |
|------|-------|------|-------|
| Audit | `<vault-auditors>` | inspect / read | `<prod-vault-compartment>` |
| Rotation operator | `<vault-key-operators>` | use | `<prod-vault-compartment>` |
| Key admin | `<vault-key-admins>` | manage + tag condition | `<prod-vault-compartment>` |

## Key audit policy (read only, no mutation)

```
Allow group <vault-auditors> to inspect vaults in compartment <prod-vault-compartment>
Allow group <vault-auditors> to read vaults in compartment <prod-vault-compartment>
Allow group <vault-auditors> to read keys in compartment <prod-vault-compartment>
Allow group <vault-auditors> to inspect key-versions in compartment <prod-vault-compartment>
```

## Key rotation (use verb — new versions only, no deletion scheduling)

```
Allow group <vault-key-operators> to use keys in compartment <prod-vault-compartment>
Allow group <vault-key-operators> to use key-delegate in compartment <prod-vault-compartment>
```

With `use`: create new key versions, enable/disable key versions.
Cannot: schedule key deletion, delete the key, import key material.

## Key destruction (manage + tag condition)

```
Allow group <vault-key-admins> to manage keys in compartment <prod-vault-compartment>
  where target.resource.tag.Lifecycle.Deletable.value = 'approved'
```

The `Lifecycle.Deletable = approved` tag must be set in a **protected tag namespace**.
Production keys must never have this tag unless actively being retired.

## CRITICAL timing

```
Minimum deletion window: 7 days
Recommended deletion window: 30 days
Cancel deadline: any time BEFORE the scheduled deletion time passes
After deletion: PERMANENT. No recovery. No OCI Support escalation path.
```

## Do not use

```
# FORBIDDEN
Allow group <vault-operators> to manage all-resources in compartment prod-vault
Allow any-user to manage keys in tenancy
```
