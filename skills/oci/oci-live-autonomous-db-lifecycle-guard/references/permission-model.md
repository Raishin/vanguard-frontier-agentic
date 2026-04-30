# Permission Model: OCI Live Autonomous DB Lifecycle Guard

## 3-tier separation

| Tier | Group | Verb | Scope |
|------|-------|------|-------|
| Audit | `<adb-auditors>` | inspect / read | `<prod-db-compartment>` |
| Operator | `<adb-operators>` | use | `<prod-db-compartment>` |
| Admin | `<adb-admins>` | manage + tag condition | `<prod-db-compartment>` |

## Baseline read (no mutation)

```
Allow group <adb-auditors> to inspect autonomous-databases in compartment <prod-db-compartment>
Allow group <adb-auditors> to read autonomous-databases in compartment <prod-db-compartment>
Allow group <adb-auditors> to read autonomous-database-backups in compartment <prod-db-compartment>
```

## Operator — start, stop, scale (use verb, no terminate/clone)

```
Allow group <adb-operators> to use autonomous-databases in compartment <prod-db-compartment>
```

With `use`: start, stop, scale CPU/storage, generate wallet.
Cannot: terminate, clone, change network-access type.

## Admin — clone and terminate (manage + tag condition)

```
Allow group <adb-admins> to manage autonomous-databases in compartment <prod-db-compartment>
  where target.resource.tag.Operations.Lifecycle.value != 'protected'
```

The `Operations.Lifecycle = protected` tag must be set in a **protected tag namespace** on all
production ADBs. Only tag-namespace admins can remove the tag.

> **IRREVERSIBILITY WARNING**
> - **Termination** is permanent. OCI does not recover terminated ADB instances.
> - **Storage scale-up** is a one-way door — storage can only grow, never shrink.
> - Both operations require dual-sign-off and a confirmed maintenance window.

## Do not use

```
# FORBIDDEN
Allow group <adb-operators> to manage autonomous-databases in tenancy
Allow any-user to use autonomous-databases in compartment prod-db
```
