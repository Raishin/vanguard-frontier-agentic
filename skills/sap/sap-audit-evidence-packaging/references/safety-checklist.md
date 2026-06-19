# Safety checklist — SAP Audit Evidence Packaging

Use before making any evidence packaging recommendation or including any excerpt from user-provided artifacts in output.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system, GRC Access Control instance, BTP subaccount, ABAP system, audit log service, or S/4HANA tenant. This skill advises on packaging only.
- Do not accept or request SAP system credentials, BTP service keys, GRC logon passwords, OAuth tokens, SAML private keys, or connection strings with embedded credentials.
- Do not include in any output: passwords, API keys, OAuth tokens, client secrets, SAML private keys, employee personal identifiable information (names, national IDs, salary data), or customer confidential data — even if the user provides them in an evidence artifact.
- If a user-provided artifact contains sensitive data, advise on redaction before including any excerpt. Never echo sensitive values back in the response.
- Do not declare a control as compliant or an evidence package as audit-ready. This skill assesses completeness and packaging quality; compliance declarations must be made by qualified auditors.
- Do not produce evidence artifacts on behalf of the user. This skill advises on what to collect, how to label it, and how to structure it; it does not generate evidence that did not exist from a real SAP system.
- Do not classify evidence as sufficient based solely on the existence of an artifact type — the artifact must also cover the full audit period and be accompanied by valid chain-of-custody metadata.

## What people get wrong

- **Treating a GRC SoD report screenshot as complete evidence**: A screenshot of a GRC SoD conflict report is not sufficient without the underlying report export (CSV, PDF, or XLS with full conflict detail), chain-of-custody metadata (extraction date, system, extractor role), and confirmation of the audit period covered.
- **Overlooking the audit period gap**: Evidence that covers 9 months of a 12-month audit period is a `high` gap, not a `medium` one. The full audit period must be covered without interruption.
- **Assuming access review completion equals evidence**: A completed access review workflow in GRC or a ticketing system is evidence of the review process, not evidence of the role assignment state. Both the review completion record and the role assignment export at that point in time are required.
- **Including live personal data in evidence packages**: GRC SoD reports, SUIM user-role exports, and IAS user lists often contain employee names, email addresses, and employee IDs. These must be redacted or pseudonymized per applicable data protection requirements before submission to an external auditor.
- **Missing chain-of-custody for evidence gathered outside GRC**: Evidence gathered via BTP CLI, CF CLI, or ABAP transaction exports is often not automatically stamped with a timestamp or extractor identity. The chain-of-custody metadata must be manually documented at the time of extraction.
- **Treating non-reproducible evidence as low risk**: If an evidence artifact cannot be regenerated from the same source (e.g., an ABAP SM20 audit log from a period that has since been archived or overwritten), it must be stored with heightened custody controls and its non-reproducibility documented explicitly.
- **Assuming a transport log is sufficient change management evidence alone**: A transport log demonstrates that transports moved; it does not demonstrate that the approval chain was followed. Approval records (workflow completion, second-pair-of-eyes sign-off) are a separate required artifact.

## When to push back

- Push back when the user asks to generate evidence that does not exist from a real SAP system (e.g., to fabricate a GRC report or produce a synthetic role assignment export).
- Push back when the user asks to include credentials, personal identifiable information, or customer data in an evidence package without redaction.
- Push back when the user asks to declare a control compliant or an audit package ready — direct to the qualified auditor or control owner for the compliance declaration.
- Push back when the user asks to extract live evidence from a SAP system — redirect to `sap-live-readonly-landscape-discovery` or `sap-live-readonly-identity-trust-discovery` for the evidence gathering step.
- Push back when the audit scope is ambiguous (audit period not defined, control framework not stated, SAP systems in scope not identified) — request clarification before advising on evidence mapping.
- Push back when the user provides an evidence artifact and asks whether it satisfies a regulatory requirement without specifying which regulatory framework or which specific control requirement is being addressed.

## Evidence labels

- `documentation-based` — grounded in SAP GRC documentation, SAP audit guides, or regulatory framework requirements (SOC 2, ISO 27001, SOX ITGC, GxP)
- `user-provided evidence` — audit evidence artifacts, control descriptions, audit scope descriptions, or written SAP landscape descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
