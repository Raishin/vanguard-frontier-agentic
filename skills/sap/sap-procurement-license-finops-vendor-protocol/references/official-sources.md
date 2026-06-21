# Official sources — SAP Procurement / License / FinOps / Vendor Protocol

Use this reference when grounding findings on BTP commercial models, consumption billing, license compliance, RISE with SAP contractual terms, SAP Ariba procurement governance, and FinOps cost attribution.

**Evidence level**: documentation-based (SAP Help Portal, SAP product pages, official SAP support resources). No live-system evidence is collected by this protocol.

## BTP commercial models and consumption billing

- SAP BTP commercial models
  https://help.sap.com/docs/btp/sap-business-technology-platform/commercial-models
  source_owner: SAP SE
  topic_supported: CPEA, BTPEA, subscription-based, and pay-as-you-go commercial models; credit consumption mechanics; cost attribution to subaccounts
  why_needed: Primary taxonomy for classifying consumption anomalies, entitlement over-provisioning, and credit burn rate findings
  evidence_level: primary
  last_verified: 2026-06-19

- Monitoring usage and consumption costs in your global account
  https://help.sap.com/docs/btp/sap-business-technology-platform/monitoring-usage-and-consumption-costs-in-your-global-account
  source_owner: SAP SE
  topic_supported: BTP cockpit usage reports, cost monitoring views, subaccount-level consumption breakdown, credit depletion alerts
  why_needed: Defines the evidence artifacts the FinOps role uses to assess consumption spike events and attribute costs to responsible subaccounts
  evidence_level: primary
  last_verified: 2026-06-19

- What is BTPEA (BTP Enterprise Agreement)
  https://help.sap.com/docs/btp/sap-business-technology-platform/what-is-btpea
  source_owner: SAP SE
  topic_supported: BTPEA structure, cloud credits, minimum commitment, overage mechanics, service coverage scope
  why_needed: Defines how BTPEA credits are allocated and consumed — required for assessing contractual exposure when actual consumption exceeds commitment
  evidence_level: primary
  last_verified: 2026-06-19

- Cloud credits
  https://help.sap.com/docs/btp/sap-business-technology-platform/cloud-credits
  source_owner: SAP SE
  topic_supported: Cloud credit allocation, credit validity periods, credit burn tracking, CPEA credit top-up procedures
  why_needed: Authoritative reference for credit lifecycle governance — required when assessing credit depletion risk and true-up obligations
  evidence_level: primary
  last_verified: 2026-06-19

## Entitlements and quota governance

- Entitlements and quotas
  https://help.sap.com/docs/btp/sap-business-technology-platform/entitlements-and-quotas
  source_owner: SAP SE
  topic_supported: Service entitlements, quota assignment, entitlement distribution to directories and subaccounts, idle entitlement identification
  why_needed: Defines the entitlement model used by the BTP entitlement governance role to classify over-provisioned or underused service allocations
  evidence_level: primary
  last_verified: 2026-06-19

## SAP License measurement and compliance

- SAP Customer Relationship and Usage Management (CRUM)
  https://support.sap.com/en/my-support/systems-installations/crum.html
  source_owner: SAP SE
  topic_supported: SAP license measurement reporting, named user classification, indirect access assessment, license audit preparation
  why_needed: Authoritative SAP support resource for license compliance events; defines the measurement methodology used to assess user count compliance
  evidence_level: primary
  last_verified: 2026-06-19

## RISE with SAP contractual and SLA governance

- RISE with SAP — What is RISE with SAP
  https://www.sap.com/products/erp/rise-with-sap/what-is-rise-with-sap.html
  source_owner: SAP SE
  topic_supported: RISE with SAP offer structure, hyperscaler options, included services, migration path, commercial model
  why_needed: Baseline reference for understanding the contractual scope of RISE — required to assess SLA breach exposure and vendor exit risk
  evidence_level: primary
  last_verified: 2026-06-19

- RISE with SAP cloud services schedule
  https://help.sap.com/docs/rise-with-sap/rise-with-sap-cloud-contract/cloud-services-schedule
  source_owner: SAP SE
  topic_supported: RISE cloud services schedule, SLA definitions, support tiers, hyperscaler-specific terms, contract schedule structure
  why_needed: Primary contractual reference for RISE SLA assessment and vendor-risk findings; defines the SLA metrics the RISE/SLA vendor-risk role evaluates
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Ariba procurement governance

- About SAP Ariba Contracts
  https://help.sap.com/docs/sap-ariba/sap-ariba-contracts/about-sap-ariba-contracts
  source_owner: SAP SE
  topic_supported: Ariba contract management lifecycle, contract compliance monitoring, contract deviation detection, maverick spend identification
  why_needed: Defines the Ariba contract governance model used by the procurement role to assess value leakage and sourcing compliance
  evidence_level: primary
  last_verified: 2026-06-19

- About Ariba Sourcing
  https://help.sap.com/docs/sap-ariba/ariba-sourcing/about-ariba-sourcing
  source_owner: SAP SE
  topic_supported: Ariba sourcing event management, supplier selection, RFx process governance, sourcing compliance
  why_needed: Reference for assessing sourcing-side value leakage when vendor agreements were established outside controlled Ariba sourcing processes
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed commercial model, entitlement mechanics, and contractual framework. It does not prove the current consumption state, license compliance posture, or contract status of the user's SAP estate. Users must supply consumption exports, entitlement inventories, license measurement reports, and contract references for concrete advisory output. All contract-specific terms, pricing, and penalty clauses are user-supplied and must be redacted before submission.
