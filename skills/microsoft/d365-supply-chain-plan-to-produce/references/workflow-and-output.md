# Workflow and output contract

Use this reference only when performing the full master-planning or production-control review, implementation guidance, operational evidence gathering, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Master plan configuration: Planning Optimization setup, master plan parameters, coverage time fences, safety margins, action message settings
- Coverage group and item coverage settings: coverage type, min/max quantities, safety stock levels, reorder points, lead time settings
- Inventory accuracy: on-hand validation, negative on-hand quantities, unposted inventory journals, unclosed production orders
- Procurement configuration: purchase trade agreements, vendor default order settings, sourcing rules, procurement policies, lead time accuracy
- Warehouse management parameters: warehouse configuration, location directives, work template setup, reservation hierarchy accuracy
- BOM and route configuration: active BOM versions, route operations accuracy, scrap percentages, finite capacity settings
- Planned order review: planned purchase orders, planned production orders, planned transfer orders, auto-firming rules
- Demand forecasting: forecast model setup, forecast reduction keys, forecast inclusion in master plans
- Intercompany planning: cross-legal-entity supply chain configuration, intercompany planned order accuracy
- Compensating controls: inventory count schedules, cycle count frequency, approval workflows for order firming

## Safe workflow

1. **Frame scope**
   - Legal entities and warehouses in scope:
   - Planning horizon (short-term, medium-term, long-term):
   - Operational driver (inventory reduction, stockout prevention, production schedule reliability, procurement efficiency):
   - Required outcome (planning config review / inventory accuracy review / production control audit):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported planning logs, on-hand inventory reports, coverage settings exports, and production order status reports for current-state claims.
   - Otherwise inspect sanitized user-provided evidence, configuration screenshots, or official Dynamics 365 Supply Chain Management documentation.
   - Label each finding as `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What items have missing or incorrect coverage settings that could cause stockout or excess inventory?
   - What on-hand inventory records are negative, unvalidated, or based on unposted journals?
   - What planned orders are near the firming horizon without review or approval?
   - What BOM versions or route operations are inactive or incorrectly configured?
   - What evidence is missing that would change the planning output verdict?
   - What procurement lead times or vendor agreements are stale or unvalidated?

4. **Recommend the smallest safe action**
   - Prefer inventory validation, coverage-settings review, and planned-order analysis before firming production or purchase orders.
   - If the safest action is to stop and gather evidence (run an on-hand inventory report or planning log first), say that plainly.
   - Production master plan runs, coverage group reconfigurations, and BOM or route activations require live-guard escalation. Do not recommend live changes without explicit human approval.

## Output contract

Return this structure:

```markdown
# D365 Supply Chain Plan-to-Produce Review: <scope>
## Executive verdict
- Status: PLAN RELIABLE / PLAN WITH CONDITIONS / PLAN NOT RELIABLE / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
