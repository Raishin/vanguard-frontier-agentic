# Official sources — SAP SuccessFactors HR Process Risk Review

Use this reference when grounding Employee Central RBP configuration assessment, org and position management review, hire-to-retire process control evaluation, payroll integration governance, data privacy and GDPR compliance assessment, and JML lifecycle review.

**Evidence level**: documentation-based (SAP Help Portal, SAP SuccessFactors Employee Central implementation documentation). No live-system evidence is collected by this skill.

## SAP SuccessFactors Employee Central — Role-Based Permissions (RBP)

- Role-Based Permissions
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/role-based-permissions
  source_owner: SAP SE
  topic_supported: RBP model overview, permission groups, permission roles, target population configuration, field-level access control in Employee Central
  why_needed: Primary reference for RBP design assessment — defines the permission group and permission role hierarchy, target population model, and field-level visibility controls used to classify RBP access risk findings
  evidence_level: primary
  last_verified: 2026-06-19

- Setting Up Role-Based Permission
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/setting-up-role-based-permission
  source_owner: SAP SE
  topic_supported: Step-by-step RBP configuration, permission role assignment to permission groups, target population scoping, sensitive data field permission categories
  why_needed: Authoritative implementation reference for RBP configuration — required to assess whether sensitive HR data fields (compensation, national ID, bank details, home address) are correctly restricted to appropriate permission roles
  evidence_level: primary
  last_verified: 2026-06-19

## SAP SuccessFactors Employee Central — Org and Position Management

- Position Management
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/position-management
  source_owner: SAP SE
  topic_supported: Position object configuration, position-to-headcount linkage, approval workflow for position creation and reclassification, position hierarchy and reporting line governance
  why_needed: Defines the position management model in Employee Central — required to assess approval workflow enforcement, headcount plan linkage integrity, and reporting line change auditability
  evidence_level: primary
  last_verified: 2026-06-19

## SAP SuccessFactors Employee Central — Hire to Retire

- Hire and Rehire
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/hire-and-rehire
  source_owner: SAP SE
  topic_supported: New hire workflow configuration, rehire duplicate detection, onboarding event triggers, hire event field completeness requirements
  why_needed: Defines the hire and rehire event model — required to assess onboarding workflow step completeness, duplicate employee record prevention controls, and hire event field coverage for downstream system provisioning
  evidence_level: primary
  last_verified: 2026-06-19

- Termination
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/termination
  source_owner: SAP SE
  topic_supported: Termination event configuration, termination action steps (access revocation triggers, final pay event, equipment return workflow), data retention post-termination
  why_needed: Defines the termination event model — authoritative reference for assessing whether termination action steps are complete, access revocation is triggered, and data archiving is configured correctly
  evidence_level: primary
  last_verified: 2026-06-19

## SAP SuccessFactors Employee Central — Integration

- Integration with SAP SuccessFactors Employee Central
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/integration-with-sap-successfactors-employee-central
  source_owner: SAP SE
  topic_supported: Employee Central integration model, Integration Center configuration, data replication to payroll processors, field mapping scope, error handling for replication events
  why_needed: Defines the Employee Central integration architecture — required to assess payroll integration field mapping correctness, replication error handling, and downstream system propagation of HR status changes
  evidence_level: primary
  last_verified: 2026-06-19

## SAP SuccessFactors Employee Central — Data Privacy and GDPR

- Data Privacy and Protection
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/data-privacy-and-protection
  source_owner: SAP SE
  topic_supported: Data privacy framework in Employee Central, consent management, data subject access request process, data retention and archiving policy configuration, cross-border transfer controls
  why_needed: Primary reference for GDPR and data privacy compliance assessment — defines the consent management model, data subject rights implementation (access, erasure, portability), and data retention configuration in Employee Central
  evidence_level: primary
  last_verified: 2026-06-19

- Personal Data in Employee Central
  https://help.sap.com/docs/successfactors-employee-central/employee-central-implementation/personal-data-in-employee-central
  source_owner: SAP SE
  topic_supported: PII field classification in Employee Central, personal data entity types, sensitive data categories (national ID, bank details, compensation, health data), field visibility controls
  why_needed: Authoritative reference for PII field identification — required to classify which Employee Central fields contain personal data and to assess whether field-level visibility controls are aligned with the data sensitivity level
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SuccessFactors Employee Central documentation describe the designed RBP model, position management configuration, hire-to-retire event model, integration framework, and data privacy capabilities. They do not prove which permission roles are assigned in the user's landscape, whether termination action steps are correctly configured, or whether GDPR erasure workflows have been tested. Users must supply RBP configuration exports, permission role lists, org chart descriptions, integration mapping documentation, data privacy impact assessment summaries, or written process descriptions for concrete assessment. Raw employee data, national IDs, bank account numbers, or salary figures must never be supplied.
