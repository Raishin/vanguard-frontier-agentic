---
name: gcp-live-kms-key-destruction-guard
description: Gate Cloud KMS key version destruction and key ring deletion against a complete CMEK dependency audit. All Cloud SQL, GCS, BigQuery, Compute Engine disk, and Secret Manager resources encrypted by the key version become permanently inaccessible once destruction completes — this guard ensures no key version is destroyed without enumerating every dependent resource and obtaining explicit operator approval.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# GCP Live KMS Key Destruction Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-kms-key-destruction-guard work. Gate every Cloud KMS key version destruction and key ring deletion with a complete CMEK dependency audit and explicit approval. Treat key version destruction as the most irreversible action in the GCP key lifecycle — no recovery is possible after the pending period expires.

## When to Use

Use this skill when:

- A Cloud KMS key version destruction is being requested or needs to be scheduled
- An operator wants to cancel a pending key version destruction before it completes
- A key ring deletion is being considered
- A CMEK dependency audit is needed before any key lifecycle change
- Evaluating whether key rotation (safe) vs. key version destruction (irreversible) is the appropriate action
- An operator needs to confirm which resources are encrypted by a specific key version

## When NOT to Use

Do not use this skill when:

- The task is key rotation only with no destruction intent (rotation is non-destructive and does not require this guard)
- The task is creating a new key ring or key (no existing data at risk)
- The task is purely reading or listing keys with no mutation intent
- The task involves AWS KMS, Azure Key Vault, or HashiCorp Vault

## Pre-Flight Checklist

Before scheduling any key version destruction, verify all of the following:

1. **Project and key ring identity confirmed** — run `gcloud kms keyrings list --location=<LOCATION> --project=<PROJECT>` and confirm the key ring name and location match the intended target.
2. **Key version status confirmed** — run `gcloud kms keys versions list --key=<KEY> --keyring=<KEYRING> --location=<LOCATION> --project=<PROJECT>` and confirm the version state (ENABLED, DISABLED, or already DESTROY_SCHEDULED).
3. **CMEK dependency audit complete** — enumerate all resources using this key version: Cloud SQL, GCS buckets, BigQuery datasets, Compute Engine disks, Secret Manager secrets. No destruction may proceed if active resources are identified.
4. **Rotation vs. destruction decision confirmed** — explicitly state whether key rotation is sufficient (for compliance rotation requirements) or destruction is genuinely required (for decommissioning or legal hold release).
5. **Pending period acknowledged** — confirm the minimum 24-hour pending destruction period; note that the key can be restored during this window.
6. **Backup and export confirmed** — for any data encrypted by this key version, confirm that the data has been migrated to a new key version or exported before destruction is scheduled.
7. **Audit log retention confirmed** — confirm that Cloud Audit Logs for this key lifecycle event will be retained for the required compliance period.

## Required Confirmation

The operator must explicitly state all of the following before any destruction is scheduled:

- "I confirm the target is key version `<VERSION_NUMBER>` of key `<KEY_NAME>` in key ring `<KEYRING_NAME>`, location `<LOCATION>`, project `<PROJECT_ID>`."
- "I have completed the CMEK dependency audit and confirmed no active resources depend on this key version."
- "I understand that key version destruction is permanent after the pending period and no recovery is possible."
- "I approve scheduling destruction of this key version."
- For key ring deletion: "I confirm all keys in this key ring have no enabled versions and I approve deletion."

## Execution Steps

1. Capture key version state and all CMEK dependencies.
2. Confirm active principal has `roles/cloudkms.admin`.
3. Present the planned destruction, CMEK dependency findings, and pending period to the operator for explicit approval.
4. Execute the mutation:
   - Schedule destruction: `gcloud kms keys versions destroy <VERSION> --key=<KEY> --keyring=<KEYRING> --location=<LOCATION> --project=<PROJECT>`
   - Cancel pending destruction: `gcloud kms keys versions restore <VERSION> --key=<KEY> --keyring=<KEYRING> --location=<LOCATION> --project=<PROJECT>`
   - Delete key ring (only if all keys have no enabled versions): `gcloud kms keyrings delete <KEYRING> --location=<LOCATION> --project=<PROJECT>`
5. Confirm the new key version state in Cloud Console or via CLI.

## Rollback Procedure

- **During pending period** (reversible): Cancel destruction with `gcloud kms keys versions restore`. This must happen before the scheduled destruction date.
- **After destruction completes** (NOT reversible): Key version destruction is permanent. No recovery path exists. CMEK-encrypted resources become permanently inaccessible.
- If destruction was accidental, immediately open a GCP Support P1 case — Google cannot recover destroyed key material but can assist with understanding the impact scope.
- For data recovery, assess whether plaintext backups or alternative encryption paths exist.

## Post-Change Verification

1. Run `gcloud kms keys versions describe <VERSION> --key=<KEY> --keyring=<KEYRING> --location=<LOCATION>` — confirm the state is `DESTROY_SCHEDULED` with the expected destruction date, or `DESTROYED` if destruction completed.
2. Check Cloud Audit Logs for the destruction event: `gcloud logging read 'protoPayload.methodName="DestroyCryptoKeyVersion"' --limit=5 --project=<PROJECT>`.
3. If the destruction was cancelled, confirm the version state is back to `DISABLED` or `ENABLED`.
4. Monitor any CMEK-dependent services for access errors in the 24 hours following the scheduled destruction date.

## Response Shape

1. Project and key ring identity confirmation
2. Key version status and scheduled destruction date if pending
3. CMEK dependency audit — all resources encrypted by this key version
4. Rotation vs. destruction assessment
5. Approval status
6. Executed destruction schedule or cancellation
7. Post-action monitoring
