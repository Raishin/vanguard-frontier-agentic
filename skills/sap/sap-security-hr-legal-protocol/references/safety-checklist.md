# Safety checklist — SAP Security HR Legal Escalation Protocol

Use before any cross-function handoff, before any action is queued to a guarded-mutating agent, and before any evidence is shared outside its originating function.

## Non-negotiables

- Do not execute any role assignment, role removal, account suspension, or account deletion without documented dual approval. All role mutations must be queued to `sap-role-assignment-guarded-operator-agent` with approval documentation attached.
- Do not share unredacted HR data (compensation, performance ratings, disciplinary records, health information, succession data) with Security, operational teams, or any party outside HR without Legal authorization and documented legal-hold or employment-law basis.
- Do not close, approve, or recommend closing a GRC access violation. GRC violation closure authority rests with the GRC/audit team only.
- Do not disclose investigation status or findings to the subject of the investigation without explicit joint authorization from HR and Legal.
- Do not accept verbal approval for any cross-function action. Written approval (ticket, email chain, or signed document) is mandatory.
- Do not initiate or recommend any employment action (warning, suspension, termination). This protocol surfaces risk findings to HR and Legal; employment decisions are theirs alone.
- Do not delete or purge evidence logs during an active investigation. Retention policy execution that would destroy relevant logs must be suspended via Legal hold.
- Do not bypass the irreversible-action gate. Any action listed under that section requires all listed approvals before the action is queued.

## What people get wrong

- **Treating SoD conflict classification as an employment finding**: An SoD conflict report establishes access risk, not misconduct. The HR and Legal functions determine whether any employment action is warranted based on the full investigation. Security's role is to surface the access evidence, not to draw conclusions about intent.
- **Sharing the full GRC conflict report with HR without redaction review**: GRC reports often contain transaction codes and system access details that, when combined with HR records, could be used to infer sensitive information about other employees or investigations. Review for cross-contamination before sharing.
- **Assuming a leaver's access lapse is always a Security issue**: JML gaps are often process failures in the HR lifecycle workflow or IPS provisioning chain. Assign root-cause investigation to the correct function before escalating as a security incident.
- **Skipping the legal-hold decision for post-termination access**: If a departed employee's account was used after the effective termination date, there may be an obligation to preserve logs for employment-law or regulatory purposes before any retention policy purge occurs.
- **Treating EAM log review as equivalent to access approval**: The existence of an EAM firefighter log does not mean the access was authorized or within scope. Log review must be completed by the designated EAM owner and controller before the access event is considered governed.
- **Conflating HR-sensitive system access with a personal data breach**: Access to SuccessFactors compensation or performance data by an unauthorized user is a policy violation and potential access control failure. Whether it constitutes a reportable personal data breach under GDPR or equivalent regulation is a Legal determination, not a Security one.

## When to push back

- Push back (and escalate immediately) when an unmitigated critical SoD conflict involving an active employee is found — escalate to GRC/audit and flag the escalation owners before any other action.
- Push back when any function requests unredacted HR data without Legal authorization — provide a redacted summary instead and require Legal sign-off for full disclosure.
- Push back when asked to confirm employment status, role legitimacy, or lifecycle event completion from memory — require HR-system-of-record confirmation.
- Push back when asked to queue a role mutation without all required written approvals in hand.
- Push back when asked to close a GRC violation — refer to the GRC/audit team.
- Push back when asked to recommend or initiate an employment action — refer to HR and Legal.

## Evidence labels

- `documentation-based` — grounded in SAP GRC Access Control, SAP SuccessFactors, SAP Cloud Identity Services, NIST SP 800-53, or ISO 27001 documentation
- `user-provided evidence` — access logs, GRC conflict reports, role exports, IPS provisioning logs, HR lifecycle confirmations, or written descriptions provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such and must not be used as the sole basis for any cross-function action
