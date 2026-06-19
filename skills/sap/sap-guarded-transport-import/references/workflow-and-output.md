# Workflow and output contract — SAP Guarded Transport Import

Use this reference for all step-by-step execution, command patterns, and output formatting.

## Step execution protocol

For each of the 17 steps, the following must be true before advancing to the next:

1. The step's required evidence is on record in this session.
2. The user has confirmed the evidence is accurate.
3. Any step-specific gate (e.g., approval, SoD check) has been explicitly cleared.

If any of these conditions is false, the session pauses at the current step until resolved.

## Command patterns by step

### Step 8 — Read-only current state

#### STMS (ABAP)
```
Display import queue: transaction STMS → Import Overview → select target system
Display transport log: transaction STMS → Transport Logs → filter by TR number
RFC call (read-only): CTS_API_GET_COFILE or equivalent display FM
```

#### CTS+ API (read)
```
GET /cts/transports/{transportId}
GET /cts/systems/{systemId}/importqueue
```

#### Cloud TMS API (read)
```
GET /v2/nodes/{nodeId}/transportRequests
GET /v2/importqueue/{nodeId}
GET /v2/nodes/{nodeId}/deployments/{deploymentId}
```

### Step 14 — Execute approved import

#### STMS (ABAP)
```
Transaction STMS → Import Overview → select target system → import approved TR only
Authorization check: user must have S_TRANSPRT with ACTVT=42 (import) for target system
```

#### Cloud TMS API (mutating — step 14 only, after step 13 gate is cleared)
```
POST /v2/nodes/{nodeId}/transportRequests/{trId}/deploy
Body: { "description": "<ticket reference from step 6>", "versionStrategy": "PICK_ALL_MATCHING_COMMITS" }
```

**Never call the POST endpoint before step 13 approval gate is documented.**

## Step completion checklist

| Step | Evidence type | Completion marker |
|------|--------------|------------------|
| 1 | user-provided / documentation-based | Transport type and content category confirmed |
| 2 | user-provided evidence | Target SID, client, tier confirmed |
| 3 | user-provided evidence | Criticality level assigned |
| 4 | user-provided evidence | Requester name and role on record |
| 5 | user-provided evidence | Approver name, role, and identity on record |
| 6 | user-provided evidence | Ticket number confirmed |
| 7 | user-provided evidence | TR list with descriptions and sequence on record |
| 8 | live evidence | Current state snapshot from target system |
| 9 | live evidence or documented exception | Diff output or dry-run result |
| 10 | user-provided / inference | Blast radius document confirmed |
| 11 | user-provided evidence | Rollback procedure confirmed feasible |
| 12 | user-provided evidence | SoD confirmed: requester ≠ approver |
| 13 | user-provided evidence | Explicit approval statement on record |
| 14 | live evidence | Import command log with timestamp |
| 15 | live evidence | Import log verified, smoke test result |
| 16 | all of the above | Audit record compiled |
| 17 | user-provided evidence | Report delivered and acknowledged |

## Output contract

Return after each step:

1. Step number and name
2. Evidence gathered for this step (labeled by evidence type)
3. Gate status: cleared / pending / blocked
4. Next step and what is needed to advance
5. Refusal marker if any gate condition is unmet (do not advance)

Return after step 17:

1. Complete audit record (all 17 steps with evidence and timestamps)
2. Final import status (success / partial / failed)
3. Post-import verification result
4. Rollback status (not needed / standing by / triggered)
5. Report delivery confirmation
