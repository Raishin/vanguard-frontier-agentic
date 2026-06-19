# Live environment access — SAP Guarded Transport Import

This reference defines credential setup, role requirements, command restrictions by step, and audit log format for guarded transport import sessions.

## Credential requirements

### STMS (ABAP Transport Management)

Required authorization objects for the transport import user:
- `S_TRANSPRT` with `ACTVT = 03` (display) — for step 8 (read-only current state)
- `S_TRANSPRT` with `ACTVT = 42` (import) — for step 14 (execute approved import), scoped to target system only
- `S_TCODE`: SE10 (display), STMS (display and import), no development transactions

Never use:
- A user with `SAP_ALL` or BASIS administrator profile
- A developer user (S_DEVELOP with ACTVT 01/02) for transport import execution
- A user with `S_TRANSPRT ACTVT = 06` (delete) — transport deletion is never permitted by this skill

### CTS+ Service User

Required role: CTS+ Transport Manager scoped to the specific CTS+ system
Not permitted: CTS+ System Administrator role (grants transport route modification rights)

### SAP Cloud TMS

Required role: **Transport Operator** scoped to the specific transport node
Not permitted: **Transport Administrator** role (grants delete and route configuration rights)

OAuth token scope: minimum required for `/v2/nodes/{nodeId}/transportRequests` GET and `/v2/nodes/{nodeId}/transportRequests/{trId}/deploy` POST only.

Never request broader scopes. Never store OAuth tokens in plaintext in any output.

## Step-by-step access matrix

| Step | Live system access | Access type | Credential scope |
|------|--------------------|-------------|-----------------|
| 1–7 | None | Advisory only | Not applicable |
| 8 | STMS / CTS+ / Cloud TMS | Read-only | S_TRANSPRT ACTVT=03 / Cloud TMS Transport Operator GET |
| 9 | STMS / Cloud TMS (if dry-run supported) | Read-only or dry-run | S_TRANSPRT ACTVT=03 / Cloud TMS Transport Operator GET |
| 10–13 | None | Advisory only | Not applicable |
| 14 | STMS / CTS+ / Cloud TMS | Mutating — import only | S_TRANSPRT ACTVT=42 / Cloud TMS Transport Operator POST (approved TRs only) |
| 15 | STMS / Cloud TMS | Read-only | S_TRANSPRT ACTVT=03 / Cloud TMS Transport Operator GET |
| 16–17 | None | Reporting only | Not applicable |

## Audit log format

Every command executed (steps 8, 14, 15) must be logged before execution:

```
[TRANSPORT_AUDIT_LOG]
Step: <step number and name>
Timestamp (UTC): <ISO 8601>
System: <ABAP SID + client OR Cloud TMS node ID>
Tool: <STMS | CTS+ API | Cloud TMS API>
Command/API call: <exact command or API endpoint, credentials omitted>
TR(s) in scope: <list of transport request numbers>
Output summary: <one-line summary>
Errors/Warnings: <none | list>
Redactions applied: <yes/no>
Approval gate cleared: <yes (step 13 documented) | not yet reached>
```

## Pre-execution gate checklist (step 13 → step 14)

Before executing any import command, all of the following must be true:

- [ ] Step 1: Transport type and content category classified
- [ ] Step 2: Target system SID, client, and tier confirmed
- [ ] Step 3: Criticality level assigned
- [ ] Step 4: Requester name and role documented
- [ ] Step 5: Approver name, role, and identity documented
- [ ] Step 6: Change management ticket number confirmed
- [ ] Step 7: Complete TR list with descriptions documented
- [ ] Step 8: Read-only current state snapshot obtained (live evidence on record)
- [ ] Step 9: Diff/dry-run completed or documented exception on record
- [ ] Step 10: Blast radius document confirmed
- [ ] Step 11: Rollback procedure documented and confirmed feasible
- [ ] Step 12: SoD confirmed — requester and approver are different individuals
- [ ] Step 13: Explicit written approval from authorized approver on record

If any item is unchecked, refuse step 14 execution until it is resolved.
