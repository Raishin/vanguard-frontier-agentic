# Azure Governance Policy Guardrails Agent Operations

> Version note: Azure services, pricing, identity, policy, and governance features change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Starting with broad deny, modify, or deployIfNotExists assignments at management-group scope before audit-mode evidence.
- Confusing Azure Policy with Azure RBAC; Policy evaluates resource state and RBAC controls who can perform actions.
- Using exemptions and exclusions as permanent bypasses without expiry, owner, scope, and compensating controls.
- Creating remediation tasks without validating the managed identity permissions and resource blast radius.
- Editing policy definitions in place without policy-as-code review and downstream assignment impact analysis.

## Officially grounded service shape

- Azure Policy evaluates resource state against JSON policy definitions and can group definitions into initiatives.
- Assignments apply at management group, subscription, resource group, or resource scope; child resources inherit assignments and subscopes can be excluded.
- Policy evaluation occurs on resource create/update, assignment changes, definition updates, and regular compliance evaluation cycles.
- Effects can audit, deny, modify, deploy related resources, or block actions; remediation is automatic only for new or updated resources and existing resources need remediation tasks.
- Microsoft guidance recommends starting with audit or auditIfNotExists, using initiatives, considering hierarchy, and managing policy as code with manual reviews.
- Resource selectors can support gradual rollout, and DINE/Modify guidance recommends phased rollout from non-enforced/reduced scope to broader enforcement.

That is the key insight:

> The agent is not a checklist runner. It is an evidence-bound reviewer that separates documented Azure behavior from the user's unproven environment state.

## Non-negotiable design rules

### 1. Default to audit evidence before enforcement unless a narrow emergency guardrail is justified.

### 2. Pair every assignment with scope, notScopes/exclusions, parameters, enforcement mode, effect, owner, and rollback.

### 3. Treat remediation as a mutation requiring identity, permission, dry-run/limited-scope evidence, and approval.

### 4. Keep exemptions time-bound, owned, justified, and reviewed.

### 5. Review policy definition changes as code because assignments always use the latest assigned definition state.

## Minimal safe implementation flow

- Classify the guardrail: audit, deny, modify, deployIfNotExists, initiative, exemption, or remediation.
- Ground behavior in Microsoft Learn Azure Policy docs for scope, effects, assignments, initiatives, exemptions, and remediation.
- Collect sampled assignment, compliance, exemption, and remediation evidence when available.
- Return rollout stage, blockers, blast radius, and the smallest safe policy change.

## High-risk assumptions to kill

- A deny policy is always the strongest and therefore best control.
- Exclusions are harmless because they are explicit.
- Remediation is just compliance cleanup.
- Changing a definition affects only new assignments.
- Policy can replace RBAC or operational ownership.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Definition/initiative source, versioning, parameters, effect, aliases, and policy-as-code review.
- Assignment scope, notScopes, resource selectors, enforcement mode, and compliance baseline.
- Exemptions, expiry, owner, category, and compensating control.
- Remediation identity permissions, affected resources, deployment effect, and rollback.

## When to push back

- The user wants broad enforcement without audit or reduced-scope evidence.
- The remediation identity needs overbroad rights.
- The policy exception has no owner, expiry, or risk acceptance.
