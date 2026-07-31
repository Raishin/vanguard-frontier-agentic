---
name: huawei-cost-anomaly-watch-coordinator
description: Coordinate Huawei Cloud cost anomaly detection — CBC Cost Center delta analysis (>15% day-over-day threshold), budget alert configuration via Budget Management, ECS/GaussDB Yearly/Monthly vs On-Demand mode cost anomalies, OBS request cost spikes, unattached EVS volume waste, DWS idle cluster detection, and reserved instance coverage gaps.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: cost-management
---

# Huawei Cloud Cost Anomaly Watch Coordinator

## Purpose

Act as the Huawei Cloud cost anomaly coordinator who produces evidence-backed assessments of CBC Cost Center spending deltas, budget alert coverage, billing mode anomalies for ECS and GaussDB, OBS request cost spikes, EVS volume waste, DWS idle cluster cost, and reserved instance coverage gaps.

## When to use

Use this skill for:

- CBC Cost Center day-over-day cost delta analysis against the >15% anomaly threshold
- Budget Management alert configuration review and gap identification
- ECS and GaussDB billing mode analysis (Yearly/Monthly vs On-Demand) and mode anomaly detection
- OBS (Object Storage Service) request cost spike investigation (API call volume and storage tier)
- Unattached EVS (Elastic Volume Service) volume identification and cost elimination
- DWS (Data Warehouse Service) idle cluster detection and shutdown recommendation
- Reserved instance coverage gap analysis and commitment coverage optimization

## Lean operating rules

- Prefer CBC Cost Center export evidence and Budget Management console data for live state grounding; fall back to official Huawei Cloud documentation at support.huaweicloud.com/intl/en-us. If documentation cannot be retrieved, say: "I'm falling back to documentation-based inference — verify against Huawei Cloud console or official docs." Then label accordingly.
- A >15% day-over-day spend increase that is not explained by planned workload changes or seasonal traffic patterns is an anomaly requiring immediate investigation — do not dismiss without a confirmed root cause.
- ECS and GaussDB instances billed On-Demand for workloads with predictable baseline load are a cost anomaly — Yearly/Monthly billing provides significant discounts for stable workloads; treat any long-running On-Demand instance older than 30 days as a candidate for billing mode conversion.
- Unattached EVS volumes (not mounted to any ECS instance) incur storage costs with zero utilization — always identify and confirm with the owner before recommending deletion; data loss is irreversible.
- OBS request cost spikes may indicate misconfigured lifecycle policies, runaway application retry loops, or unexpected data access patterns — investigate API call volume breakdown (GET, PUT, LIST) before concluding.
- DWS clusters that have had no query activity in the past 7 days are candidates for pausing or termination — verify with the owning team before recommending shutdown, as DWS pause/resume has a cold-start delay.
- Reserved instance coverage gaps mean baseline ECS or RDS workloads are billed at On-Demand rates — quantify the monthly savings potential before presenting the recommendation.
- Budget alerts without escalation actions (SMS, email, function trigger) provide visibility without response capability — verify alert actions are configured for all budget policies.
- Never ask for AK/SK credentials, account billing identifiers beyond what is needed for analysis, or customer data.
- Separate confirmed facts from inference. If state was not queried or shown, say so.

## References

Load these only when needed:

- [Official sources](references/official-sources.md) — use when grounding Huawei Cloud CBC, Budget Management, and CES service behavior or checking the detailed source list.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cost anomaly review or formatting the final answer.

## Response minimum

Return, at minimum:

- CBC Cost Center delta summary and anomaly threshold assessment with evidence level,
- budget alert configuration coverage and gap analysis,
- ECS/GaussDB billing mode anomaly findings,
- OBS request cost spike root cause assessment,
- unattached EVS volume waste identification,
- DWS idle cluster cost findings,
- reserved instance coverage gap analysis and savings estimate,
- prioritized cost remediation actions with effort and impact estimates.
