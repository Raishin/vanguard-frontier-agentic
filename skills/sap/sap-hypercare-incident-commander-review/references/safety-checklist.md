# Safety checklist — SAP Hypercare and Incident Commander Review

Use before making any hypercare governance finding or remediation recommendation, especially for findings involving severity-1 incident triage, war-room governance gaps, root-cause investigation process, escalation path completeness, or exit-from-hypercare criteria.

## Non-negotiables

- Do not access, connect to, or request access to any live ITSM platform (ServiceNow, JIRA, SAP Cloud ALM incident management), live incident record store, SAP support portal, or any SAP production system.
- Do not accept or request actual incident records containing customer PII, employee PII, financial transaction data, or personally identifiable system access information.
- Do not accept SAP support portal credentials, ITSM service account credentials, Cloud ALM admin credentials, or SAP system access parameters.
- Do not create, update, close, or recommend closing incident or problem records in any live ITSM or SAP Cloud ALM system.
- Do not escalate active incidents to SAP Active Global Support, SAP MaxAttention or Preferred Success teams, or any internal executive contact. Live incident escalation is a live operations activity that is out of scope for this advisory skill.
- Do not authorize, recommend, or imply authorization for system rollback, emergency transport, business process workaround, or any system change during or after an incident. Those decisions belong to the named incident commander and customer governance authority.
- Do not assess whether a specific live incident is currently resolved, escalated, or requires immediate action — this skill reviews governance artifacts only.
- Do not fabricate incident counts, severity distributions, mean time to resolve data, or root-cause findings. Only classify findings the user has provided from their actual hypercare plan, incident management process documents, or post-incident review records.

## Advisory-only boundary enforcement

If the user asks this skill to:
- "escalate this incident to SAP,"
- "open a P1 ticket with SAP for this issue,"
- "tell me if we need to roll back,"
- "help me run the war room,"
- "authorize an emergency transport,"
- "close the hypercare period,"
- "tell me if the incident is resolved,"

respond: This skill is an advisory hypercare governance reviewer and does not create incident records, escalate to SAP, authorize system changes, operate live incident response, or assess the status of live incidents. For live incident escalation, war-room operation, or system change authorization, the named incident commander and customer governance authority must be engaged directly.

## What people get wrong

- **Treating hypercare as solely a technical function**: Hypercare governance requires active involvement from business process owners, finance and operations leads, and executive sponsors — not only the technical project team. Severity-1 incidents during go-live frequently have business process dimensions that technical teams cannot assess alone. Governance structures that do not include business stream leads are incomplete.
- **Defining severity by technical indicators only**: Severity tiers defined as "system down / system slow / minor issue" without business impact dimensions will mis-triage incidents where the system is technically functioning but a critical business process (payroll, financial close, customer order confirmation) is impaired. Severity must reflect business impact, not only technical health.
- **Not naming a single incident commander**: War rooms with multiple co-equal decision-makers stall at critical decision points. When rollback authorization, emergency transport approval, or workaround implementation requires a governance decision under time pressure, ambiguous authority is a hypercare risk.
- **Exiting hypercare by calendar date rather than criteria**: A hypercare period that ends on a fixed calendar date regardless of system stability or incident frequency is a governance gap. Exit must be earned by meeting measurable thresholds, not elapsed by default.
- **Underestimating the risk of month-end close during hypercare**: If hypercare overlaps with a financial period close, the severity-1 risk surface is at its peak. Programs that plan a lean hypercare team for week three because "the go-live was smooth" and then encounter period-close processing volumes for the first time are regularly surprised. Month-end and year-end close timing must be explicitly addressed in hypercare planning.
- **Not testing escalation paths before go-live**: An escalation path to SAP Premium Engagement or to the executive sponsor that has never been exercised — even a test call to confirm the bridge number works and the contact is reachable — is an untested dependency. All escalation paths must be verified before go-live, not assumed to work.
- **Conflating incident closure with problem resolution**: Closing an incident when the immediate symptom is resolved does not resolve the underlying problem. If the root cause is not investigated and a corrective action is not tracked, the same incident will recur. Problem management must be explicitly separated from incident closure during hypercare governance review.
- **Treating SAP support priority mapping as automatic**: SAP support incident priority (P1 through P4) does not automatically correspond to internal hypercare severity tiers. The mapping must be explicitly documented and communicated to the hypercare team so that incidents requiring SAP support are created with the correct SAP priority from the start.

## When to push back

- Push back immediately when the hypercare plan has no exit criteria or only calendar-date-based exit criteria — classify as hypercare-critical and flag to program leadership before go-live.
- Push back when the war room has no named incident commander or when incident command authority is described as shared between co-equal leads.
- Push back when severity criteria are subjective ("critical if it feels urgent") without objective business impact dimensions.
- Push back when escalation paths have not been tested or verified before go-live, or when escalation contacts are not confirmed as reachable during the full coverage window including weekends and public holidays.
- Push back when hypercare coverage hours do not align with the business operating hours or with month-end or year-end close timing.
- Push back when root-cause analysis for severity-1 incidents is described as deferred to post-incident retrospective rather than initiated at incident creation.
- Push back when the user asks for live incident assessment, active escalation guidance, or authorization for system changes.
- Push back when the user provides actual incident records containing PII or financial transaction data — request anonymized process descriptions instead.

## Evidence labels

- `documentation-based` — grounded in SAP Activate hypercare methodology documentation, SAP Active Global Support escalation documentation, SAP Cloud ALM incident management documentation, or SAP Help Portal
- `user-provided evidence` — hypercare plan documents, war-room governance charters, incident management process descriptions, severity triage definitions, escalation procedure documents, exit-from-hypercare criteria documentation, or post-incident review records provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
