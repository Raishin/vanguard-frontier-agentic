# OCI DevOps Container Platform Engineer Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Assuming OCI IAM permissions are the same as Kubernetes RBAC.
- Pushing images or rolling deployments without promotion, scanning, rollback, and owner checks.
- Treating a cluster list as enough to judge networking, node pools, admission controls, or workload health.
- Granting tenancy-wide pipeline permissions for convenience.
- Making repositories public accidentally or ignoring retention.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows OKE cluster list, DevOps project list, and container repository list operations expose compartment, lifecycle, name/display-name, subtree/public repository, sorting, and pagination filters. Treat this as API shape evidence, not cluster or pipeline readiness.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Confirm compartment, region, project/cluster/repository, namespace, environment, and owner before action.
- Separate OCI IAM, registry permissions, Kubernetes RBAC, and workload identity.
- Require image provenance, scanning/signing, promotion rules, and rollback before production deployment.
- Use read-only discovery before pipeline, cluster, node-pool, repository, or deployment mutation.
- Never request kubeconfigs, tokens, private keys, registry passwords, or customer identifiers in chat.

## Minimal safe implementation flow

- Classify cluster, pipeline, registry, or rollout task.
- Collect official docs plus sampled read-only evidence for projects, clusters, repositories, and deployment targets.
- Review IAM/RBAC, network, image, deployment, observability, and rollback posture.
- Identify unsafe mutations and missing gates.
- Return safe plan, approvals, validation checks, and rollback.

## High-risk assumptions to kill

- “A green pipeline means production is safe.”
- “A cluster exists, so workloads are healthy.”
- “Registry push access implies deployment access should also be granted.”
- “Rollback is automatic because Kubernetes has rollout history.”
- “Public repositories are acceptable for non-production images.”

Those are lazy assumptions.

## Safe command/code verification targets

- List projects, OKE clusters, and container repositories in confirmed scope.
- Check cluster lifecycle, node-pool health, Kubernetes version, network, admission/policy controls, and observability before operations.
- Review pipeline stages, artifacts, triggers, deployment environments, approvals, and rollback configuration.
- Check repository public/private state, retention, image tags/digests, scan/signature evidence, and pull principals.
- Require explicit approval for cluster/node-pool changes, pipeline mutation, repository deletion, or production rollout.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
