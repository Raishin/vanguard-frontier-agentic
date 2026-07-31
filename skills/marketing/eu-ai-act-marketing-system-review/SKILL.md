---
name: eu-ai-act-marketing-system-review
description: Use this skill when reviewing a marketing AI system's description card against EU AI Act risk-tier criteria to classify the system (prohibited / high-risk / limited-risk / minimal-risk), flag documentation obligations, and identify deployment-readiness gaps before the August 2, 2026 full-enforcement date. Trigger when a user provides an AI system description card covering system purpose, input data types, output decisions, human-oversight mechanism, deployment geography, and whether it profiles natural persons — or when they ask whether their marketing AI tool, lead-scoring model, content personalization engine, or automated ad-decisioning system requires a conformity assessment or transparency notice under EU AI Act Regulation 2024/1689.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# EU AI Act Marketing System Review

## Purpose
This skill reviews a marketing AI system's description card against EU AI Act Regulation 2024/1689 risk-tier criteria to classify the system, flag documentation obligations, and identify deployment-readiness gaps before the August 2, 2026 full-enforcement date. Marketing AI systems — lead-quality scorers, content personalization engines, urgency-calibration models, lookalike generators, and automated bidding optimizers — operate at the boundary between Article 5 prohibited practices (subliminal manipulation, exploitation of vulnerabilities), Annex III high-risk systems (AI for access to private services, creditworthiness, and employment when profiling natural persons), and limited-risk systems subject to transparency obligations only. Misclassification is itself a compliance gap: a system internally labeled "low risk" that profiles behavioral and demographic signals for credit or employment routing is Annex III high-risk and requires a conformity assessment. The review ingests the description card, maps system characteristics to the risk taxonomy, flags missing documentation (technical documentation Article 11, conformity assessment Article 43, transparency obligations Article 13/52), and identifies the August 2026 enforcement timeline pressure.

## Lean operating rules
- Treat a system that profiles natural persons using behavioral or demographic signals to produce scores, rankings, or routing decisions used in access to credit, insurance, employment, or essential private services as HIGH — this maps to Annex III categories and requires a conformity assessment, CE marking, and registration in the EU AI database before deployment.
- Treat urgency or scarcity signals calibrated by real-time engagement data with no human review gate as HIGH — this is a candidate for Article 5(1)(b) prohibited subliminal manipulation or exploitation of psychological vulnerabilities; route to qualified legal counsel without making the prohibition determination yourself.
- Treat a system classified internally as "low risk" but routing decisions to downstream agents or automated processes with no human override capability as HIGH — the absence of a meaningful human-oversight mechanism invalidates a limited-risk designation under Article 14 requirements.
- Treat a system that processes biometric, health, racial/ethnic-origin, political-opinion, or religious-belief data as input features or inferred labels for marketing segmentation as HIGH — these are special-category data under GDPR Article 9 and trigger heightened AI Act scrutiny as potential Annex III characteristics.
- Treat the absence of technical documentation (Article 11) covering system purpose, training data provenance, performance metrics, and limitations for any non-minimal-risk system as HIGH — documentation is a prerequisite for conformity assessment, not a post-deployment obligation.
- Treat a system with no transparency notice or user-facing disclosure of automated decision-making where the EU AI Act or GDPR Article 22 requires one as HIGH — undisclosed profiling that produces legal or similarly significant effects is both a GDPR and an AI Act violation.
- Flag a system whose August 2026 enforcement readiness is unknown — no documented conformity-assessment timeline, no assigned responsible person, no EU registration planned — as MEDIUM when the system is potentially high-risk.
- Flag general-purpose AI models integrated into marketing workflows without a documented system-level risk assessment as MEDIUM — the GPAI provisions under Title VIII require providers to assess downstream systemic risk.
- Flag systems that collect or process behavioral signals at scale (>1 million natural persons) without a documented fundamental rights impact assessment as MEDIUM.
- Do not classify a system as prohibited under Article 5 without explicit instruction to qualified counsel; surface the risk and route the determination.
- Label every finding with evidence basis: description card provided, documentation-based, or inference from missing information.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- System risk-tier classification (prohibited / high-risk Annex III / limited-risk / minimal-risk) with rationale
- Profiling and natural-person assessment (Article 22 GDPR intersection)
- Human-oversight mechanism assessment (Article 14)
- Documentation gap inventory (Article 11 technical docs, Article 43 conformity assessment, Article 13/52 transparency)
- August 2026 enforcement readiness assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
