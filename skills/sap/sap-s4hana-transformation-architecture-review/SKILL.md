---
name: sap-s4hana-transformation-architecture-review
description: Review an SAP S/4HANA transformation approach for architectural soundness. Assesses brownfield conversion, greenfield reimplementation, and selective data transition (SDT) strategies against SAP Activate methodology, deployment model options (Cloud Public Edition, Cloud Private Edition, RISE with SAP), SAP Readiness Check findings, simplification item impact, and fit-to-standard alignment. Advisory only — does not access or modify live systems.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: architecture
  lifecycle: experimental
---

# SAP S/4HANA Transformation Architecture Review

## Purpose

Assess the architectural approach to an SAP S/4HANA transformation program. Review the chosen migration strategy (brownfield, greenfield, or selective data transition), evaluate alignment with SAP Activate methodology, qualify the selected deployment model (Cloud Public Edition, Cloud Private Edition, or RISE with SAP), interpret SAP Readiness Check output and simplification item impact, and evaluate fit-to-standard posture. Surface architectural risks, strategy mismatches, and prioritized recommendations. Does not access, read, or modify live SAP systems.

## When to use

Use this skill when the user asks to:

- assess whether brownfield (system conversion), greenfield (new implementation), or selective data transition (SDT) is the right approach for their S/4HANA transformation,
- review an existing transformation program plan against SAP Activate phases (Discover, Prepare, Explore, Realize, Deploy, Run),
- evaluate deployment model options — SAP S/4HANA Cloud Public Edition (multi-tenant SaaS), SAP S/4HANA Cloud Private Edition, or RISE with SAP — and their architectural trade-offs,
- interpret SAP Readiness Check results (simplification items, business functions, custom code impact, technical prerequisites),
- review the organization's fit-to-standard workshop coverage and scope of configuration versus custom development,
- assess transformation readiness: organizational change management, data quality prerequisites, integration landscape impacts, and testing strategy,
- review phasing, sequencing, and scope decisions in a multi-wave or multi-country S/4HANA rollout.

## When not to use

- When the user needs a review of custom ABAP code against clean core — use `sap-clean-core-debt-review` or `sap-custom-code-remediation-review`.
- When the request is about data migration execution readiness — use `sap-data-migration-cutover-readiness`.
- When the request is about specific ABAP extensibility or RAP architecture — use `sap-abap-cloud-rap-review`.
- When the request is about transport mechanics — use `sap-guarded-transport-import`.
- When live system discovery is needed before architecture review — use `sap-live-readonly-landscape-discovery` first.

## Does not touch live systems

This skill operates on user-provided documentation, SAP Readiness Check reports, simplification item lists, transformation project plans, scope documents, fit-to-standard workshop outputs, or described architectural decisions. It does not connect to any SAP system, run system checks, read configuration tables, or access BTP cockpit. All live system evidence must be provided by the user.

## Lean operating rules

- Strategy first. Classify the transformation approach (brownfield / greenfield / SDT / hybrid) before assessing any other dimension. Strategy choice drives all downstream architectural decisions.
- SAP Activate is the reference methodology. All phasing and deliverable recommendations use SAP Activate (Discover, Prepare, Explore, Realize, Deploy, Run) as the structural framework.
- Deployment model has architectural consequences. S/4HANA Cloud Public Edition restricts extensibility to key-user and side-by-side patterns only. Private Edition and RISE offer more ABAP extensibility but do not equal on-premise freedom. Always qualify the deployment model before discussing extensibility.
- Simplification items are upgrade blockers until resolved. SAP Readiness Check simplification item hits must be classified by impact severity (conversion-blocking, functional change, manual re-implementation required) before a transformation timeline is assessed.
- Fit-to-standard reduces risk. Configurations that align with SAP standard processes reduce upgrade risk, future clean core compliance burden, and support cost. Scope deviations must be explicitly justified.
- Evidence from official SAP sources takes precedence over memory or training data.
- Do not assume deployment model from context alone. Confirm with the user whether the target is Cloud Public Edition, Cloud Private Edition, RISE with SAP on hyperscaler, or on-premise managed.
- Do not fabricate simplification item counts or Readiness Check findings. Only classify findings the user has provided.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP Activate methodology docs, S/4HANA deployment guides, SAP Readiness Check documentation, or SAP Help Portal
- `user-provided evidence` — SAP Readiness Check reports, project plans, scope documents, fit-to-standard outputs, or architectural descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled

## Live-environment rules

**This skill does not touch live systems.** There is no credential, API call, RFC connection, or SAP system access in this skill's execution path. Users must supply Readiness Check output, transformation plans, project documentation, or written descriptions of their landscape and transformation decisions for this skill to review.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — strategy classification taxonomy, assessment dimensions, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, common assessment mistakes, when to push back.
- [Official sources](references/official-sources.md) — SAP Activate methodology, Readiness Check, S/4HANA deployment, simplification items.

## Response minimum

Return, at minimum:

- **Problem classification**: transformation strategy type (brownfield / greenfield / SDT / hybrid), deployment model, and SAP Activate phase alignment.
- **Evidence used**: documentation-based / user-provided evidence / inference.
- **Risk level**: transformation-blocking / high-risk / medium-risk / low-risk per dimension assessed.
- **Recommended action**: strategy recommendation or gap remediation path with rationale grounded in official SAP guidance.
- **Refusal / escalation triggers**: if live system data is required before assessment, redirect to `sap-live-readonly-landscape-discovery`; if custom code impact is the primary concern, redirect to `sap-custom-code-remediation-review`.
- **Business impact**: timeline risk, license model impact, extensibility restrictions, and organizational change management implications.
- **Next verification step**: which SAP Activate deliverable or Readiness Check finding to validate next before the transformation proceeds.
