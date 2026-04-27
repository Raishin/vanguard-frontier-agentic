# Quality Bar

## Minimum bar

Every asset must be:

- cloud-relevant,
- metadata-complete,
- provenance-clear,
- security-reviewed,
- compatible with at least one declared harness,
- linked to official documentation when making provider claims.

## Rejection triggers

Reject assets that:

- are generic prompts with cloud labels pasted on,
- make unverified claims,
- encourage broad admin permissions without justification,
- mutate infrastructure without approval gates,
- include secrets or customer-specific data,
- duplicate an existing asset without meaningful improvement.

## Security-sensitive assets

IAM, RBAC, Terraform, Kubernetes, database, incident response, and MCP assets need explicit threat notes. If the workflow could affect production, require human approval and rollback expectations.
