# Workflow and output contract

Use this reference for full execution of `azure-platform-automation-devops`.

## Workflow

1. **Classify the request**
   - Identify service/domain, resource scope, environment, production impact, and whether mutation is requested.
   - Identify whether the task needs documentation-only guidance, sampled read-only current-state evidence, or sanitized user evidence.

2. **Ground in current sources**
   - Prefer Microsoft Learn documentation through the user's configured documentation MCP.
   - Read the component operations guide before issuing design, safety, or readiness conclusions.
   - Treat current-state claims as unproven unless supported by sampled read-only evidence or sanitized user-provided evidence.

3. **Stress-test the plan**
   - Kill broad permissions, vague ownership, missing rollback, missing validation, and unsupported production-readiness claims.
   - Separate facts from inference.
   - State blockers before recommendations.

4. **Recommend minimal safe action**
   - Prefer read-only inspection, preview, what-if, dry run, diagnostic query, or staged rollout before mutation.
   - Require explicit approval for live or destructive actions.
   - Keep the recommendation scoped and reversible where possible.

5. **Validate and hand off**
   - Name verification targets and evidence gaps.
   - Provide safe next actions and escalation criteria.
   - Do not claim tenant, subscription, resource, quota, or incident state that was not observed.

## Output contract

Return:

1. Scope and target
2. Evidence level: documentation-based, sampled read-only evidence, user-provided evidence, repo evidence, or inference
3. Key findings and risks
4. Blockers or missing evidence
5. Minimal safe next actions
6. Verification targets
7. Rollback, cleanup, or reversal path where applicable
