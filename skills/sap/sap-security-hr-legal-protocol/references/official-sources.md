# Official sources — SAP Security HR Legal Escalation Protocol

Use this reference when grounding SoD conflict classification, HR data sensitivity assessment, identity lifecycle governance, GRC emergency access review, and regulatory compliance framing for cross-functional escalations.

**Evidence level**: documentation-based (SAP Help Portal, SAP GRC Access Control documentation, SAP SuccessFactors data protection guidance, NIST SP 800-53, ISO 27001). No live-system evidence is collected by this skill.

## SAP GRC Access Control

- What is SAP GRC Access Control
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/what-is-sap-grc-access-control
  source_owner: SAP SE
  topic_supported: GRC Access Control capabilities, Access Risk Analysis, Emergency Access Management, Business Role Management, User Access Review
  why_needed: Primary reference for SoD conflict classification and escalation requirements used throughout this protocol
  evidence_level: primary
  last_verified: 2026-06-19

- Segregation of Duties
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/segregation-of-duties
  source_owner: SAP SE
  topic_supported: SoD ruleset structure, risk classification levels (critical/high/medium/low), mitigation control assignment
  why_needed: Defines the SoD conflict risk taxonomy applied when classifying fraud-sensitive access patterns in this protocol
  evidence_level: primary
  last_verified: 2026-06-19

- Access Risk Analysis
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/access-risk-analysis
  source_owner: SAP SE
  topic_supported: ARA execution model, risk simulation, batch risk analysis, remediation workflow
  why_needed: Required to understand how GRC generates the SoD conflict reports that serve as evidence in this protocol
  evidence_level: primary
  last_verified: 2026-06-19

- Emergency Access Management
  https://help.sap.com/docs/sap-grc-access-control/sap-grc-access-control/emergency-access-management
  source_owner: SAP SE
  topic_supported: Firefighter ID assignment, session logging, log review workflow, EAM owner and controller roles
  why_needed: Required to assess privileged-access anomalies for firefighter users — defines logging obligations and log review requirements referenced in this protocol
  evidence_level: primary
  last_verified: 2026-06-19

## SAP SuccessFactors — HR Data Protection

- Data Protection and Privacy in Employee Central
  https://help.sap.com/docs/successfactors-employee-central/employee-central/data-protection-and-privacy-in-employee-central
  source_owner: SAP SE
  topic_supported: HR data sensitivity classifications, personal data categories in SuccessFactors Employee Central, data subject rights, purpose limitation
  why_needed: Primary reference for HR data sensitivity assessment and redaction policy — defines which SuccessFactors data elements are personal or sensitive personal data
  evidence_level: primary
  last_verified: 2026-06-19

- Data Protection and Privacy in SAP SuccessFactors
  https://help.sap.com/docs/successfactors-platform/successfactors-platform/data-protection-and-privacy
  source_owner: SAP SE
  topic_supported: SuccessFactors platform-level data protection, consent management, data purge, audit logging
  why_needed: Platform-level data protection obligations that govern how HR evidence may be collected, shared, and retained under this protocol
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Identity Services

- What is Identity Authentication
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/what-is-identity-authentication
  source_owner: SAP SE
  topic_supported: IAS application model, corporate IdP federation, MFA enforcement, risk-based authentication
  why_needed: Required to assess identity-layer evidence when evaluating account compromise or access anomaly trigger conditions
  evidence_level: primary
  last_verified: 2026-06-19

## Risk and Control Frameworks

- NIST Special Publication 800-53 Rev 5 — Security and Privacy Controls for Information Systems and Organizations
  https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final
  source_owner: NIST
  topic_supported: AC (Access Control), AU (Audit and Accountability), IR (Incident Response), PS (Personnel Security) control families; insider threat program guidance
  why_needed: Secondary framework reference for insider-risk escalation framing and audit package requirements; NIST AC-5 (Separation of Duties) and PS-4 (Personnel Termination) directly support the leaver and SoD escalation paths in this protocol
  evidence_level: secondary
  last_verified: 2026-06-19

- ISO/IEC 27001 — Information Security Management Systems
  https://www.iso.org/standard/27001
  source_owner: ISO/IEC
  topic_supported: Annex A controls for access management, HR security, incident management, and compliance; risk treatment requirements
  why_needed: Secondary framework providing the ISO 27001 Annex A context for the access control and HR security obligations cited in this protocol's escalation and decision-rights tables
  evidence_level: secondary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed capabilities and configuration models for GRC Access Control, SuccessFactors, and Cloud Identity Services. It does not prove which conflicts exist in the user's GRC system, what HR data was accessed, or which lifecycle events were missed. Users must supply GRC conflict reports, access logs, role exports, IPS provisioning logs, and HR-provided lifecycle confirmations for concrete protocol execution.
