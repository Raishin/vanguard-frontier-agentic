# Official sources — SAP Security IAM GRC and SoD Review

Use this reference when grounding IAS configuration assessment, IPS connector review, XSUAA scope and role template evaluation, GRC Access Control ruleset analysis, and SoD conflict classification.

**Evidence level**: documentation-based (SAP Help Portal, SAP Cloud Identity Services documentation, SAP GRC Access Control documentation). No live-system evidence is collected by this skill.

## SAP Cloud Identity Services — Identity Authentication (IAS)

- What is Identity Authentication
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/what-is-identity-authentication
  source_owner: SAP SE
  topic_supported: IAS capabilities overview, application assignment model, corporate IdP federation, authentication policies
  why_needed: Primary reference for IAS design model — defines the application, corporate IdP, and authentication policy hierarchy used to classify IAS findings
  evidence_level: primary
  last_verified: 2026-06-19

- Configure risk-based authentication for an application
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/configure-risk-based-authentication-for-application
  source_owner: SAP SE
  topic_supported: Risk-based authentication policies, MFA enforcement rules, IP range restrictions, user attribute conditions
  why_needed: Defines the MFA enforcement model in IAS — required to assess whether privileged users have adequate MFA protection configured
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Identity Services — Identity Provisioning (IPS)

- What is Identity Provisioning
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/what-is-identity-provisioning
  source_owner: SAP SE
  topic_supported: IPS source and target connector model, user and group provisioning lifecycle, transformation script model, job scheduling
  why_needed: Primary reference for IPS connector architecture — defines the source/target connector model and transformation pipeline used to classify IPS provisioning governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Authorization and Trust Management (XSUAA)

- What is Authorization and Trust Management Service
  https://help.sap.com/docs/btp/sap-business-technology-platform/what-is-authorization-and-trust-management-service
  source_owner: SAP SE
  topic_supported: XSUAA service overview, OAuth 2.0 token issuance, scope and role template model, trust to SAP IAS and external IdPs
  why_needed: Defines the XSUAA authorization model — required to classify scope design, role template construction, and trust configuration findings
  evidence_level: primary
  last_verified: 2026-06-19

- Application security descriptor configuration syntax (xs-security.json)
  https://help.sap.com/docs/btp/sap-business-technology-platform/application-security-descriptor-configuration-syntax
  source_owner: SAP SE
  topic_supported: xs-security.json schema, scope definitions, role template attributes, role collection declarations, foreign scope references
  why_needed: Authoritative schema reference for xs-security.json review — required to assess scope granularity, role template construction, and least-privilege compliance in XSUAA application security descriptors
  evidence_level: primary
  last_verified: 2026-06-19

## Trust and federation

- Trust and federation with identity providers
  https://help.sap.com/docs/btp/sap-business-technology-platform/trust-and-federation-with-identity-providers
  source_owner: SAP SE
  topic_supported: Subaccount trust configuration, SAML/OIDC federation to SAP IAS and external IdPs, attribute mapping for group-based role collection assignment
  why_needed: Defines the BTP trust configuration model — required to classify trust misconfigurations, missing attribute mappings, and excessive trust scope findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP GRC Access Control and SoD

- What is SAP GRC Access Control
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/what-is-sap-grc-access-control
  source_owner: SAP SE
  topic_supported: GRC Access Control capabilities, Access Risk Analysis (ARA), Emergency Access Management (EAM), Business Role Management, User Access Review
  why_needed: Primary reference for GRC Access Control model — defines the ARA, EAM, and business role management capabilities used to classify GRC governance findings
  evidence_level: primary
  last_verified: 2026-06-19

- Segregation of Duties
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/segregation-of-duties
  source_owner: SAP SE
  topic_supported: SoD ruleset structure, function and permission entry definitions, risk classification levels (critical/high/medium/low), mitigation control assignment
  why_needed: Defines the SoD conflict classification model — authoritative reference for risk level taxonomy, ruleset coverage assessment, and mitigation control requirements
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and GRC Access Control documentation describe the designed IAM model, XSUAA configuration syntax, and GRC ruleset capabilities. They do not prove which roles are assigned in the user's landscape, what SoD conflicts exist in the user's GRC system, or whether mitigation controls have been approved. Users must supply SoD conflict reports, role lists, xs-security.json files, IAS exports, IPS connector configurations, or written descriptions of their IAM landscape for concrete assessment.
