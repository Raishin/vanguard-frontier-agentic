# OCI Live OKE Rollout Guard Operations

> Version note: OCI service behavior, catalog entries, and tooling change over time. Verify exact command syntax, permissions, regional availability, feature maturity, and catalog targets before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Promoting because the deployment object succeeded while pods, PDBs, ingress, or metrics are unhealthy.
- Treating canary or blue-green as safe without traffic-split and rollback evidence.
- Running rollback without confirming target revision and artifact provenance.
- Ignoring node capacity, image pull, secrets, network policy, and service health.

## Officially grounded service shape

- Official OCI documentation describes DevOps as CI/CD for deployments to OCI platforms including OKE and supports blue-green and canary strategies.
- Official OCI documentation describes OKE as managed Kubernetes infrastructure, but Kubernetes rollout health still depends on workload, node, network, and application evidence.
- OCI API evidence through the user’s configured read-only OCI MCP shows cluster listing is compartment-scoped and can filter by lifecycle state and name.
- OCI API evidence through the user’s configured read-only OCI MCP shows DevOps deployment listing can filter by compartment, project, pipeline, lifecycle state, display name, time, sorting, and pagination.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate DevOps control-plane status from Kubernetes workload health and application SLO evidence.
- Require explicit approval before promote, rollback, resume, abort, or production traffic shift.
- Validate readiness, liveness, PDBs, replicas, events, metrics, logs, ingress, backend health, and rollback revision.
- Do not expose kubeconfigs, tokens, secrets, endpoint details, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm cluster, namespace, workload, pipeline, strategy, traffic path, and requested decision using sanitized labels.
- Use official docs for DevOps/OKE behavior and sampled read-only evidence for API shape/current deployment state.
- Assess canary/blue-green health, workload readiness, traffic routing, and rollback target.
- Return verdict, blockers, approval state, safe next actions, and post-action validation checks.

## High-risk assumptions to kill

- “Deployment SUCCEEDED means users are safe.”
- “Canary passed because no one complained.”
- “Rollout undo returns exactly to the previous production state.”
- “Cluster ACTIVE means workload capacity is healthy.”

Those are lazy assumptions.

## Safe command/code verification targets

- Check cluster and deployment lifecycle without exposing identifiers.
- Check pods, replicas, events, PDBs, services, ingress, metrics, logs, and backend health from sanitized evidence.
- Confirm traffic split, artifact version, rollback revision, and approval gate.
- Validate post-action SLOs and alert state before closing.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations or live-guard dispatch have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to promote or rollback without workload health evidence.
- The rollback target or artifact provenance is unclear.
- The evidence includes kubeconfig, tokens, secrets, endpoints, or customer data.
