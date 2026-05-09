---
name: gcp-live-iam-policy-change-guard
description: Gate IAM binding mutations, org policy changes, and Service Account key creation against the GCP resource hierarchy. IAM bindings at org level propagate to all folders and projects — this guard enforces blast-radius assessment, audit-trail confirmation, and explicit authority approval before any policy mutation is executed.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# GCP Live IAM Policy Change Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-iam-policy-change-guard work. Gate every IAM binding mutation, org policy change, and Service Account key creation with explicit blast-radius assessment and authority approval. Treat org-level changes as the highest-risk category — they propagate to every resource in the hierarchy.

## When to Use

Use this skill when:

- An IAM binding must be added, modified, or removed at org, folder, or project level
- An Organization Policy constraint is being enabled, disabled, or overridden
- A new Service Account key is being requested or created
- A custom role is being created or modified
- An operator needs to audit the current IAM policy for a resource before making changes
- Detecting and remediating over-privileged bindings or stale service account keys

## When NOT to Use

Do not use this skill when:

- The task is a read-only IAM audit with no mutation intent (use `roles/iam.securityReviewer` alone)
- The task involves Kubernetes RBAC within GKE only (no GCP IAM changes)
- The task is creating a new GCP project with default IAM (no pre-existing bindings at risk)
- The task is unrelated to GCP identity and access management

## Pre-Flight Checklist

Before executing any IAM mutation, verify all of the following:

1. **Resource hierarchy level confirmed** — explicitly state whether the target is organization, folder, or project level. Run `gcloud organizations list`, `gcloud resource-manager folders list`, or `gcloud projects describe <PROJECT>` as appropriate.
2. **Active principal confirmed** — run `gcloud auth list` and `gcloud config get-value account` to confirm the identity executing the change.
3. **Current IAM policy captured** — run `gcloud [projects|resource-manager folders|organizations] get-iam-policy <TARGET>` and document the current bindings before any change.
4. **Blast-radius assessed** — for org-level changes, enumerate all child folders and projects that will inherit the binding.
5. **Change justification documented** — the operator must state the business reason, the specific principal(s) affected, and the role being added or removed.
6. **Service Account key inventory** — if a key is being created, run `gcloud iam service-accounts keys list --iam-account <SA>` and confirm no unexpired keys already exist for this purpose.
7. **Rollback plan documented** — identify which existing binding will be restored if the change must be reverted. For removals, confirm the operator has a path to restore access if needed.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the target resource is `<ORG_ID / FOLDER_ID / PROJECT_ID>` at the `<org/folder/project>` level."
- "I confirm the principal is `<PRINCIPAL_EMAIL>` and the role change is `<ADD/REMOVE> <ROLE>`."
- "I understand that org-level bindings propagate to all child resources and I have authority to make this change."
- "I approve this IAM change."
- For Service Account key creation: "I acknowledge this creates a long-lived credential and confirm Workload Identity is not available for this use case."

## Execution Steps

1. Capture pre-change IAM policy snapshot.
2. Confirm active principal has `roles/resourcemanager.organizationAdmin` (for org changes) or `roles/resourcemanager.projectIamAdmin` (for project changes).
3. Present the planned change, blast-radius assessment, and key inventory to the operator for explicit approval.
4. Execute the mutation:
   - Add binding: `gcloud [projects|resource-manager folders|organizations] add-iam-policy-binding <TARGET> --member=<MEMBER> --role=<ROLE>`
   - Remove binding: `gcloud [projects|resource-manager folders|organizations] remove-iam-policy-binding <TARGET> --member=<MEMBER> --role=<ROLE>`
   - Create SA key: `gcloud iam service-accounts keys create <KEY_FILE> --iam-account=<SA>`
   - Set org policy: `gcloud resource-manager org-policies set-policy <POLICY_FILE> --organization=<ORG_ID>`
5. Capture post-change policy snapshot and diff against pre-change snapshot.

## Rollback Procedure

- **Binding addition** (reversible): Remove the binding with `remove-iam-policy-binding`. Take effect is immediate.
- **Binding removal** (reversible but risky): Re-add the binding with `add-iam-policy-binding`. If the removal caused a lockout, use a break-glass identity to restore.
- **Service Account key creation** (NOT reversible on creation — revoke is the mitigation): Disable and delete the key immediately with `gcloud iam service-accounts keys disable` and `gcloud iam service-accounts keys delete`.
- **Org policy change** (reversible): Delete the policy override to restore inherited behavior: `gcloud resource-manager org-policies delete <CONSTRAINT> --organization=<ORG_ID>`.

## Post-Change Verification

1. Run `gcloud [projects|resource-manager folders|organizations] get-iam-policy <TARGET>` — confirm the policy reflects the intended change and no unintended bindings were added.
2. Test access for the affected principal using `gcloud auth application-default login` or `gcloud projects test-iam-permissions` to verify the change has the expected effect.
3. Check Cloud Audit Logs for the change: `gcloud logging read 'resource.type="project" AND protoPayload.methodName="SetIamPolicy"' --limit=10 --project=<PROJECT>`.
4. For org policy changes, confirm constraint enforcement is active using `gcloud resource-manager org-policies describe <CONSTRAINT> --organization=<ORG_ID>`.

## Response Shape

1. Resource hierarchy level and target confirmed
2. Current IAM policy inventory
3. Proposed binding change and blast-radius assessment
4. Service Account key inventory if applicable
5. Approval status
6. Applied IAM change
7. Post-change access verification
