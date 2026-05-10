# Official sources

Use this reference only when grounding Contabo Object Storage API behavior, S3 compatibility, bucket operations, or access policy configuration.

## Contabo documentation

Use these as starting points, not as proof of the user's live bucket state, object inventory, or access policy configuration:

- https://api.contabo.com/#tag/Object-Storages — Object Storage API (list instances, create/manage storage instances, region codes)
- https://api.contabo.com/ — Contabo OpenAPI reference (authentication flows, request headers including x-request-id, error response schemas)
- https://docs.contabo.com/ — Contabo user documentation (Object Storage setup, S3 compatibility guide, endpoint URLs by region, access key management)
- https://github.com/contabo/cntb — cntb CLI tool (Object Storage commands; credential and access key configuration)

## Grounding rule

Official Contabo documentation describes Object Storage API endpoints, S3-compatible endpoint URLs by region, access key management patterns, and available storage instance configurations at the time of publication. It does not prove the user's current bucket inventory, object count, active retention policies, ACL configuration, or the actual data at risk in a deletion operation. Prefer live read-only API responses and S3 list commands for current-state claims. Label any bucket configuration, endpoint URL, or storage behavior claim sourced only from documentation as `documentation-based`. Always confirm the correct S3 endpoint URL for the target region from official docs — do not assume the default AWS endpoint or a URL from a prior session.
