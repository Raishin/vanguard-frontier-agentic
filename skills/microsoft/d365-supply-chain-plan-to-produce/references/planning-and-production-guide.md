# Planning and production guide

Use this reference for Dynamics 365 Supply Chain Management master planning and production control domain-specific failure modes, safe planning review workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Run Planning Optimization and firm the planned orders.

Wrong. Planning Optimization output is only as reliable as its inputs: on-hand inventory accuracy, coverage settings, BOM and route validity, demand forecasts, and vendor lead time data. Firming incorrect planned orders without review creates purchase commitments, production orders, and transfer orders that cannot easily be reversed at scale.

Common bad assumptions:

- Planning Optimization output is always correct if the system runs without errors.
- Coverage groups configured at go-live are still accurate after item portfolio or supplier changes.
- Negative on-hand quantities in the system will be resolved by the next inventory count without impacting planning.
- Auto-firming planned orders is safe because the system will only create what is needed.
- BOM versions activated at go-live do not need periodic review against engineering change orders.
- Safety stock levels set during implementation are still appropriate after demand pattern changes.

## Planning failure modes

- Coverage group settings not updated after item lead time changes, producing planned orders with incorrect replenishment timing.
- On-hand inventory inflated by unposted inventory adjustment journals, causing Planning Optimization to under-suggest replenishment and creating production stockouts.
- Negative on-hand quantities from inventory variances causing planning to calculate inflated requirements on dependent components.
- Auto-firming horizon set too wide, firming planned orders before human review, creating incorrect purchase or production commitments.
- BOM version with incorrect scrap percentages or component quantities causing planned production orders to under-order materials.
- Safety margins (reorder margin, receipt margin, issue margin) not configured, causing planned orders to be scheduled without lead time buffers and arriving late.
- Intercompany master planning not synchronized across legal entities, creating planned transfer orders that do not match demand signals.
- Stale demand forecasts included in master plans after demand patterns shift, inflating planned supply beyond actual requirements.

## High-risk planning control gaps (examples from plan-to-produce)

- On-hand inventory not validated before the planning run (distorted net requirements produce incorrect planned order quantities)
- Coverage type set to Period for fast-moving items (batches all demand into single large orders, creating cash flow and warehouse capacity risk)
- Planned orders auto-firmed within the firming horizon without planner review (incorrect commitments that require manual cancellation)
- BOM version inactive or referencing obsolete components (planned production orders cannot be released to production without BOM correction)
- Safety stock set to zero for single-source critical components (no buffer against supplier delays)
- Warehouse location directives not configured for new item locations (warehouse management work cannot be generated, blocking receipt or pick)
- Procurement policy requiring purchase order approval bypassed during rush procurement (unauthorized spend commitments)

These gaps represent the highest-risk plan-to-produce scenarios per supply chain operations and internal control guidance. Verify that each gap is addressed before approving planning output or production schedule reliability.

## Minimum safe planning review workflow

1. Confirm scope: legal entities, warehouses, item groups, and planning horizon in review.
2. Validate on-hand inventory: run on-hand inventory report, confirm no unposted adjustment journals, resolve negative on-hand quantities.
3. Review coverage settings: confirm coverage groups are current for the item portfolio; review item-level overrides for anomalies.
4. Review safety stock levels: confirm documented and appropriate for current demand patterns and supplier lead times.
5. Review BOM and route versions: confirm active versions are current, component quantities are correct, and routes match production capacity.
6. Run Planning Optimization and review the planning log for warnings, errors, and action messages.
7. Review planned orders within the firming horizon: confirm quantities, dates, and vendor or production resource assignments are reasonable.
8. Confirm auto-firming settings: verify the auto-firming horizon does not exceed the safe review window for planners.
9. For procurement: confirm vendor lead times in trade agreements are current before approving planned purchase orders.
10. Require live-guard escalation for any production configuration change before recommending it.

## Verification targets

- On-hand inventory: validated, no unposted journals, no unexplained negative quantities
- Coverage groups: current, documented, reviewed after item or supplier changes
- Safety stock: documented levels per item, appropriate for lead time variability
- BOM versions: active versions current, component quantities and scrap percentages validated
- Route configurations: operations sequences current, finite capacity constraints documented
- Planning log: reviewed for action messages, warnings, and errors
- Planned order review: planner sign-off on firmed orders within the firming horizon
- Auto-firming horizon: set within the safe planner review window

## When to push back

Push back if the user asks to:

- approve master planning output without validating on-hand inventory accuracy first
- accept negative on-hand quantities as normal without investigating root cause
- firm planned orders in bulk without planner review of quantities, dates, and vendors
- extend the auto-firming horizon beyond the planner's practical review capacity
- approve a master plan run without confirming BOM and route versions are current and active
- set safety stock to zero for critical or single-source items without documented risk acceptance
- make production coverage group or master plan parameter changes without live-guard escalation and explicit human approval
- trust planning output from a system where demand forecasts are stale or coverage groups have not been reviewed since go-live
