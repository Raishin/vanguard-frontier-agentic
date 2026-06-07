# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/controltower/latest/userguide/what-is-control-tower.html
- https://docs.aws.amazon.com/controltower/latest/userguide/aws-multi-account-landing-zone.html
- https://docs.aws.amazon.com/organizations/latest/userguide/orgs_introduction.html
- https://docs.aws.amazon.com/controltower/latest/controlreference/control-reference.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Control Tower automates multi-account governance with controls, account provisioning, and drift prevention/detection around a landing zone.
- AWS multi-account landing-zone guidance emphasizes OU structure, workload isolation, and separating production from non-production environments.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Control Tower and AWS Organizations as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled product availability is not proof that a landing zone is enabled, healthy, drift-free, or compliant in the user's organization.

Review implications:
- Require evidence for organization structure, OUs/accounts, SCPs, delegated admin, controls/guardrails, identity center, logging/audit accounts, network boundaries, and drift.
- Do not infer landing-zone health from service availability; inspect Control Tower state, Organizations policy attachments, Config/CloudTrail coverage, and exceptions.
