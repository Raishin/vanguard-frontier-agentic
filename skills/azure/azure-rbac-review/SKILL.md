---
name: azure-rbac-review
description: Use this skill for Azure RBAC, Entra-backed access, role assignment, custom role, scope, subscription, management group, or least-privilege review tasks. Trigger when the user asks whether Azure access is too broad or how to grant access safely.
metadata:
  author: github: Raishin
---

# Azure RBAC Review

## Purpose

Review Azure access decisions against least privilege, scope minimization, and operational safety.

## Workflow

1. Identify the target scope: management group, subscription, resource group, resource, or data plane.
2. Identify principal type: user, group, service principal, managed identity, workload identity, or application.
3. Prefer built-in roles with narrow scope before custom roles.
4. Challenge dangerous defaults:
   - `Owner` for routine operations,
   - `Contributor` at subscription scope,
   - `User Access Administrator` without strong governance,
   - custom roles with wildcard actions,
   - permanent assignments where time-bound access is appropriate.
5. Check whether data-plane permissions are separate from control-plane RBAC.

## Output

Return:

- current access summary,
- risk findings,
- least-privilege alternative,
- validation commands or portal checks,
- assumptions and missing facts.

## Security notes

Do not suggest broad tenant, management-group, or subscription access unless the user has explicitly justified the blast radius.
