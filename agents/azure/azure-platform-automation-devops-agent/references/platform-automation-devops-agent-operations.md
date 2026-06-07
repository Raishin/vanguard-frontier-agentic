# Azure Platform Automation DevOps operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

Automation does not make platform change safe. The safe pattern separates plan or what-if identities from apply identities, forces human review for production, and treats drift as evidence, not noise.

## Officially grounded service shape

Microsoft guidance emphasizes IaC for landing-zone updates, read-only identities for plan and what-if, human approval for production apply, governed pipelines, reusable code across environments, and drift detection. That is the key insight: the pipeline is a control plane, not a convenience wrapper.

## Non-negotiable design rules

1. Separate bootstrap, plan, and apply responsibilities.
2. Use read-only plan or what-if evidence before production writes.
3. Require human approval and rollback evidence for production-impacting stages.
4. Keep secrets out of repository and pipeline definitions.
5. Treat out-of-band changes as drift that must be reconciled or consciously accepted.

## Minimal safe implementation flow

1. Classify platform, landing-zone, application, or emergency automation scope.
2. Review IaC source, pipeline identity, environment separation, and approval boundaries.
3. Ground Azure deployment behavior in Microsoft Learn and label it documentation-based.
4. Check preview output, validation gates, drift posture, secret handling, and rollback target.
5. Return blockers and the smallest safe pipeline or IaC change.

## High-risk assumptions to kill

- CI/CD approval means the change is safe.
- Plan and apply can safely share the same privileged identity.
- Portal hotfixes can stay outside IaC indefinitely.
- A successful deployment proves policy and security compliance.

## Safe command/code verification targets

- Bicep what-if, ARM validation, Terraform plan, static checks, and policy compliance results.
- Pipeline identity scopes and separation between preview and apply.
- Environment promotion path, approvals, rollback stage, and drift reconciliation evidence.

## When to push back

- Production apply lacks human review.
- Pipeline identity is broader than the target scope.
- Secrets are embedded in IaC or pipeline variables.
- Emergency change has no backlog item to bring state back to code.
