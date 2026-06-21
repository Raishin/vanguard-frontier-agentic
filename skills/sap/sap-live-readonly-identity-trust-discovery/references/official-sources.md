# Official sources — SAP Live Read-Only Identity and Trust Discovery

Use this reference when grounding IAS, IPS, XSUAA, and BTP trust enumeration commands and API calls.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during sessions is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP Cloud Identity Services — Identity Authentication Service (IAS)

- What Is Identity Authentication
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/what-is-identity-authentication
  source_owner: SAP SE
  topic_supported: IAS overview, application types, corporate identity provider federation, risk-based authentication, MFA policies
  why_needed: Authoritative source for IAS application and corporate IdP enumeration scope; defines what configuration objects this skill can discover in read-only mode
  evidence_level: primary
  last_verified: 2026-06-19

- Configure Applications in IAS
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/configure-applications
  source_owner: SAP SE
  topic_supported: IAS application configuration — authentication schemes, corporate IdP trust, risk-based authentication policies, API access settings
  why_needed: Defines the IAS application object model and REST API paths for read-only enumeration of application settings and corporate identity provider assignments
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud Identity Services — Identity Provisioning Service (IPS)

- What Is Identity Provisioning
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/what-is-identity-provisioning
  source_owner: SAP SE
  topic_supported: IPS overview, connector types (source, target, proxy), provisioning job types (read job, resync), transformation script structure
  why_needed: Defines the IPS connector and job object model; establishes which API endpoints are read-only (GET) versus job-triggering (POST)
  evidence_level: primary
  last_verified: 2026-06-19

- Manage Provisioning Systems in IPS
  https://help.sap.com/docs/cloud-identity-services/cloud-identity-services/manage-provisioning-systems
  source_owner: SAP SE
  topic_supported: IPS source and target system configuration — connector properties, transformation scripts, job scheduling, job history access
  why_needed: Defines the IPS REST API paths for reading connector configuration and job history without triggering provisioning jobs
  evidence_level: primary
  last_verified: 2026-06-19

## SAP BTP Authorization and Trust Management Service (XSUAA)

- What Is Authorization and Trust Management Service
  https://help.sap.com/docs/btp/sap-business-technology-platform/what-is-authorization-and-trust-management-service
  source_owner: SAP SE
  topic_supported: XSUAA overview, role collections, role templates, scopes, OAuth 2.0 trust configuration in BTP Cloud Foundry environment
  why_needed: Authoritative source for XSUAA role collection and scope enumeration; defines which XSUAA REST API calls are read-only (GET)
  evidence_level: primary
  last_verified: 2026-06-19

- Application Security Descriptor Configuration Syntax (xs-security.json)
  https://help.sap.com/docs/btp/sap-business-technology-platform/application-security-descriptor-configuration-syntax
  source_owner: SAP SE
  topic_supported: xs-security.json structure — scopes, role templates, role collections, OAuth2 configuration — read to understand the objects enumerated by this skill
  why_needed: Provides the schema for interpreting XSUAA role template and scope output returned by read-only enumeration
  evidence_level: primary
  last_verified: 2026-06-19

## BTP Trust and Federation

- Trust and Federation with Identity Providers
  https://help.sap.com/docs/btp/sap-business-technology-platform/trust-and-federation-with-identity-providers
  source_owner: SAP SE
  topic_supported: BTP subaccount trust configuration, SAML 2.0 and OIDC trust to external and corporate identity providers, principal propagation, attribute mapping, default identity provider
  why_needed: Defines the BTP trust configuration object model and the BTP CLI commands for read-only trust enumeration (`btp list security/trust`, `btp get security/trust`)
  evidence_level: primary
  last_verified: 2026-06-19

## Role Collections

- Managing Role Collections in BTP
  https://help.sap.com/docs/btp/sap-business-technology-platform/managing-role-collections
  source_owner: SAP SE
  topic_supported: Role collection creation, assignment to users and groups, built-in role collections, BTP CLI role collection management commands
  why_needed: Defines the BTP CLI read-only commands for role collection enumeration (`btp list security/role-collection`, `btp get security/role-collection`) and the role collection assignment model
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes available API endpoints and CLI commands. It does not prove which endpoints are available in the user's specific IAS tenant version, which IPS connector types are provisioned, or whether the credential in scope actually restricts write access. Users must confirm credential scope and target system identity before any live command is executed.
