# IAM Permissions — GCP Live Cloud Run Traffic Migration Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/run.viewer` | View Cloud Run service configurations, revision details, traffic splits, and IAM policies without mutation rights |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/run.developer` | Update traffic splits, modify min-instances settings, delete revisions, and deploy new revisions | Bind at the project level; consider binding at the service level for tighter scope if supported |

## Narrowing Guidance

- Bind `roles/run.developer` at the **project** level scoped to the specific service region where possible.
- For traffic split changes only (no new deployments), `roles/run.developer` is the minimum required role — there is no narrower "traffic-only" predefined role.
- For services that accept traffic from unauthenticated users, also review `roles/run.invoker` bindings — do not add invoker access as part of a traffic migration without explicit authorization.
- Use Cloud Run IAM conditions to restrict `run.developer` access to specific services by resource name where the cloud provider supports it.
- Consider requiring a two-person approval process for 100% traffic migrations in production — split traffic to 10% first, observe, then escalate to 100%.

## Anti-Patterns — Never Grant

- `roles/owner` — grants full resource and billing control; never appropriate for a Cloud Run guard agent.
- `roles/editor` — overly broad; grants write access to nearly all GCP services.
- `roles/run.admin` to CI/CD service accounts without traffic migration scope restriction.
- Deleting the only known-good revision before confirming the new revision is healthy.
- Granting `roles/run.invoker` to `allUsers` without explicit review of public access implications during a traffic migration.

## Audit Trail

All Cloud Run service mutations (traffic updates, revision deployments, revision deletions) are logged in **Cloud Audit Logs** under `run.googleapis.com`. Ensure `ADMIN_WRITE` and `DATA_WRITE` audit log types are enabled for the Cloud Run service in the target project. Cloud Run also emits request logs to Cloud Logging — configure log-based metrics for error rate and latency to support post-migration health checks.
