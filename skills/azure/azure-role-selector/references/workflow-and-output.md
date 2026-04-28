# Workflow and Output Contract

## Workflow

1. Parse the request exactly.
   - Identify the target resource type and exact operations.
   - Identify principal type: user, group, service principal, managed identity, or workload identity.
   - Identify whether the ask is permanent, temporary, human, or machine access.
2. Separate permission planes.
   - Control plane: Azure Resource Manager actions such as create, update, delete, deploy, read configuration, or assign policy.
   - Data plane: service data actions such as reading blobs, secrets, queues, tables, or database data.
   - If the user mixes both, keep them separate in the answer.
3. Minimize scope before role selection.
   - Prefer resource scope over resource-group scope.
   - Prefer resource-group scope over subscription scope.
   - Prefer subscription scope over management-group scope.
   - Treat broad inherited scope as a risk that must be justified.
4. Prefer the narrowest built-in role.
   - Check whether a Microsoft built-in role already matches the required actions.
   - Reject habitual escalation to `Owner`, `Contributor`, or other broad roles unless the requested actions truly require them.
   - If a role includes meaningful extra privilege, say so explicitly.
5. Decide whether custom role fallback is justified.
   - Only consider a custom role when no built-in role safely matches the needed actions.
   - State which required actions are missing from the closest built-in role.
   - Keep custom-role scope and assignable scopes narrow.
   - Avoid wildcard-heavy custom roles unless the user has explicitly accepted the blast radius.
   - If the design requires `DataActions`, remember that Microsoft documents these custom roles as not assignable at management-group scope.
6. Recommend assignment scope.
   - Return the lowest workable scope.
   - Call out inherited access risk if the user asks for a broader scope than needed.
   - If the ask spans multiple resources, say whether one shared scope is acceptable or whether split assignments are safer.
7. Define the validation path.
   - Validate the role definition or built-in role against the requested actions.
   - Validate the assignment at the chosen scope.
   - Validate the real task with a bounded operator test instead of assuming the grant works.

## Output contract

Return all of the following:

- requested access summary,
- control-plane needs,
- data-plane needs,
- recommended built-in role first, or explicit custom-role fallback rationale,
- recommended assignment scope,
- validation path,
- risks, assumptions, and any missing facts.

Use this response shape:

```text
Requested access
- <principal> needs <actions> on <resource>

Plane split
- Control plane: ...
- Data plane: ...

Recommended role
- Built-in role: <name> at <scope>
- Why: <why it fits>
- Gaps or excess privilege: <if any>

Custom role fallback
- Needed: yes|no
- Why: <only if built-in roles do not fit>

Validation path
- Confirm role definition
- Confirm assignment scope
- Perform bounded task test

Risks and assumptions
- ...
```

## Eval gate

Treat the skill output as failing if any of the following are missing:

- the requested actions were not parsed into concrete operations,
- control plane and data plane were not separated,
- a built-in role search was skipped,
- a custom role was suggested without stating why built-in roles failed,
- the assignment scope was omitted,
- the validation path did not include an actual bounded verification step,
- risks or assumptions were omitted when facts are incomplete.

Minimum scenarios this skill should handle:

1. read-only storage access,
2. narrow deploy-oriented application access,
3. custom-role fallback when built-in roles are too broad or incomplete.

## Safety notes

- Do not recommend `Owner` for routine operations.
- Do not recommend `Contributor` by default for application onboarding.
- Do not blur Azure control-plane RBAC with service data-plane permissions.
- Do not suggest management-group or subscription-wide grants unless the blast radius is explicitly justified.
- Do not invent custom roles when a built-in role is already close enough and safer.
- Do not claim least privilege if the answer has not identified excess privilege, missing facts, or validation steps.
