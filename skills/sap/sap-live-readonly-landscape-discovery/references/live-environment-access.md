# Live environment access — SAP Live Read-Only Landscape Discovery

This reference defines credential setup, role requirements, audit log format, and redaction rules for live read-only landscape discovery sessions.

## Credential setup

### BTP CLI

Required role: **Subaccount Viewer** (for subaccount-scope discovery) or **Global Account Viewer** (for cross-subaccount enumeration).

Never use: Global Account Administrator, Subaccount Administrator, or any role with entitlement assignment or service instance management rights.

Setup:
```bash
# Login to BTP CLI (non-interactive with service key or browser flow)
btp login --url https://cli.btp.cloud.sap

# Verify current active context
btp --info

# Confirm role assignment before discovery
btp get security/role-collection <ROLE_COLLECTION_NAME> --subaccount <SUBACCOUNT_ID>
```

### Cloud Foundry CLI

Required role: **OrgAuditor** (org-scope) or **SpaceAuditor** (space-scope).

Never use: OrgManager, SpaceManager, SpaceDeveloper — these carry write permissions.

```bash
# Login (use SSO where possible to avoid password in shell history)
cf login --sso

# Confirm current user and role
cf org-users <ORG_NAME>
cf space-users <ORG_NAME> <SPACE_NAME>
```

### kubectl / Kyma

Required: A ClusterRoleBinding or RoleBinding that grants only `get`, `list`, `watch` verbs. No `create`, `update`, `delete`, `patch`.

```bash
# Verify current permissions before discovery
kubectl auth can-i --list --namespace <NAMESPACE>
```

Confirm the output contains only read verbs before proceeding.

### ABAP RFC/HTTP display user

Required: A user with display-only authorization profiles. Recommended authorization objects:
- `S_TCODE`: display transactions only (SE10, SE16N display, STMS_IMPORT display)
- `S_DEVELOP`: display only (`ACTVT = 03`), no create/change/delete
- `S_TRANSPRT`: display only (`ACTVT = 03`)
- No `S_DATASET` write, no `S_PROGRAM` execute

Never use an ABAP user with BASIS administrator profile or emergency user (`SAP_ALL`).

## Redaction rules

The following must be redacted from all output, logs, and evidence before returning to the user:

| Data type | Redaction token |
|-----------|----------------|
| OAuth client_secret | `[REDACTED:CLIENT_SECRET]` |
| Service key credentials block | `[REDACTED:SERVICE_KEY]` |
| VCAP_SERVICES credential values | `[REDACTED:VCAP_CREDENTIALS]` |
| ABAP logon password | `[REDACTED:ABAP_PASSWORD]` |
| API key or token value | `[REDACTED:API_KEY]` |
| BTP platform user password | `[REDACTED:BTP_PASSWORD]` |

The data type name and structure (key names, service names) may remain in output. Only the value is redacted.

## Audit log format

Every command executed must be logged in the following format before the command is run:

```
[COMMAND_LOG]
Timestamp (UTC): <ISO 8601>
System: <BTP global account ID or CF org/space or ABAP SID>
Tool: <btp | cf | kubectl | abap-http>
Command: <exact command with arguments, credentials omitted>
Output summary: <one-line summary of what was returned>
Redactions applied: <yes/no — what was redacted>
```

Example:
```
[COMMAND_LOG]
Timestamp (UTC): 2026-06-19T10:32:00Z
System: BTP Global Account abc-123
Tool: btp
Command: btp list accounts/subaccount --global-account abc-123
Output summary: 4 subaccounts enumerated (dev, test, staging, prod)
Redactions applied: no
```

## Approval gate checklist

Before the first live command in any session:

- [ ] User has confirmed live read-only access is authorized for this session
- [ ] Credential scope confirmed as viewer/auditor (role name or profile confirmed)
- [ ] Target system and scope explicitly identified by user (global account ID / ABAP SID / CF org)
- [ ] Redaction rules reviewed and applied to output
- [ ] User understands no state changes will be made

Do not proceed until all five items are confirmed.
