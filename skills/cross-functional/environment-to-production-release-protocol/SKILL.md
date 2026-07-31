---
name: environment-to-production-release-protocol
description: Use this skill when a Power Platform or Dynamics 365 solution must progress through a structured dev-to-test-to-production release pipeline using managed solutions and Power Platform pipelines, when rollback readiness must be verified before go-live, or when a deployment approval gate must be enforced. Defines the full ALM release flow — solution packaging, pipeline stage progression, pre-deployment validation, approval, deployment, rollback verification, and post-deployment confirmation. Does not authorize production deployments directly; all production-impacting actions require human approval from the environment owner or release manager. Does not replace a qualified Power Platform admin or ALM specialist.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: delivery
  lifecycle: experimental
---

# Environment-to-Production Release Protocol

## Purpose
This skill defines how Power Platform and Dynamics 365 solutions are packaged, validated, and promoted from development through test to production using managed solutions and Power Platform pipelines. It enforces the principle that only managed solutions reach production environments, that every deployment stage is preceded by pre-flight validation, and that a tested rollback path exists before a production deployment is approved. No agent authorizes a production deployment; that is a human decision by the environment owner or release manager.

## When to use
- A solution in a development environment is ready to be promoted to test or production via a Power Platform pipeline.
- A release manager needs a structured checklist and gate review before approving a production deployment.
- A rollback plan must be confirmed and tested before a go-live window opens.
- A Copilot Studio agent or Power Automate flow packaged in a solution needs to be promoted through ALM stages.
- A deployment failed and a structured rollback or recovery must be initiated.

## When NOT to use
- The solution has not been exported as a managed solution — unmanaged solutions must not be deployed to test or production.
- The pipeline has not been configured by an admin in the Power Platform admin center — ad hoc deployments bypass this protocol.
- The matter is a hotfix to an already-live production environment requiring emergency change procedures — use the change-request-to-go-live-protocol with expedited gates instead.
- The deployment targets an environment outside Power Platform (e.g., pure Azure infrastructure) — this protocol does not cover Azure DevOps pipelines for non-Power Platform workloads.

## Participating agents
- `power-platform-alm-pipelines-agent` — primary: validates solution packaging, pipeline stage configuration, dependency checks, and deployment readiness
- `copilot-studio-agent-governance-alm-agent` — secondary: validates Copilot Studio agent configurations, topic schemas, and governance controls packaged in the solution

## Inputs required
- Solution name and version (current and new)
- Target pipeline and stage ID (from Power Platform pipelines configuration)
- Development environment reference
- Target environment reference (test or production)
- Connection references and environment variable values for target environment
- Rollback plan or prior deployment artifact reference

## Evidence required
- Power Platform pipeline configuration (pipeline name, stages, environments)
- Solution checker results (no critical violations)
- Connection reference and environment variable configuration status for target environment
- Prior deployment history (if available) from the Power Platform admin center deployment hub
- Approval records from prior pipeline stages (test must be deployed before production)

## Workflow

1. **Validate solution packaging** — confirm the solution is exported as managed; confirm the version is incremented; confirm no unmanaged layers will be left in the target environment.
2. **Run pre-flight checks** — execute Power Platform solution checker against the solution; flag any critical or high-severity violations; confirm all solution dependencies are present in the target environment.
3. **Verify pipeline stage order** — confirm that the solution has been deployed to all prerequisite stages (e.g., test before production) per the pipeline configuration; pipelines enforce stage order and the same solution version is promoted.
4. **Validate connection references and environment variables** — confirm all connection references are configured and valid in the target environment; confirm environment variable values are set; flag any missing configurations.
5. **Confirm rollback artifact** — verify that a prior managed solution version (or unmanaged recovery artifact) is available and documented in the deployment history; confirm rollback steps are written and owner is identified.
6. **Escalation gate: managed-solution-only in production** — if the solution is unmanaged or contains unmanaged layers targeting production, stop and refuse; escalate to Power Platform admin.
7. **Escalation gate: rollback tested** — confirm rollback procedure has been rehearsed or can be executed from deployment history; if untested, flag as risk item and require release manager acknowledgment before proceeding.
8. **Escalation gate: approval** — require human approval from environment owner or release manager before deploying to production; record approval reference.
9. **Initiate pipeline deployment** — invoke the pipeline deployment for the confirmed stage; monitor deployment status (in-progress, succeeded, failed, canceled).
10. **Post-deployment validation** — confirm solution import succeeded; validate connection references are live; run smoke tests or confirm with functional owner that critical paths are working.
11. **Hypercare confirmation** — confirm hypercare period start; identify support owner for post-deployment issues; schedule post-deployment review.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Managed-solution-only | Solution is unmanaged OR contains unmanaged layers targeting production | Stop; refuse; escalate to Power Platform admin |
| Rollback tested | Rollback plan is undocumented or untested | Flag; require release manager acknowledgment; do not deploy until acknowledged |
| Approval | Human approval from environment owner or release manager not on record | Hold; do not initiate deployment; escalate to release manager |
| Pre-flight failures | Solution checker returns critical violations OR dependencies missing | Stop; return to development for remediation |
| Stage order | Test stage not successfully completed before production stage | Stop; enforce stage order per pipeline configuration |

## Refusal triggers
- A request is made to bypass pipeline stage order and deploy directly to production — refuse; enforce stage order.
- A request is made to deploy an unmanaged solution to a production or test environment — refuse; only managed solutions may be deployed to non-development environments.
- Credentials, service principal secrets, or tenant IDs are requested to initiate or validate a deployment — refuse; work from sanitized configuration signals only.
- A rollback path does not exist and the release manager has not acknowledged the risk — refuse to proceed until acknowledged.

## Handoff rules
- Every handoff carries: solution name, version, pipeline name, stage, deployment status, pre-flight results, connection reference status, rollback plan reference, approval reference, open questions, and a do-not-do list.
- No agent triggers a production deployment without a recorded human approval.
- Post-deployment, the primary agent confirms deployment success and hands off to the functional owner for hypercare.

## KPIs
- Percentage of production deployments that pass all gates without a manual override
- Number of deployment failures requiring rollback
- Time from solution packaging to production deployment approval
- Rollback execution time (from failure detection to rollback completion)

## References
- [Overview of pipelines in Power Platform](https://learn.microsoft.com/power-platform/alm/pipelines)
- [Run pipelines in Power Platform](https://learn.microsoft.com/power-platform/alm/run-pipeline)
- [Admin deployment page in Power Platform](https://learn.microsoft.com/power-platform/alm/admin-deployment-hub)
- [Recommendations for standardizing tools and processes — Power Platform ALM](https://learn.microsoft.com/power-platform/well-architected/operational-excellence/tools-processes)
