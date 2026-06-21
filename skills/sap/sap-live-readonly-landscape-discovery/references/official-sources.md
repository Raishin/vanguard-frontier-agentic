# Official sources — SAP Live Read-Only Landscape Discovery

Use this reference when grounding BTP CLI commands, CF CLI commands, destination service access, and ABAP landscape object enumeration.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during sessions is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP BTP CLI

- BTP CLI command reference
  https://help.sap.com/docs/btp/btp-cli-command-reference/btp-cli-command-reference
  source_owner: SAP SE
  topic_supported: Full BTP CLI command catalog — used to verify which commands are list/get/describe (allowed) vs create/update/delete (forbidden)
  why_needed: Authoritative source for BTP CLI read-only command identification
  evidence_level: primary
  last_verified: 2026-06-19

- BTP account administration using BTP CLI
  https://help.sap.com/docs/btp/sap-business-technology-platform/account-administration-using-sap-btp-command-line-interface-btp-cli
  source_owner: SAP SE
  topic_supported: BTP CLI setup, login, context management, subaccount navigation
  why_needed: Required to establish read-only CLI session and navigate subaccount hierarchy
  evidence_level: primary
  last_verified: 2026-06-19

- Managing entitlements and quotas using BTP CLI
  https://help.sap.com/docs/btp/sap-business-technology-platform/managing-entitlements-and-quotas-using-the-btp-cli
  source_owner: SAP SE
  topic_supported: Entitlement and quota read operations via BTP CLI
  why_needed: Defines read-only entitlement enumeration commands (btp list accounts/entitlements)
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Foundry CLI

- CF CLI command reference for SAP BTP
  https://help.sap.com/docs/cloud-foundry/sap-cloud-foundry-command-line-interface/cf-command-reference
  source_owner: SAP SE
  topic_supported: CF CLI read-only commands (cf apps, cf services, cf service-instances, cf env) and role requirements
  why_needed: Authoritative source for CF read-only command identification and SpaceAuditor/OrgAuditor role mapping
  evidence_level: primary
  last_verified: 2026-06-19

## Destinations

- Destinations on SAP BTP
  https://help.sap.com/docs/btp/sap-business-technology-platform/destinations
  source_owner: SAP SE
  topic_supported: Destination service overview, destination types (HTTP, RFC, Mail), destination properties
  why_needed: Context for read-only destination enumeration and interpreting destination configuration output
  evidence_level: primary
  last_verified: 2026-06-19

- Destination Service REST API
  https://help.sap.com/docs/connectivity/sap-btp-connectivity-cf/destination-service-rest-api
  source_owner: SAP SE
  topic_supported: REST API for reading destination configurations — GET operations only for this skill
  why_needed: Defines allowed GET endpoints for destination enumeration when using REST API access
  evidence_level: primary
  last_verified: 2026-06-19

## Authorization and Trust Management

- SAP Authorization and Trust Management Service (XSUAA)
  https://help.sap.com/docs/btp/sap-business-technology-platform/sap-authorization-and-trust-management-service-in-the-cloud-foundry-environment
  source_owner: SAP SE
  topic_supported: Role collections, trust configurations, identity provider setup — read-only enumeration context
  why_needed: Required for read-only enumeration of trust configurations and role collection assignments
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP System Landscape

- S/4HANA Cloud landscape and system administration
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/3e56cde2e1044df5b5f3a5ecc8e59d27.html
  source_owner: SAP SE
  topic_supported: ABAP system landscape overview, transport routes, system definitions — read-only context
  why_needed: Defines ABAP landscape object types this skill can enumerate in read-only mode
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes available CLI commands and API endpoints. It does not prove which commands are available in the user's installed CLI version, which entitlements exist in their account, or whether their assigned role actually restricts write access. Users must confirm credential scope before any live command is executed.
