# Official sources — SAP Integration / Platform / Business Operations Protocol

Use this reference when grounding findings on SAP Integration Suite iFlow design, adapter behavior, error handling, Cloud ALM health monitoring, Event Mesh delivery, and BTP platform entitlements relevant to integration scenarios.

**Evidence level**: documentation-based (SAP Help Portal). No live-system evidence is collected by this protocol.

## SAP Integration Suite — product and iFlow design

- What is SAP Integration Suite
  https://help.sap.com/docs/integration-suite/sap-integration-suite/what-is-sap-integration-suite
  source_owner: SAP SE
  topic_supported: Integration Suite capabilities overview, Cloud Integration, API Management, Event Mesh, Open Connectors, Integration Advisor; product scope and runtime architecture
  why_needed: Baseline taxonomy for classifying which Integration Suite capability is involved in a failure — required to activate the correct participating role
  evidence_level: primary
  last_verified: 2026-06-19

- Integration flow design guidelines
  https://help.sap.com/docs/integration-suite/sap-integration-suite/integration-flow-design-guidelines
  source_owner: SAP SE
  topic_supported: iFlow design best practices, message exchange patterns, idempotency, retry configuration, dead-letter handling, logging strategy
  why_needed: Primary reference for assessing whether iFlow design contributed to a failure; defines the quality bar against which reviewer findings are classified
  evidence_level: primary
  last_verified: 2026-06-19

- Error handling in integration flows
  https://help.sap.com/docs/integration-suite/sap-integration-suite/error-handling-in-integration-flows
  source_owner: SAP SE
  topic_supported: Exception subprocess configuration, escalation handling, retry policies, dead-letter queue handling, error message enrichment
  why_needed: Defines the canonical error-handling patterns — required to assess whether a flow failure is a design defect, a configuration gap, or a runtime/infrastructure issue
  evidence_level: primary
  last_verified: 2026-06-19

- Adapter concepts
  https://help.sap.com/docs/integration-suite/sap-integration-suite/adapter-concepts
  source_owner: SAP SE
  topic_supported: Adapter types (SOAP, REST, OData, SFTP, AS2, JDBC, JMS, etc.), adapter connection properties, credential management, receiver adapter retry behavior
  why_needed: Reference for adapter-level failure classification — required when a connectivity failure, authentication error, or throttling response needs to be attributed to a specific adapter type
  evidence_level: primary
  last_verified: 2026-06-19

## Message processing and monitoring

- Message processing log
  https://help.sap.com/docs/integration-suite/sap-integration-suite/message-processing-log
  source_owner: SAP SE
  topic_supported: Message processing log structure, log levels, error status codes, attachment handling, retention policies, access via Operations view
  why_needed: Defines the primary monitoring artifact submitted as evidence — required to interpret error codes, message status transitions, and retry counts in user-provided log excerpts
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — operations and integration monitoring

- SAP Cloud ALM operations
  https://help.sap.com/docs/cloud-alm/applicationhelp/operations
  source_owner: SAP SE
  topic_supported: Cloud ALM operations workspace, health monitoring, job monitoring, alert management, integration and exception monitoring across the SAP managed landscape
  why_needed: Primary reference for the Cloud ALM SRE incident role — defines the health monitoring framework and alert classification model used to triage platform and integration incidents
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud ALM integration monitoring
  https://help.sap.com/docs/cloud-alm/applicationhelp/integration-monitoring
  source_owner: SAP SE
  topic_supported: Integration and exception monitoring in Cloud ALM, message flow visibility, error drill-down, cross-system end-to-end tracing
  why_needed: Defines the integration-specific monitoring evidence the Cloud ALM SRE role uses to correlate iFlow failures with business-process impact and SLA breach thresholds
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Event Mesh — event delivery and dead-letter handling

- What is SAP Event Mesh
  https://help.sap.com/docs/sap-event-mesh/sap-event-mesh/what-is-sap-event-mesh
  source_owner: SAP SE
  topic_supported: SAP Event Mesh product overview, messaging concepts, topic subscriptions, queues, event broker capabilities, client SDK
  why_needed: Baseline taxonomy for event delivery failure classification — required to distinguish between topic publication failures, subscription delivery failures, and dead-letter queue accumulation
  evidence_level: primary
  last_verified: 2026-06-19

- Dead-letter queue in SAP Event Mesh
  https://help.sap.com/docs/sap-event-mesh/sap-event-mesh/dead-letter-queue
  source_owner: SAP SE
  topic_supported: Dead-letter queue behavior, message retry exhaustion, DLQ inspection, reprocessing options, message loss risk
  why_needed: Defines the dead-letter queue mechanics — required to assess whether stuck or failed Event Mesh messages are recoverable and what action is needed to reprocess or discard them
  evidence_level: primary
  last_verified: 2026-06-19

## BTP entitlements — platform layer context for integration failures

- Entitlements and quotas
  https://help.sap.com/docs/btp/sap-business-technology-platform/entitlements-and-quotas
  source_owner: SAP SE
  topic_supported: Service entitlements, quota assignments, entitlement management lifecycle — relevant when integration failures are caused by quota exhaustion or missing service entitlements at the BTP platform layer
  why_needed: Required when the BTP account entitlement governance reviewer role is activated — confirms whether platform-layer entitlement gaps are a contributing factor to integration failures
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed capabilities, configuration options, and operational behavior of SAP Integration Suite, Cloud ALM, and Event Mesh. It does not prove the root cause of a specific failure in the user's tenant, nor does it confirm the configuration state of a specific iFlow or adapter. Users must supply message processing log excerpts, Cloud ALM alert data, adapter configuration descriptions, and incident timelines for concrete advisory output. All credentials, tokens, and PII must be redacted before submission.
