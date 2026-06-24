# Live environment access — SAP Live Read-Only Identity and Trust Discovery

This reference defines credential setup, role requirements, audit log format, and redaction rules for live read-only identity and trust discovery sessions.

## Credential setup

### SAP Cloud Identity Services — Identity Authentication Service (IAS) API

Required role: Read-only API user or IAS administrator scoped to application listing and corporate identity provider enumeration. Do not use a full IAS tenant administrator who can also create or modify applications.

Recommended approach: create a dedicated IAS service user with the `Manage Applications` permission set to read-only scope, or use the IAS System as Applications option with a `Get` scope API key.

```bash
# IAS REST API — list all applications (GET only)
curl -s -X GET \
  "https://<IAS_TENANT>.accounts.ondemand.com/Applications/v1/" \
  -H "Authorization: Bearer <READ_ONLY_ACCESS_TOKEN>"

# IAS REST API — list corporate identity providers
curl -s -X GET \
  "https://<IAS_TENANT>.accounts.ondemand.com/IdentityProviders/v1/" \
  -H "Authorization: Bearer <READ_ONLY_ACCESS_TOKEN>"
```

Never include the `<READ_ONLY_ACCESS_TOKEN>` value in any output. Replace with `[REDACTED:IAS_TOKEN]` before logging.

### SAP Cloud Identity Services — Identity Provisioning Service (IPS) API

Required role: IPS administrator account with read-only access to source and target system configuration and job history. Do not use credentials with provisioning job execution rights.

```bash
# IPS REST API — list all provisioning source systems (GET only)
curl -s -X GET \
  "https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems" \
  -u "<IPS_USER>:[REDACTED:IPS_PASSWORD]" \
  -H "Accept: application/json"

# IPS REST API — get a specific source system connector configuration
curl -s -X GET \
  "https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems/<SYSTEM_ID>" \
  -u "<IPS_USER>:[REDACTED:IPS_PASSWORD]" \
  -H "Accept: application/json"

# IPS REST API — list provisioning job history for a system (read-only)
curl -s -X GET \
  "https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems/<SYSTEM_ID>/Jobs" \
  -u "<IPS_USER>:[REDACTED:IPS_PASSWORD]" \
  -H "Accept: application/json"
```

### BTP CLI — trust and role collection enumeration

Required role: **Subaccount Viewer** or **Security Administrator (read-only)** for subaccount-scope discovery; **Global Account Viewer** for cross-subaccount trust enumeration.

Never use: Global Account Administrator, Subaccount Administrator, Security Administrator (with write permissions), or any role that can assign or modify trust configurations.

```bash
# Login to BTP CLI
btp login --url https://cli.btp.cloud.sap

# Confirm active context and role
btp --info

# List all trust configurations for a subaccount (read-only)
btp list security/trust --subaccount <SUBACCOUNT_ID>

# Get details of a specific trust configuration
btp get security/trust <TRUST_CONFIG_NAME> --subaccount <SUBACCOUNT_ID>

# List all role collections in a subaccount
btp list security/role-collection --subaccount <SUBACCOUNT_ID>

# Get a specific role collection (roles and assignments)
btp get security/role-collection "<ROLE_COLLECTION_NAME>" --subaccount <SUBACCOUNT_ID>
```

### XSUAA service API (via CF CLI or BTP API)

Required role: **SpaceAuditor** or **OrgAuditor** in Cloud Foundry; use a service key with only `roles.read` and `role-collections.read` scopes on the XSUAA instance.

Never use: a XSUAA service key with `roles.write`, `role-collections.write`, or `user.write` scopes.

```bash
# CF CLI — confirm current user and role (read-only)
cf org-users <ORG_NAME>
cf space-users <ORG_NAME> <SPACE_NAME>

# CF CLI — list service instances (to locate XSUAA instance)
cf service-instances

# XSUAA API — list role collections (GET only, using service key access token)
curl -s -X GET \
  "<XSUAA_URL>/sap/rest/authorization/v2/rolecollections" \
  -H "Authorization: Bearer [REDACTED:XSUAA_TOKEN]"
```

## Redaction rules

The following must be redacted from all output, logs, and evidence before returning to the user:

| Data type | Redaction token |
|-----------|----------------|
| IAS access token or bearer token | `[REDACTED:IAS_TOKEN]` |
| IPS system user password | `[REDACTED:IPS_PASSWORD]` |
| XSUAA client secret | `[REDACTED:XSUAA_CLIENT_SECRET]` |
| XSUAA access token | `[REDACTED:XSUAA_TOKEN]` |
| BTP platform user password | `[REDACTED:BTP_PASSWORD]` |
| OAuth client secret (any) | `[REDACTED:CLIENT_SECRET]` |
| Personal email addresses beyond audit necessity | `[REDACTED:PII_EMAIL]` |
| Personal user names beyond audit necessity | `[REDACTED:PII_USERNAME]` |
| SAML signing certificate private key | `[REDACTED:SAML_PRIVATE_KEY]` |

The data type name, key structure, and service names may remain in output. Only sensitive values are redacted.

## Audit log format

Every command executed must be logged in the following format before the command is run:

```
[COMMAND_LOG]
Timestamp (UTC): <ISO 8601>
System: <IAS tenant ID | IPS tenant | BTP subaccount ID | XSUAA instance name>
Tool: <curl-ias | curl-ips | btp | cf | curl-xsuaa>
Command: <exact command with arguments; credential values replaced with redaction tokens>
Output summary: <one-line summary of what was returned>
Redactions applied: <yes/no — what was redacted>
```

Example:
```
[COMMAND_LOG]
Timestamp (UTC): 2026-06-19T11:15:00Z
System: IAS Tenant abc-def-ghi.accounts.ondemand.com
Tool: curl-ias
Command: GET /Applications/v1/ Authorization: Bearer [REDACTED:IAS_TOKEN]
Output summary: 12 IAS applications enumerated; 3 with corporate IdP federation, 9 with IAS-local authentication
Redactions applied: yes — bearer token replaced with [REDACTED:IAS_TOKEN]
```

## Approval gate checklist

Before the first live command in any session:

- [ ] User has confirmed live read-only access is authorized for this session
- [ ] Credential scope confirmed as viewer/auditor/read-only API user (role name or API scope confirmed)
- [ ] Target system and scope explicitly identified by user (IAS tenant ID, IPS tenant, BTP subaccount ID, XSUAA instance)
- [ ] Redaction rules reviewed and will be applied to all output
- [ ] Personal data minimization plan confirmed for IAS and IPS user enumeration
- [ ] User understands no state changes will be made

Do not proceed until all six items are confirmed.
