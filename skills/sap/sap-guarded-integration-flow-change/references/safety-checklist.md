# Safety checklist — SAP Guarded Integration Flow Change

Use before every step and unconditionally before step 14 execution.

## Non-negotiables

- Do not reach step 14 (execute) without documented integration-owner approval on record in this session.
- Do not allow self-approval. The requester (step 4) and the integration owner / approver (step 5) must be different individuals. If they are the same, refuse execution.
- Do not deploy artifacts not on the approved list from step 7.
- Do not deploy any artifact for which a diff (step 9) has not been produced and reviewed.
- Do not skip or reorder steps. The sequence is mandatory.
- Do not use Integration Suite tenant administrator credentials. Use an Integration Developer role scoped to the specific tenant.
- Do not accept, log, echo, or include Cloud Integration service key credential values, OAuth client secrets, or process integration runtime user passwords in any output.
- Do not attempt self-recovery deployments without initiating a new 17-step sequence.
- Do not deploy to a production tenant without a documented rollback plan that identifies the specific previous version to redeploy (step 11).
- Do not ignore ERROR or RETRY status messages in post-deployment monitoring (step 15). Assess every failure before closing the session.

## What people get wrong

- **Treating verbal integration-owner approval as sufficient**: Step 13 requires explicit written approval documented in the session. Verbal or assumed approval does not clear the gate.
- **Skipping the diff because the change seems minor**: Step 9 is mandatory. Minor-looking changes (e.g., a single script step update) can affect routing behavior, adapter retry configuration, or security material references in ways that only a diff reveals.
- **Assuming QA tenant deployments are low risk**: QA tenants often connect to external partner test systems with real data and SLAs. The full sequence applies.
- **Batching multiple iFlow deployments without per-artifact diff**: Each iFlow must have its own diff documented in step 9. Batching without per-artifact review is forbidden.
- **Using the wrong role for deployment**: The deploying user must have the Integration Developer role for the specific tenant. A tenant administrator role is over-privileged.
- **Not accounting for message-in-flight impact during deployment**: Deploying an iFlow while messages are being processed can cause message loss or duplication. The blast radius assessment in step 10 must address in-flight message handling.
- **Treating a WARNING in message monitoring as a pass**: WARNING or RETRY status in message processing logs can indicate transient failures that persist. Step 15 must assess every non-Completed status before the session is closed.
- **Not identifying downstream partner impact**: The blast radius (step 10) must name every external partner system, API consumer, and dependent iFlow. An iFlow change that alters a message structure or endpoint URL is a breaking change for downstream partners.

## When to push back

- Push back when the user asks to "just deploy it" without completing prior steps.
- Push back when the requester and integration owner are the same person.
- Push back when no change management ticket number is provided.
- Push back when a diff of artifact changes has not been produced.
- Push back when the rollback plan is "we'll redeploy if something goes wrong" without naming the specific version to restore.
- Push back when the target is a production tenant and no blast radius covering downstream partners has been documented.
- Push back when step 15 message monitoring shows ERROR or RETRY status and the user wants to close the session without assessment.
- Push back when the user provides a credential with tenant administrator scope instead of Integration Developer scope.

## Evidence labels

- `live evidence` — directly observed from a live Cloud Integration tenant; include API endpoint, response summary, timestamp, tenant ID
- `documentation-based` — grounded in SAP official docs; no live access
- `user-provided evidence` — stated, uploaded, or confirmed by the user in this session
- `inference` — derived reasoning; must always be labeled as such

## SoD verification matrix

| Role | Can request? | Can approve? |
|------|-------------|-------------|
| Integration developer | Yes | No |
| Functional consultant | Yes | No |
| Integration owner (accountable for iFlow and downstream partners) | No | Yes |
| Delivery lead / project manager | No | Yes (for non-production tenants; integration owner required for production) |
| Requester and approver in same person | No — SoD violation | Refuse execution |
| Automated CI/CD pipeline without human integration-owner approval | No | Refuse — human integration-owner approval required for production |
