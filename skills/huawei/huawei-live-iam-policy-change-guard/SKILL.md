---
name: huawei-live-iam-policy-change-guard
description: Gate IAM fine-grained policy and SCP mutations — account-wide blast radius, privilege escalation, and potential full access denial. SCP deny statements at Organizations level cascade to ALL member accounts and cannot be overridden by IAM policies. Prevents dangerous policy mutations from proceeding without blast-radius assessment, agency trust enumeration, and explicit operator approval.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# Huawei Live IAM Policy Change Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-iam-policy-change-guard work. Gate IAM fine-grained policy mutations, SCP changes, and agency trust relationship grants. Insist on blast-radius assessment, privilege escalation review, and rollback posture evidence before execution. Treat any ambiguous approval, unscoped policy, or SCP mutation as a stop condition.

## When to Use

Use this skill when:

- An IAM fine-grained custom policy is being created, modified, or deleted
- A system policy (e.g., ECS FullAccess, OBS FullAccess) is being attached to or detached from a user, group, or agency
- An SCP (Service Control Policy) is being created, modified, or applied at the Organizations level
- An agency trust relationship is being created or modified (cross-account access grants)
- Enterprise project IAM permission bindings are being changed
- A user or group is being added to a privileged IAM group
- An MFA policy is being changed for privileged accounts

## When NOT to Use

Do not use this skill when:

- The task is read-only IAM inspection with no mutation intent
- The task involves resource-level changes unrelated to IAM (e.g., ECS instance start/stop)
- The task is purely reviewing documentation or explaining IAM concepts

## Pre-Flight Checklist

Before executing any IAM mutation, verify all of the following:

1. **Account identity confirmed** — confirm the account ID, master vs. member account distinction, and active IAM principal.
2. **Enterprise project scope confirmed** — confirm whether the policy change is account-wide or scoped to a specific enterprise project.
3. **Active principal confirmed** — confirm the active IAM user or agency token has the required IAM FullAccess policy (for mutations) or IAM ReadOnlyAccess (for audits).
4. **Current policy inventory captured** — list all policies currently attached to the target user/group/agency before proceeding.
5. **SCP inventory reviewed** — if the task involves member account IAM, list active SCPs at the org level to understand what is blocked or allowed before any change.
6. **Proposed change blast-radius assessed** — enumerate all services, principals, and enterprise projects affected by the proposed change.
7. **Privilege escalation check** — confirm the proposed change does not grant the ability to create/attach FullAccess policies, create agencies, or escalate beyond the current principal's privilege level.
8. **Rollback path confirmed** — document the reverse action (detach policy, delete agency, etc.) before executing.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the target account is `<ACCOUNT_ID>` and the IAM principal is `<USER/GROUP/AGENCY_NAME>`."
- "I confirm the proposed change is `<DESCRIBE_CHANGE>` and the enterprise project scope is `<SCOPE>`."
- "I have reviewed the active SCP inventory and confirmed this change is not blocked by an SCP deny."
- "I have assessed the blast radius: `<ENUMERATE_AFFECTED_SERVICES_AND_PRINCIPALS>`."
- "I approve this IAM change."

For SCP mutations, additionally require:
- "I confirm I have org-admin authority to apply this SCP change."
- "I confirm this SCP change has been tested in a non-production member account."

## Execution Steps

1. Capture pre-change state: list all current policy attachments, active SCPs, and agency trust relationships for the target principal.
2. Confirm active principal IAM FullAccess authority.
3. Present the planned change and its blast radius to the operator for explicit approval.
4. Execute the mutation via Huawei Cloud IAM console or API:
   - Attach policy to user/group: IAM console > Users/User Groups > Permissions > Grant Permissions
   - Create/modify fine-grained policy: IAM console > Permissions > Custom Policies
   - Create/modify agency: IAM console > Agencies
   - Apply SCP: Organizations console > Policies > Service Control Policies > Attach
5. Verify the change took effect by re-listing policies for the target principal.
6. Test access for a dependent service or resource to confirm the intended effect.

## Rollback Procedure

- **Policy detach** (reversible): Detach the policy from the user/group/agency via IAM console.
- **Fine-grained policy deletion** (reversible if no dependencies): Delete the custom policy; verify no principals rely on it.
- **Agency deletion** (reversible): Delete the agency via IAM console; cross-account access is immediately revoked.
- **SCP removal** (reversible): Detach the SCP from the org unit or member account; IAM policies in member accounts resume normal evaluation.
- Document all rollback actions in CTS and the associated change ticket.

## Post-Change Verification

1. Re-list policies attached to the target principal — confirm the change is applied.
2. Verify dependent services can still authenticate and authorize correctly.
3. Check CTS logs for any unexpected policy evaluation errors following the change.
4. For SCP changes, verify member account access for a sample set of previously allowed operations.
5. If MFA policy changed, verify at least one admin user retains console access.

## Response Shape

1. Account and IAM principal confirmed
2. Current policy/SCP inventory
3. Proposed change and blast-radius
4. Agency trust assessment
5. Approval status
6. Applied change
7. Post-change access verification
