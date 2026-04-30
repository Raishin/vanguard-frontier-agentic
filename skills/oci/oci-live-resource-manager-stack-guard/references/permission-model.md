# Permission Model: OCI Live Resource Manager Stack Guard

## OCI verb hierarchy reminder

`inspect` ⊂ `read` ⊂ `use` ⊂ `manage`

## 3-tier separation

| Tier | Group | Verb | Scope |
|------|-------|------|-------|
| Auditor | `<rms-auditors>` | inspect / read | `<prod-compartment>` |
| Planner | `<rms-planners>` | use | `<prod-compartment>` |
| Operator | `<rms-operators>` | manage | `<prod-compartment>` |

## Baseline read policy (auditors — no mutation)

```
Allow group <rms-auditors> to inspect orm-stacks in compartment <prod-compartment>
Allow group <rms-auditors> to read orm-stacks in compartment <prod-compartment>
Allow group <rms-auditors> to inspect orm-jobs in compartment <prod-compartment>
Allow group <rms-auditors> to read orm-jobs in compartment <prod-compartment>
```

## Plan-only policy (create plan jobs, cannot apply or destroy)

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

## Service-principal policies (Resource Manager service itself)

```
Allow service ResourceManager to manage orm-stacks in compartment <prod-compartment>
Allow service ResourceManager to read secret-family in compartment <prod-compartment>
Allow service ResourceManager to use tag-namespaces in tenancy
```

Add resource-type rights for whatever the stack provisions, e.g.
`Allow service ResourceManager to manage instance-family in compartment <X>`.
Do not grant `manage all-resources` even to the service principal.

## Platform concurrency note

OCI Resource Manager allows only one running job at a time per stack.
This is platform-enforced — no additional concurrency control is needed.

## Do not use

```
# FORBIDDEN
Allow any-user to manage all-resources in tenancy
Allow group <rms-operators> to manage all-resources in compartment prod
```
