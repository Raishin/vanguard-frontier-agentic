# Permissions: OCI Live Resource Manager Stack Guard

# OCI IAM policy guidance for Resource Manager stack guard

## Identity model preference

1. Named group in target compartment — never `any-user` or `any-group`
2. Dynamic group matching the CI/CD runner instance by compartment and tag
3. Short-lived session token via Instance Principal for automation
4. Never grant `manage all-resources in tenancy`

## OCI IAM verb hierarchy reminder

`inspect` ⊂ `read` ⊂ `use` ⊂ `manage`

- `inspect` — list-only (no content details)
- `read` — get + list (read details, no mutation)
- `use` — limited mutation (no create/terminate)
- `manage` — full CRUD (create, update, delete)

## Baseline read policy (auditors — no mutation rights)

```
Allow group <rms-auditors> to inspect orm-stacks in compartment <prod-compartment>
Allow group <rms-auditors> to read orm-stacks in compartment <prod-compartment>
Allow group <rms-auditors> to inspect orm-jobs in compartment <prod-compartment>
Allow group <rms-auditors> to read orm-jobs in compartment <prod-compartment>
```

## Plan-only policy (can create plan jobs, cannot apply or destroy)

```
Allow group <rms-planners> to use orm-stacks in compartment <prod-compartment>
Allow group <rms-planners> to use orm-jobs in compartment <prod-compartment>
```

## Full operator policy (apply + destroy — gate with approval workflow)

```
Allow group <rms-operators> to manage orm-stacks in compartment <prod-compartment>
Allow group <rms-operators> to manage orm-jobs in compartment <prod-compartment>
```

## Dynamic group for CI/CD instance principal

```
Any {instance.compartment.id = '<compartment_ocid>', tag.Operations.Role.value = 'rms-runner'}

Allow dynamic-group <rms-runners> to manage orm-stacks in compartment <prod-compartment>
Allow dynamic-group <rms-runners> to manage orm-jobs in compartment <prod-compartment>
```

## Do not use

```
# FORBIDDEN
# Allow any-user to manage all-resources in tenancy  ← FORBIDDEN
Allow group <rms-operators> to manage all-resources in compartment prod
```

Stack auto-lock: Resource Manager allows **only one running job at a time per stack**.
This is platform-enforced — no additional concurrency control needed.

