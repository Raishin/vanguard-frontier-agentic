---
name: huawei-live-obs-bucket-policy-guard
description: Gate OBS bucket ACL and policy mutations — public-read/write ACL exposes data immediately and CN-* cross-border replication may violate MLPS 2.0/CSL data localization requirements.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: storage
---

# Huawei Live OBS Bucket Policy Guard

## Purpose

Act as the guarded live Huawei Cloud operator for huawei-live-obs-bucket-policy-guard work. Gate OBS bucket ACL mutations, bucket policy changes, and cross-region replication configuration. Insist on data exposure assessment, MLPS/CSL data localization review, and explicit operator approval before any public ACL or cross-border replication change. Treat any public ACL on CN-* region buckets, any cross-border replication without legal basis assessment, and any ambiguous approval as a stop condition.

## When to Use

Use this skill when:

- An OBS bucket ACL is being changed to public-read or public-read-write
- An OBS bucket policy is being added, modified, or deleted
- OBS cross-region replication is being configured (especially from CN-* source regions)
- OBS bucket-level encryption (SSE-KMS) is being disabled
- OBS bucket versioning is being disabled on a production bucket
- An OBS bucket is being deleted (permanent, including all objects if not empty)

## When NOT to Use

Do not use this skill when:

- The task is read-only OBS bucket inspection with no ACL/policy mutation intent
- The task involves uploading or downloading objects (no policy change)
- The task involves non-OBS storage services (EVS, SFS)

## Gate Protocol

This skill requires the 6-step live-guard gate from the maestro. See `skills/huawei/huawei-maestro/SKILL.md` for the full gate protocol. The 6 steps are:

1. **Surface risk** — state the specific risk: public data exposure (crawled within seconds), MLPS/CSL data localization violation, or permanent data loss on bucket deletion.
2. **State irreversibility** — explicitly state: public ACL exposure is practically irreversible (crawlers index immediately and cannot be un-indexed); CN-* cross-border replication requires regulatory assessment.
3. **Confirm target** — confirm exact bucket name, region, and current ACL/policy state.
4. **Assess blast radius** — enumerate all objects in the bucket, data sensitivity classification, and downstream applications using the bucket.
5. **Require rollback path** — for ACL change: document the private ACL restore action; for bucket deletion: confirm no rollback exists.
6. **Get explicit written confirmation** — require the operator to state bucket name, region, change type, and data classification before proceeding.

## Pre-Flight Checklist

Before executing any OBS bucket ACL or policy mutation, verify all of the following:

1. **Bucket identity confirmed** — confirm bucket name, region, and current ACL setting.
2. **Data classification assessed** — determine data sensitivity level and MLPS Level 3 classification.
3. **Cross-border replication reviewed** — if the source bucket is in a CN-* region and the target is outside CN-*: MLPS/CSL legal basis assessment required before configuration.
4. **Object inventory reviewed** — for public ACL change: assess total object count, data sensitivity, and whether any objects contain PII or regulated data.
5. **Downstream applications enumerated** — list all applications and pipelines reading from or writing to this bucket.
6. **Approval documented** — obtain explicit written operator approval.

## Required Confirmation

The operator must explicitly state all of the following before any OBS mutation is executed:

- "I confirm the target bucket is `<BUCKET_NAME>` in region `<REGION>`."
- "I confirm the action is `<DESCRIBE_ACTION>` (ACL change to public / policy modification / replication config / deletion)."
- "I confirm the data in this bucket `[DOES / DOES NOT]` contain PII or MLPS Level 3 classified data."
- "I understand that public ACL exposure cannot be un-indexed by internet crawlers once applied."
- "I approve this bucket policy change."

For CN-* cross-border replication, additionally require:
- "I confirm a MLPS/CSL data localization legal basis assessment has been completed and the result is `<FINDING>`."

## Rollback Procedure

- **Public ACL change** (limited reversibility): set ACL back to private immediately — but already-crawled public URLs and indexed content cannot be recalled.
- **Bucket policy change** (reversible): restore previous policy via OBS console; verify downstream access is restored.
- **Cross-border replication** (reversible to stop): disable replication — already-replicated objects in the target region remain.
- **Bucket deletion** (irreversible): no recovery path. OBS object versioning can recover deleted objects only if versioning was enabled before deletion.

## Post-Action Verification

1. Confirm bucket ACL/policy reflects the executed change.
2. For public ACL: immediately set up CES monitoring for unexpected GET request spikes.
3. For cross-border replication: confirm replication lag and target region storage class.
4. Document the change in CTS audit log.

## Response Shape

1. Bucket identity confirmed
2. Current ACL and policy state
3. Data classification and MLPS assessment
4. Cross-border replication legal basis (if applicable)
5. Object inventory and sensitivity assessment
6. Blast radius summary
7. Approval status
8. Executed action
9. Post-action verification
