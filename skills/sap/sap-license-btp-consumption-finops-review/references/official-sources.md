# Official sources — SAP License and BTP Consumption FinOps Review

Use this reference when grounding license category classification, BTP commercial model assessment, consumption optimization guidance, FUE and digital access licensing evaluation, and cost allocation best practices.

**Evidence level**: documentation-based (SAP BTP Help Portal, SAP Support Portal, SAP product documentation). No live-system evidence is collected by this skill.

## BTP commercial models

- What Is the Consumption-Based Commercial Model
  https://help.sap.com/docs/btp/sap-business-technology-platform/what-is-consumption-based-commercial-model
  source_owner: SAP SE
  topic_supported: Overview of BTP consumption-based commercial model, differences between CPEA, Pay-As-You-Go, and subscription options, credit unit mechanics
  why_needed: Primary reference for understanding the BTP commercial model landscape — required to classify the user's commercial model before assessing optimization levers
  evidence_level: primary
  last_verified: 2026-06-19

- Cloud Platform Enterprise Agreement (CPEA)
  https://help.sap.com/docs/btp/sap-business-technology-platform/cloud-platform-enterprise-agreement
  source_owner: SAP SE
  topic_supported: CPEA credit model, credit consumption mechanics, service pricing under CPEA, credit expiry, CPEA entitlement management, forecasting and budget controls
  why_needed: Canonical reference for CPEA credit optimization — defines how credits are consumed, how services are priced against credits, and how unused credits expire; required for all CPEA FinOps assessments
  evidence_level: primary
  last_verified: 2026-06-19

- BTP Commercial Models Overview
  https://help.sap.com/docs/btp/sap-business-technology-platform/commercial-models
  source_owner: SAP SE
  topic_supported: Comparison of BTP commercial models (CPEA, subscription, Pay-As-You-Go), service eligibility per model, switching between models, model selection criteria
  why_needed: Required to assess commercial model fit — determines whether the user's consumption pattern is better served by CPEA, subscription, or Pay-As-You-Go and identifies mismatches that drive unnecessary spend
  evidence_level: primary
  last_verified: 2026-06-19

## BTP consumption monitoring and cost management

- Monitor Usage and Costs
  https://help.sap.com/docs/btp/sap-business-technology-platform/monitor-usage-and-costs
  source_owner: SAP SE
  topic_supported: BTP cockpit usage and cost monitoring, consumption reports, cost center allocation, budget alerts, subaccount-level cost visibility
  why_needed: Defines the SAP-native cost monitoring capabilities — required to assess whether the user has adequate visibility into BTP consumption and cost allocation, and to identify gaps in cost observability
  evidence_level: primary
  last_verified: 2026-06-19

- Managing Entitlements and Quotas Using the Cockpit
  https://help.sap.com/docs/btp/sap-business-technology-platform/managing-entitlements-and-quotas-using-the-cockpit
  source_owner: SAP SE
  topic_supported: BTP entitlement assignment and quota management, service plan entitlements, directory and subaccount quota distribution
  why_needed: Required to assess whether entitlement assignments align with actual consumption — the basis for identifying underutilization and quota right-sizing opportunities
  evidence_level: primary
  last_verified: 2026-06-19

## SAP licensing and digital access

- SAP License Audit and Compliance
  https://support.sap.com/en/my-support/license-audit-and-compliance.html
  source_owner: SAP SE
  topic_supported: SAP True-Up process, License Administration Workbench (LAW), license measurement methodology, audit preparation, indirect and digital access measurement
  why_needed: Primary reference for understanding SAP's license audit and True-Up methodology — required for audit readiness assessment and for interpreting LAW measurement output
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Digital Access
  https://www.sap.com/products/erp/s4hana/digital-access.html
  source_owner: SAP SE
  topic_supported: Digital access licensing model, document-based pricing, which third-party integration scenarios require digital access licenses, digital access adoption program
  why_needed: Primary reference for digital access licensing — defines which integration scenarios create digital access license obligations and how document counts are measured; required for indirect access risk assessment
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP BTP commercial model documentation and SAP licensing guidance describe the designed commercial and measurement frameworks. They do not prove what licenses or BTP entitlements the user holds, what their actual consumption is, or how SAP would measure their system in a True-Up. Users must supply entitlement reports, BTP consumption exports, SAP for Me data, or LAW measurement output for concrete FinOps assessment. This skill does not provide legal advice on license contract terms.
