# OCI Load Balancer Traffic Engineer Operations

> Version note: OCI service behavior, catalog entries, and tooling change over time. Verify exact command syntax, permissions, regional availability, feature maturity, and catalog targets before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Changing listeners or backend sets without draining, health, and rollback evidence.
- Treating ACTIVE as proof traffic is flowing correctly.
- Ignoring certificates, backend security rules, route tables, DNS, and health checks.
- Replacing backends without understanding session persistence and client impact.

## Officially grounded service shape

- Official OCI documentation describes Load Balancer as automated traffic distribution from one entry point to multiple backend servers reachable from a VCN.
- Official OCI documentation says backend sets include a load balancing policy, health check policy, and backend servers; a backend set must be associated with listeners for the load balancer to work.
- Official OCI documentation warns that changing a backend set load-balancing policy can temporarily interrupt traffic and drop active connections.
- OCI API evidence through the user’s configured read-only OCI MCP shows load balancer listing is compartment-scoped and can filter by display name, lifecycle state, detail level, sorting, and pagination.
- OCI API evidence through the user’s configured read-only OCI MCP shows backend-set listing is load-balancer scoped and uses ETag support for concurrency-sensitive operations.
- OCI API evidence through the user’s configured read-only OCI MCP shows Network Load Balancer listing is compartment-scoped and can filter by lifecycle state, display name, sorting, and pagination.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Separate L7 Load Balancer from L4 Network Load Balancer behavior.
- Require current listeners, backend sets, health checks, certificates, security rules, route/DNS, logs, metrics, and owner evidence before traffic changes.
- Treat public exposure, TLS changes, backend replacement, policy changes, and failover as high risk.
- Do not expose private endpoints, certificates, keys, customer CIDRs, customer data, or sensitive identifiers.

## Minimal safe implementation flow

- Confirm LB/NLB type, target traffic path, listener, backend set, backend, certificate, and requested decision.
- Use official docs for service behavior and sampled read-only evidence for API shape/current state.
- Assess health, routing, TLS, security rules, logging, metrics, DNS, and rollback/drain path.
- Return verdict, blockers, traffic risks, safer changes, and validation checks.

## High-risk assumptions to kill

- “ACTIVE means healthy.”
- “Backend health check success equals application success.”
- “Listener changes are harmless.”
- “Rollback is just changing the backend back.”

Those are lazy assumptions.

## Safe command/code verification targets

- List LB/NLB lifecycle, listeners, backend sets, backends, and health without exposing identifiers.
- Check health policy, certificates, TLS mode, security rules, route/DNS, logs, metrics, and backend drain plan.
- Validate ETag/current-state capture before mutations.
- Confirm synthetic and user-path checks after traffic change.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations or live-guard dispatch have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks to change traffic without health and rollback evidence.
- The change opens public exposure or weakens TLS without explicit approval.
- The evidence includes private endpoints, certificates, keys, or customer data.
