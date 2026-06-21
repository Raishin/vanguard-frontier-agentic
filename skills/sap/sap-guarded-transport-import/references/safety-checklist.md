# Safety checklist — SAP Guarded Transport Import

Use before every step and unconditionally before step 14 execution.

## Non-negotiables

- Do not reach step 14 (execute) without a documented approval from an authorized approver on record in this session.
- Do not allow self-approval. The requester (step 4) and the approver (step 5) must be different individuals. If they are the same, refuse execution.
- Do not import transport requests not on the approved TR list from step 7.
- Do not skip or reorder steps. The sequence is mandatory.
- Do not use BASIS administrator credentials. Use a transport administrator role scoped to the target system only.
- Do not accept, log, echo, or include ABAP logon passwords, STMS service user passwords, CTS+ service key credential values, or Cloud TMS OAuth tokens in any output.
- Do not attempt self-recovery after a failed import without initiating a new 17-step sequence.
- Do not import into a production system without a documented rollback plan (step 11).

## What people get wrong

- **Treating verbal approval as sufficient**: Step 13 requires explicit written approval documented in the session. Verbal or assumed approval does not clear the gate.
- **Assuming DEV → QA imports are low risk**: QA systems run integrated testing and UAT. Imports into QA must follow the full sequence.
- **Batching multiple TRs without per-TR classification**: Each transport request must appear in step 7 with its description. Batch importing unlisted TRs is forbidden.
- **Using the wrong user for import**: The import user must have S_TRANSPRT authorization for the target system. A developer user or BASIS admin is not appropriate.
- **Skipping step 9 because "it's a config transport"**: Configuration transports can overwrite production settings. Diff and dry-run are mandatory regardless of transport type.
- **Not documenting the blast radius for customizing transports**: Customizing transports (client-specific) can affect system-wide business process settings. Blast radius must include affected business processes, not just ABAP objects.
- **Proceeding after a WARNING in the import log**: WARNING entries in STMS/Cloud TMS import logs can indicate partial failures. Step 15 must assess every WARNING before the session is closed.

## When to push back

- Push back when the user asks to "just run the import" without completing prior steps.
- Push back when the requester and approver are the same person.
- Push back when no change management ticket number is provided.
- Push back when the rollback plan is "we'll figure it out if something goes wrong."
- Push back when the import target is production and no dry-run or diff has been performed.
- Push back when the user provides a credential with more than transport-administrator scope (e.g., BASIS admin, SAP_ALL).
- Push back when post-import verification (step 15) shows errors and the user wants to proceed without assessment.

## Evidence labels

- `live evidence` — directly observed from a live SAP system; include command, timestamp, system SID
- `documentation-based` — grounded in SAP official docs; no live access
- `user-provided evidence` — stated, uploaded, or confirmed by the user in this session
- `inference` — derived reasoning; must always be labeled as such

## SoD verification matrix

| Role | Can request? | Can approve? |
|------|-------------|-------------|
| Developer (Z-developer role) | Yes | No |
| Functional consultant | Yes | No |
| Change manager | No | Yes |
| Basis administrator | No | Yes (for technical approval only) |
| Project manager | No | Yes |
| Developer and change manager in same person | No — SoD violation | Refuse execution |
