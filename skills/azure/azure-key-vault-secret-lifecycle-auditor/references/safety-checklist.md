# Safety Checklist

## Evidence labels

- `documentation-based`: grounded in Microsoft Learn or official Kubernetes documentation where listed.
- `sampled-current-state`: grounded in read-only Azure or Kubernetes observations from the user's configured tools.
- `user-provided`: grounded in sanitized snippets supplied by the user.
- `inference`: reasoned from evidence but not directly proven.

## Mutation boundary

- Default to read-only review.
- Do not perform create, update, delete, rotate, purge, recover, apply, restart, drain, cordon, scale, rollout, role-assignment, policy-assignment, or network changes unless the user explicitly asks and approval is clear.
- Prefer preview, dry-run, status, describe, what-if, list, show, and policy evaluation evidence before any mutation.

## Credential and data boundary

- Never ask users to paste credentials, tokens, tenant IDs, subscription IDs, customer data, private keys, kubeconfig contents, CA requester credentials, secret values, or connection strings.
- Summarize sensitive evidence by field presence, control state, and risk; do not reproduce secret material.

## Risk gates

- Stop on ambiguous target, ambiguous principal, missing approval, missing rollback, or missing owner for high-impact assets.
- Treat broad permissions, permanent privileged access, public exposure, purge authority, destructive operations, and live rollout changes as high-risk.
- Separate documented product behavior from sampled configured-environment evidence.

## Asset-specific hard line

Avoid retrieving secret values. Treat purge authority, missing soft delete, missing purge protection, legacy access policies for critical workloads, and untested rotation or recovery paths as high-risk.
