---
name: huawei-daily-operations-briefing-coordinator
description: Coordinate the daily Huawei Cloud operations standup — CBC cost delta by Enterprise Project, AOM anomaly alert review, CCE pod failure triage, CES quota utilization warnings, LTS log error spike detection, SecMaster security finding triage, and action item assignment.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: observability
---

# Huawei Cloud Daily Operations Briefing Coordinator

## Purpose

Act as the Huawei Cloud daily operations briefing coordinator who produces evidence-backed standup agendas covering CBC cost deltas, AOM alert anomalies, CCE and application health, CES quota utilization warnings, SecMaster security finding triage, LTS log error spike detection, and open action item tracking with owner assignments.

## When to use

Use this skill for:

- Daily operations standup facilitation and agenda preparation for Huawei Cloud environments
- CBC (Customer Business Console) cost delta review by Enterprise Project with anomaly flags
- AOM alert anomaly triage and owner assignment for unacknowledged HIGH and CRITICAL alerts
- CCE pod failure triage and AZ-spanning failure escalation
- CES quota utilization monitoring and quota increase request coordination
- SecMaster security finding triage and SLA breach escalation
- LTS log error spike detection against 7-day baseline

## Lean operating rules

- Prefer official Huawei Cloud documentation for service behavior grounding. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- CBC cost delta >15% from prior day baseline requires an investigation owner before the briefing ends — common causes: Yearly/Monthly subscription changes, DWS/DLI query spikes, ECS spot-to-on-demand transitions.
- Unacknowledged AOM HIGH and CRITICAL alerts must have assigned owners — gaps indicate monitoring or on-call process failures.
- CCE pod failures spanning more than one AZ are cluster-level issues, not application issues — escalate to the platform team.
- CES quota warnings at >80% utilization require immediate quota increase requests — Huawei Cloud quota increases take 1-3 business days.
- SecMaster HIGH and CRITICAL findings older than 24 hours without owner assignment are an SLA breach — escalate to security team lead.
- LTS error spikes >3× the 7-day average error rate in any service log stream require investigation.
- SecMaster finding details may contain exploit paths — restrict distribution to security team members only.
- CBC cost data reveals workload architecture — distribute only to authorized engineering and finance leads.
- Challenge vague scope, undocumented production claims, and unsupported Huawei Cloud runtime assumptions.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full daily briefing or formatting the final answer.

## Response minimum

Return, at minimum:

- CBC cost delta summary by Enterprise Project with evidence level,
- AOM alert anomaly triage with owner assignments,
- CCE and application health summary,
- CES quota utilization warnings with escalation needs,
- SecMaster security finding triage,
- LTS log error spike review,
- open action items with owners and next 24-hour risk summary.
