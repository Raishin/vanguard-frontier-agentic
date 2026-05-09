# IAM Permissions — GCP Live IAM Policy Change Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/iam.securityReviewer` | Read IAM policies, service account details, and org policy state without mutation rights |
| `roles/resourcemanager.folderViewer` | Read folder IAM policies for blast-radius scoping |
| `roles/resourcemanager.projectViewer` | Read project IAM policies and resource metadata |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/resourcemanager.organizationAdmin` | Set and modify IAM policies at the organization level; manage org policies | Bind only to a break-glass service account; require MFA and approval gate before use |
| `roles/iam.serviceAccountAdmin` | Create, delete, and manage Service Accounts and their keys | Bind at project level; never at org or folder level without explicit justification |

## Narrowing Guidance

- Bind `roles/resourcemanager.organizationAdmin` only to a dedicated break-glass identity — never to developer or CI/CD service accounts.
- For project-scoped IAM changes, use `roles/resourcemanager.projectIamAdmin` instead of `organizationAdmin`.
- Prefer Workload Identity Federation over Service Account key creation to avoid long-lived credential exposure.
- Use `gcloud projects add-iam-policy-binding` with `--condition` flags to scope bindings to specific resources or time windows where possible.
- Enforce `constraints/iam.disableServiceAccountKeyCreation` org policy to block SA key creation by default.

## Anti-Patterns — Never Grant

- `roles/owner` — grants billing, IAM, and full resource control; never appropriate for an IAM guard agent.
- `roles/editor` — overly broad write access; bypasses the intent of least-privilege IAM management.
- `roles/resourcemanager.organizationAdmin` to CI/CD service accounts — blast radius is the entire organization.
- Service Account keys stored in VCS, environment variables, or container images — use Workload Identity instead.
- Granting `roles/iam.serviceAccountTokenCreator` on high-privilege service accounts without audit logging.

## Audit Trail

All IAM binding changes are logged in **Cloud Audit Logs** under `cloudresourcemanager.googleapis.com` and `iam.googleapis.com`. Ensure `DATA_WRITE` and `ADMIN_READ` log types are enabled at the organization level. Consider exporting audit logs to BigQuery or Cloud Storage for retention beyond the default 400-day window.
