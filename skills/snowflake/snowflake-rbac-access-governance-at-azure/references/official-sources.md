# Official sources

Use this reference only when you need source grounding for Snowflake service behavior or the detailed source list.

## Snowflake documentation

Use these as starting points, not as proof of the user's live Snowflake account state:
- https://docs.snowflake.com/en/user-guide/security-access-control-overview
- https://docs.snowflake.com/en/user-guide/security-access-control-considerations
- https://docs.snowflake.com/en/user-guide/network-policies
- https://docs.snowflake.com/en/user-guide/oauth-azure
- https://docs.snowflake.com/en/user-guide/scim-azure

## Grounding rule

Official documentation explains Snowflake service behavior. It does not prove the user's current account, edition, role hierarchy, quota, resource configuration, or operational state. Prefer read-only Snowflake MCP or SQL evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Service facts from official docs:
- ACCOUNTADMIN encapsulates SYSADMIN and SECURITYADMIN; it must have at minimum two active holders, MFA enforced on all, and must not be used for routine object creation or ETL workloads.
- SECURITYADMIN holds MANAGE GRANTS; SYSADMIN holds object ownership. SoD requires these roles remain separate.
- USERADMIN provides CREATE USER and CREATE ROLE without grant privileges; use for provisioning, not policy administration.
- PUBLIC is automatically granted to every user and role; never assign sensitive object privileges to PUBLIC.
- Network policies restrict inbound IP access at account, user, or security-integration level. On Azure, AZURELINKID-type rules restrict access to a specific Azure subscription ID.
- MFA phased enforcement requires password-bearing users to enroll in MFA. Service accounts must use TYPE=SERVICE with key-pair or OAuth; password authentication is prohibited for service users.
- Entra ID External OAuth (SECURITY INTEGRATION TYPE=OAUTH OAUTH_TYPE=AZURE_AD) issues tokens scoped to Snowflake role; token issuer must match the Entra ID tenant. Do not grant ACCOUNTADMIN to OAuth service principals.
- SCIM provisioning uses the AAD_PROVISIONER system role (not ACCOUNTADMIN) to own provisioned users and groups. Default password for SCIM-provisioned users is unset when SAML SSO is active.

Review implications:
- Do not approve broad ACCOUNTADMIN grants from intent alone. Require justification, MFA evidence, minimum-holder audit, and break-glass exception documentation.
- Do not approve PUBLIC grants for any non-trivial object. Require explicit object scope and business justification.
- Documentation cannot prove the user's actual role hierarchy, network policy state, or MFA enrollment.
