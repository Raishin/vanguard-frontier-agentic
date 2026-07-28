# Routing Taxonomy And Modes

The domain-to-specialist map, the parallel-dispatch ceiling, and the out-of-board handoff table.

- Single mode routes to exactly one specialist; parallel (N) is capped at four and used only when the task genuinely spans that many domains.
- Out-of-board handoffs are mandatory: cloud deployment and managed services (cloud boards); Kubernetes rollout/admission/network policy (kubernetes); Terraform/IaC (terraform); OpenTelemetry Collector and Prometheus infrastructure (observability boards); artifact signing and SLSA attestation (sigstore); GPU infrastructure (nvidia); data-warehouse administration (databricks/snowflake); accounting/finance policy, legal/regulatory interpretation, HR (those boards); web frontend (frontend); generic QA (qa).
- A production-mutation request is never dispatched to a specialist — it is gated to a named human owner with the approval and rollback requirement.
