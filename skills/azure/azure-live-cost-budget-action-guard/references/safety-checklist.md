# Safety Checklist

## Evidence labels

- `documentation-based`: grounded in Microsoft Learn or listed official documentation.
- `sampled-current-state`: grounded in read-only Azure or Kubernetes observations from the user's configured tools.
- `user-provided`: grounded in sanitized snippets supplied by the user.
- `inference`: reasoned from evidence but not directly proven.

## Mutation boundary

- Default to read-only review.
- Do not perform create, update, delete, rotate, purge, recover, apply, swap, reset, complete, deploy, assign, revoke, deallocate, quota, budget, or policy changes unless the user explicitly asks and approval is clear.
- Prefer preview, what-if, dry-run, status, describe, list, show, diff, activity-log, and policy evaluation evidence before any mutation.

## Credential and data boundary

- Never ask users to paste credentials, tokens, tenant IDs, subscription IDs, customer data, private keys, kubeconfig contents, CA requester credentials, secret values, connection strings, or raw environment dumps.
- Summarize sensitive evidence by field presence, control state, and risk; do not reproduce secret material.

## Risk gates

- Stop on ambiguous target, ambiguous principal, missing approval, missing rollback, missing financial owner, or missing asset owner for high-impact assets.
- Treat broad permissions, permanent privileged access, public exposure, purge authority, destructive deployment behavior, quota increases, budget automation, and production slot swaps as high-risk.
- Separate documented product behavior from sampled configured-environment evidence.

## Asset-specific hard line

Never approve quota increases, budget threshold raises, automated cost actions, or high-cost SKU provisioning without explicit financial owner approval, cost data latency caveat, rollback or stop action, and scope confirmation.
