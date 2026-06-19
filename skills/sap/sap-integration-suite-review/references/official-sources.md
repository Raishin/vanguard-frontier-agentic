# Official sources — SAP Integration Suite Review

Use this reference when grounding Cloud Integration iFlow design, API Management policy assessment, Event Mesh configuration review, security material evaluation, and monitoring posture assessment.

**Evidence level**: documentation-based (SAP Help Portal, SAP Integration Suite documentation). No live-system evidence is collected by this skill.

## Cloud Integration (SAP Cloud Integration / CPI)

- SAP Cloud Integration overview
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/sap-cloud-integration
  source_owner: SAP SE
  topic_supported: Cloud Integration capabilities, iFlow design concepts, adapter catalog, message processing model
  why_needed: Primary entry point for Cloud Integration documentation; establishes the iFlow design model used to classify integration findings
  evidence_level: primary
  last_verified: 2026-06-19

- Define exception subprocess
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/define-exception-subprocess
  source_owner: SAP SE
  topic_supported: Exception subprocess design, error boundary definition, failed message routing from exception subprocesses
  why_needed: Authoritative reference for the mandatory error handling pattern — any iFlow without an exception subprocess fails the error handling non-negotiable
  evidence_level: primary
  last_verified: 2026-06-19

- Idempotent process call handles duplicates
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/idempotent-process-call-handles-duplicates
  source_owner: SAP SE
  topic_supported: Idempotent receiver pattern in Cloud Integration, duplicate detection using message IDs, JMS retry idempotency
  why_needed: Defines the idempotent process call pattern — required to assess whether retry-capable iFlows implement duplicate protection
  evidence_level: primary
  last_verified: 2026-06-19

- Managing security material
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/managing-security-material
  source_owner: SAP SE
  topic_supported: Credential store, keystore, OAuth credential management, certificate-based adapter authentication
  why_needed: Defines the secure credential storage model — required to flag plaintext credentials in iFlow properties as critical findings
  evidence_level: primary
  last_verified: 2026-06-19

- Message monitoring
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/message-monitoring
  source_owner: SAP SE
  topic_supported: Cloud Integration message monitoring, processing log levels, alerting, retry state visibility
  why_needed: Defines the operational monitoring model — required to assess whether iFlows provide adequate observability for production operations
  evidence_level: primary
  last_verified: 2026-06-19

## API Management

- SAP API Management overview
  https://help.sap.com/docs/sap-api-management/sap-api-management/sap-api-management
  source_owner: SAP SE
  topic_supported: API Management architecture, API proxy design, API product and plan model, developer portal
  why_needed: Primary reference for API Management design model used to classify API proxy governance findings
  evidence_level: primary
  last_verified: 2026-06-19

- OAuth V2.0 policy (API Management)
  https://help.sap.com/docs/sap-api-management/sap-api-management/oauth-v2-0
  source_owner: SAP SE
  topic_supported: OAuth 2.0 policy enforcement in API Management proxies, grant type support, token validation configuration
  why_needed: Defines the OAuth policy model — required to classify API proxies without inbound OAuth enforcement as critical security findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Advanced Event Mesh / Event Mesh

- What is SAP Advanced Event Mesh
  https://help.sap.com/docs/sap-advanced-event-mesh/sap-advanced-event-mesh/what-is-sap-advanced-event-mesh
  source_owner: SAP SE
  topic_supported: Event broker topology, topic design, queue configuration, consumer group model, access control
  why_needed: Defines the Event Mesh architecture model — required to classify topic namespace design, queue binding, and consumer isolation findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Integration Suite documentation describes the designed behavior of iFlow components, API Management policies, and Event Mesh configuration options. It does not prove which iFlows are deployed in the user's tenant, whether policies are correctly configured, or whether monitoring alerts are active. Users must supply iFlow exports, API proxy descriptors, Event Mesh configuration exports, or written descriptions for concrete assessment.
