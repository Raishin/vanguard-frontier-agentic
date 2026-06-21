# Workflow and output contract — SAP Manufacturing Execution Risk Review

Use this reference for all manufacturing execution control finding classification, risk level assignment, remediation path selection, and output formatting.

## Manufacturing execution control finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `production-order-governance` | `order-release-no-availability-check` | Production order released without a completed availability check (missing parts list review), creating a material shortage risk at the relevant production operation |
| `production-order-governance` | `order-confirmation-bypass` | Production order operation can be confirmed without completing required upstream operations or without quality inspection results for in-process inspection operations |
| `production-order-governance` | `order-release-sod-gap` | The same user or role can both create and release production orders without a scheduling or production planning supervisor review step |
| `capacity-planning-scheduling` | `infinite-scheduling-unconstrained` | Production orders are scheduled using infinite scheduling with no capacity leveling governance, allowing unrestricted scheduling beyond physical work center capacity without escalation |
| `capacity-planning-scheduling` | `capacity-overload-no-escalation` | Work center capacity overload does not trigger an escalation workflow or rescheduling proposal — production schedule cannot be physically executed as planned |
| `capacity-planning-scheduling` | `available-capacity-misconfigured` | Work center available capacity is not configured to reflect actual shift patterns and planned maintenance downtimes — capacity utilization reporting is unreliable |
| `mrp-exception-management` | `exception-backlog-reschedule-out` | High volume of unresolved exception message 30 (reschedule out) or 60 (cancel) — indicates excess planned orders or procurement proposals that will result in over-procurement or excess inventory |
| `mrp-exception-management` | `exception-backlog-shortage` | High volume of unresolved exception message 10 (bring forward) or 20 (reschedule in) — indicates production schedule or material availability risk that will impact customer delivery |
| `mrp-exception-management` | `mrp-horizon-design-gap` | MRP planning horizon or firming horizon is not aligned with the production lead time or supplier lead time — generates unreliable procurement proposals or freezes planned orders prematurely |
| `shop-floor-integration` | `dm-mes-integration-error-no-alert` | SAP Digital Manufacturing-to-SAP PP integration errors are not monitored with automated alerting — production order status in SAP does not reflect actual shop-floor execution state |
| `shop-floor-integration` | `operation-confirmation-status-gap` | DM/MES operation confirmation does not update the SAP PP production order operation status in real time — goods movement postings and order completion status are delayed or incorrect |
| `shop-floor-integration` | `goods-movement-integration-failure` | DM/MES-triggered goods receipt or component consumption posting fails to reach the S/4HANA material ledger — creates inventory valuation discrepancy |
| `qm-integration` | `quality-hold-bypass` | Quality inspection stock can be transferred to unrestricted use without a completed quality usage decision from an authorized QM user — non-conforming material can enter production or shipment without quality management approval |
| `qm-integration` | `inspection-lot-missing` | Inspection lots are not being created for a production operation where in-process or goods receipt inspection is required by the quality plan or regulatory requirement |
| `qm-integration` | `usage-decision-no-dual-control` | Quality usage decision in a regulated manufacturing environment (GMP, FDA 21 CFR Part 11, ISO 13485) does not require dual-control or electronic signature — single user can accept non-conforming material |
| `backflush-goods-movement` | `backflush-sod-gap` | User or role has combined authority to trigger backflush (component consumption posting) and to perform inventory quantity adjustments — enables undetected component consumption manipulation |
| `backflush-goods-movement` | `backflush-no-physical-verification` | Backflush at milestone confirmation posts component consumption without physical count or scanner verification — creates phantom goods issue risk when actual consumption differs from BOM quantity |
| `backflush-goods-movement` | `goods-movement-authorization-gap` | Movement type 261 (goods issue for production order) or 101 (goods receipt against production order) can be posted by users without a production order context — enables unauthorized inventory movements |
| `manufacturing-exceptions` | `exception-resolution-backlog` | Manufacturing exceptions (material shortage, capacity overload, quality hold, operation delay) are not categorized, escalated, and resolved within target production cycle times |
| `manufacturing-exceptions` | `quality-notification-routing-gap` | Quality notifications created from manufacturing exceptions are not routed to the quality management team with a defined response time target — quality issues remain unresolved on the shop floor |
| `manufacturing-sod` | `bom-routing-modification-order-creation-combined` | User or role has combined authority to modify bill of materials or routing records and create production orders referencing those records — enables undetected production quantity or component manipulation |
| `manufacturing-sod` | `usage-decision-stock-transfer-combined` | User or role has combined authority to process quality usage decisions and post the resulting stock transfer from quality inspection to unrestricted use — full quality hold bypass path |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Quality hold bypass in a regulated manufacturing environment (GMP, FDA 21 CFR, ISO 13485, AS9100) enabling non-conforming product release; usage decision dual-control gap in a regulated environment with electronic records requirements |
| `high` | Quality hold bypass in a non-regulated environment; backflush SoD gap enabling undetected component consumption manipulation; DM/MES integration error without alerting causing production order status inaccuracy; goods movement integration failure creating inventory valuation discrepancy; BOM/routing modification combined with production order creation; combined usage decision and stock transfer authority; production order release without availability check in a material shortage environment |
| `medium` | MRP exception backlog above resolution threshold for reschedule-out or shortage categories; capacity overload without escalation workflow; available capacity misconfiguration causing unreliable utilization reporting; inspection lot missing for a required operation type; backflush without physical verification in a high-value component environment; manufacturing exception resolution backlog; quality notification routing gap |
| `low` | Best practice deviation in order type parameter design; scheduling margin key optimization opportunity; minor MRP horizon design gap for a low-value material category |

## Remediation path decision tree

For each finding:

1. **Is this a quality hold bypass in a regulated manufacturing environment?**
   - Yes → `critical`. Immediately escalate to the quality manager, regulatory affairs team, and internal audit team. Halt shipment of any batch or lot produced during the period when the bypass path was active. Implement usage decision authorization controls and electronic signature requirements per FDA 21 CFR Part 11 or applicable standard before production resumes for the affected material.
   - No → continue.

2. **Is this a backflush SoD gap, combined usage decision and stock transfer authority, or BOM/routing modification combined with production order creation?**
   - Yes → `high`. Separate the relevant authorization objects at the role level. For backflush SoD: separate MIGO goods movement authority (MB_MIGO_GI) from inventory adjustment authority. For usage decision and stock transfer: assign usage decision authority (QM_QMEL_WF, C_QMEL_VGR) to a different role than the stock posting authorization. For BOM/routing modification: restrict CS02 and CA02 access to master data stewards only, not production order creators.
   - No → continue.

3. **Is this a DM/MES integration error without alerting or goods movement integration failure?**
   - Yes → `high`. Implement automated integration error monitoring with alert notification to the production controller and S/4HANA inventory team. Establish a reprocessing workflow with a defined resolution target. Reconcile any production order status discrepancies between DM/MES and SAP PP and any inventory valuation differences between EWM/MES and the S/4HANA material ledger.
   - No → continue.

4. **Is this a production order release without availability check?**
   - Yes → `high`. Enable availability check at order release in the production order type parameters. Configure the availability check rule to cover all relevant material requirement types (BOM components). Define the handling of missing parts (warning vs. hard stop based on material criticality).
   - No → continue.

5. **Is this an MRP exception backlog or capacity overload finding?**
   - Yes → `medium`. For MRP exceptions: define exception resolution governance (target processing time per exception category, responsible planner assignment, weekly exception review cadence). For capacity overload: implement capacity leveling strategy with finite scheduling for constrained work centers, and configure automated escalation for overload beyond a defined percentage threshold.
   - No → continue.

6. **Is this a missing inspection lot, quality notification routing gap, or manufacturing exception resolution backlog?**
   - Yes → `medium`. For missing inspection lots: verify inspection type assignment in the material master and quality plan. Add the required inspection type. For quality notification routing: define resolution time targets by priority code and assign responsible QM team. For exception resolution backlog: implement exception aging monitoring and escalation triggers in the production exception management process.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — production order status reports, MRP exception aging reports, capacity utilization summaries, QM inspection lot reports, DM/MES integration status exports, backflush configuration summaries, BOM and routing change logs, or written descriptions of the manufacturing execution landscape.
2. **Classify each finding** by manufacturing execution control domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical quality hold bypass findings immediately** — escalate before any other remediation discussion; halt production or shipment if in a regulated environment.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical QM hold bypass in regulated environments first; then high backflush SoD, integration, and combined authority findings; then medium MRP, capacity, inspection lot, and exception management gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Manufacturing execution control domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected plant, production order type, work center, material, inspection type, integration channel, or role/authorization object (if provided), and specific control gap
5. Recommended remediation action (QM usage decision dual-control, quality hold bypass closure, backflush SoD role separation, DM/MES integration error alerting, availability check enforcement, MRP exception resolution governance, capacity leveling strategy, inspection lot type correction, manufacturing exception escalation, BOM/routing modification access restriction, etc.)
6. Manufacturing execution control posture after remediation
7. Escalation notice for any critical QM hold bypass finding in a regulated environment — explicit statement that this requires quality manager, regulatory affairs team, and internal audit team sign-off, and that affected batches or lots must be reviewed before release
8. Prioritized remediation sequence
9. Confirmation that no production orders were released, no operations were confirmed, no goods movements were posted, no quality usage decisions were processed, and no BOM or routing records were modified in this review
