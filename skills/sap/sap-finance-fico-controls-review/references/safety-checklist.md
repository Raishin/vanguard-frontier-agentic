# Safety checklist — SAP Finance FI-CO Controls Review

Use before making any FI-CO control remediation recommendation, especially for findings involving SoD in financial postings, validation/substitution rule changes, period management, or parallel ledger differences.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP S/4HANA Finance system, Fiori launchpad, SAP GUI session, backend RFC, or production financial database. This skill reviews artifacts only.
- Do not accept or request SAP logon credentials, RFC connection details, or direct financial database access.
- Do not post, reverse, park, approve, or modify any financial document. There is no document creation (FB01, F-02, FB50, MIRO, or equivalent), no reversal (FB08, MR8M), no payment run (F110), and no clearing (F-44, F-32) in this skill's execution path. Recommendations always describe configuration changes, not direct financial transactions.
- Do not approve, close, or recommend closing an unmitigated critical SoD exposure in financial postings. Only the audit team and GRC team with appropriate authority can accept a financial process SoD risk. State this explicitly when critical SoD is found.
- Do not recommend implementing configuration changes (validation rule edits, tolerance group changes, period variant modifications) directly in a production SAP system. All recommendations must first be tested in a development or quality system.
- Do not use memory alone to assert whether a specific combination of SAP authorization objects (F_BKPF_BUK, F_BKPF_KOA, F_KNA1_BUK, etc.) creates an SoD conflict. SoD classification must be grounded in user-provided role exports, GRC conflict reports, or official SAP documentation.
- Do not conflate posting period authorization (S_PERIOD_OPEN or T_REXFI_P) with document posting authorization (F_BKPF_BUK). They are separate authorization objects covering different aspects of the period close control.

## What people get wrong

- **Treating substitutions as always benign**: Substitution rules in OBB1 that silently overwrite cost center, profit center, or tax code fields are often added for efficiency reasons without a control impact assessment. A substitution that overwrites a cost center based on a plant or company code mapping can bypass a required cost center validation, creating an undetected mis-posting.
- **Overlooking prior-period posting risk**: An open prior-period posting window (e.g., the previous fiscal period remaining open for weeks after month-end close) is a material control risk. Prior-period adjustments should require documented approval, not simply be enabled by the posting period variant remaining open.
- **Conflating Financial Close Cockpit with financial control completion**: A completed FCC task list only confirms that tasks were marked complete — it does not prove that the underlying financial reconciliations or controls were actually performed correctly. FCC governance and financial control substance are separate layers.
- **Missing parallel ledger completeness**: Reviews that confirm the leading ledger (IFRS or US GAAP) but do not verify that all parallel non-leading ledgers (local GAAP, tax) are complete and reconciled miss a common audit finding area. Extension ledger postings in particular require review to confirm they are not used to override approved base ledger entries.
- **Assuming intercompany clearing accounts balance automatically**: Intercompany G/L postings do not automatically create an offsetting entry in the partner company code in all configurations. Without the intercompany reconciliation hub or a strict cross-company code posting configuration, imbalanced intercompany positions can accumulate undetected.
- **Treating document type as a sufficient posting control**: Document types (DR, KR, SA, etc.) restrict which account types can be posted, but they do not enforce cost object assignment completeness or field-level validation. Field status groups and validation rules provide the substantive field-level control — document type alone is not sufficient.
- **Ignoring manual journal entry control in the SOX ITGC context**: SOX ITGC controls over financial reporting require that manual journal entries above a materiality threshold be subject to approval. A landscape where all users with G/L posting authority can post manual journal entries without an approval step is a SOX ITGC gap regardless of other compensating controls.

## When to push back

- Push back (and escalate) when a critical SoD exposure in financial postings (create + approve + pay + reverse combined) is found — do not proceed with other recommendations until this is escalated.
- Push back when the user asks to confirm FI-CO control compliance from memory alone without providing configuration exports, role lists, or validation/substitution descriptions.
- Push back when the request requires live SAP system access (SAP GUI session, RFC call, Fiori OData API) — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to post, reverse, approve, or modify any financial document — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to approve or close an audit finding or SoD risk without GRC/audit team authorization — this skill is advisory only.
- Push back when parallel ledger differences are identified but the user cannot provide a documented reconciliation explanation — do not accept that differences are expected without evidence.

## Evidence labels

- `documentation-based` — grounded in SAP S/4HANA Finance, SAP Help Portal FI-CO documentation, SAP Financial Closing Cockpit documentation, or SAP audit and compliance guidance
- `user-provided evidence` — validation/substitution exports, role lists, posting period variant descriptions, FCC task list exports, parallel ledger configuration notes, intercompany reconciliation summaries, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
