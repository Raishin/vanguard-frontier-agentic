# Workflow and output contract — SAP EWM TM Logistics Execution Review

Use this reference for all logistics execution control finding classification, risk level assignment, remediation path selection, and output formatting.

## Logistics execution control finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `warehouse-process-types` | `confirmation-bypass-path` | Warehouse process type configuration allows a transfer order or task to be confirmed without a required physical scanning verification or supervisor override for quantity deviations |
| `warehouse-process-types` | `unauthorized-goods-movement-path` | Warehouse process type permits a goods movement (goods receipt, goods issue, stock transfer) to be triggered without the required upstream document or authorization step |
| `warehouse-process-types` | `activity-area-assignment-gap` | Activity area assignment does not route tasks to the correct queue for the physical warehouse zone, causing tasks to be executed in the wrong sequence or location |
| `wave-task-management` | `wave-release-sod-gap` | The same user or role can both release waves (initiating goods movement) and confirm individual tasks (completing goods movement) without a supervisory review step |
| `wave-task-management` | `task-confirmation-no-scan` | Task confirmation can be performed manually without barcode or RFID scan verification — enables phantom confirmations without physical goods movement |
| `wave-task-management` | `wave-template-grouping-gap` | Wave template grouping criteria do not align with physical warehouse layout or customer priority rules, causing suboptimal pick paths or SLA-impacting task sequencing |
| `slotting-bin-assignment` | `bin-capacity-overrun-risk` | Slotting profile or storage section search strategy allows bin quantity limits to be exceeded, creating physical over-packing risk and inventory location accuracy failures |
| `slotting-bin-assignment` | `mixed-storage-restriction-missing` | Mixed storage restrictions are not configured for incompatible product categories (hazardous materials, temperature-sensitive goods, high-value items), creating regulatory compliance or damage risk |
| `slotting-bin-assignment` | `unauthorized-bin-access` | Bin assignment logic does not restrict access to high-value or controlled goods storage locations by user role or warehouse process type |
| `freight-order-carrier-management` | `carrier-rate-deviation-uncontrolled` | TM carrier selection rule allows carrier assignment that deviates from the agreed freight agreement rate without a documented approval step, creating freight cost leakage |
| `freight-order-carrier-management` | `unauthorized-carrier-substitution` | Freight order can be reassigned to an alternative carrier outside the approved carrier list without a second authorization step — subcontracting risk |
| `freight-order-carrier-management` | `freight-agreement-governance-gap` | Freight agreement or rate table can be modified without a four-eyes approval, enabling unauthorized rate changes that benefit specific carriers or routing lanes |
| `dock-yard-management` | `gate-in-gate-out-gap` | Gate in/gate out process does not capture vehicle identity, dock door assignment, or arrival timestamp, creating a gap in goods arrival and departure traceability |
| `dock-yard-management` | `dock-appointment-governance-gap` | Dock appointments can be scheduled, modified, or cancelled without authorization controls, enabling unauthorized goods arrivals or departures outside scheduled windows |
| `shipment-execution-exceptions` | `pod-match-to-order-gap` | Proof of delivery (POD) records can be confirmed without matching to the originating freight order or outbound delivery, creating a revenue recognition and freight settlement gap |
| `shipment-execution-exceptions` | `exception-resolution-backlog` | Freight unit or freight order exceptions are not categorized, escalated, and resolved within target cycle times, indicating a systemic exception management process failure |
| `shipment-execution-exceptions` | `claims-management-gap` | Freight claims management is not configured or is manually managed outside SAP TM, creating freight cost recovery leakage and carrier accountability gaps |
| `ewm-s4hana-integration` | `goods-movement-integration-failure` | EWM transfer order confirmation does not trigger the corresponding S/4HANA MIGO goods movement posting — creates invisible inventory discrepancy in the S/4HANA material ledger |
| `ewm-s4hana-integration` | `integration-error-no-alerting` | EWM-to-S/4HANA integration errors are not monitored with automated alerting and reprocessing workflows — errors can accumulate undetected, causing inventory valuation inaccuracies |
| `tm-s4hana-integration` | `freight-accrual-posting-gap` | TM freight cost documents are not transferred to FI-AP for accrual posting, creating unbooked freight cost liabilities and understated logistics cost in the P&L |
| `logistics-sod` | `goods-receipt-quantity-adjustment-combined` | User or role has combined authority to confirm goods receipt and adjust inventory quantities — enables undetected inventory shrinkage or inflation |
| `logistics-sod` | `carrier-assignment-rate-approval-combined` | User or role has combined authority to assign carriers to freight orders and approve freight agreement rate changes — enables self-serving carrier rate manipulation |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Undetected goods movement bypass path circumventing all confirmation controls; combined inventory fraud authority (goods receipt confirmation + quantity adjustment + inventory adjustment release) |
| `high` | Unauthorized task confirmation without scanning verification; carrier assignment overriding agreed freight rate without approval; EWM-to-S/4HANA integration failure without alerting; unmatched POD processing; wave release and task confirmation SoD gap; freight agreement governance gap |
| `medium` | Slotting configuration creating bin capacity overrun risk; mixed storage restriction missing for regulated goods; dock appointment governance gap; freight exception backlog above resolution target; TM-to-FI freight accrual posting gap; claims management gap |
| `low` | Best practice deviation in wave template grouping; minor activity area assignment gap in low-throughput zone; wave template optimization opportunity |

## Remediation path decision tree

For each finding:

1. **Is this an undetected goods movement bypass path circumventing all confirmation controls?**
   - Yes → `critical`. Escalate immediately to the warehouse operations manager, internal audit team, and S/4HANA inventory controller. Do not process further warehouse transactions until the bypass path is closed. Implement mandatory scanning confirmation for the affected process type and add a supervisor override approval for quantity deviations.
   - No → continue.

2. **Is this a wave release and task confirmation SoD gap, or combined goods receipt and quantity adjustment authority?**
   - Yes → `high`. Separate wave release authorization from task confirmation authorization at the role and activity level. Assign task confirmation to a different role than wave release. For inventory quantity adjustment authority, separate goods receipt confirmation from physical inventory adjustment roles.
   - No → continue.

3. **Is this a carrier rate deviation or freight agreement governance gap?**
   - Yes → `high`. Add a mandatory approval step for carrier assignment deviating from the agreed freight agreement rate. Implement four-eyes approval for freight agreement and rate table modifications. Restrict carrier substitution authority to a logistics procurement role separate from freight order management.
   - No → continue.

4. **Is this an EWM-to-S/4HANA integration failure without alerting?**
   - Yes → `high`. Implement automated integration error monitoring with alert notification to the warehouse controller and S/4HANA inventory team. Establish a reprocessing workflow with a target resolution time. Reconcile any accumulated inventory discrepancies between EWM and S/4HANA.
   - No → continue.

5. **Is this a POD match-to-order gap or freight accrual posting gap?**
   - Yes → `high` (POD) or `medium` (freight accrual). For POD: enforce match-to-originating-freight-order validation before POD confirmation. For freight accrual: implement automated transfer of TM freight cost documents to FI-AP and configure accrual reversal on actual invoice receipt.
   - No → continue.

6. **Is this a slotting, dock management, or exception resolution finding?**
   - Yes → `medium`. Configure bin capacity limits in the storage type search strategy. Add mixed storage restrictions for regulated product categories. Enforce dock appointment authorization controls. Define exception escalation targets and configure exception monitoring reports.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — warehouse process type configuration summaries, wave analysis reports, slotting configuration exports, inventory discrepancy summaries, freight order exception logs, carrier assignment reports, dock appointment scheduling summaries, EWM/TM integration status reports, or written descriptions of the logistics execution landscape.
2. **Classify each finding** by logistics execution control domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical goods movement bypass findings immediately** — escalate before any other remediation discussion.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical bypass and inventory fraud findings first; then high wave/task SoD, carrier rate, and integration findings; then medium slotting, dock, and exception management gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Logistics execution control domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected warehouse, process type, wave template, freight order type, integration channel, role or authorization object (if provided), and specific control gap
5. Recommended remediation action (scanning enforcement, SoD role separation, carrier rate approval step, freight agreement four-eyes, integration error alerting, POD match enforcement, slotting capacity rule correction, dock tracking gap closure, exception escalation workflow, freight accrual transfer setup, etc.)
6. Logistics execution control posture after remediation
7. Escalation notice for any critical goods movement bypass or inventory fraud finding — explicit statement that this requires warehouse operations manager, internal audit team, and inventory controller sign-off before proceeding
8. Prioritized remediation sequence
9. Confirmation that no goods movements were posted, no warehouse tasks were confirmed, no freight orders were released, and no inventory quantities were adjusted in this review
