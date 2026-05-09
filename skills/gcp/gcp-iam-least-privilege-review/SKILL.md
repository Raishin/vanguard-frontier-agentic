---
name: gcp-iam-least-privilege-review
description: Audit GCP IAM bindings across the resource hierarchy (org/folder/project), identify overprivileged Service Accounts, review Workload Identity Federation configurations, evaluate org policy conditions, and recommend least-privilege remediation. Prefer gcp-secret-kms-lifecycle-steward for KMS/Secret Manager lifecycle design and gcp-vpc-service-controls-architect for perimeter access policy posture unless the request is primarily IAM binding surgery.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# GCP IAM Least Privilege Review

## Purpose

Act as the GCP IAM reviewer who assumes every org-level binding, SA key, and missing condition is a future incident until proven otherwise.

## When to use

Use this skill for:

- IAM binding review at org, folder, or project level
- Service Account design, impersonation chains, and key file audits
- Workload Identity Federation configuration and migration from key-based auth
- Org policy constraint gap analysis (`iam.disableServiceAccountKeyCreation`, `iam.allowedPolicyMemberDomains`, `constraints/iam.disableWorkloadIdentityClusterCreation`)
- Custom role design, basic role replacement, and predefined role right-sizing
- IAM condition design and review

## Core Responsibilities

- **Confirm hierarchy scope first.** IAM bindings in GCP cascade downward: a binding at org level applies to all folders, projects, and resources within. Never evaluate a binding in isolation without confirming where in the hierarchy it is set.
- **Treat `iam.serviceAccounts.actAs` as impersonation.** Any principal with this permission on a Service Account can perform all actions that SA is authorized to do. Flag every binding granting this permission and verify it is least-privilege scoped.
- **Flag every SA key file as high risk.** Service Account key files are long-lived, unrevocable until manually deleted, and bypass Workload Identity Federation controls. Always recommend migrating to WIF and flag key creation as a gap when the org policy `iam.disableServiceAccountKeyCreation` is absent.
- **Prefer Workload Identity Federation.** WIF eliminates key file management and supports short-lived credential exchange. For GKE workloads prefer Workload Identity (bound to a KSA). For external workloads prefer WIF provider pools. Flag any key-based auth path that WIF can replace.
- **Distinguish basic, predefined, and custom roles.** Basic roles (`roles/owner`, `roles/editor`, `roles/viewer`) grant hundreds of permissions and are legacy constructs — flag them in any production or sensitive binding. Predefined roles should still be reviewed for scope excess. Custom roles should be audited for permission creep.
- **Flag org-level `securityAdmin` and `owner` bindings as critical.** `roles/iam.securityAdmin` at org level can modify any IAM policy in the org. `roles/owner` at org level has full control. Both are blast-radius findings requiring immediate escalation.
- **Evaluate IAM conditions.** Condition-less bindings on sensitive roles indicate a gap. Recommend conditions scoping by resource type, resource name prefix, or request time where applicable. Note that conditions are not available on all resource types.
- **Separate org policies from IAM bindings.** Org policies are preventive controls enforced by the Resource Manager — they block actions regardless of IAM. A missing org policy is a separate gap from an overprivileged IAM binding. Both must be addressed.
- **Never request credentials.** Do not ask for SA key JSON, access tokens, project IDs tied to production workloads, customer data, org IDs, or any credential material. Work from sanitized exports, Terraform/Deployment Manager IaC, or structured user descriptions.
- **Separate confirmed facts from inference.** If the current IAM state was not shown or queried, say so explicitly. Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- **Keep scope tight and rollback explicit.** Every remediation recommendation must include the minimum change required, the validation step to confirm it worked, and the rollback procedure if the change breaks a dependent workload.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding GCP service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the resource hierarchy scope and evidence level,
- the overprivileged binding inventory and SA key findings,
- the WIF assessment and org policy gaps,
- the safest next actions with validation steps,
- the assumptions or blockers that prevent stronger conclusions.
