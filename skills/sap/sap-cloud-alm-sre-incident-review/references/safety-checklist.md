# Safety checklist — SAP Cloud ALM SRE and Incident Review

Use before making any operations governance finding or remediation recommendation, especially for findings involving production monitoring gaps, incident severity classification, SLA breach risk, or integration exception monitoring.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP Cloud ALM tenant, real-time health monitoring dashboard, incident record store, alert configuration interface, or managed SAP system. This skill reviews governance artifacts only.
- Do not accept or request actual incident records containing customer PII, employee PII, personally identifiable system user data, or production system performance data that identifies individual user behavior.
- Do not accept SAP Cloud ALM admin credentials, API tokens, managed system connection parameters, or ITSM integration service account credentials.
- Do not recommend changes to production alert thresholds or monitoring configuration without first testing the change in a non-production Cloud ALM environment and confirming the expected behavior with the operations team.
- Do not recommend disabling or suppressing alerts for a production system to reduce alert fatigue without first confirming that root-cause noise reduction (threshold recalibration, alert grouping) has been attempted.
- Do not close or recommend closing a problem record for a recurring incident pattern without confirming that a verified root-cause fix has been implemented and validated over a sufficient observation period.
- Do not validate SLA compliance or confirm SLA adherence from memory alone. SLA performance assessment must be grounded in user-provided SLA definitions and Cloud ALM reporting documentation.
- Do not assert that a production system is fully monitored without user-provided confirmation of the managed system registration list and configured health checks in Cloud ALM.

## What people get wrong

- **Confusing alert configuration with monitoring coverage**: A managed system can be registered in Cloud ALM without health checks configured, or health checks can be configured without alert rules. Registration, health check enablement, and alert rule creation are three separate steps. Missing any one of them creates a gap even if the others are in place.
- **Treating silent alerts as working alerts**: An alert rule that fires but has no notification channel configured produces no operational value. Alert rules must be verified end-to-end — from event trigger through notification delivery to the responsible team.
- **Assuming ITSM integration is bidirectional**: Cloud ALM integration with ServiceNow or JIRA typically pushes alert-generated incidents to the ITSM. It does not always pull incident status updates back to Cloud ALM. Incident lifecycle in Cloud ALM may diverge from the ITSM record if the integration is unidirectional. Both directions must be reviewed.
- **Overlooking business process monitoring as an SRE concern**: Business process monitoring in Cloud ALM tracks business KPIs (order confirmation time, goods receipt posting delay, payment run completion) rather than technical infrastructure metrics. SRE teams sometimes treat business process monitoring as an application team concern and leave it unconfigured. Unconfigured business process monitoring means business-critical SLA breaches are invisible to operations.
- **Conflating incident management with problem management**: An incident is a service disruption event; a problem is the underlying root cause. Closing incidents without creating problem records for recurring patterns means the root cause is never formally investigated and the same incident pattern repeats. Both processes must be assessed separately.
- **Missing the alert-to-incident traceability requirement**: Regulatory and audit requirements for SAP operations (SOX IT general controls, ISO 20000) typically require an audit trail from alert detection through incident creation to resolution. Missing alert-to-incident linkage means the audit trail has a gap even if both the alert and the incident exist.
- **Assuming Cloud ALM covers all SAP services automatically**: Cloud ALM health monitoring covers a specific list of SAP cloud services and on-premise systems. Not all SAP services are supported as managed systems. Confirm coverage against the current SAP Cloud ALM managed system catalog before asserting monitoring completeness.

## When to push back

- Push back immediately when a production system with no health monitoring coverage is identified — this is a critical finding requiring immediate escalation to the operations team, not a deferred remediation item.
- Push back when the user requests assessment of actual incident records, real-time health monitoring data, or alert event streams — request anonymized configuration summaries or process documentation instead.
- Push back when the user proposes to disable alert rules as a first response to alert fatigue without attempting threshold recalibration or alert grouping.
- Push back when the request requires live Cloud ALM tenant access, real-time dashboard inspection, or managed system connection — state that live inspection is out of scope and ask the user to supply the relevant configuration documentation.
- Push back when the user proposes to close a problem record for a recurring incident pattern without a verified root-cause fix and a defined observation period.
- Push back when the user asks to confirm SLA compliance based on this advisory review alone without providing SLA definitions and Cloud ALM SLA performance reports.

## Evidence labels

- `documentation-based` — grounded in official SAP Cloud ALM application help documentation (help.sap.com)
- `user-provided evidence` — Cloud ALM monitoring configuration descriptions, alert rule lists, business process monitoring key figure definitions, incident management process documentation, SLA definitions, or written operations governance descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
