# Live environment access — SAP Guarded Role Assignment

This reference defines credential setup, role requirements, command restrictions by step, and audit log format for guarded role assignment sessions.

## Credential requirements

### SAP BTP (Role Collection Management)

Required authorization for the executing user:
- Role collection: **User and Role Administrator** scoped to the specific target subaccount only
- Not permitted: Global Account Administrator (grants cross-subaccount write access)
- Not permitted: Subaccount Administrator (grants configuration changes beyond role assignment)

API token scope (BTP Security API):
- Minimum: `xs_security.token.client_credentials` with `uaa.resource` scope for the target subaccount
- Never request organization-level or global account-level token scopes

### S/4HANA and SAP NetWeaver (SU01 / PFCG)

Required authorization objects for the role assignment user:
- `S_USER_AGR` with `ACTVT = 03` (display) — for step 8 (read-only current state)
- `S_USER_AGR` with `ACTVT = 22` (assign roles) — for step 14 (execute approved change), scoped to target client only
- `S_TCODE`: SU01 (display and change), PFCG (display only for role content inspection), SUIM (display)
- Not permitted: `SAP_ALL` or any BASIS administrator profile
- Not permitted: `S_USER_GRP` with ACTVT=01/02 (create/modify user master records) — this skill covers role assignment only, not user creation

### SAP Cloud Identity Services (IAS / IPS)

Required service account scope:
- IAS role: **User Administrator** for the specific IAS tenant
- IPS role: provisioning operator for the specific IPS system
- Not permitted: Tenant Administrator role (grants IAS system configuration rights)
- Never store IAS client ID or client secret in any output

### SAP Access Control / Cloud Identity Access Governance (SoD checks — step 9 only)

Required access:
- SAP GRC AC: Risk Analyst or Access Request Processor role — read access to SoD risk simulation
- Cloud IAG: Risk Analyst scope for the relevant business unit
- Not permitted: GRC System Administrator role (grants ruleset modification rights)

## Step-by-step access matrix

| Step | Live system access | Access type | Credential scope |
|------|--------------------|-------------|-----------------|
| 1–7 | None | Advisory only | Not applicable |
| 8 | BTP Security API / SU01 / IAS API | Read-only | BTP User+Role Admin GET / S_USER_AGR ACTVT=03 / IAS User Admin GET |
| 9 | SAP GRC / Cloud IAG / BTP API | Read-only (SoD simulation) | GRC Risk Analyst / Cloud IAG Risk Analyst / BTP User+Role Admin GET |
| 10–13 | None | Advisory only | Not applicable |
| 14 | BTP Security API / SU01 / IAS API | Mutating — assignment/revocation only | BTP User+Role Admin PUT/DELETE / S_USER_AGR ACTVT=22 / IAS User Admin PATCH |
| 15 | BTP Security API / SU01 / IAS API / GRC | Read-only | Same as step 8 + GRC Risk Analyst |
| 16–17 | None | Reporting only | Not applicable |

## Audit log format

Every command executed (steps 8, 9, 14, 15) must be logged before execution:

```
[ROLE_ASSIGNMENT_AUDIT_LOG]
Step: <step number and name>
Timestamp (UTC): <ISO 8601>
System: <BTP subaccount GUID OR S/4HANA SID + client OR IAS tenant ID>
Tool: <BTP Security API | S/4HANA SU01 | SAP IAS API | SAP GRC | Cloud IAG>
Command/API call: <exact command or API endpoint, credentials omitted>
Target user(s)/group(s): <list of user IDs or group names>
Role(s) in scope: <list of role collection names or PFCG role names>
SoD pre-check result: <passed (no conflicts) | failed (<conflict list>) | not yet run>
Output summary: <one-line summary>
Errors/Warnings: <none | list>
Redactions applied: <yes/no>
Approval gate cleared: <yes (step 13 documented) | not yet reached>
```

## Pre-execution gate checklist (step 13 → step 14)

Before executing any role assignment or revocation command, all of the following must be true:

- [ ] Step 1: Role change type, role type, and scope classified
- [ ] Step 2: Target system, subaccount/client, and tier confirmed
- [ ] Step 3: Criticality level assigned
- [ ] Step 4: Requester name and role documented
- [ ] Step 5: Approver name, role, and identity documented (different from requester)
- [ ] Step 6: Access request or change management ticket number confirmed
- [ ] Step 7: Complete role list with target user(s)/group(s) and validity period documented
- [ ] Step 8: Read-only current role assignment snapshot obtained (live evidence on record)
- [ ] Step 9: SoD pre-check passed (no Critical or High conflicts); effective-permission diff produced
- [ ] Step 10: Blast radius document confirmed
- [ ] Step 11: Rollback procedure documented and confirmed feasible
- [ ] Step 12: SoD confirmed — requester and approver are different individuals; SoD pre-check result is pass
- [ ] Step 13: Explicit written approval from authorized approver on record

If any item is unchecked, refuse step 14 execution until it is resolved.
If step 9 returned any SoD conflict, refuse step 14 execution regardless of other steps.
