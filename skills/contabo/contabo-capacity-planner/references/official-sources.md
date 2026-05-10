# Official sources

Use this reference only when grounding Contabo instance specifications, region availability, addon capabilities, or API behavior for capacity planning.

## Contabo documentation

Use these as starting points, not as proof of current product availability or the user's live account state:

- https://api.contabo.com/ — Contabo OpenAPI reference (instance creation, product IDs, region codes, addon parameters)
- https://docs.contabo.com/ — Contabo user documentation (instance types, region coverage, Cloud-Init guides, Private Networking setup)
- https://github.com/contabo/cntb — cntb CLI tool (instance listing, secret management, Cloud-Init deployment)
- https://api.contabo.com/#tag/Instances — Instance operations API (product selection, region, userData, SSH secret IDs, addons)

## Grounding rule

Official Contabo documentation describes available instance tiers, region codes, addon capabilities, and API contract behavior at the time of publication. It does not prove current product availability in a specific region, live pricing, the user's active instances, or whether a particular addon can be added after creation. Prefer user-provided sanitized evidence or live read-only API responses for current-state claims. Label any instance specification or region availability claim sourced only from documentation as `documentation-based`. Confirm region-specific product availability via the API or Contabo support before committing to a multi-region plan.
