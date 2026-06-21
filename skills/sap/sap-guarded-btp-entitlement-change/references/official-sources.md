# Official sources — SAP Guarded BTP Entitlement Change

Use this reference when grounding SAP BTP entitlement management procedures, quota allocation, service subscriptions, and usage monitoring.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during steps 8, 9, and 14 is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP BTP Entitlements and Quotas

- SAP BTP entitlements and quotas — concepts
  https://help.sap.com/docs/btp/sap-business-technology-platform/entitlements-and-quotas
  source_owner: SAP SE
  topic_supported: Entitlement model concepts — service plans, quota, distribution to directories and subaccounts, global account vs. subaccount entitlement scope, commercial models (free / metered / subscription / block)
  why_needed: Authoritative source for the entitlement model used in steps 1 and 3 (classify and criticality); defines quota, service plan, and billing model concepts required to assess cost impact in step 9
  evidence_level: primary
  last_verified: 2026-06-19

- SAP BTP — managing entitlements and quotas using the cockpit
  https://help.sap.com/docs/btp/sap-business-technology-platform/managing-entitlements-and-quotas-using-the-cockpit
  source_owner: SAP SE
  topic_supported: Step-by-step cockpit procedures for assigning entitlements to directories and subaccounts, adjusting quota, enabling and disabling service plans, bulk entitlement configuration
  why_needed: Grounds step 14 execution procedures and step 8 read-only inspection for users operating via the BTP cockpit rather than the Entitlements API
  evidence_level: primary
  last_verified: 2026-06-19

- SAP BTP global account administration
  https://help.sap.com/docs/btp/sap-business-technology-platform/global-account-administration
  source_owner: SAP SE
  topic_supported: Global account hierarchy, directory structure, subaccount governance, administrator roles and least-privilege guidance for global account operations
  why_needed: Grounds step 2 (target confirmation) and least-privilege credential rules in live-environment-access.md; defines the difference between Global Account Administrator and Entitlements Administrator scope
  evidence_level: primary
  last_verified: 2026-06-19

## SAP BTP Entitlements API

- SAP BTP Entitlements API (SAP Cloud Management Service)
  https://help.sap.com/docs/btp/btp-admin-and-ops-neo/entitlements-api
  source_owner: SAP SE
  topic_supported: Entitlements API endpoints — GET assigned entitlements, GET available service plans, PUT assign entitlement quota to subaccount; authentication and service key setup for the Cloud Management Service
  why_needed: Defines the exact API calls permitted for read-only state inspection (step 8) and entitlement assignment execution (step 14); grounds the command patterns in the workflow reference
  evidence_level: primary
  last_verified: 2026-06-19

## SAP BTP Service Subscriptions

- SAP BTP — subscribe to multitenant applications using the cockpit
  https://help.sap.com/docs/btp/sap-business-technology-platform/subscribe-to-multitenant-applications-using-the-cockpit
  source_owner: SAP SE
  topic_supported: Service subscription model, subscribing and unsubscribing from multitenant applications, subscription status and dependency management
  why_needed: Grounds step 1 (classify subscription changes), step 10 (blast radius for subscription removals — dependent applications lose access), and step 14 (subscription enable/remove procedures)
  evidence_level: primary
  last_verified: 2026-06-19

## SAP BTP Cloud Management Service (APIs)

- SAP BTP account administration using the Cloud Management Service APIs
  https://help.sap.com/docs/btp/sap-business-technology-platform/account-administration-using-apis-of-the-sap-cloud-management-service
  source_owner: SAP SE
  topic_supported: Cloud Management Service API overview — entitlements, provisioning, subscription management; authentication setup; scope requirements for Entitlements Administrator and Subaccount Service Administrator
  why_needed: Authoritative source for the API authentication and scope setup described in live-environment-access.md; provides the token endpoint and scope details for Entitlements API calls in steps 8 and 14
  evidence_level: primary
  last_verified: 2026-06-19

## SAP BTP Usage and Billing Monitoring

- SAP Usage Data Management Service — monitoring usage via APIs
  https://help.sap.com/docs/btp/sap-business-technology-platform/monitoring-usage-information-using-apis-of-the-sap-usage-data-management-service
  source_owner: SAP SE
  topic_supported: Usage Data Management Service API — GET metered consumption data, usage reports by subaccount and service, cost exposure tracking
  why_needed: Grounds step 9 (cost-impact assessment using current consumption baseline) and step 15 (post-change billing verification — confirm metered tracking is active and cost center is correct)
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the BTP entitlement model design intent, cockpit procedures, and API specifications. It does not prove which entitlements are currently assigned in the user's global account, what the current metered consumption is, or whether the user's SAP BTP contract includes the service plan being requested. Users must supply live evidence from steps 8, 9, and 15 for concrete entitlement and cost assessment. SAP BTP pricing and commercial terms must be confirmed with the user's SAP contract or Customer Success Manager — never inferred from documentation alone.
