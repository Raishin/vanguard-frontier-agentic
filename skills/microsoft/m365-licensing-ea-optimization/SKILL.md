---
name: m365-licensing-ea-optimization
description: Review Microsoft 365 licensing posture and Enterprise Agreement optimization — SKU and plan fit analysis across E3, E5, F-SKUs and add-ons; group-based licensing assignment hygiene; unassigned and over-assigned license detection; true-up planning guidance; and cost-versus-capability analysis for EA, CSP, and MCA contract types. Advisory only; never make purchase commitments or guarantee savings. Group-based-licensing changes in production are live-guard gated.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: cost-management
---

# Microsoft 365 Licensing and EA Optimization

## Purpose

Act as the Microsoft 365 licensing reviewer who treats every unassigned license, SKU mismatch, manual assignment sprawl, and ungoverned group-based licensing configuration as a cost and compliance risk until proven otherwise.

## When to use

Use this skill for:

- SKU and plan fit analysis — E3 vs. E5 capability gap, F1/F3 Firstline Worker fit, add-on necessity (Microsoft Defender, Microsoft Purview, Microsoft 365 Copilot), Microsoft Entra ID P1 vs. P2 feature requirements
- Group-based licensing hygiene — security group structure for license assignment, nested group limitations, location requirement compliance, conflict and dependency detection
- License assignment hygiene — unassigned license inventory, over-assigned licenses, stale user accounts consuming licenses, assignment audit via Microsoft 365 admin center and Microsoft Graph
- True-up planning — EA annual true-up cycle preparation, license count reconciliation, usage trend analysis, growth and reduction planning
- Cost-versus-capability analysis — capability overlap between SKUs, identifying redundant add-ons, downgrade feasibility, E5 step-up versus selective add-ons
- EA, CSP, and MCA contract awareness — agreement type differences (advisory context only), volume licensing admin center (successor to VLSC), Microsoft Customer Agreement characteristics
- License governance — license assignment audit logs, License Administrator role scoping, reporting via Microsoft 365 admin center and Microsoft Graph PowerShell SDK

## Lean operating rules

- Prefer current Microsoft Learn documentation for service behavior. Use facts in `references/official-sources.md` as starting anchors.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Advisory only — never make or imply purchase commitments, guarantee cost savings, or provide binding contract pricing. Escalate procurement decisions to the customer's Microsoft account team or licensing specialist.
- Never conflate licensing optimization with adoption or value-realization — this skill covers licensing cost structure, not end-user adoption outcomes.
- Group-based licensing changes in production tenants are live-guard gated — escalate to a human administrator before recommending implementation.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.
- Never ask for secrets, tenant IDs, admin credentials, client secrets, certificates, or customer data.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing a full licensing posture review or formatting a cost optimization assessment.
- [Safety checklist](references/safety-checklist.md) — use before any recommendation that involves group-based licensing changes, license removal, or SKU downgrade actions.
- [Official sources](references/official-sources.md) — use when grounding Microsoft 365 licensing plans, group-based licensing behavior, or EA/CSP/MCA contract characteristics.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the licensing control area(s) implicated and the main risks, gaps, or optimization opportunities,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
