# Safety checklist — SAP EWM TM Logistics Execution Review

Use before making any logistics execution control remediation recommendation, especially for findings involving goods movement confirmation, carrier assignment, freight agreement governance, or EWM/TM–S/4HANA integration.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP EWM system, SAP TM system, SAP S/4HANA Inventory Management system, Fiori launchpad, SAP GUI session, backend RFC, or production logistics or warehouse database. This skill reviews artifacts only.
- Do not accept or request SAP logon credentials, RFC connection details, warehouse management system API keys, or direct logistics database connectivity.
- Do not post goods movements, confirm warehouse tasks, release or modify freight orders, create or cancel delivery documents, adjust inventory quantities, process proof of delivery, create dock appointments, or modify carrier assignments. There is no transfer order creation, no MIGO goods movement, no freight order release, no carrier assignment change, no inventory adjustment, and no POD confirmation in this skill's execution path. Recommendations always describe configuration and process design changes, not direct logistics actions.
- Do not approve, close, or recommend closing an unmitigated critical goods movement bypass finding or inventory fraud authorization finding. Only the warehouse operations manager, internal audit team, and S/4HANA inventory controller with appropriate authority can accept a logistics execution integrity risk. State this explicitly when critical findings are identified.
- Do not recommend implementing configuration changes (process type modification, wave template rule changes, carrier selection rule updates, freight agreement rate changes) directly in a production SAP EWM or TM system. All recommendations must first be tested in a development or quality system.
- Do not use memory alone to assert whether a specific combination of SAP EWM or TM authorization objects creates an SoD conflict. Logistics SoD classification must be grounded in user-provided role exports, GRC conflict reports, or official SAP EWM/TM documentation.

## What people get wrong

- **Treating manual task confirmation as equivalent to scan-verified confirmation**: Many EWM implementations allow manual task confirmation as an override for scanning failures. If manual override is available to all warehouse users without supervisory approval, the scanning control is effectively bypassed. Assess whether manual override requires a separate authorization or supervisor sign-off.
- **Overlooking carrier substitution risk in TM subcontracting**: TM subcontracting chains (where a primary carrier further subcontracts to a secondary carrier) can introduce unauthorized carriers into the shipment execution chain. If the subcontracting relationship is not governed by an approved subcontracting agreement in SAP TM, the actual carrier performing the transport may differ from the contracted carrier without business visibility.
- **Missing EWM-to-S/4HANA integration error backlog**: EWM integration monitor errors accumulate silently if no alerting is configured. A backlog of unprocessed integration errors means that S/4HANA inventory quantities and values do not reflect actual warehouse stock — a material financial reporting risk that may not be visible in standard inventory reports.
- **Conflating dock appointment scheduling with actual arrival control**: A dock appointment system only controls planned arrivals. If gate in/gate out processes are not enforced with physical vehicle checks and system-recorded timestamps, unscheduled trucks can unload or load goods outside the appointment window without system visibility.
- **Assuming slotting optimization is purely operational**: Slotting configurations that allow bin capacity limits to be exceeded create physical risk (over-packed bins, falling goods, safety hazards) and inventory accuracy risk (multiple products recorded in a bin beyond the bin's physical capacity). Slotting capacity governance is both an operational and a control matter.
- **Overlooking freight claims management as a revenue and cost recovery control**: Unclaimed freight damages, shortages, and overcharges represent a direct cost leakage. If SAP TM claims management is not configured, or if claims are tracked outside the system, the freight cost recovery process is not auditable and is likely incomplete.
- **Treating POD processing as purely operational**: Proof of delivery confirmation in SAP TM triggers freight settlement, revenue recognition confirmation, and potentially billing to the customer (if freight charges are passed through). POD records that can be confirmed without matching to the originating freight order bypass these downstream financial controls.

## When to push back

- Push back (and escalate) when an undetected goods movement bypass path or combined inventory fraud authorization is found — do not proceed with other recommendations until this is escalated to the warehouse operations manager, internal audit team, and inventory controller.
- Push back when the user asks to confirm EWM or TM control compliance from memory alone without providing configuration exports, wave analysis data, freight exception logs, or integration status summaries.
- Push back when the request requires live SAP EWM or TM system access (SAP GUI session, RFC call, Fiori OData API, direct warehouse database query) — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to post goods movements, confirm warehouse tasks, release freight orders, modify carrier assignments, or adjust inventory quantities — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to approve or close a warehouse or transportation audit finding without operations manager and audit team authorization — this skill is advisory only.
- Push back when EWM-to-S/4HANA integration errors are identified but the user cannot provide integration monitor status reports or inventory reconciliation data — do not accept that integration is clean without evidence.

## Evidence labels

- `documentation-based` — grounded in SAP Extended Warehouse Management, SAP Transportation Management, SAP S/4HANA Logistics, or SAP Help Portal EWM/TM documentation
- `user-provided evidence` — warehouse process type configuration summaries, wave analysis reports, slotting configuration exports, inventory discrepancy summaries, freight order exception logs, carrier assignment reports, dock appointment scheduling summaries, EWM/TM integration status reports, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
