# Workflow and Output Contract

## Workflow

1. Identify the governing hierarchy first:
   - tenant root management group,
   - intermediate management groups,
   - subscriptions,
   - resource groups,
   - exceptional resources that may need carve-outs.
2. Classify the requested control:
   - audit-only visibility,
   - `deny` prevention,
   - `modify` mutation,
   - `deployIfNotExists` deployment/remediation,
   - initiative bundling for repeated baseline controls.
3. Decide whether the control belongs in:
   - a single policy definition,
   - an initiative for baseline packaging,
   - an existing landing-zone baseline,
   - or not in policy at all because the ask is process-only or too brittle.
4. Choose assignment scope deliberately:
   - prefer the highest scope that matches the real control boundary,
   - do not assign at broad scope by habit,
   - verify inheritance impact on child subscriptions and resource groups,
   - call out when management-group placement is justified versus excessive.
5. Design exclusions and exemptions separately:
   - exclusions for scope carve-outs,
   - exemptions for approved exception handling,
   - narrow both by resource type, location, or defined exception boundary where possible.
6. Evaluate guardrail content explicitly for common governance cases:
   - required tags and tag value standards,
   - allowed locations,
   - allowed resource types,
   - allowed or denied SKUs where built-in policy coverage exists,
   - baseline initiatives that bundle related controls.
7. Challenge remediation and mutation risk before recommending enforcement:
   - `modify` and `deployIfNotExists` need managed identity, permissions, and rollback thought,
   - remediation can change existing resources,
   - deny can block live deployment paths if staged badly.
   - remember that assignment at management-group scope still evaluates subscription/resource-group resources; do not imply magical tenant-object coverage.
8. Recommend rollout sequencing:
   - observe with audit first when facts are incomplete,
   - pilot on a lower, representative scope,
   - measure non-compliance and exception volume,
   - then tighten to enforce where justified.
9. State the rollback and exception path:
   - remove or disable the assignment,
   - narrow scope,
   - replace deny with audit temporarily,
   - use time-bounded exemptions instead of permanent policy erosion.

## Output contract

Return:

- current governance summary,
- target control objective,
- recommended policy versus initiative shape,
- assignment scope recommendation and inheritance impact,
- exclusion and exemption strategy,
- remediation or mutation risk,
- staged rollout plan,
- rollback or exception path,
- assumptions, missing facts, and evidence used.

## Eval gate

Treat the answer as incomplete unless it does all of the following:

- identifies the actual governing scope,
- separates audit, deny, modify, and remediation concerns,
- recommends assignment placement instead of hand-waving “use policy,”
- addresses exclusions or exemptions for brownfield reality,
- flags rollout risk for deny or remediation effects,
- gives enforceable guardrails for tags, regions, SKUs, or baseline initiatives when those are in scope.

Fail the response if it recommends root-scope sprawl, ignores inheritance, or proposes enforcement without change-safety notes.

## Safety notes

- Do not recommend tenant-root or broad management-group assignments without explicit blast-radius justification.
- Do not recommend `deny`, `modify`, or `deployIfNotExists` as a default first move in production.
- Do not hide remediation side effects; existing resources may be changed or left non-compliant depending on policy effect.
- Do not treat exclusions as a dumping ground for weak design; prefer narrow, accountable exceptions.
- Do not claim governance is solved by policy alone; ownership, operating process, and lifecycle updates still matter.
