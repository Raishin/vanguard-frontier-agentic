# Safety checklist — SAP Release Cutover Finance Controls Protocol

Use before any go/no-go decision, before any approval gate is cleared, and before any transport is queued to `sap-guarded-transport-import-operator-agent`.

## Non-negotiables

- Do not queue any transport for production import without all required written approvals. Transport imports execute via `sap-guarded-transport-import-operator-agent` only after complete approval documentation is confirmed.
- Do not approve or recommend approval of a freeze exception without Finance Controls written assessment of the change risk.
- Do not declare a cutover GO without a fully assessed readiness checklist, Finance Controls sign-off, and all Business Process Owner sign-offs documented in writing.
- Do not recommend rollback of financial configuration without Finance Controls written confirmation that the rollback restores rather than compounds the inconsistency.
- Do not accept verbal approval for any cross-function release, cutover, or financial controls action. Written approval is mandatory for all gate clearances.
- Do not bypass the irreversible-action gate. Any action listed under that section of SKILL.md requires all listed written approvals before execution.
- Do not recommend importing a transport modifying posting period variants, account determination, tax codes, or revenue recognition configuration while the Financial Closing Cockpit is running an active close task for the affected company code.
- Do not import or recommend importing a transport classified `revenue-recognition-impact` or `inventory-valuation-impact` without CFO written approval and, if prior-period financials are affected, external auditor notification.

## What people get wrong

- **Treating a QA import success as sufficient for production approval**: A successful QA import confirms technical transport integrity and basic functional correctness in a non-production environment. It does not confirm that the change is safe to import in the current financial period, that no collision with other production transports exists, or that Finance Controls has assessed the impact. All three are separate gates.
- **Assuming that a small or low-risk transport does not require Finance Controls review**: Transport size (number of objects) is not a reliable indicator of financial impact. A single posting period variant change is a one-object transport that can lock or unlock financial periods for an entire company code. All transports with FICO, CO, SD billing, or MM valuation objects require Finance Controls classification.
- **Importing transports in queue order without collision analysis**: The SAP transport import queue processes transports in sequence, but the order in the queue may not reflect the dependency order of the changes. Importing in queue order without collision analysis can overwrite a more recent change with an earlier version of the same configuration object.
- **Skipping the rollback plan because the cutover seems straightforward**: Every go-live cutover carries risk that can only be managed with a pre-tested and pre-approved rollback plan. The rollback plan must be tested in a dress rehearsal and approved before cutover execution begins — not drafted after a problem occurs.
- **Treating hypercare incident workarounds as permanent solutions**: A workaround implemented during hypercare to unblock a business process is a temporary measure, not a resolution. The production correction transport must be developed, tested, and approved through the same governance gate as any other production change — hypercare status does not exempt changes from Finance Controls review.
- **Conflating period-end close completion with posting period lock**: The Financial Closing Cockpit may show all tasks complete, but posting periods may still be open for corrections. Conversely, posting periods may be locked before all close tasks are formally signed off. Confirm both the closing cockpit status and the posting period variant separately when assessing period-sensitive financial impact.

## When to push back

- Push back (and escalate immediately) when a transport modifying financial configuration is proposed for import during a production freeze without Finance Controls written approval — escalate to the Finance Controls lead and Release Governance Board before any other action.
- Push back when asked to declare cutover GO without all required sign-offs confirmed in writing.
- Push back when a freeze exception request is classified as non-critical but the requesting team cannot provide test evidence or a rollback plan.
- Push back when a rollback is proposed for a financial configuration transport without Finance Controls written confirmation of the rollback impact on financial integrity.
- Push back when asked to clear an irreversible-action gate without all required written approvals documented.
- Push back when asked to confirm that a transport has no financial impact from the transport description alone — require Finance Controls classification based on the actual object list.
- Push back when a hypercare production correction is proposed without Release Manager and Finance Controls approval, even if business pressure is high.

## Evidence labels

- `documentation-based` — grounded in SAP transport management, SAP Activate methodology, SAP Financial Closing Cockpit, SAP Revenue Accounting and Reporting, SAP Material Ledger, PCAOB AS 2201, or NIST SP 800-53 documentation
- `user-provided evidence` — transport manifests, QA import results, collision analysis outputs, readiness checklist status, financial impact assessments, approval records, or incident records provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such and must not be used as the sole basis for any go/no-go decision or gate clearance
