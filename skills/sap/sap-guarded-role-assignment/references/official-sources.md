# Official sources — SAP Guarded Role Assignment

Use this reference when grounding SAP role collection management, PFCG authorization procedures, Cloud Identity Services operations, and GRC/access governance controls.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during steps 8, 9, and 14 is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP BTP Role Collections

- SAP BTP role collections and roles in global accounts, directories, and subaccounts
  https://help.sap.com/docs/btp/sap-business-technology-platform/role-collections-and-roles-in-global-accounts-directories-and-subaccounts
  source_owner: SAP SE
  topic_supported: BTP role collection structure, role assignment scopes, global account vs. subaccount role collections, predefined vs. custom role collections
  why_needed: Authoritative source for BTP role collection architecture used in steps 1, 2, 7, 8, and 14 when the target is a BTP subaccount or global account
  evidence_level: primary
  last_verified: 2026-06-19

- SAP BTP — assign role collections to users or user groups
  https://help.sap.com/docs/btp/sap-business-technology-platform/assign-role-collections-to-users-or-user-groups
  source_owner: SAP SE
  topic_supported: Step-by-step role collection assignment and revocation procedures for BTP subaccounts; API and cockpit methods; group-based assignment
  why_needed: Defines the exact assignment and revocation operations permitted in step 14 and the read-only inspection methods used in step 8
  evidence_level: primary
  last_verified: 2026-06-19

- SAP BTP security in the Cloud Foundry environment
  https://help.sap.com/docs/btp/sap-business-technology-platform/security-in-the-cloud-foundry-environment
  source_owner: SAP SE
  topic_supported: Cloud Foundry org/space role model, role bindings, service instance permissions, scoped authorization patterns
  why_needed: Grounds steps 3 and 10 (criticality and blast radius) for Cloud Foundry environment role assignments, particularly for integration and application runtime access
  evidence_level: primary
  last_verified: 2026-06-19

## SAP NetWeaver / S/4HANA PFCG Authorization Management

- SAP NetWeaver PFCG role maintenance and authorization management
  https://help.sap.com/docs/SAP_NETWEAVER_750/6dae0b55c6264f94b4e7e5f2e696d5d2/4a31e3fd18b44f7e9e3d2c16cdcd5e31.html
  source_owner: SAP SE
  topic_supported: PFCG transaction, composite role management, user assignment to roles, authorization object maintenance, SU01 user administration
  why_needed: Authoritative source for PFCG-based role assignment procedures and authorization object scope used in steps 8, 9, and 14 for ABAP/S/4HANA targets
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Identity Services (IAS / IPS)

- SAP Cloud Identity Services — manage users
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/manage-users
  source_owner: SAP SE
  topic_supported: Identity Authentication Service (IAS) user management, user account attributes, role assignments for IAS-managed users
  why_needed: Required for steps 8 and 14 when the target user is managed via SAP IAS and role assignment flows through IAS/IPS provisioning
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud Identity Services — manage groups
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/manage-groups
  source_owner: SAP SE
  topic_supported: Group creation and management in IAS, group-to-role-collection mapping, IPS group provisioning to target systems
  why_needed: Grounds step 7 (scope documentation) and step 14 when role changes target IAS/IPS groups rather than individual users
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Access Control / Cloud Identity Access Governance

- SAP Access Control — access risk analysis and SoD management
  https://help.sap.com/docs/SAP_ACCESS_CONTROL/a44f200cb83c4f0fa06c50c73e67e7c8/e5c4e14d9f804a53a1baa2f8d4e12c87.html
  source_owner: SAP SE
  topic_supported: SoD risk analysis, access risk simulation for proposed role assignments, access request workflow, risk remediation and mitigation
  why_needed: Authoritative source for the SoD pre-check methodology used in step 9; defines risk levels, conflict types, and remediation paths; grounds step 12 (SoD verification)
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes role management design intent, authorization object requirements, and API specifications. It does not prove which roles are currently assigned to a user in the target system, what SoD conflicts exist in the user's landscape configuration, or whether the target system's GRC ruleset matches the default SAP Access Control rulebook. Users must supply live evidence from steps 8, 9, and 15 for concrete access assessment.
