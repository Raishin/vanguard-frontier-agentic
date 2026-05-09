---
name: huawei-live-kms-key-destruction-guard
description: Gate DEW/KMS key deletion and disable operations — all CSMS secrets and DBSS-encrypted database data become permanently unrecoverable once the key deletion window passes.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# Huawei Live KMS Key Destruction Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-kms-key-destruction-guard work. Gate DEW/KMS key deletion and disable operations. Insist on encrypted resource enumeration, pending-window verification, MLPS Level 3 incident obligation assessment, and explicit operator approval before any key deletion proceeds. Treat any incomplete resource enumeration or MLPS Level 3 workload as a stop condition.

## When to Use

Use this skill when:

- A DEW/KMS key is being scheduled for deletion (entering pending deletion state)
- A DEW/KMS key is being disabled (encrypted data becomes temporarily inaccessible)
- A KMS key version is being disabled (old ciphertext encrypted under that version becomes inaccessible)
- A CSMS secret is being deleted (permanent)
- A CBH (Cloud Bastion Host) session recording policy is being deleted or retention period reduced
- A DBSS SQL audit policy is being disabled or deleted

## When NOT to Use

Do not use this skill when:

- The task is read-only KMS key inspection with no deletion or disable intent
- The task involves creating a new KMS key or rotating to a new key version
- The task involves ECS, CCE, database instances, or other non-DEW services without encryption key context

## Gate Protocol

This skill requires the 6-step live-guard gate from the maestro. See `skills/huawei/huawei-maestro/SKILL.md` for the full gate protocol. The 6 steps are:

1. **Surface risk** — state the specific risk: permanent data loss for all resources encrypted by this key.
2. **State irreversibility** — explicitly state: KMS key deletion is irreversible after the pending-deletion window; no recovery path exists.
3. **Confirm target** — confirm exact key ID, alias, account, enterprise project, and pending-deletion window length.
4. **Assess blast radius** — enumerate ALL resources encrypted by this key: OBS objects (SSE), ECS disk encryption, database CMK encryption, CSMS secrets, DBSS-encrypted data.
5. **Require rollback path** — document the cancel-pending-deletion action (available only within the pending window); after window expires there is no rollback.
6. **Get explicit written confirmation** — require the operator to state key ID, window length, enumerated encrypted resources, and MLPS classification before proceeding.

## Pre-Flight Checklist

Before scheduling any KMS key for deletion, verify all of the following:

1. **Key identity confirmed** — confirm key ID, alias, key type (symmetric/asymmetric), and current status.
2. **Pending deletion window reviewed** — confirm the window length (minimum 7 days, maximum 1096 days); longer windows provide more recovery time.
3. **Encrypted resource enumeration complete** — list all OBS buckets using this key for SSE, all ECS instances with disk encryption using this key, all GaussDB/RDS instances with CMK encryption, all CSMS secrets encrypted by this key, all DBSS-encrypted databases.
4. **MLPS Level 3 classification checked** — determine if any encrypted resource is subject to MLPS Level 3; if yes, key deletion constituting data destruction triggers mandatory 24-hour incident reporting.
5. **Cancel-pending-deletion procedure documented** — confirm the operator knows how to cancel pending deletion within the window (DEW console > Keys > Cancel Pending Deletion).
6. **Approval documented** — obtain explicit written operator approval including key ID, window, and MLPS obligation acknowledgment.

## Required Confirmation

The operator must explicitly state all of the following before key deletion is scheduled:

- "I confirm the target key is `<KEY_ID>` (`<KEY_ALIAS>`) in account `<ACCOUNT_ID>`, enterprise project `<ENTERPRISE_PROJECT>`."
- "I confirm the pending deletion window is `<DAYS>` days and I understand the key is permanently destroyed after this window."
- "I have enumerated all resources encrypted by this key: `<LIST_ENCRYPTED_RESOURCES>`."
- "I confirm all enumerated resources have been migrated to a new key or decommissioned."
- "I confirm this key `[IS / IS NOT]` subject to MLPS Level 3 classification."
- "I approve scheduling this key for deletion."

For MLPS Level 3 classified keys, additionally require:
- "I confirm that this key deletion constitutes data destruction and I will file the mandatory incident report within 24 hours."

## Rollback Procedure

- **Within pending-deletion window** (reversible): DEW console > Keys > select key > Cancel Pending Deletion — key returns to Enabled or Disabled state.
- **After window expires** (irreversible): no recovery path. All data encrypted by this key is permanently inaccessible.
- Document the cancel-pending-deletion deadline prominently in the change ticket.

## Post-Action Verification

1. Confirm key status shows `Pending Deletion` with the correct deletion date.
2. Verify the pending-deletion window matches the approved window length.
3. Test access to a sample encrypted resource to confirm it is still accessible within the window.
4. If MLPS Level 3 data destruction: initiate incident report process within 24 hours.

## Response Shape

1. Key identity confirmed
2. Pending deletion window and expiry date
3. Encrypted resource enumeration
4. MLPS Level 3 classification
5. Blast radius summary
6. Rollback window deadline
7. Approval status
8. Executed action
9. Post-action verification
