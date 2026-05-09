---
name: alibaba-live-ram-policy-change-guard
description: Gate RAM policy/role mutations against the Alibaba Cloud account hierarchy. RAM AdministratorAccess assignment, policy deletion with active STS tokens, and Resource Directory Control Policy changes carry account-wide or org-wide blast radius. This guard enforces blast-radius assessment, STS token impact analysis, and explicit authority approval before any policy mutation is executed.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# Alibaba Cloud Live RAM Policy Change Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-ram-policy-change-guard work. Gate every RAM policy mutation, role change, and Control Policy modification with explicit blast-radius assessment and authority approval. Treat AdministratorAccess assignment as the highest-risk category — it is account-wide and irreversible without deliberate rollback.

## When to Use

Use this skill when:

- A RAM policy must be created, modified, or deleted
- A RAM role is being created, deleted, or having policies attached/detached
- A RAM user is being granted or revoked access to a policy
- AdministratorAccess or any system policy with broad permissions is being assigned
- A Resource Directory Control Policy constraint is being created, modified, or deleted for an OU
- An operator needs to audit the current RAM policy and role inventory before making changes
- Detecting and remediating over-privileged RAM users, roles, or stale policy attachments

## When NOT to Use

Do not use this skill when:

- The task is a read-only RAM audit with no mutation intent
- The task involves Kubernetes RBAC within ACK only (no RAM changes)
- The task is creating a new RAM user with read-only access (low risk, no live-guard required)
- The task is unrelated to Alibaba Cloud identity and access management

## Pre-Flight Checklist

Before executing any RAM mutation, verify all of the following:

1. **Account identity confirmed** — explicitly state the target Alibaba Cloud account ID. Confirm via `aliyun ram GetAccountAlias` or the console.
2. **Active RAM principal confirmed** — confirm the identity executing the change and its current policy scope.
3. **Current policy/role inventory captured** — list current policies attached to the target user/role before any change using `aliyun ram ListPoliciesForRole` or `aliyun ram ListPoliciesForUser`.
4. **Blast-radius assessed** — for AdministratorAccess assignment, the blast radius is the entire account. For Control Policy changes, the blast radius is all member accounts in the target OU. Document this explicitly.
5. **Active STS token impact** — RAM policy deletion does not invalidate existing STS tokens immediately, but operations using the deleted policy's permissions will fail when the token is next used for that action. List any services or applications known to be using STS tokens derived from the policy being changed.
6. **Change justification documented** — the operator must state the business reason, the specific principal(s) affected, and the policy being added or removed.
7. **Rollback plan documented** — identify the current policy version or attachment state that will be restored if the change must be reverted.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the target account is `<ACCOUNT_ID>`."
- "I confirm the principal is `<RAM_USER_NAME / ROLE_NAME>` and the policy change is `<ATTACH/DETACH/CREATE/DELETE> <POLICY_NAME>`."
- "I understand the blast radius of this change: `<scope statement>`."
- "I have assessed the active STS token impact and it is `<acceptable / none known>`."
- "I approve this RAM change."
- For AdministratorAccess assignment: "I confirm I have the authority to grant account-wide admin access and this is explicitly required."

## Execution Steps

1. Capture pre-change RAM policy and role inventory snapshot.
2. Confirm active RAM principal has `AliyunRAMFullAccess` (assumed via STS for specific change only).
3. Present the planned change, blast-radius assessment, and STS token impact to the operator for explicit approval.
4. Execute the mutation via the RAM console or Alibaba Cloud CLI:
   - Attach policy to role: `aliyun ram AttachPolicyToRole --PolicyType <System/Custom> --PolicyName <NAME> --RoleName <ROLE>`
   - Detach policy from role: `aliyun ram DetachPolicyFromRole --PolicyType <System/Custom> --PolicyName <NAME> --RoleName <ROLE>`
   - Attach policy to user: `aliyun ram AttachPolicyToUser --PolicyType <System/Custom> --PolicyName <NAME> --UserName <USER>`
   - Delete custom policy: `aliyun ram DeletePolicy --PolicyName <NAME>`
   - Create custom policy: `aliyun ram CreatePolicy --PolicyName <NAME> --PolicyDocument <DOCUMENT>`
5. Capture post-change inventory snapshot and diff against pre-change snapshot.

## Rollback Procedure

- **Policy attachment** (reversible): Detach the policy with `DetachPolicyFromRole` or `DetachPolicyFromUser`. Effect is immediate.
- **Policy detachment** (reversible but risky): Re-attach the policy. If the detachment caused a service lockout, restore it immediately.
- **Policy deletion** (NOT reversible): A deleted custom policy cannot be recovered. Re-create the policy from the documented pre-change version. System policies (AliyunXxxAccess) are not deletable.
- **AdministratorAccess assignment** (reversible — detach the policy): Detach `AdministratorAccess` from the principal immediately if granted in error.

## Post-Change Verification

1. Run `aliyun ram ListPoliciesForRole --RoleName <ROLE>` or `aliyun ram ListPoliciesForUser --UserName <USER>` — confirm the policy list reflects the intended change.
2. Test access for the affected principal to verify the change has the expected effect.
3. Check ActionTrail for the change event: query for `EventName` containing the mutation operation.
4. For Control Policy changes, confirm constraint enforcement in Resource Directory console.

## Response Shape

1. Account and RAM principal confirmed
2. Current policy/role inventory
3. Proposed change and blast-radius assessment
4. Active STS token impact
5. Approval status
6. Applied change
7. Post-change access verification
