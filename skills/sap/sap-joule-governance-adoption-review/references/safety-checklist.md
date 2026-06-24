# Safety checklist — SAP Joule Governance and Adoption Review

Use before making any Joule governance finding or remediation recommendation, especially for findings involving data access boundaries, write-back capabilities, audit logging, or regulated workflow use.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP Joule session, Joule interaction log store, connected SAP system (S/4HANA, SuccessFactors, Ariba, BTP), or Joule administration console. This skill reviews governance artifacts only.
- Do not accept or process actual Joule interaction logs, user prompt history, AI-generated response content from production Joule sessions, or production business data surfaced by Joule (financial figures, employee data, procurement prices).
- Do not accept SAP system credentials, BTP service keys, OAuth tokens, or Joule integration certificates.
- Do not recommend enabling additional Joule write-back skills without first confirming that a user confirmation step is in place and that the skill activation has been through the organization's rollout governance process.
- Do not recommend disabling Joule interaction logging as a performance or privacy optimization in a deployment used for regulated business processes.
- Do not validate the Joule authorization boundary from memory alone. Joule's enforcement of the underlying SAP authorization model is configuration-dependent. Direct the user to verify the current authorization boundary behavior against SAP Joule security documentation and sandbox testing before production deployment.
- Do not assess hallucination risk by generating or evaluating actual Joule responses. Hallucination and over-trust risk assessment is based on governance policy coverage, verification requirements, and training documentation — not on testing the model.

## What people get wrong

- **Assuming Joule inherits authorizations automatically**: Joule's integration with the underlying SAP authorization model (S/4HANA authorization objects, SuccessFactors RBP) requires correct configuration at the connected system level. Joule does not automatically enforce the narrowest possible authorization boundary — misconfigured connections can expose broader data than the user is authorized to see in the native UI.
- **Treating Joule read capabilities as risk-free**: Even read-only Joule skills can expose sensitive business data (compensation data from SuccessFactors, financial positions from S/4HANA, supplier prices from Ariba) if role-aware configuration is incorrect or if cross-application aggregation is not governed. Read-only does not mean governance-free.
- **Overlooking cross-application data aggregation risk**: Joule can surface data from multiple connected SAP solutions in a single response. A user who is authorized to see a subset of data in each system individually may see an unauthorized combined view when Joule aggregates across systems. Cross-application scope requires explicit cross-system access rights governance.
- **Confusing Joule interaction logging with SAP system audit logs**: Joule interaction logging (capturing prompts and responses in Joule) is separate from the audit logs in the underlying SAP systems (S/4HANA change documents, SuccessFactors audit trail, Ariba audit log). Both are needed for a complete compliance audit trail. Missing Joule interaction logs means there is no record of what the user asked or what Joule responded, even if the underlying system recorded the data access.
- **Assuming acceptable-use policy covers AI outputs automatically**: Most organizational IT or data policies do not address generative AI output validation, prohibited uses of AI-generated content in regulated decisions, or escalation paths when Joule produces incorrect or harmful responses. An AI-specific acceptable-use policy for Joule is a separate governance requirement.
- **Treating Joule rollout as a one-time event**: SAP Joule capabilities expand with each SAP solution release. New skills and action types may be automatically available in connected systems after a product update. Without a rollout governance process for new capability activation, the organization's approved Joule scope can expand without review.

## When to push back

- Push back immediately if a confirmed Joule data access boundary breach is identified — suspend the affected capability and escalate to the security and data protection team before any other recommendation is made.
- Push back when the user requests assessment of actual Joule interaction logs or production AI-generated response content — request anonymized or aggregated governance summaries instead.
- Push back when the user proposes to enable a Joule write-back skill without a confirmation step and without a rollout governance review.
- Push back when the user proposes to disable Joule interaction logging in a regulated business process deployment.
- Push back when the request requires live Joule session access, Joule skill execution, or connected SAP system inspection — state that live inspection is out of scope and ask the user to supply the relevant configuration documentation.
- Push back when the user asks to confirm that Joule's authorization boundary is correctly enforced based on this advisory review alone — require sandbox testing against the current SAP Joule security documentation before production sign-off.

## Evidence labels

- `documentation-based` — grounded in official SAP Joule service guide, SAP BTP documentation, SAP AI Core documentation, or SAP Trust Center AI guidance (help.sap.com or sap.com/trust-center)
- `user-provided evidence` — Joule configuration documentation, skill activation lists, data access boundary descriptions, audit log configuration summaries, acceptable-use policy documents, or written governance posture descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
