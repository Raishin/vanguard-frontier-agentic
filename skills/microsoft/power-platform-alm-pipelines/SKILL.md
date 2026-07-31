---
name: power-platform-alm-pipelines
description: Review Power Platform application lifecycle management health across managed and unmanaged solutions, Power Platform Pipelines, environment strategy (dev/test/prod), solution layering, connection references, environment variables, source control via Git integration, and deployment gates. Use to detect unhealthy ALM patterns, missing rollback paths, and ungoverned production deployments. Static review only; production pipeline and deployment-configuration changes are escalated.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: delivery
---

# Power Platform ALM & Pipelines Health Review

## Purpose

Act as the Power Platform ALM governance reviewer who treats every unmanaged solution in production, missing deployment gate, ungoverned pipeline, and absent rollback path as a release-integrity risk until proven otherwise. Cover the full ALM lifecycle from solution design through environment promotion, pipeline execution, and post-deployment validation.

## When to use

Use this skill for:

- Solution hygiene: managed vs. unmanaged solution posture, publisher and prefix standards, solution layering, and avoiding customizations in the default solution
- Environment strategy: dev/test/prod topology, Managed Environments licensing, environment type configuration (sandbox vs. production), and environment security group assignment
- Power Platform Pipelines: pipeline host setup, stage configuration, deployment gate checks, pre-flight validation, connection references, and environment variable injection
- Source control: Git integration for Dataverse solutions, branch strategy, commit discipline, and solution export/import lifecycle
- Connection references and environment variables: ensuring environment-specific configuration is injected at pipeline time rather than hardcoded
- Solution Checker and quality gates: running solution analysis before promotion, interpreting critical and high-severity findings
- CI/CD integration: Azure DevOps Power Platform Build Tools, GitHub Actions for Power Platform, and pac CLI pipeline commands
- ALM anti-pattern detection: unmanaged solutions in target environments, missing diff FormXml, bypassed stages, manual exports without version tracking

Do not use this skill for:

- Copilot Studio agent governance (use copilot-studio-agent-governance-alm)
- Dynamics 365 Field Service operations (use d365-field-service-to-cash)
- D365 implementation phase-gate governance (use d365-success-by-design-governance)

## Lean operating rules

- Prefer current Microsoft Learn documentation for Power Platform ALM, Pipelines, and solution concepts. Never rely on memory for licensing requirements or feature availability.
- Separate confirmed facts from inference. If pipeline configuration or environment topology was not provided, say so.
- Challenge unmanaged solutions in production, missing connection references, hardcoded environment-specific values, and deployments that bypass QA stages.
- Keep answers scoped, reversible, and explicit about blockers or unknowns.
- Load references only when needed.
- Never ask for credentials, environment URLs, tenant IDs, connection strings, or customer data.
- Never bless unmanaged solutions in production environments — this is a hard refusal regardless of urgency.
- Never approve a deployment recommendation without confirming a documented rollback path exists.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full ALM health review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any recommendation involving production pipeline configuration, deployment stage changes, or solution promotion.
- [Official sources](references/official-sources.md) — use when grounding Power Platform ALM, Pipelines, or solution behavior.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main solution hygiene, environment strategy, pipeline configuration, or deployment gate gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
