# Safety checklist — SAP Procurement / License / FinOps / Vendor Protocol

Use before finalizing any finding, handoff package, or escalation trigger. This checklist is mandatory for all advisory sessions involving contractual actions, entitlement reductions, or license compliance events.

## Non-negotiables

- Do not recommend reducing entitlement quota or terminating a subscription without first confirming (with the user) that no active service instances are consuming that quota. Reducing below active consumption terminates running workloads.
- Do not invoke or recommend invoking `sap-btp-entitlement-guarded-operator-agent` from within this protocol. The operator gate is human-mediated only. This protocol produces the package; a human submits it.
- Do not accept raw SAP contract pricing, commercial unit prices, supplier margins, or penalty amounts without confirming they have been redacted. Refuse and request redaction before proceeding.
- Do not produce advisory output on a consumption spike without a consumption snapshot. Findings without evidence must be labeled `inference` and explicitly noted as unverified.
- Do not recommend triggering a RISE exit clause or contract escalation without identifying the named Procurement lead and SAP Account Executive who must co-approve the escalation.
- Do not classify a finding as `critical` without tracing the specific contractual breach, SLA violation, or license non-compliance from user-provided evidence or official SAP documentation.
- Do not recommend a license reclassification (reducing named user count) without confirming that the current measurement period has been closed and the measurement report has been finalized. Mid-period reclassification creates compliance exposure.

## What people get wrong

- **Treating CPEA and BTPEA credits as interchangeable**: CPEA and BTPEA have different service coverage scopes, credit validity periods, and top-up mechanics. A consumption spike under one model does not behave identically under the other. Confirm the user's commercial model before assessing burn rate.
- **Conflating BTP service entitlements with SAP S/4HANA named user licenses**: BTP service entitlements govern cloud service consumption (API calls, instances, memory). SAP named user licenses govern human-user access to SAP applications. They are separately measured, separately billed, and separately audited.
- **Assuming idle subscriptions can be terminated without dependency checking**: A BTP subscription that shows zero direct consumption may still be depended upon by an integration flow, an extension application, or a trust configuration. Zero consumption does not mean zero dependency.
- **Recommending credit top-up without checking credit validity dates**: CPEA/BTPEA credits have expiry dates. Purchasing additional credits near the end of the validity period may not resolve the burn issue if credits expire before consumption. Confirm credit validity before recommending a purchase.
- **Missing auto-renewal windows**: SAP and Ariba contracts often have auto-renewal provisions with notice periods of 30-90 days. Missing the notice window forfeits the option to renegotiate or exit. Always confirm upcoming renewal dates before closing an advisory session involving contract risk.
- **Conflating vendor lock-in with high switching cost**: Vendor lock-in is an architectural dependency with no documented exit path. High switching cost is a known, quantified risk with a mitigation plan. The RISE/SLA vendor-risk role assesses lock-in; the FinOps role assesses switching cost. Do not merge the two assessments.

## When to push back

- Push back when the user asks for a consumption forecast without providing a consumption snapshot or historical usage trend. Forecasts without data are `inference` and must be labeled as such.
- Push back when the user asks to recommend a contract exit clause activation without providing the contract reference or exit notice period. Contract actions without contract evidence are out of scope.
- Push back when the user asks to directly execute an entitlement change, subscription termination, or credit purchase from within this protocol. This protocol is advisory only; redirect the user to the appropriate operator gate with the completed handoff package.
- Push back when the user provides contract pricing or supplier margin data without redaction. Request redaction before accepting the evidence.
- Push back when the user asks to confirm a license compliance finding from memory alone. License compliance findings require user-provided measurement reports or SAP CRUM data.

## Evidence labels

- `documentation-based` — grounded in SAP BTP Help Portal, RISE with SAP documentation, SAP Ariba product documentation, or SAP support resources (e.g., CRUM)
- `user-provided evidence` — consumption exports, entitlement inventories, license measurement reports, contract references, or Ariba compliance reports supplied by the user
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label explicitly and note the assumption being made
