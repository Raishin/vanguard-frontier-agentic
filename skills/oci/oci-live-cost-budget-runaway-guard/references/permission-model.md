# Permission Model: OCI Live Cost Budget Runaway Guard

## 3-tier separation

| Tier | Group | Verb | Scope |
|------|-------|------|-------|
| Audit | `<cost-auditors>` | inspect / read | tenancy |
| Operator | `<cost-operators>` | use | tenancy / compartment |
| Admin | `<cost-admins>` | manage | tenancy |

## Budget read (audit — no mutation)

```
Allow group <cost-auditors> to inspect usage-budgets in tenancy
Allow group <cost-auditors> to read usage-budgets in tenancy
Allow group <cost-auditors> to inspect costs in tenancy
Allow group <cost-auditors> to read costs in tenancy
```

## Budget write (manage — budgets are tenancy-scoped resources)

```
Allow group <cost-admins> to manage usage-budgets in tenancy
```

## Quota inspection and resource search

```
Allow group <cost-admins> to inspect quota in tenancy
Allow group <cost-admins> to read quota in tenancy
Allow group <cost-admins> to use resource-search in tenancy
```

## Cost operators (middle tier — adjust budgets, cannot delete)

`use usage-budgets` permits update + alert rule changes. It does NOT permit
budget creation or deletion — those remain with `<cost-admins>`.

```
Allow group <cost-operators> to use usage-budgets in tenancy
Allow group <cost-operators> to read costs in tenancy
Allow group <cost-operators> to use ons-topics in compartment <cost-alerts-compartment>
```

## GPU/HPC shape gate via compartment quota

```
set compute-core-count quota gpu-vm-count to 0 in compartment <default-compute>
```

This physically prevents GPU shape provisioning without a quota increase — a harder gate than IAM deny policies.

## Do not use

```
# FORBIDDEN
Allow group <cost-admins> to manage all-resources in tenancy
Allow group <cost-admins> to manage compute-instances in tenancy
```
