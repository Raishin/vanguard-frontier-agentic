# Official sources — SAP Cloud ALM SRE and Incident Review

Use this reference when grounding health monitoring coverage assessment, alert rule calibration review, integration and exception monitoring evaluation, business process monitoring review, incident and problem management process assessment, root-cause analysis workflow review, and SLA and service continuity governance assessment.

**Evidence level**: documentation-based (SAP Help Portal, SAP Cloud ALM application help documentation). No live-system evidence is collected by this skill.

## SAP Cloud ALM — Product Overview

- What is SAP Cloud ALM
  https://help.sap.com/docs/cloud-alm/applicationhelp/what-is-sap-cloud-alm
  source_owner: SAP SE
  topic_supported: SAP Cloud ALM product scope, use case areas (implementation, operations, service management), supported SAP managed systems (cloud and on-premise), licensing and provisioning model
  why_needed: Primary reference for SAP Cloud ALM capability scope — defines which SAP services and systems are supported as managed systems, which operations use cases are covered, and the product boundary used to assess monitoring coverage completeness
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Health Monitoring

- Health Monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/health-monitoring
  source_owner: SAP SE
  topic_supported: Health check configuration for SAP cloud services and on-premise systems, managed system registration, health status threshold definition, alert generation from health status degradation, monitoring scope per system type
  why_needed: Primary reference for health monitoring coverage assessment — defines the health check model, managed system registration requirements, and health status threshold configuration used to classify monitoring gap and miscalibration findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Integration and Exception Monitoring

- Integration and Exception Monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/integration-and-exception-monitoring
  source_owner: SAP SE
  topic_supported: Integration scenario monitoring in Cloud ALM, iFlow exception surfacing from SAP Integration Suite, API proxy error rate monitoring, interface error alert configuration, end-to-end integration scenario health
  why_needed: Authoritative reference for integration exception monitoring assessment — defines which integration scenarios can be monitored through Cloud ALM, how iFlow exceptions are surfaced, and what alert types are available for integration failure detection
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Business Process Monitoring

- Business Process Monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/business-process-monitoring
  source_owner: SAP SE
  topic_supported: Business process monitoring key figure configuration, step-level process health tracking, SLA breach detection for process milestones, business user notification configuration, process health dashboard
  why_needed: Primary reference for business process monitoring assessment — defines the key figure configuration model, process milestone SLA definition capability, and notification model used to classify business process monitoring coverage and SLA breach detection findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Intelligent Event Processing (Alerting)

- Intelligent Event Processing
  https://help.sap.com/docs/cloud-alm/applicationhelp/intelligent-event-processing
  source_owner: SAP SE
  topic_supported: Alert rule configuration, event processing pipeline in Cloud ALM, notification channel setup, alert routing to operations teams, alert suppression and grouping, integration with ITSM platforms for alert-to-incident automation
  why_needed: Primary reference for alerting governance assessment — defines the alert rule configuration model, notification channel types, ITSM integration for automated incident creation, and alert grouping and suppression capabilities used to classify alert threshold and routing findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — IT Task Management (Incident and Problem Management)

- IT Task Management
  https://help.sap.com/docs/cloud-alm/applicationhelp/it-task-management
  source_owner: SAP SE
  topic_supported: Incident and problem record management in Cloud ALM, severity classification model, escalation path configuration, ITSM platform integration, knowledge article linkage, task assignment and workflow
  why_needed: Required to assess incident and problem management governance — defines the incident severity model, problem record linkage capability, ITSM integration patterns, and escalation workflow configuration used to classify incident management process findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Real User Monitoring

- Real User Monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/real-user-monitoring
  source_owner: SAP SE
  topic_supported: Real user experience monitoring for SAP Fiori and browser-based SAP applications, response time tracking, error rate detection, user journey performance metrics, alert generation from performance degradation
  why_needed: Required to assess root-cause analysis workflow completeness — defines the Real User Monitoring capability available in Cloud ALM for detecting performance degradation at the user experience layer and supporting root-cause investigation
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Synthetic User Monitoring

- Synthetic User Monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/synthetic-user-monitoring
  source_owner: SAP SE
  topic_supported: Synthetic transaction monitoring for SAP Fiori applications, proactive availability and performance testing, alert generation from synthetic check failures, test scenario configuration
  why_needed: Required to assess proactive monitoring coverage — defines the synthetic monitoring capability for detecting SAP application availability issues before real users are impacted, used to classify gaps in proactive health detection
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SAP Cloud ALM application help documentation describe the designed monitoring model, alerting capabilities, incident management features, and integration monitoring scope. They do not prove which managed systems are registered in the user's Cloud ALM tenant, what alert thresholds are configured, whether SLAs are formally defined, or how the organization's incident management process operates. Users must supply Cloud ALM configuration descriptions, alert rule lists, business process monitoring key figure definitions, incident management process documentation, SLA definitions, or written operations governance descriptions for concrete assessment.
