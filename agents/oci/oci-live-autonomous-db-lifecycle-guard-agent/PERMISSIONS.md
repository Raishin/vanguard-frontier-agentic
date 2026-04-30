# Permissions: OCI Live Autonomous DB Lifecycle Guard

# OCI IAM policy for Autonomous DB lifecycle guard

## Identity model preference

1. Separate groups for readers, operators (start/stop/scale), and admins (clone/terminate)
2. `use` verb for operators — prevents terminate and clone
3. `manage` with tag condition for admins — allows terminate only when protection tag is absent
4. Defined-tag namespace for protection tagging (use a protected namespace, not freeform)

## Baseline read (no mutation)

```
Allow group <adb-auditors> to inspect autonomous-databases in compartment <prod-db-compartment>
Allow group <adb-auditors> to read autonomous-databases in compartment <prod-db-compartment>
Allow group <adb-auditors> to read autonomous-database-backups in compartment <prod-db-compartment>
```

## Operations — start, stop, scale (use verb, no terminate/clone)

```
Allow group <adb-operators> to use autonomous-databases in compartment <prod-db-compartment>
```

With `use` the operator can: start, stop, scale CPU/storage, generate wallet.
The operator CANNOT: terminate, clone to new, change network-access type.

## Admin — clone and terminate (manage + tag condition)

```
Allow group <adb-admins> to manage autonomous-databases in compartment <prod-db-compartment>
  where target.resource.tag.Operations.Lifecycle.value != 'protected'
```

Tag condition: `manage` verbs only succeed if the ADB's defined tag
`Operations.Lifecycle` is NOT set to `protected`. Set this tag on all production ADBs
in a protected tag namespace (so only tag-namespace admins can remove it).

> **IRREVERSIBILITY WARNING — read before granting `manage`:**
>
> - **Termination** is permanent. OCI does not recover terminated ADB instances.
>   The 60-day automatic backup retention window expires; after that, no recovery path exists.
> - **Storage scale-up** (`ocpuCount` or `dataStorageSizeInTBs` increase) cannot be reversed.
>   You can scale CPU down, but storage can only grow — never shrink.
> - Both operations must require dual-sign-off and a confirmed maintenance window
>   before this role is used. The tag-condition gate is a necessary but insufficient control.

## Do not use

```
# FORBIDDEN
Allow group <adb-operators> to manage autonomous-databases in tenancy
Allow any-user to use autonomous-databases in compartment prod-db
```

