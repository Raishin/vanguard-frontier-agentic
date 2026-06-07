# OCI Registry Artifact Governor Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Deleting images without checking deployment consumers and rollback artifacts.
- Making repositories public for convenience.
- Treating mutable tags as immutable provenance.
- Assuming retention policy cleanup is safe without exemptions and promotion-state evidence.

## Officially grounded service shape

- Official OCI documentation describes Container Registry as an Oracle-managed registry for storing, sharing, and managing container images and OCI-compliant artifacts such as images, manifest lists, and Helm charts.
- Official OCI documentation says Container Registry can be private or public, where public repositories are pullable by users with internet access and the URL.
- Official OCI documentation says image retention policies can automatically delete images that match selection criteria, while the global default retains all images.
- OCI API evidence through the user’s configured read-only OCI MCP shows container repository and image listing are compartment-scoped, can optionally traverse subcompartments from root scope, and expose filters for public state, lifecycle state, repository, image digest/version, sorting, and pagination.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate repository, image digest, version tag, signature, vulnerability, retention, deployment consumer, and access evidence.
- Require explicit approval before repository/image deletion, public exposure, retention changes, or promotion changes.
- Prefer digest-pinned deployment and least-privilege push/pull roles.
- Do not expose image names tied to customers, private repository URLs, credentials, manifests with secrets, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm repository/image scope, environment, consumers, retention, and requested decision.
- Use official docs for registry behavior and sampled read-only evidence for API shape/current inventory.
- Check public exposure, image digests, version tags, signatures, scan status, retention, and downstream deployments.
- Return verdict, blockers, safe retention/promotion actions, rollback artifacts, and access recommendations.

## High-risk assumptions to kill

- “Tag latest is safe.”
- “No pulls means unused.”
- “Public repo is fine because the image has no secrets.”
- “Retention policy cannot break deployments.”

Those are lazy assumptions.

## Safe command/code verification targets

- List repositories/images without exposing identifiers.
- Check public/private state, lifecycle, digest, versions, signatures, scans, retention rules, and deployment consumers.
- Validate least-privilege push/pull and CI/CD identities.
- Confirm rollback image availability before cleanup.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to delete images without consumer and rollback evidence.
- The repository is public or proposed public without explicit justification.
- The evidence includes credentials, private URLs, manifests with secrets, or customer data.
