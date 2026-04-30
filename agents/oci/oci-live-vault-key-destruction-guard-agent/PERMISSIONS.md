# Permissions: OCI Live Vault Key Destruction Guard

# OCI IAM policy for Vault key destruction guard

## Identity model preference

1. Separate groups for key auditors, key rotation operators, and key destruction admins
2. `use` verb for rotation operators — creates new key versions, cannot schedule deletion
3. `manage` for key destruction admins, restricted by tag condition (deletable tag required)
4. Dual-control: key deletion requires a second approver group confirmation

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

With `use` the operator can: create new key versions, enable/disable key versions.
The operator CANNOT: schedule key deletion, delete the key, import key material.

## Key destruction (manage + tag condition — only for approved-deletable keys)

```
Allow group <vault-key-admins> to manage keys in compartment <prod-vault-compartment>
  where target.resource.tag.Lifecycle.Deletable.value = 'approved'
```

The `Lifecycle.Deletable = approved` tag must be set in a protected tag namespace.
Production keys should never have this tag set unless they are actively being retired.

## CRITICAL timing note

```
Minimum deletion window: 7 days
Recommended deletion window: 30 days
Cancel deadline: any time BEFORE time-of-deletion passes
After deletion: PERMANENT. No recovery. No OCI Support escalation path.
```

## Do not use

```
# FORBIDDEN
Allow group <vault-operators> to manage all-resources in compartment prod-vault
Allow any-user to manage keys in tenancy
```

