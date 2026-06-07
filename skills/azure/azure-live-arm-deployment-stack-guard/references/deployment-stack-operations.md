# Azure ARM Deployment Stack Operations

Use this reference for current, source-grounded service behavior and the hard live-operation gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating what-if as optional because the template is in source control.
- Using deleteAll or deleteResources without listing every managed resource and dependency.
- Ignoring deny settings exclusions and assuming they protect data-plane child objects.
- Bypassing stack-out-of-sync warnings by default.
- Claiming rollback when the change deletes or mutates stateful resources.

## Officially grounded service shape

Microsoft Learn evidence says Deployment Stacks manage a group of resources as one unit, can detach or delete resources removed from the template through actionOnUnmanage, and can protect managed resources with deny settings. Documentation also calls out limitations: stacks do not manage implicitly created resources, deny settings apply to control plane only, some portal support is absent, and out-of-sync warnings require managed-resource review before bypass.

- ARM/Bicep deployments and Deployment Stacks are live control-plane changes.
- Deployment Stacks add lifecycle ownership, managed-resource tracking, action-on-unmanage behavior, and deny settings.
- Deny settings can block write/delete control-plane actions but not all data-plane operations or implicit resources.
- Action-on-unmanage determines whether removed template resources detach or delete.
- Deployment-stack commands can create or update existing stacks, so idempotency assumptions must be verified.

## Non-negotiable design rules

- Confirm deployment scope: resource group, subscription, or management group.
- Require exact template, parameters, target scope, and current stack resource list before mutation.
- Default action-on-unmanage to detach unless deletion is explicitly approved and proven safe.
- Review deny-settings mode, child-scope behavior, excluded actions, and excluded principals.
- Never bypass out-of-sync warnings without reviewing managed resources.

## Minimal safe implementation flow

- Scope deployment, stack, template, parameters, active principal, and approval owner.
- Collect what-if output where supported, existing deployments, stack resources, deny settings, locks, and stateful resources.
- Classify each change as create, modify, delete, detach, or protection change.
- Gate execution on rollback limitations and explicit approval.
- After action, verify provisioning state, managed-resource list, deny settings, and drift indicators.

## High-risk assumptions to kill

- Source-controlled Bicep does not prove safe live state; deployment stacks can delete, detach, or lock resources based on current managed-resource tracking.
- Deny settings are not a universal lock. Microsoft Learn documents control-plane limits, implicit-resource gaps, data-plane exclusions, and current portal limitations.
- `deleteAll` or `deleteResources` is not cleanup; it is destructive lifecycle ownership and needs an explicit managed-resource inventory.
- A stack-out-of-sync bypass is not a normal retry flag. It is only defensible after reviewing the managed resources the service believes it controls.
- Rollback is not credible for deleted resource groups, networking, identity, Key Vault, or stateful resources without separate recovery evidence.

## Safe command/code verification targets

- Capture stack scope, template source, parameters, deny settings, action-on-unmanage, and managed-resource list before mutation.
- Run what-if or an equivalent reviewed diff where supported; if unavailable, label that gap and require stronger human approval.
- Search proposed commands/templates for destructive flags and deny-setting exclusions before execution.
- Verify provisioning state, synchronized managed-resource list, deny assignments, and locks after action.
- Treat bypass flags and delete flags as high-risk findings unless approval and recovery evidence are attached.

## Safe verification targets

- What-if or equivalent managed-resource diff is attached and reviewed.
- No unmanaged deletion is hidden behind action-on-unmanage.
- Deny settings match intended protection and do not lock out operators unexpectedly.
- Stateful resources have backup/restore or detach-first plan.
- Stack status and managed-resource list are synchronized after change.

## When to push back

- The user asks to run deployment without what-if or equivalent diff.
- Template provenance or parameters are unknown.
- The change deletes resource groups, Key Vault objects, databases, networking, or identity resources without recovery evidence.
- A bypass flag is requested as routine behavior.
