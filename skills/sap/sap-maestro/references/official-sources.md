# Official sources — SAP Maestro

Use this reference when grounding SAP domain taxonomy and BTP product classification.

**Evidence level**: documentation-based (official SAP Help Portal). No live-system evidence is collected by this skill.

## SAP BTP taxonomy and account model

- SAP Business Technology Platform — product home
  https://help.sap.com/docs/btp/sap-business-technology-platform/sap-business-technology-platform
  source_owner: SAP SE
  topic_supported: BTP product taxonomy, service portfolio overview
  why_needed: Authoritative domain classification for all BTP-native services

- What is SAP BTP
  https://help.sap.com/docs/btp/sap-business-technology-platform/what-is-sap-btp
  source_owner: SAP SE
  topic_supported: Platform overview, pillars (Database & Data Management, Analytics, Application Development, Integration, AI)
  why_needed: Top-level domain taxonomy used by the routing table

- BTP Regions
  https://help.sap.com/docs/btp/sap-business-technology-platform/regions
  source_owner: SAP SE
  topic_supported: Regional availability of services; routing decisions that depend on region scope
  why_needed: Routing to live-tier skills must account for regional availability

- BTP Account Model
  https://help.sap.com/docs/btp/sap-business-technology-platform/account-model
  source_owner: SAP SE
  topic_supported: Global account → directory → subaccount hierarchy; scope of routing decisions
  why_needed: Disambiguates which tenant scope a routed request applies to

- Entitlements and Quotas
  https://help.sap.com/docs/btp/sap-business-technology-platform/entitlements-and-quotas
  source_owner: SAP SE
  topic_supported: Service entitlement context for routing decisions involving provisioned vs. unprovisioned services
  why_needed: Prevents routing to skills whose target service is not entitled in the target subaccount

- BTP Cockpit
  https://help.sap.com/docs/btp/sap-business-technology-platform/sap-btp-cockpit
  source_owner: SAP SE
  topic_supported: Cockpit navigation model; reference for subaccount-scoped routing
  why_needed: Clarifies cockpit scope when routing to landscape discovery or provisioning skills

- Cloud Management Tools Feature Set B
  https://help.sap.com/docs/btp/sap-business-technology-platform/cloud-management-tools-feature-set-b
  source_owner: SAP SE
  topic_supported: Feature Set B account management APIs; relevant for routing to API-driven landscape skills
  why_needed: Routing to API-based skills requires confirming Feature Set A vs B capability boundary

## Grounding rule

SAP Help Portal documentation describes product taxonomy and service scope. It does not prove which services are entitled, deployed, or reachable in the user's specific BTP account. Routing decisions must be validated against user-provided account context before a live-tier dispatch is proposed.
