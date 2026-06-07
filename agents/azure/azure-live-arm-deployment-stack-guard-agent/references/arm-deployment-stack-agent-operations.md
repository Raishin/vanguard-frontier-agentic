# Azure Live ARM Deployment Stack Guard Agent Operations

> Version note: Azure Resource Manager, Bicep, deployment stack behavior, deny-setting enforcement, and CLI/PowerShell support change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, parameter files containing secrets, private keys, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating Deployment Stacks as ordinary deployments while ignoring managed-resource deletion and detach behavior.
- Running stack updates without reviewing `actionOnUnmanage`, which can detach or delete resources removed from the template.
- Assuming deny settings protect every resource; they only apply to supported explicitly managed control-plane resources.
- Bypassing stack out-of-sync warnings without proving the managed resource list is accurate.
- Using what-if output as approval instead of as input to a separate human approval gate.

## Officially grounded service shape

- Deployment Stacks manage a group of Azure resources as a single unit from Bicep or ARM templates.
- Removing a resource from the template makes it unmanaged; `actionOnUnmanage` controls whether Azure detaches or deletes unmanaged resources.
- Deny settings can block delete or write/delete operations for managed resources, but limitations exist for implicit resources, data-plane resources, tags, management-group deny assignments, and Key Vault secrets.
- Azure CLI and PowerShell expose stack create/update/delete flows with deny-settings and action-on-unmanage options.
- Microsoft Learn warns not to bypass stack out-of-sync errors until the managed resource list has been thoroughly reviewed.

That is the key insight:

> The agent is a live infrastructure mutation gate. It must prove target scope, what-if or preview evidence, deny settings, unmanaged-resource behavior, and rollback/detach posture before allowing execution.

## Non-negotiable design rules

### 1. Never execute a deployment or stack update without target-scope confirmation, preview evidence, impact summary, approval, and rollback posture.

### 2. Treat `deleteAll`, `deleteResources`, `detachAll`, deny settings, excluded principals, and bypass flags as high-risk controls.

### 3. Block execution when stack managed-resource inventory is unknown or stack out-of-sync warnings are unresolved.

### 4. Prefer read-only template validation, what-if, stack show/list/export, and policy/RBAC evidence before mutation.

### 5. Label configured-environment observations as sampled and bounded to the scope, stack, and time window.

## Minimal safe implementation flow

- Confirm target scope, deployment or stack name, template source, parameter source, desired action, approval state, and rollback owner.
- Ground behavior in Microsoft Learn ARM what-if and Deployment Stacks guidance.
- Collect read-only evidence for what-if diff, managed resources, deny settings, action-on-unmanage, excluded principals/actions, locks, policy conflicts, and recent deployments.
- Decide: validate, update, detach, delete, or block; if action is live, require explicit human approval.
- Verify post-action deployment state, managed-resource inventory, deny settings, and open risks.

## High-risk assumptions to kill

- A successful what-if means the change is approved.
- Deny settings protect implicit resources and data-plane children.
- Detach is always safer than delete, or delete is always intended.
- Stack warnings can be bypassed because the template looks right.
- Documentation proves this scope's managed resource list, RBAC, or production readiness.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Target management group, subscription context, resource group, deployment/stack name, location, and template source.
- What-if output, deployment validation output, managed-resource list, existing stack state, denied operations, excluded principals/actions, and child-scope application.
- `actionOnUnmanage` mode, resource deletion or detach candidates, Key Vault secret handling, implicit resource limitations, and stack out-of-sync status.
- Rollback template reference, parameter reference, detach plan, lock interaction, policy conflicts, and post-deploy verification commands.
- Approval record, impact summary, maintenance window, and verification evidence.

## When to push back

- The target scope, template source, parameter source, approval state, or rollback owner is ambiguous.
- The plan includes delete behavior without named resources and explicit approval.
- The user wants to paste credentials, secret-bearing parameters, or raw environment dumps.
- The requested action would bypass warnings or mutate live state without preview and rollback evidence.
