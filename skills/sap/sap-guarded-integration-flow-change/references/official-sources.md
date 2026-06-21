# Official sources — SAP Guarded Integration Flow Change

Use this reference when grounding SAP Cloud Integration deployment procedures, OData API usage, message monitoring operations, and artifact versioning.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during steps 8, 9, and 14 is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP Cloud Integration — Deployment

- SAP Cloud Integration — deploy integration artifacts
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/deploy-integration-artifacts
  source_owner: SAP SE
  topic_supported: Deployment of iFlows, value mappings, script collections, and API providers to a Cloud Integration runtime; deployment status lifecycle; error states and recovery
  why_needed: Authoritative source for the deployment procedures used in step 14 and the deployment status inspection used in step 15; defines allowed artifact types for step 1
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud Integration — manage integration content
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/manage-integration-content
  source_owner: SAP SE
  topic_supported: Integration package and artifact management in the design workspace; undeploy, activate, and deactivate operations; content transport between tenants
  why_needed: Grounds step 7 (scope documentation) and step 11 (rollback plan) with the exact package management and previous-version redeploy procedures available in Cloud Integration
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud Integration — versioning of artifacts
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/versioning-of-artifacts
  source_owner: SAP SE
  topic_supported: Artifact versioning in the design workspace, draft vs. active versions, version history, restoring previous versions
  why_needed: Authoritative source for step 9 (diff of artifact changes between design workspace version and deployed version) and step 11 (rollback via previous version redeploy); defines the versioning model used to produce the change diff
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Integration — OData API

- SAP Cloud Integration OData API reference
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/odata-api
  source_owner: SAP SE
  topic_supported: Cloud Integration OData API endpoints — GET IntegrationRuntimeArtifacts (read deployed status), POST deploy, GET MessageProcessingLogs; authentication and service key setup
  why_needed: Defines the exact API calls permitted for read-only state inspection (step 8), artifact deployment (step 14), and post-deployment verification (step 15); grounds the command patterns in the workflow reference
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Integration — Message Monitoring

- SAP Cloud Integration — message processing log
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/message-processing-log
  source_owner: SAP SE
  topic_supported: Message processing log (MPL) structure, status values (Completed / Failed / Retry / Escalated), log levels, correlation IDs, attachment inspection
  why_needed: Grounds step 15 (post-deployment verification via message monitoring); defines the status values and log structure used to confirm successful processing after deployment
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud Integration — monitor message processing
  https://help.sap.com/docs/cloud-integration/sap-cloud-integration/monitor-message-processing
  source_owner: SAP SE
  topic_supported: Message monitoring cockpit, monitoring tile navigation, filtering by integration flow and time range, throughput analysis
  why_needed: Provides the monitoring procedures for step 8 (read-only current state — error rate baseline) and step 15 (post-deployment verification — throughput resume confirmation)
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Integration Suite — Capabilities and Roles

- SAP Integration Suite — activating and managing capabilities
  https://help.sap.com/docs/integration-suite/sap-integration-suite/activating-and-managing-capabilities
  source_owner: SAP SE
  topic_supported: Integration Suite capability activation, role collection assignments for Cloud Integration, API Management, and Event Mesh; least-privilege role guidance
  why_needed: Grounds step 5 (integration owner identification) and the least-privilege credential rules in live-environment-access.md; defines the Integration Developer and Integration Suite roles required for deployment operations
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes Cloud Integration deployment design intent, OData API specifications, and monitoring architecture. It does not prove which artifacts are currently deployed in the user's tenant, what the current message processing error rate is, or whether downstream partner systems are reachable from the target tenant. Users must supply live evidence from steps 8, 9, and 15 for concrete integration state assessment.
