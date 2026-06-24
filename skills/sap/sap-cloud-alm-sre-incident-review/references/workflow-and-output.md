# Workflow and output contract — SAP Cloud ALM SRE and Incident Review

Use this reference for all finding classification, risk level assignment, remediation path selection, and output formatting.

## SRE governance domain taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `health-monitoring` | `production-system-not-monitored` | Production SAP service or managed system not registered in SAP Cloud ALM health monitoring scope |
| `health-monitoring` | `health-check-not-configured` | Managed system registered in Cloud ALM but health checks not configured or enabled for the relevant service type |
| `health-monitoring` | `threshold-miscalibrated` | Health status threshold set too loosely — degradation is not detected until after SLA breach has already occurred |
| `health-monitoring` | `monitoring-scope-gap` | Specific SAP service component (e.g., a dedicated integration tenant, a secondary HANA instance) missing from the monitored scope of an otherwise registered managed system |
| `alerting` | `notification-channel-not-configured` | Alert rule exists but notification channel (email, Microsoft Teams, PagerDuty, ServiceNow) is not configured — alert fires silently |
| `alerting` | `alert-routing-misconfigured` | Alert notification does not reach the responsible operations team for the affected system or business process |
| `alerting` | `alert-fatigue-risk` | Alert thresholds set so sensitively that the operations team receives high volumes of low-priority alerts, reducing the signal-to-noise ratio for critical events |
| `alerting` | `no-itsm-integration` | Cloud ALM alerts do not automatically create incidents in the organization's ITSM platform — manual incident creation introduces delay and inconsistency |
| `integration-monitoring` | `critical-interface-not-monitored` | Integration scenario on the critical business process path (order-to-cash, procure-to-pay, financial close, payroll) has no Cloud ALM exception monitoring coverage |
| `integration-monitoring` | `iflow-exceptions-not-surfaced` | iFlow exceptions from SAP Integration Suite are not routed to Cloud ALM integration monitoring |
| `integration-monitoring` | `no-end-to-end-monitoring` | Integration scenario is monitored at the middleware level but not end-to-end from source to target system |
| `business-process-monitoring` | `critical-process-key-figure-missing` | No Cloud ALM business process monitoring key figure is configured for a business-critical process milestone (order confirmation, goods receipt, payment run, payroll posting) |
| `business-process-monitoring` | `sla-breach-detection-absent` | Business process monitoring configured but no SLA breach threshold is defined for process milestone completion time |
| `business-process-monitoring` | `no-business-user-notification` | Business process SLA breach or exception occurs without notification to the business process owner or affected business users |
| `incident-management` | `severity-classification-gap` | Incident severity classification does not account for business impact — critical business process disruptions may be classified at lower severity than warranted |
| `incident-management` | `escalation-path-not-defined` | No documented or configured escalation path for high or critical severity incidents in Cloud ALM or linked ITSM |
| `incident-management` | `no-problem-record-for-recurring-incident` | Three or more incidents with the same root cause within a 30-day window without a linked problem record and active investigation |
| `incident-management` | `major-incident-rca-not-completed` | Major incident closed without a documented root-cause analysis or corrective action record |
| `root-cause-analysis` | `no-rum-or-synthetic-monitoring` | No Real User Monitoring or synthetic monitoring configured for critical SAP Fiori user journeys — detection relies on user-reported issues |
| `root-cause-analysis` | `alert-incident-traceability-gap` | Alert events are not linked to corresponding incident records — no audit trail from detection to resolution |
| `root-cause-analysis` | `no-cross-system-dependency-map` | No cross-system dependency documentation available in Cloud ALM for root-cause investigation of incidents spanning multiple SAP systems |
| `sla-continuity` | `sla-not-formally-defined` | No formally defined SLA (availability target, response time, resolution time) for a production SAP service covered by Cloud ALM |
| `sla-continuity` | `sla-breach-alerting-absent` | SLA is defined but no Cloud ALM alert is configured to fire when the SLA breach threshold is approached or reached |
| `sla-continuity` | `sla-reporting-not-scheduled` | No scheduled SLA performance report configured in Cloud ALM — SLA adherence is not reviewed by service management |

## Risk classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Production system with no health monitoring coverage at all; complete loss of visibility into a business-critical SAP service; SLA reporting absent for a regulatory-reporting-relevant service with contractual SLA obligations |
| `high` | Alert threshold miscalibration that allows SLA breach before alerting; critical integration interface with no exception monitoring; business-critical incident severity misconfiguration; absence of SLA definition for a contractually committed service; no ITSM integration causing manual incident creation delay for critical alerts |
| `medium` | Problem record absence for recurring incidents; incomplete root-cause analysis for closed major incidents; no SLA breach alerting despite defined SLA; missing business user notification for process SLA breach; no Real User Monitoring for critical Fiori journeys |
| `low` | Best practice deviation without immediate operational risk: missing knowledge article for known issues, suboptimal alert grouping, documentation gap for escalation path that informally exists |

## Remediation path decision tree

For each finding:

1. **Is this a production system with no health monitoring coverage?**
   - Yes → `critical`. Register the system as a managed system in SAP Cloud ALM immediately. Configure health checks per the SAP Cloud ALM health monitoring documentation for the relevant system type. Do not defer production monitoring gaps.
   - No → continue.

2. **Is this an alert threshold that allows SLA breach before detection, or a critical integration interface with no exception monitoring?**
   - Yes → `high`. Recalibrate the alert threshold to give the operations team sufficient time to respond before SLA breach. Configure Cloud ALM integration and exception monitoring for the critical interface. Define notification channels and ITSM integration.
   - No → continue.

3. **Is this a business-critical incident with missing severity classification or escalation path?**
   - Yes → `high`. Update the incident severity classification model to include business impact criteria. Define and document escalation paths for high and critical severity incidents. Test the escalation workflow.
   - No → continue.

4. **Is this a recurring incident pattern without a problem record and root-cause investigation?**
   - Yes → `medium`. Create a problem record in Cloud ALM or the linked ITSM. Assign an owner and initiate a root-cause investigation. Set a target resolution date. Review all linked incidents for shared root cause.
   - No → continue.

5. **Is this an SLA definition gap, missing SLA breach alerting, or absent SLA reporting?**
   - Yes → `medium` (escalating to `critical` if the SLA is contractually committed and regulatory reporting is at risk). Define the SLA formally in Cloud ALM. Configure breach threshold alerting. Schedule SLA performance reporting for service management review.
   - No → classify as `low` and provide SRE best practice guidance.

## Workflow

1. **Receive artifacts** — Cloud ALM monitoring configuration descriptions, alert rule lists, business process monitoring key figure definitions, incident management process documentation, SLA definitions, or written operations governance descriptions.
2. **Classify each finding** by SRE governance domain and finding class above.
3. **Assign risk level** per risk classification table (critical / high / medium / low).
4. **Flag critical monitoring gaps immediately** — production systems with no health monitoring coverage must be escalated to the operations team and remediated before other findings are discussed.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical monitoring gaps first; then high alerting, integration monitoring, and incident management findings; then medium problem management and SLA gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. SRE governance domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. System or process detail (if applicable): managed system name or type, integration scenario, business process, alert rule name, or SLA definition
5. Recommended governance control per finding (managed system registration, health check configuration, alert threshold recalibration, integration monitoring enablement, ITSM integration, problem record creation, SLA definition, SLA breach alerting, etc.)
6. Operations posture after remediation: MTTD and MTTR impact estimate, monitoring coverage completeness, business process visibility
7. Escalation notice for critical production monitoring gaps — explicit statement that the operations team must be alerted immediately and remediation must not be deferred
8. Prioritized remediation sequence
