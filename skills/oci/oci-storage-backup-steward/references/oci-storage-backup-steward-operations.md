# OCI Storage Backup Steward Operations Reference

## What people get wrong

- Backup configured means recovery works.
- A lifecycle delete rule is harmless because the bucket still exists.
- File-system snapshots are a disaster-recovery plan by themselves.
- Volume backups prove application consistency without quiesce, crash-consistency, or database coordination evidence.
- Bucket summary listings prove versioning, retention, replication, encryption, or object-level risk.

## Officially grounded service shape

- Object Storage bucket listing is compartment-scoped and returns summary fields; user-defined bucket metadata requires follow-up reads.
- Object lifecycle policies can archive, tier, abort multipart uploads, or delete supported resources; deletion rules need explicit retention approval.
- Block Volume backup listing is compartment-scoped and can filter by volume, source backup, display name, lifecycle state, sorting, and pagination.
- File Storage file-system listing requires compartment and availability-domain scope and can filter by lifecycle state, source snapshot, parent file system, and snapshot policy.
- Volume backups can restore within the stored region; region, availability domain, replication, and restore-drill evidence still decide recovery readiness.

## Non-negotiable design rules

- Separate inventory evidence from restore-readiness evidence.
- Require explicit approval before delete, retention reduction, lifecycle-rule enablement, backup deletion, replication cutover, or broad IAM changes.
- For every recovery claim, capture scope, timestamp, region, compartment, resource type, retention target, and the tested restore path.
- Treat pre-authenticated requests, public buckets, permissive export options, and wildcard storage policies as high-risk until proven intentional.
- Never request or store namespace, tenancy, compartment, volume, bucket, file-system, support, or customer identifiers in committed docs.

## Minimal safe implementation flow

- Classify storage surface and data sensitivity.
- Collect official documentation evidence for service behavior and limits.
- Use OCI API evidence through the user’s configured read-only OCI MCP only for command shape or sanitized current-state observations.
- Compare retention, lifecycle, backup, replication, IAM, and restore-drill evidence against stated RTO/RPO.
- Return blockers before suggesting mutations.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The user wants to delete backups without owner, retention, and restore evidence.
- The user treats lifecycle rules as a cost-only optimization.
- The request asks for credentials, config contents, private keys, OCIDs, namespaces, or customer data.
