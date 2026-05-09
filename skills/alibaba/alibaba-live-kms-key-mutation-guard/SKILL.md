---
name: alibaba-live-kms-key-mutation-guard
description: Gate KMS key deletion and disable operations. All data encrypted with a deleted CMK (OSS SSE-KMS, ECS encrypted disks, RDS/PolarDB TDE) becomes permanently and irrecoverably inaccessible. This guard enforces complete CMK dependency audits, deletion window confirmation, and explicit operator approval before any key state mutation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# Alibaba Cloud Live KMS Key Mutation Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-kms-key-mutation-guard work. Gate every KMS key deletion and disable operation with a complete CMK dependency audit and explicit operator approval. Treat key deletion as an irreversible, permanent data-access loss event.

## When to Use

Use this skill when:

- A KMS CMK (Customer Master Key) deletion is requested or scheduled
- A KMS CMK is being disabled
- A pending key deletion needs to be cancelled
- A key rotation is being configured or triggered
- An operator needs to audit CMK-dependent resources before a key operation
- A key is being re-enabled after a disable operation

## When NOT to Use

Do not use this skill when:

- The task is a read-only KMS audit with no mutation intent
- The task involves only alias operations (alias creation/deletion does not affect key availability)
- The task is creating a new CMK (no existing data at risk)
- The task involves envelope encryption key management at the application layer with no KMS API calls

## Key State Model

Alibaba Cloud KMS keys have the following states:

- **Enabled**: Key is active and can be used for encryption/decryption.
- **Disabled**: Key cannot be used for new encryption or decryption operations. **This is reversible** — re-enable restores full functionality. Existing encrypted data remains accessible once re-enabled.
- **PendingDeletion**: Key is scheduled for deletion. The deletion window is 7–30 days (default: 30 days). **This can be cancelled** during the pending window.
- **Deleted**: Key has been permanently destroyed. **All encrypted data is irrecoverable.**

Always recommend disable over deletion when the operator is uncertain about CMK dependencies.

## CMK Dependency Categories

Before any key deletion or disable, audit ALL of the following:

- **OSS SSE-KMS**: Buckets using this CMK for server-side encryption. Objects become inaccessible if the key is deleted.
- **ECS Encrypted Disks**: System and data disks encrypted with this CMK. Instances cannot decrypt disk data.
- **ECS Snapshot Encryption**: Snapshots created from encrypted disks using this CMK.
- **RDS Transparent Data Encryption (TDE)**: RDS instances with TDE enabled using this CMK.
- **PolarDB TDE**: PolarDB clusters with TDE using this CMK.
- **SLB/ALB HTTPS Certificates**: Certificates stored in KMS used by load balancers.
- **Secrets Manager (SSM)**: Secrets encrypted with this CMK.

## Pre-Flight Checklist

Before executing any KMS key mutation, verify all of the following:

1. **Key identity confirmed** — confirm the exact key ID (not alias). Run `aliyun kms DescribeKey --KeyId <KEY_ID>` to confirm key metadata, current state, and region.
2. **Active RAM principal confirmed** — confirm the identity has `AliyunKMSFullAccess` assumed via STS for this specific operation.
3. **CMK dependency audit complete** — enumerate all OSS buckets, ECS disks, RDS instances, PolarDB clusters, and SSM secrets using this key. This audit must be complete before any mutation.
4. **Key state assessed** — confirm whether the key is Enabled, Disabled, or already in PendingDeletion state.
5. **Disable vs. delete decision** — if the operator is uncertain about dependencies, recommend disable first. Only proceed to deletion after all dependencies are migrated or confirmed non-critical.
6. **Deletion window confirmed** — if proceeding with deletion, confirm the scheduled deletion window (7–30 days). Default is 30 days.
7. **Rollback plan** — for disable: re-enable is the rollback. For deletion: no rollback after the window expires.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the key is `<KEY_ID>` in region `<REGION>` in account `<ACCOUNT_ID>`."
- "I have completed the CMK dependency audit and the following dependencies are confirmed: `<list or NONE>`."
- "I understand that key deletion is permanent after the `<N>`-day deletion window and all dependent encrypted data will be irrecoverable."
- "I approve this key `<disable / deletion>` action."
- For deletion: "I confirm all CMK-dependent data has been migrated or is non-critical."

## Execution Steps

1. Capture pre-change key state: `aliyun kms DescribeKey --KeyId <KEY_ID>`.
2. Enumerate CMK dependencies (see CMK Dependency Categories above).
3. Present the planned change, dependency audit results, and deletion window to the operator for explicit approval.
4. Execute the mutation:
   - Disable key: `aliyun kms DisableKey --KeyId <KEY_ID>`
   - Schedule deletion: `aliyun kms ScheduleKeyDeletion --KeyId <KEY_ID> --PendingWindowInDays <7-30>`
   - Cancel deletion: `aliyun kms CancelKeyDeletion --KeyId <KEY_ID>`
   - Re-enable key: `aliyun kms EnableKey --KeyId <KEY_ID>`
5. Confirm new key state: `aliyun kms DescribeKey --KeyId <KEY_ID>`.

## Rollback Procedure

- **Key disable** (reversible): `aliyun kms EnableKey --KeyId <KEY_ID>` — restores full encryption/decryption capability immediately.
- **Key scheduled for deletion** (reversible during pending window): `aliyun kms CancelKeyDeletion --KeyId <KEY_ID>` — cancels the scheduled deletion. Act before the deletion window expires.
- **Key deleted** (NOT reversible): A deleted key cannot be recovered. All data encrypted with this key is permanently inaccessible. Contact Alibaba Cloud Support immediately — there is no recovery path.

## Post-Change Verification

1. Confirm key state matches expected state: `aliyun kms DescribeKey --KeyId <KEY_ID>`.
2. For disable: verify that applications dependent on this key are gracefully handling the disable state.
3. For scheduled deletion: confirm deletion date and set a reminder to cancel if needed.
4. Check ActionTrail for the key mutation event.
5. Monitor CloudMonitor for any application-level errors indicating unexpected CMK access failures.

## Response Shape

1. Key ID and region confirmed
2. Key status (enabled/disabled/pending-deletion)
3. CMK dependency audit (OSS, ECS, RDS, PolarDB using this key)
4. Disable vs. delete assessment
5. Scheduled deletion window
6. Approval status
7. Post-action dependency verification
