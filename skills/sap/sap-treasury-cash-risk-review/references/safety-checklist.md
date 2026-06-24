# Safety checklist — SAP Treasury Cash Risk Review

Use before making any treasury control remediation recommendation, especially for findings involving payment release authority, bank account management governance, hedge documentation, or treasury SoD.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP S/4HANA Treasury system, SAP Bank Communication Manager, SWIFT network, banking portal, Fiori launchpad, SAP GUI session, backend RFC, or production treasury or cash database. This skill reviews artifacts only.
- Do not accept or request SAP logon credentials, RFC connection details, SWIFT credentials, banking portal access, or direct treasury database connectivity.
- Do not initiate, approve, release, cancel, or modify any payment, bank transfer, trade, hedge, settlement, financial instrument, or bank account record. There is no payment run initiation, no payment release, no bank transfer, no FX trade execution, no hedge designation or de-designation, no financial instrument creation or settlement, and no bank account opening, modification, or closing in this skill's execution path. Recommendations always describe configuration and process design changes, not direct treasury actions.
- Do not approve, close, or recommend closing an unmitigated critical payment fraud finding (single-user payment release, combined BAM/payment-release authority). Only the treasury, audit, and GRC team with appropriate authority can accept a treasury fraud risk. State this explicitly when critical findings are identified.
- Do not recommend implementing configuration changes (BAM approval workflow, payment release authorization, bank communication channel settings) directly in a production SAP system. All recommendations must first be tested in a development or quality system.
- Do not use memory alone to assert whether a specific combination of SAP treasury authorization objects (F_PAYR_BUK, F_REGU_BUK, TR_BANKI, TR_BANKA) creates an SoD conflict. Treasury SoD classification must be grounded in user-provided role exports, GRC conflict reports, or official SAP TRM documentation.
- Do not assert hedge accounting compliance (IFRS 9 or IAS 39) from documentation review alone without confirming that hedge designation documentation, effectiveness testing results, and risk component identification are complete and up to date.

## What people get wrong

- **Treating payment approval workflow as sufficient without verifying authorization object separation**: A payment approval workflow configured in SAP Bank Communication Manager may require two approvers in the business process, but if both approvers share the same authorization object (F_PAYR_BUK with the same payment company code), one person can technically approve and release independently. Always verify authorization object scope matches the intended dual-control design.
- **Overlooking BAM shadow bank accounts**: Bank accounts maintained in legacy banking master data (house bank accounts in FI-AP) outside the SAP Bank Account Management module may not be subject to BAM governance controls. A complete bank account governance review must cover both BAM-managed accounts and any residual house bank accounts in BNKA or T012 maintained outside BAM.
- **Conflating hedge designation documentation with hedge accounting eligibility**: Completing IFRS 9 or IAS 39 designation documentation in SAP TRM does not automatically ensure hedge accounting eligibility. Effectiveness testing must also be completed, the hedging instrument must be a qualifying instrument, and the hedged item must be an eligible item. Documentation completeness and accounting eligibility are separate assessment dimensions.
- **Missing in-house cash dual-control at aggregation**: In-house cash payment factories often have dual-control requirements at the individual payment request level but a single approver at the payment factory aggregation and external release level. The aggregation stage is where large outbound payments are formed — dual-control here is as important as at the request level.
- **Assuming electronic bank statement automation eliminates cash position risk**: Automated bank statement processing (CAMT.053, MT940, BAI2) eliminates manual data entry errors but does not prevent delayed postings, processing failures, or incorrect liquidity item assignments. Cash position accuracy requires both automation and exception monitoring.
- **Ignoring currency exposure from in-house bank internal accounts**: In-house bank internal accounts denominated in foreign currencies create FX exposure at the in-house bank entity level that may not be captured in the treasury FX exposure determination. Review whether in-house bank FX positions flow into the TRM exposure management module.
- **Overlooking bank communication channel integrity**: SFTP or file-based host-to-bank transmission without digital signature or file integrity verification is a payment fraud enabler. A tampered payment file could redirect payment instructions between the SAP system and the bank without detection.

## When to push back

- Push back (and escalate) when single-user external payment release authority or combined BAM/payment-release authority is found — do not proceed with other recommendations until this is escalated to the treasury, audit, and GRC team.
- Push back when the user asks to confirm treasury control compliance from memory alone without providing authorization object exports, bank account master data reports, or BAM configuration summaries.
- Push back when the request requires live SAP Treasury system access (SAP GUI session, RFC call, Fiori OData API, SWIFT network inspection) — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to initiate, approve, release, or modify any payment, trade, hedge, bank transfer, or bank account record — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to approve or close a treasury audit finding or SoD risk without GRC/audit team authorization — this skill is advisory only.
- Push back when hedge documentation deficiencies are identified but the user cannot provide the actual hedge designation documentation and effectiveness testing results — do not accept that hedge accounting is valid without evidence.

## Evidence labels

- `documentation-based` — grounded in SAP S/4HANA Treasury and Risk Management, SAP Cash Management, SAP Bank Account Management, SAP Bank Communication Manager, or SAP Help Portal TRM documentation
- `user-provided evidence` — authorization object exports, bank account master data reports, BAM configuration summaries, payment run logs, hedge documentation exports, financial instrument type summaries, in-house cash configuration notes, bank statement processing descriptions, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
