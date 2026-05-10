# Official sources

Use this reference only when grounding Contabo instance lifecycle API behavior, product specifications, or cntb CLI usage for create, reinstall, or cancel operations.

## Contabo documentation

Use these as starting points, not as proof of the user's live instance state or current product availability:

- https://api.contabo.com/#tag/Instances — Instance operations API (create, reinstall, cancel, list, get — includes product IDs, region codes, image IDs, Cloud-Init userData, SSH secret ID parameters)
- https://api.contabo.com/ — Contabo OpenAPI reference (authentication flows, request headers including x-request-id, error response schemas)
- https://docs.contabo.com/ — Contabo user documentation (instance management guides, Cloud-Init setup, contract period terms, cancellation policy)
- https://github.com/contabo/cntb — cntb CLI tool (instance create, reinstall, cancel commands; credential configuration; token refresh patterns)

## Grounding rule

Official Contabo documentation describes API endpoint behavior, available product IDs, supported image IDs, and contract terms at the time of publication. It does not prove the user's current instance inventory, active contract state, remaining period, available product IDs in a specific region at the time of the request, or OAuth2 token validity. Prefer live read-only API responses or user-provided sanitized evidence for current-state claims. Label any product availability, image ID, or contract term claim sourced only from documentation as `documentation-based`. Always refresh the OAuth2 token immediately before the mutation call — do not rely on a token that was retrieved earlier in the session.
