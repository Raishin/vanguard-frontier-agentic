---
name: huawei-live-cost-budget-action-guard
description: Gate Huawei Cloud CBC budget threshold changes, Reserved Instance purchases, and CUD commitments — budget threshold reduction can trigger service suspension and RI/CUD are non-refundable committed spend.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-08"
  category: finops
---

# Huawei Live Cost Budget Action Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-cost-budget-action-guard work. Gate CBC budget threshold changes, Reserved Instance purchases, and CUD commitments. Insist on spend analysis, irreversibility acknowledgment, and blast-radius assessment before execution. Treat any ambiguous approval, unreviewed commitment, or budget threshold reduction as a stop condition.

## When to Use

Use this skill when:

- A CBC budget threshold is being reduced (may trigger service suspension if threshold falls below current spend)
- A CBC budget threshold is being created or increased (governs future suspension risk)
- A Reserved Instance (RI) is being purchased (committed spend, non-refundable)
- A CUD (Committed Use Discount) is being activated (hourly commitment × term, non-refundable)
- A Cost Center tag policy is being changed in a way that affects cost attribution or budget scope
- An enterprise project cost transfer is being executed that may shift spend between budget-governed projects

## When NOT to Use

Do not use this skill when:

- The task is read-only CBC cost analysis with no purchase or threshold-change intent
- The task involves resource-level changes unrelated to billing (e.g., ECS instance start/stop)
- The task is purely reviewing cost reports or explaining RI/CUD concepts with no purchase decision

## Gate Protocol

This skill requires the 6-step live-guard gate from the maestro. See `skills/huawei/huawei-maestro/SKILL.md` for the full gate protocol. The 6 steps are:

1. **Surface risk** — state the specific risk: service suspension, non-refundable spend, or cost attribution impact.
2. **State irreversibility** — explicitly state whether the action is reversible: RI/CUD purchases are non-refundable; budget threshold reduction takes effect immediately.
3. **Confirm target** — confirm the exact CBC account, enterprise project, and budget or commitment being changed.
4. **Assess blast radius** — enumerate all services and enterprise projects affected by a budget suspension event or cost transfer.
5. **Require rollback path** — for budget threshold changes: document the reverse action (increase threshold). For RI/CUD: state that there is no refund path.
6. **Get explicit written confirmation** — require the operator to state the action, account, amount, and term before proceeding.

## Pre-Flight Checklist

Before executing any budget or commitment action, verify all of the following:

1. **Account and project confirmed** — confirm CBC account ID and enterprise project scope.
2. **Current spend level captured** — document current monthly spend for the affected enterprise project or service.
3. **Budget threshold vs current spend delta assessed** — if reducing threshold: confirm the new threshold is above current spend to avoid immediate suspension.
4. **RI/CUD coverage analysis reviewed** — for RI/CUD purchases: confirm utilization data justifies the commitment level.
5. **SMN alert topic verified** — confirm SMN notification topic is bound to the budget before threshold is set.
6. **Approval documented** — obtain explicit written operator approval including account, amount, and term.

## Required Confirmation

The operator must explicitly state all of the following before any budget or commitment action is executed:

- "I confirm the CBC account is `<ACCOUNT_ID>` and the enterprise project is `<ENTERPRISE_PROJECT>`."
- "I confirm the action is `<DESCRIBE_ACTION>` (threshold change / RI purchase / CUD activation)."
- "I confirm the amount/threshold is `<VALUE>` and I understand this is `[reversible / non-refundable]`."
- "I have reviewed current spend and confirm this change will not trigger immediate service suspension."
- "I approve this action."

## Rollback Procedure

- **Budget threshold reduction** (limited reversibility): increase the threshold back to the previous level via CBC console — if suspension already triggered, service restoration may require support intervention.
- **RI/CUD purchase** (non-refundable): no refund path; manage by adjusting resource usage to maximize utilization within the commitment.
- Document all actions in CBC audit log and the associated change record.

## Response Shape

1. Action type and account confirmed
2. Current spend level
3. Irreversibility assessment
4. Blast radius (service suspension scope)
5. Approval status
6. Executed action
7. Post-action verification
