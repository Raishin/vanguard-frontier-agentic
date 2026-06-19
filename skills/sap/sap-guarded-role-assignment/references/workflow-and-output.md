# Workflow and output contract — SAP Guarded Role Assignment

Use this reference for all step-by-step execution, command patterns, and output formatting.

## Step execution protocol

For each of the 17 steps, the following must be true before advancing to the next:

1. The step's required evidence is on record in this session.
2. The user has confirmed the evidence is accurate.
3. Any step-specific gate (e.g., SoD pre-check, approval, SoD verification) has been explicitly cleared.

If any of these conditions is false, the session pauses at the current step until resolved.

## Command patterns by step

### Step 8 — Read-only current state

#### BTP Security API (read)
```
GET /security/v1/subaccounts/{subaccountGuid}/users/{userId}/roleCollections
GET /security/v1/subaccounts/{subaccountGuid}/groups/{groupId}/roleCollections
```

#### BTP Cockpit (read)
```
BTP Cockpit → Subaccount → Security → Users → select user → view role collections
BTP Cockpit → Subaccount → Security → Role Collections → view assigned users
```

#### S/4HANA PFCG / SU01 (read)
```
Transaction SU01 → display user → Roles tab (display assigned roles and validity periods)
Transaction PFCG → display role → User tab (display assigned users)
Transaction SUIM → User by roles → display users holding a specific role
```

#### SAP Cloud Identity Services (IAS) API (read)
```
GET /service/scim/Users/{userId}    (returns group memberships and assigned roles)
GET /service/scim/Groups/{groupId}  (returns group members)
```

### Step 9 — SoD pre-check and diff

#### SAP Access Control / Cloud IAG (SoD simulation)
```
SAP GRC → Access Management → Access Risk Analysis → Simulate Risk for User
  Input: user ID + proposed role(s) to add
  Output: SoD risk report (Conflict / No Conflict per function pair)

Cloud IAG → Risk Analysis → Run Risk Analysis for proposed assignment
  Input: user + proposed role collection / role
  Output: SoD risk level (Critical / High / Medium / Low / None)
```

**If any Critical or High SoD risk is returned: refuse assignment. Document result as step 9 evidence. Do not proceed to step 10.**

#### Effective-permission diff (BTP)
```
Current role collections: [list from step 8]
Proposed role collections after change: [step 7 list merged with step 8 state]
Net-new permissions: [roles added] → [derived authorizations]
Net-removed permissions: [roles removed] → [derived authorizations]
```

### Step 14 — Execute approved change

#### BTP Security API (mutating — step 14 only, after step 13 gate is cleared)
```
PUT /security/v1/subaccounts/{subaccountGuid}/users/{userId}/roleCollections
Body: { "roleCollectionNames": ["<approved role collection 1>", "<approved role collection 2>"] }

DELETE /security/v1/subaccounts/{subaccountGuid}/users/{userId}/roleCollections/{roleCollectionName}
  (for revocation of a specific role collection)
```

#### S/4HANA SU01 (mutating — step 14 only)
```
Transaction SU01 → change user → Roles tab → add or remove approved roles → save
  Authorization required: S_USER_AGR with ACTVT=22 (change user role assignment)
  Scoped to target client only
```

#### SAP Cloud Identity Services (IAS) API (mutating — step 14 only)
```
PATCH /service/scim/Users/{userId}
Body (assign group): { "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"], "Operations": [{"op": "add", "path": "groups", "value": [{"value": "<groupId>"}]}] }
Body (remove group): { "schemas": ["urn:ietf:params:scim:api:messages:2.0:PatchOp"], "Operations": [{"op": "remove", "path": "groups", "value": [{"value": "<groupId>"}]}] }
```

**Never call mutating endpoints before step 13 approval gate is documented.**

## Step completion checklist

| Step | Evidence type | Completion marker |
|------|--------------|------------------|
| 1 | user-provided / documentation-based | Role change type, role type, and scope confirmed |
| 2 | user-provided evidence | Target system, subaccount/client, and tier confirmed |
| 3 | user-provided evidence | Criticality level assigned |
| 4 | user-provided evidence | Requester name and role on record |
| 5 | user-provided evidence | Approver name, role, and identity on record |
| 6 | user-provided evidence | Ticket number confirmed |
| 7 | user-provided evidence | Role list with target user/group and validity period on record |
| 8 | live evidence | Current role assignment snapshot from target system |
| 9 | live evidence or documented exception | SoD pre-check result (pass/fail) and effective-permission diff |
| 10 | user-provided / inference | Blast radius document confirmed |
| 11 | user-provided evidence | Rollback procedure confirmed feasible |
| 12 | user-provided evidence | SoD confirmed: requester ≠ approver; SoD pre-check passed |
| 13 | user-provided evidence | Explicit approval statement on record |
| 14 | live evidence | Execution log with timestamp |
| 15 | live evidence | Post-change role check verified; second SoD check passed |
| 16 | all of the above | Audit record compiled |
| 17 | user-provided evidence | Report delivered and acknowledged |

## Output contract

Return after each step:

1. Step number and name
2. Evidence gathered for this step (labeled by evidence type)
3. Gate status: cleared / pending / blocked
4. Next step and what is needed to advance
5. Refusal marker if any gate condition is unmet (do not advance)
6. SoD status if step 9 or step 12 (pass / fail / pending)

Return after step 17:

1. Complete audit record (all 17 steps with evidence and timestamps)
2. Final change status (success / partial / failed)
3. Post-change verification result including second SoD check
4. Rollback status (not needed / standing by / triggered)
5. Report delivery confirmation
