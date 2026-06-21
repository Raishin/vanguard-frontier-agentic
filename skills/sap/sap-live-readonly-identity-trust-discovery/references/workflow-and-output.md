# Workflow and output contract — SAP Live Read-Only Identity and Trust Discovery

Use this reference for all discovery workflows, command selection, and output formatting.

## Allowed command patterns

### IAS API (read-only, GET only)

```bash
# List all IAS applications
GET https://<IAS_TENANT>.accounts.ondemand.com/Applications/v1/

# Get a specific IAS application configuration
GET https://<IAS_TENANT>.accounts.ondemand.com/Applications/v1/<APPLICATION_ID>

# List all corporate identity providers configured in IAS
GET https://<IAS_TENANT>.accounts.ondemand.com/IdentityProviders/v1/

# Get risk-based authentication policy for an application (read-only)
GET https://<IAS_TENANT>.accounts.ondemand.com/Applications/v1/<APPLICATION_ID>/authentication/rba
```

### IPS API (read-only, GET only)

```bash
# List all source and target systems (connectors)
GET https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems

# Get a specific connector configuration
GET https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems/<SYSTEM_ID>

# List provisioning job history (read status only; does not trigger a job)
GET https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Systems/<SYSTEM_ID>/Jobs

# Get a specific job run result (read-only)
GET https://<IPS_TENANT>.accounts.ondemand.com/ips/service/ProvisioningService/v2/Jobs/<JOB_ID>
```

### BTP CLI — trust and role collection (read-only)

```bash
# List trust configurations for a subaccount
btp list security/trust --subaccount <SUBACCOUNT_ID>

# Get details of a specific trust configuration
btp get security/trust <TRUST_CONFIG_NAME> --subaccount <SUBACCOUNT_ID>

# List all role collections in a subaccount
btp list security/role-collection --subaccount <SUBACCOUNT_ID>

# Get a role collection with its role assignments
btp get security/role-collection "<ROLE_COLLECTION_NAME>" --subaccount <SUBACCOUNT_ID>
```

### XSUAA API (read-only, GET only)

```bash
# List all role collections in the XSUAA instance
GET <XSUAA_URL>/sap/rest/authorization/v2/rolecollections

# Get a specific role collection with role template assignments
GET <XSUAA_URL>/sap/rest/authorization/v2/rolecollections/<ROLE_COLLECTION_NAME>

# List all scopes defined in the XSUAA instance
GET <XSUAA_URL>/sap/rest/authorization/v2/apps/<CLIENT_ID>/scopes

# List all role templates defined for an application
GET <XSUAA_URL>/sap/rest/authorization/v2/apps/<CLIENT_ID>/roleTemplates
```

## Discovery workflow

1. **Confirm authorization gate**
   - Confirm with the user that live read-only access is authorized for this session.
   - Confirm credential scope is viewer/auditor/read-only API user role only.
   - Confirm target system and scope (IAS tenant, IPS tenant, BTP subaccount, XSUAA instance).

2. **Select discovery scope**
   - Identify which identity and trust layer(s) are in scope: IAS applications / IAS corporate IdP federation / IPS connectors / XSUAA role collections / BTP trust configurations.
   - Select the minimum command set needed.

3. **Execute read-only commands**
   - Run only allowed command patterns above.
   - Log every command with timestamp.
   - Redact credential values, access tokens, and personal data beyond audit necessity before logging.

4. **Structure evidence**
   - Organize output by resource type and identity domain.
   - Label all data as `live evidence` with command and timestamp.

5. **Return output** per the output contract below.

## Output contract

Return:

1. Discovery scope (what identity and trust data was enumerated and why)
2. Authorization gate confirmation (role confirmed read-only: yes/no)
3. Command log (every command, timestamp, summary of output, redactions applied)
4. Structured evidence (resources enumerated, organized by identity domain: IAS / IPS / XSUAA / BTP trust)
5. Redaction log (what was redacted and why)
6. Next step (how this evidence feeds into the downstream advisory: IAM review, SoD assessment, compliance audit)
7. Escalation trigger (any forbidden action encountered, any unexpected write attempt by the tool, or any personal data scope requiring review)
