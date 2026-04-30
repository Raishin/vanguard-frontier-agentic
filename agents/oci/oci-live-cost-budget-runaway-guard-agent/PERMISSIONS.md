# Permissions: OCI Live Cost Budget Runaway Guard

# OCI IAM policy for cost budget runaway guard

## Identity model preference

1. Named cost-governance group with tenancy-scoped budget management
2. Separate cost-auditors (inspect/read only) from cost-admins (manage)
3. GPU provisioning gates via compartment quota policies — not IAM `manage`
4. Never grant `manage compute-instances in tenancy` to the cost-guard role

## Budget read (audit, no mutation)

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

OCI policy-based IAM supports tier separation by verb. Cost operators can
re-tune budget thresholds and notification rules without holding `manage`
delete rights:

```
Allow group <cost-operators> to use usage-budgets in tenancy
Allow group <cost-operators> to read costs in tenancy
Allow group <cost-operators> to use ons-topics in compartment <cost-alerts-compartment>
```

`use usage-budgets` permits update + alert rule changes; it does NOT permit
budget creation or deletion — those remain with `<cost-admins>`.

## Cost-tracking tag namespace management

```
Allow group <cost-admins> to manage tag-namespaces in compartment <cost-tracking-compartment>
Allow group <cost-admins> to use tag-namespaces in tenancy
```

## GPU/HPC shape gate via compartment quota (strongest control)

Set a compartment-level quota to prevent GPU provisioning without explicit increase:

```
set compute-core-count quota gpu-vm-count to 0 in compartment <default-compute>
```

This physically prevents any GPU shape from being provisioned without a quota
increase request — a harder gate than IAM deny policies.

## Do not use

```
# FORBIDDEN
# Allow group <cost-admins> to manage all-resources in tenancy  ← FORBIDDEN
Allow any-group to manage compute-instances in tenancy
Allow group <cost-admins> to manage compute-instances in tenancy
  # Cost guard should not have VM create/stop rights — escalate to compute operator
```

