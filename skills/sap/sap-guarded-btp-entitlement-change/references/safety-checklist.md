# Safety checklist — SAP Guarded BTP Entitlement Change

Use before every step and unconditionally before step 14 execution.

## Non-negotiables

- Do not reach step 14 (execute) without documented dual approval from both the platform owner and FinOps approver on record in this session.
- Do not allow self-approval. The requester (step 4) must be different from both the platform owner and the FinOps approver (step 5). If the requester is the same as either approver, refuse execution.
- Do not increase entitlements without FinOps approval. Technical necessity does not override this gate.
- Do not change entitlements for services not on the approved list from step 7.
- Do not skip or reorder steps. The sequence is mandatory.
- Do not execute entitlement changes without a documented cost-impact assessment (step 9 diff). Changes with undocumented cost impact are refused.
- Do not use Global Account Administrator credentials when Entitlements Administrator scope is sufficient for step 14.
- Do not accept, log, echo, or include BTP service key credential values, OAuth tokens, or cockpit session tokens in any output.
- Do not attempt self-recovery changes without initiating a new 17-step sequence.
- Do not remove an entitlement or subscription without confirming and documenting all active consumers in step 10. Removal without consumer identification is refused.

## What people get wrong

- **Treating one approver as sufficient**: Step 13 requires explicit written approval from both the platform owner and the FinOps approver. One approver clearing the gate for both is not permitted.
- **Skipping the cost-impact assessment because the quota increase seems small**: Step 9 is mandatory regardless of the quota delta. Metered services can accumulate significant cost at scale. The assessment must be documented even if the delta is expected to be negligible.
- **Assuming non-production global account changes are low risk**: Non-production global accounts share contract entitlement pools with production in many BTP contracts. An entitlement moved to a non-production subaccount may reduce available quota for production applications.
- **Not identifying active consumers before removing an entitlement**: Applications that depend on a removed entitlement will fail immediately. Step 10 must name every application and service consuming the entitlement being removed.
- **Using Global Account Administrator credentials for routine entitlement management**: The Entitlements Administrator role collection is sufficient for step 14. Global Account Administrator grants additional rights (trust configuration, directory management) that are not needed and violate least-privilege.
- **Treating "we'll monitor costs" as a FinOps approval substitute**: A monitoring commitment is not an approval. FinOps must provide explicit written approval naming the service plan and quota amount before step 14.
- **Batching multiple subaccount changes without per-subaccount cost-impact assessment**: Each subaccount change may have different consumer dependencies and cost exposure. Batching without per-subaccount assessment is refused.

## When to push back

- Push back when the user asks to "just enable the service" without completing prior steps.
- Push back when the requester is the same as either the platform owner or FinOps approver.
- Push back when FinOps approval is absent for any entitlement increase.
- Push back when no cost-impact assessment has been produced (step 9).
- Push back when no change management or FinOps budget ticket is provided.
- Push back when the change targets removal of an entitlement without a documented list of active consumers.
- Push back when the rollback plan is "we'll reassign the entitlement if something goes wrong" without specifying the exact quota values to restore.
- Push back when step 15 reveals application disruption or unexpected billing events and the user wants to close the session without assessment.
- Push back when the user provides Global Account Administrator credentials for a task that Entitlements Administrator can perform.

## Evidence labels

- `live evidence` — directly observed from a live BTP global account; include API endpoint, response summary, timestamp, global account ID
- `documentation-based` — grounded in SAP official docs or SAP BTP pricing documentation; no live access
- `user-provided evidence` — stated, uploaded, or confirmed by the user in this session
- `inference` — derived reasoning; must always be labeled as such

## SoD verification matrix

| Role | Can request? | Can approve (platform)? | Can approve (FinOps)? |
|------|-------------|------------------------|----------------------|
| Developer / architect | Yes | No | No |
| Subaccount administrator | Yes | No | No |
| Platform owner (accountable for global account architecture) | No | Yes | No |
| FinOps manager / cost controller | No | No | Yes |
| IT manager | No | Yes (for non-production only; platform owner required for production) | No |
| Requester = platform owner or FinOps | No — SoD violation | Refuse execution | Refuse execution |
| Single approver for both platform and FinOps roles | No — dual approval required | Refuse execution | Refuse execution |
