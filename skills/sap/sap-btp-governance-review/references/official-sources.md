# Official sources — SAP BTP Governance Review

Use this reference when grounding BTP account model governance, entitlement design, environment provisioning, role collection management, and trust configuration assessments.

**Evidence level**: documentation-based (SAP BTP Help Portal). No live-system evidence is collected by this skill.

## BTP account model and hierarchy

- SAP BTP account model
  https://help.sap.com/docs/btp/sap-business-technology-platform/account-model
  source_owner: SAP SE
  topic_supported: Global account, directory, and subaccount structure; account hierarchy concepts; isolation boundaries
  why_needed: Primary taxonomy for classifying account structure governance findings and evaluating subaccount design decisions
  evidence_level: primary
  last_verified: 2026-06-19

- Directories
  https://help.sap.com/docs/btp/sap-business-technology-platform/directories
  source_owner: SAP SE
  topic_supported: BTP directory creation, managed lifecycle, entitlement and role collection inheritance, directory-level governance controls
  why_needed: Authoritative source for evaluating whether directory structure is used appropriately to enforce governance boundaries across subaccounts
  evidence_level: primary
  last_verified: 2026-06-19

## Entitlements and quotas

- Entitlements and quotas
  https://help.sap.com/docs/btp/sap-business-technology-platform/entitlements-and-quotas
  source_owner: SAP SE
  topic_supported: Service entitlements, quota distribution, assignment to directories and subaccounts, entitlement management lifecycle
  why_needed: Defines the entitlement assignment model used to classify sprawl, over-provisioning, and least-privilege deviations
  evidence_level: primary
  last_verified: 2026-06-19

## Environments

- Cloud Foundry environment
  https://help.sap.com/docs/btp/sap-business-technology-platform/cloud-foundry-environment
  source_owner: SAP SE
  topic_supported: Cloud Foundry org/space structure, quota plans, environment-level service bindings and access controls
  why_needed: Defines CF org/space governance model and quota separation — required to assess CF-level over-provisioning alongside BTP entitlements
  evidence_level: primary
  last_verified: 2026-06-19

- Kyma environment
  https://help.sap.com/docs/btp/sap-business-technology-platform/kyma-environment
  source_owner: SAP SE
  topic_supported: Kyma cluster provisioning, module selection, service binding, Kyma operator model, namespace-level isolation
  why_needed: Defines the Kyma governance model for BTP subaccount-level Kubernetes workloads — required when Kyma environments are in scope
  evidence_level: primary
  last_verified: 2026-06-19

## Role collections and access control

- Role collections and roles in global accounts, directories, and subaccounts
  https://help.sap.com/docs/btp/sap-business-technology-platform/role-collections-and-roles-in-global-accounts-directories-and-subaccounts
  source_owner: SAP SE
  topic_supported: Role collection structure, role assignment scope (global account / directory / subaccount), built-in vs. custom role collections
  why_needed: Primary reference for classifying role collection governance findings including over-permissive assignments and direct user-to-role binding
  evidence_level: primary
  last_verified: 2026-06-19

- Security administration: managing authentication and authorization
  https://help.sap.com/docs/btp/sap-business-technology-platform/security-administration-managing-authentication-and-authorization
  source_owner: SAP SE
  topic_supported: Platform-level authentication, authorization administration, role collection assignment via identity provider groups
  why_needed: Defines best practice for IdP-group-based role collection assignment over direct user assignment — key for access lifecycle governance
  evidence_level: primary
  last_verified: 2026-06-19

## Trust and identity federation

- Trust and federation with identity providers
  https://help.sap.com/docs/btp/sap-business-technology-platform/trust-and-federation-with-identity-providers
  source_owner: SAP SE
  topic_supported: Subaccount-level trust configuration, SAML/OIDC federation, identity provider registration, attribute mapping
  why_needed: Defines the trust configuration model — required to identify unused, misconfigured, or overly permissive trust configurations in BTP subaccounts
  evidence_level: primary
  last_verified: 2026-06-19

- Platform identity provider
  https://help.sap.com/docs/btp/sap-business-technology-platform/platform-identity-provider
  source_owner: SAP SE
  topic_supported: Default platform identity provider (SAP ID Service), switching to a custom platform IdP, platform user lifecycle
  why_needed: Defines the platform IdP model separate from application IdP — important for distinguishing platform-level vs. application-level trust risks
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP BTP Help Portal documentation describes the designed governance model and configuration options. It does not prove what entitlements, role collections, or trust configurations exist in the user's BTP global account, nor whether the account follows these guidelines. Users must supply account exports, role collection lists, entitlement summaries, or architectural descriptions for concrete governance assessment.
