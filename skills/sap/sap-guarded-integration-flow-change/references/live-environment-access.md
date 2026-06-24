# Live environment access — SAP Guarded Integration Flow Change

This reference defines credential setup, role requirements, command restrictions by step, and audit log format for guarded integration flow change sessions.

## Credential requirements

### SAP Cloud Integration OData API

Required service key scopes for the executing service instance:
- Step 8 (read-only): `MonitoringDataRead` and `WorkspacePackagesRead`
- Step 9 (diff, read-only): `WorkspacePackagesRead`
- Step 14 (deploy, mutating): `WorkspacePackagesConfigure` and `NodeManager.deployer`
- Step 15 (verify, read-only): `MonitoringDataRead`

Not permitted:
- `AuthGroup.Administrator` or `AuthGroup.IntegrationDeveloper` with tenant-wide scope for step 14 — scope to the specific package or integration flow
- Any scope that permits security material configuration (keystore, credential store) unless explicitly in scope and approved

### Integration Suite User (Cockpit access)

Required role collection for the deploying user:
- **PI_Integration_Developer** for the specific Cloud Integration tenant
- Not permitted: **PI_Administrator** (grants tenant-level configuration rights beyond deployment)
- Not permitted: **Integration_Provisioner** (grants capability activation and subscription changes)

### OAuth token handling

- Obtain OAuth tokens via the service key `tokenurl` endpoint with client credentials grant
- Tokens are short-lived; do not cache or reuse tokens across sessions
- Never store client ID, client secret, or bearer token values in any output or log entry

## Step-by-step access matrix

| Step | Live system access | Access type | Credential scope |
|------|--------------------|-------------|-----------------|
| 1–7 | None | Advisory only | Not applicable |
| 8 | Cloud Integration OData API | Read-only | MonitoringDataRead + WorkspacePackagesRead |
| 9 | Cloud Integration Design API | Read-only (diff) | WorkspacePackagesRead |
| 10–13 | None | Advisory only | Not applicable |
| 14 | Cloud Integration OData API | Mutating — deploy only | WorkspacePackagesConfigure + NodeManager.deployer |
| 15 | Cloud Integration OData API / Monitoring API | Read-only | MonitoringDataRead |
| 16–17 | None | Reporting only | Not applicable |

## Audit log format

Every command executed (steps 8, 9, 14, 15) must be logged before execution:

```
[INTEGRATION_FLOW_AUDIT_LOG]
Step: <step number and name>
Timestamp (UTC): <ISO 8601>
Tenant: <Cloud Integration tenant ID and sub-domain>
Tool: <Cloud Integration OData API | Design API | Monitoring API>
Command/API call: <exact API endpoint and method, credentials omitted>
Artifact(s) in scope: <artifact IDs and version numbers>
Output summary: <one-line summary>
Errors/Warnings: <none | list>
Redactions applied: <yes/no>
Approval gate cleared: <yes (step 13 documented) | not yet reached>
```

## Pre-execution gate checklist (step 13 → step 14)

Before executing any deployment command, all of the following must be true:

- [ ] Step 1: Artifact type and change type classified
- [ ] Step 2: Target tenant ID, tier, and sub-domain confirmed
- [ ] Step 3: Criticality level assigned
- [ ] Step 4: Requester name and role documented
- [ ] Step 5: Integration owner name, role, and accountability scope documented (different from requester)
- [ ] Step 6: Change management ticket number confirmed
- [ ] Step 7: Artifact list with IDs, versions, and dependency map documented
- [ ] Step 8: Deployed artifact status and message monitoring baseline obtained (live evidence on record)
- [ ] Step 9: Diff of artifact changes produced and on record
- [ ] Step 10: Blast radius document confirmed (downstream partners, throughput impact, in-flight message handling)
- [ ] Step 11: Rollback procedure (specific previous version to redeploy) documented and confirmed feasible
- [ ] Step 12: SoD confirmed — requester and integration owner / approver are different individuals
- [ ] Step 13: Explicit written integration-owner approval on record, naming the specific artifact IDs and versions

If any item is unchecked, refuse step 14 execution until it is resolved.
