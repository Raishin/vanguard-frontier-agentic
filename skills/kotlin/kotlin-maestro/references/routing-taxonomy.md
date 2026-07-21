# Routing Taxonomy And Modes

The domain-to-specialist map, the parallel-dispatch ceiling, and the out-of-board handoff table.

- Single mode routes to exactly one specialist; parallel (N) is capped at four and used only when the task genuinely spans that many domains.
- Out-of-board handoffs are mandatory for generic JVM/GC, virtual threads, generic Spring Boot, JPA tuning, Kafka, and generic Java deserialization (Java board); cluster/deploy (kubernetes/cloud); telemetry platform and SLOs (OpenTelemetry/Prometheus); signing and SLSA attestation (sigstore); web frontend (frontend); generic QA (qa).
- A production-mutation request is never dispatched to a specialist — it is gated to a named human owner.
