# Safety checklist — SAP Guarded Role Assignment

Use before every step and unconditionally before step 14 execution.

## Non-negotiables

- Do not reach step 14 (execute) without a documented approval from an authorized approver on record in this session.
- Do not allow self-approval. The requester (step 4) and the approver (step 5) must be different individuals. If they are the same, refuse execution.
- Do not assign any role that produces an SoD conflict identified in step 9 — even with explicit approval from the security owner. SoD conflicts are a hard stop.
- Do not assign roles not on the approved list from step 7.
- Do not skip or reorder steps. The sequence is mandatory.
- Do not use global account administrator or tenant administrator credentials. Use a user with User and Role Administrator scope for the specific target subaccount or system client only.
- Do not accept, log, echo, or include BTP service keys, IAS client secrets, OAuth tokens, or ABAP logon passwords in any output.
- Do not attempt self-recovery after a failed assignment without initiating a new 17-step sequence.
- Do not grant SAP_ALL, Administrator role collections, or any other unrestricted authorization profile under any circumstance — not even temporarily.
- Do not assign roles without a documented validity period for time-limited access grants.

## What people get wrong

- **Treating verbal approval as sufficient**: Step 13 requires explicit written approval documented in the session. Verbal or assumed approval does not clear the gate.
- **Skipping the SoD pre-check because the role seems harmless**: Step 9 is mandatory regardless of role type. Configuration roles can create SoD conflicts with existing financial roles.
- **Assuming QA user assignments are low risk**: QA systems often replicate production authorization landscapes and run integrated UAT. The full sequence applies.
- **Batching multiple users without per-user SoD checks**: Each target user may have different existing roles. The SoD pre-check must be run per user, not once for the role collection.
- **Using the wrong credential scope for assignment**: The executing user must have User and Role Administrator authorization scoped to the target subaccount or client only. Global Account Administrator scope is too broad.
- **Skipping step 9 because GRC is not available**: If no GRC or Cloud IAG tool is available, document the reason, request a manual SoD review from the security owner, and treat the result as an explicit exception — not a pass.
- **Treating "temporary access" as an exemption from the sequence**: All assignments — including time-limited ones — require the full 17 steps, with the validity period documented in step 7.
- **Proceeding when step 15 reveals unintended permissions**: If the post-change effective-permission check shows access beyond the intended scope, stop immediately and execute the rollback plan.

## When to push back

- Push back when the user asks to "just assign the role" without completing prior steps.
- Push back when the requester and approver are the same person.
- Push back when no access request or change management ticket number is provided.
- Push back when the SoD pre-check returns any Critical or High risk conflict.
- Push back when the requested role is SAP_ALL, Administrator, or any broadly privileged collection.
- Push back when the target system is production and no diff of effective permissions has been produced.
- Push back when post-change verification (step 15) shows unintended access and the user wants to proceed without assessment or rollback.
- Push back when the user provides a credential with global account administrator or tenant administrator scope.

## Evidence labels

- `live evidence` — directly observed from a live SAP system; include command, timestamp, system SID or subaccount GUID
- `documentation-based` — grounded in SAP official docs; no live access
- `user-provided evidence` — stated, uploaded, or confirmed by the user in this session
- `inference` — derived reasoning; must always be labeled as such

## SoD verification matrix

| Role | Can request? | Can approve? |
|------|-------------|-------------|
| End user (business process owner) | Yes | No |
| Functional consultant | Yes | No |
| Security officer / GRC analyst | No | Yes |
| Compliance officer | No | Yes |
| IT manager / project manager | No | Yes (with security owner co-signature for high-criticality roles) |
| Requester and approver in same person | No — SoD violation | Refuse execution |
| Automated provisioning without human approver | No | Refuse — human approver required |
