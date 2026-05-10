# Official sources

Use this reference only when grounding Contabo service behavior or verifying the source list for routing decisions.

## Contabo documentation

Use these as starting points, not as proof of the user's live Contabo account state:

- https://api.contabo.com/ — Contabo OpenAPI reference (all endpoints)
- https://docs.contabo.com/ — Contabo user documentation and guides
- https://github.com/contabo/cntb — cntb CLI tool (source, releases, usage)
- https://api.contabo.com/#tag/Instances — Instance operations API
- https://api.contabo.com/#tag/Object-Storages — Object Storage API

## Grounding rule

Official Contabo documentation describes platform behavior, available products, and API contracts. It does not prove the user's current instance inventory, contract state, billing posture, region availability at the time of the request, or active OAuth2 token validity. Prefer user-provided sanitized evidence or live API responses for current-state claims. Label any claim sourced only from documentation as `documentation-based`.
