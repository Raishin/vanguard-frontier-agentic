# Official Sources — Alibaba Cloud Live KMS Key Mutation Guard

Authoritative Alibaba Cloud documentation for KMS key lifecycle and CMK dependency operations.

## Core References

- **KMS Overview** — https://www.alibabacloud.com/help/en/kms
  KMS architecture, CMK types (software vs. hardware), key states, and key lifecycle.

- **Schedule Key Deletion** — https://www.alibabacloud.com/help/en/kms/user-guide/schedule-key-deletion
  How to schedule CMK deletion, configure the pending deletion window (7–30 days), and cancel a scheduled deletion.

- **Disable and Enable a CMK** — https://www.alibabacloud.com/help/en/kms/user-guide/disable-or-enable-a-cmk
  Disabling a key without deleting it — reversible operation that prevents new encryption/decryption.

- **Key Rotation** — https://www.alibabacloud.com/help/en/kms/user-guide/automatic-key-rotation
  Automatic and manual key rotation, key version management, and how rotation affects existing encrypted data.

- **OSS Server-Side Encryption** — https://www.alibabacloud.com/help/en/oss/user-guide/server-side-encryption
  How OSS uses KMS CMKs for SSE-KMS, and the dependency between OSS objects and KMS key availability.

- **ECS Disk Encryption** — https://www.alibabacloud.com/help/en/ecs/user-guide/encryption-overview
  ECS disk encryption using KMS CMKs, and the impact of key deletion on encrypted disk access.

- **RDS TDE** — https://www.alibabacloud.com/help/en/rds/user-guide/configure-tde-for-an-arn-instance
  Transparent Data Encryption for RDS using KMS CMKs.

- **PolarDB TDE** — https://www.alibabacloud.com/help/en/polardb/user-guide/tde
  Transparent Data Encryption for PolarDB clusters using KMS CMKs.

- **KMS ActionTrail Events** — https://www.alibabacloud.com/help/en/actiontrail
  Audit logging for KMS key mutations; querying ScheduleKeyDeletion, DisableKey, and EnableKey events.
