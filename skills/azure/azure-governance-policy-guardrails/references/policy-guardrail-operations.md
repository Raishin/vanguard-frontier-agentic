# Azure Policy guardrail operations

## What people get wrong

- They use Azure Policy as a workload deployment mechanism instead of a governance and compliance mechanism.
- They roll out deny or modify at management-group scope before seeing audit impact.
- They forget that policy assignments inherit and that explicit deny requires changing or excluding the denying assignment.
- They run remediation without checking the managed identity permissions and affected resources.
- They treat exemptions as permanent fixes instead of governed exceptions.

## Officially grounded service shape

Microsoft Learn describes Azure Policy as a service for enforcing organizational standards and assessing compliance at scale. Definitions can be grouped into initiatives and assigned to management group, subscription, resource group, or resource scopes. Effects such as audit, deny, modify, and deployIfNotExists behave differently and evaluate at different times. DINE and Modify remediation require managed identities with enough permissions. Microsoft guidance recommends starting with audit/auditIfNotExists and using staged rollout patterns for DINE/Modify controls.

## Non-negotiable design rules

1. Define business objective, target resource types, and scope before choosing an effect.
2. Prefer audit or DoNotEnforce/canary rollout before deny, modify, or DINE at broad scope.
3. Review inheritance, exclusions, exemptions, and explicit-deny behavior.
4. Use initiatives for related controls and parameter consistency.
5. Grant remediation identities only the permissions required by the policy.
6. Give exemptions an owner, reason, category, expiration, and review process.
7. Manage policy definitions, initiatives, and assignments as code with review.

## Minimal safe implementation flow

1. Draft policy or initiative and map each effect to desired behavior.
2. Assign at narrow canary scope with audit or enforcement disabled when practical.
3. Review compliance state, noncompliance causes, false positives, and pipeline failures.
4. Validate managed identity permissions for DINE/Modify remediation.
5. Define exemptions and notScopes with expiry and ownership.
6. Move to enforcement in stages by scope, resource selector, or management group path.
7. Monitor compliance, remediation failures, and deployment impact after rollout.

## High-risk assumptions to kill

- Broad-scope deny is dangerous without audit impact, false-positive review, and a rollback path.
- `modify` and `deployIfNotExists` are mutation paths; remediation identity permissions and affected resources must be reviewed before rollout.
- Exemptions and `notScopes` can make compliance look better than reality if ownership, reason, category, and expiry are missing.
- Assignment inheritance means a resource can be blocked by a parent policy even when local scope looks clean.
- Azure Policy should not be used as a substitute for application deployment orchestration or configuration management.

## Safe command/code verification targets

- Inspect policy and initiative JSON for mode, aliases, parameters, effect, effect overrides, definition versions, and resource selectors.
- Review assignment files for scope, enforcement mode, non-compliance messages, `notScopes`, exemptions, and staged rollout tiers.
- Check remediation definitions for managed identity type, roleDefinitionIds, least-privilege role assignments, resource filters, count, parallelism, and failure threshold.
- Verify CI/CD gates collect compliance results and fail when noncompliance, false positives, or application health impact diverges from expectations.
- Confirm rollback can disable or narrow assignment, revert definition/initiative version, stop remediation, or remove high-risk effects.

## Safe verification targets

- Policy definition mode, effect, aliases, parameters, and resource provider applicability.
- Initiative composition and parameter wiring.
- Assignment scope, notScopes, exemptions, enforcement mode, and resource selectors.
- Compliance states and noncompliance reasons.
- Remediation task settings, identity permissions, resource count, failure threshold, and deployment summary.
- Rollback plan: disable assignment, revert definition, reduce scope, or remove remediation.

## When to push back

Push back on broad deny without audit data, DINE/Modify without identity review, permanent exemptions, policy-as-deployment misuse, or compliance claims that ignore excluded and exempt resources.
