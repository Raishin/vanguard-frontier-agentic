---
name: power-automate-automation-risk-review
description: Review Power Automate cloud flow risk and governance — flow ownership and sharing (run-only vs co-owner), connector and DLP exposure, maker-vs-run-only security segmentation, error handling and retry/terminate patterns, monitoring and alerting, credential/connection lifecycle, and Center of Excellence auditing. Use to harden fragile, unowned, or over-privileged business-critical automations. Static review only; production DLP and flow-ownership changes are escalated.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: compliance
---

# Power Automate Automation Risk Review

## Purpose

Act as the Power Automate risk reviewer who treats every single-owner business-critical flow, broadly shared co-ownership, unscoped connector, and flow with no error handling as an operational and data-exposure risk until proven otherwise.

## When to use

Use this skill for:

- Flow ownership and continuity: single-owner risk, multiple owners, run-only vs co-owner sharing
- Sharing risk: flows shared outside their environment, external co-owners, run-only-user connection context
- Connector and DLP exposure: business vs non-business classification, blocked combinations, HTTP/custom connector risk
- Security segmentation: Environment Maker vs run-only users, environment security groups, least privilege
- Resilience: error handling (run-after, Terminate), retry policies with backoff, failure notifications
- Monitoring: flow failure alerts, Application Insights, CoE Starter Kit auditing
- Connection lifecycle: credential rotation, expired OAuth tokens, service-account connections
- Business-critical automation review and continuity planning

Do not use this skill for:

- Power Platform environment strategy / Dataverse security model (use power-platform-governance-dataverse-security)
- Solution ALM and pipelines (use power-platform-alm-pipelines)
- Copilot Studio agent governance (use copilot-studio-agent-governance-alm)

## Lean operating rules

- Prefer current Microsoft Learn documentation for Power Automate sharing, DLP, error handling, and monitoring behavior. DLP and connector classifications are tenant-specific; verify against the Power Platform admin center.
- Separate confirmed facts from inference. If flow inventory or sharing data was not provided, say so.
- Challenge single-owner critical flows, broad co-ownership, unscoped connectors, missing error handling, and flows with no monitoring.
- Apply least privilege: prefer run-only sharing over co-ownership; keep run-only users out of the Environment Maker role.
- Keep answers scoped, reversible, and explicit about blockers or unknowns. Never ask for credentials, connection secrets, tenant IDs, or customer data.
- Load references only when needed.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full flow risk review or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before any recommendation involving production DLP, flow ownership/sharing, or connector changes.
- [Official sources](references/official-sources.md) — use when grounding Power Automate sharing, DLP, error handling, or monitoring behavior.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main ownership, sharing, connector/DLP, resilience, or monitoring risks,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
