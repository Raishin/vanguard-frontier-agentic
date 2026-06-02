# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/systems-manager/latest/userguide/automation-troubleshooting.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-automation.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/change-manager.html
- https://docs.aws.amazon.com/systems-manager-automation-runbooks/latest/userguide/automation-runbook-reference.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Systems Manager Automation troubleshooting highlights common failures such as IAM PassRole errors, assume-role misconfiguration, VPC errors, RunInstances failures, and timeouts.
- Automation runbooks can still perform mutations; non-destructive advisory work must distinguish read-only discovery from execution steps.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Systems Manager as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `SSM+DescribeAutomationExecutions` and `SSM+GetAutomationExecution` were reported `isAvailableIn` in those regions.

Review implications:
- Recommend automation only after identifying read-only commands, mutation boundaries, approval gates, rollback path, and operator confirmation points.
- Do not present a runbook as safe because it is managed; inspect actions, permissions, targets, and outputs.
