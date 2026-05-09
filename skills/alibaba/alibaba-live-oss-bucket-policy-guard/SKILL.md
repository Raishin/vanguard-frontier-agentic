---
name: alibaba-live-oss-bucket-policy-guard
description: Gate OSS bucket ACL and policy mutations — public-read/write ACL exposes data to internet crawlers within seconds; CN-* cross-border replication requires DSL Article 31 assessment.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: storage
---

# Alibaba Cloud Live OSS Bucket Policy Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-oss-bucket-policy-guard work. Gate every OSS bucket ACL and policy mutation with a full impact assessment and explicit operator approval. Treat public-read/write ACL changes as immediate, practically irreversible data exposure events.

## When to Use

Use this skill when:

- An OSS bucket ACL is being changed (private → public-read, public-read-write, or any permissive setting)
- An OSS bucket policy is being created, modified, or deleted
- Cross-region replication rules are being configured or modified for CN-* buckets
- Object ownership settings or CORS policies are being changed on production buckets
- A bucket lifecycle policy is being modified in ways that affect object access
- An operator needs to audit current bucket ACL and policy before a mutation

## When NOT to Use

Do not use this skill when:

- The task is a read-only OSS bucket audit with no mutation intent
- The task involves object-level operations (upload, download, delete objects) rather than bucket-level policy changes
- The task involves only OSS lifecycle policies that do not affect access control

## Key Risk Facts

- **OSS ACL `public-read-write`** exposes all objects immediately to any internet user. Internet crawlers index publicly exposed OSS buckets within seconds to minutes. Reversing the ACL back to private cannot un-index data that was already crawled. This exposure is practically irreversible in its data-leak consequences.
- **OSS ACL `public-read`** makes all objects readable by the internet. Depending on object sensitivity, this may be acceptable for CDN use cases or catastrophic for PII/business data.
- **Cross-border replication from CN-* to international regions** requires a completed CAC Data Security Law (DSL) Article 31 security assessment. Initiating replication without a completed assessment violates Chinese law.
- **Bucket policy deletion** removes all fine-grained access controls, potentially expanding access to all authenticated Alibaba Cloud users depending on ACL settings.
- **All mutations** require the 6-step live-guard gate.

## Pre-Flight Checklist

Before executing any OSS bucket ACL or policy mutation, verify all of the following:

1. **Bucket identity confirmed** — confirm the exact bucket name, region, and owner account. Run `aliyun oss stat oss://<BUCKET>` or use the OSS Console to confirm bucket metadata.
2. **Current ACL and policy captured** — document the current bucket ACL and bucket policy before any change. This is the rollback baseline.
3. **Object sensitivity assessed** — estimate the classification and sensitivity of objects stored in this bucket. Public access to PII, credentials, financial records, or internal business data is a critical incident.
4. **Cross-border replication check** — if the bucket is in a CN-* region and replication targets an international region, DSL Article 31 assessment must be completed first.
5. **Blast radius assessed** — how many objects are in scope? what services or users depend on the current access model? what applications will break if access changes?
6. **Rollback plan confirmed** — document the exact prior ACL and policy for immediate restoration if needed.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the bucket is `<BUCKET_NAME>` in region `<REGION>` in account `<ACCOUNT_ID>`."
- "I have reviewed the current ACL (`<CURRENT_ACL>`) and policy and the proposed change is `<SPECIFIC_CHANGE>`."
- "I have assessed the data sensitivity of objects in this bucket: `<ASSESSMENT>`."
- "I understand the blast radius of this change: `<DESCRIPTION>`."
- For public-read or public-read-write ACL: "I understand that internet crawlers may index exposed data within seconds and that this exposure cannot be reversed for already-crawled data. I accept this risk."
- For CN-* cross-border replication: "I confirm a completed DSL Article 31 assessment is on file for this data transfer."
- "I approve this OSS bucket ACL/policy change."

## Execution Steps

1. Capture pre-change bucket state: `aliyun oss stat oss://<BUCKET>` and policy output.
2. Present the planned change, current ACL/policy, and blast radius to the operator for explicit approval.
3. Execute the mutation:
   - Set ACL: `aliyun oss set-acl oss://<BUCKET> <ACL>` or via OSS Console.
   - Set bucket policy: via OSS Console > Bucket > Permissions > Bucket Policy, or Alibaba Cloud OSS API.
   - Configure replication: via OSS Console > Bucket > Data Replication, with DSL assessment confirmed.
4. Confirm the new ACL and policy are in effect.

## Rollback Procedure

- **ACL change** (reversible): Restore the previous ACL immediately — `aliyun oss set-acl oss://<BUCKET> private`. Note: this stops new exposure but cannot undo data already crawled or accessed.
- **Bucket policy change** (reversible): Restore the prior policy document via OSS Console or API.
- **Cross-border replication** (reversible): Disable the replication rule via OSS Console; note that objects already replicated to the destination remain there.

## Post-Change Verification

1. Confirm new ACL is in effect: `aliyun oss stat oss://<BUCKET>`.
2. Confirm bucket policy reflects the intended rules.
3. Test access from an unauthorized principal to verify the access model matches intent.
4. Check ActionTrail for the bucket mutation event.
5. For public exposure: monitor OSS access logs for unexpected crawler traffic.

## Response Shape

1. Bucket name, region, and account confirmed
2. Current ACL and policy captured
3. Data sensitivity and blast radius assessment
4. Cross-border replication DSL assessment status (if applicable)
5. Operator confirmation received
6. Execution confirmation
7. Post-change verification results
