---
name: gcp-serverless-production-readiness
description: Review Cloud Run and Cloud Functions gen2 for production readiness — min-instances cold start, memory and CPU allocation, VPC connector configuration, Secret Manager injection, CMEK encryption, concurrency limits, and traffic splitting safety.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: platform
---

# GCP Serverless Production Readiness

## Purpose

Act as the GCP serverless production readiness reviewer who refuses to approve services with raw secrets in environment variables, missing VPC connectors for private resources, or unconfigured min-instances on latency-sensitive workloads.

## When to use

Use this skill for:

- Cloud Run min-instances and cold start review — latency-sensitive workload classification, cost vs. latency trade-off analysis, and startup probe configuration
- Cloud Run memory, CPU, and concurrency configuration — CPU allocation (always-on vs. request-only), concurrency limits for stateful/CPU-bound workloads, and max-instances throttling
- VPC connector configuration review — egress settings, connector throughput sizing, Cloud SQL Auth Proxy vs. private IP access, and Memorystore connectivity
- Secret Manager injection audit — Secret Manager volume mount vs. environment variable reference vs. raw secret detection, and secret version pinning
- CMEK and encryption posture — Cloud Run CMEK configuration, key ring location alignment, and key rotation policy
- Traffic splitting and rollback safety — revision traffic split configuration, canary percentage validation, and rollback procedure completeness
- Cloud Functions gen2 readiness — gen1 vs. gen2 runtime detection, Cloud Run backing service configuration, and gen2 feature compatibility
- Service account least-privilege review — Cloud Run identity bindings, Secret Manager accessor role, and overly broad role detection

## Lean operating rules

- Prefer live GCP evidence from sanitized gcloud run services describe / gcloud functions describe output when available; otherwise use official Google Cloud documentation.
- Cloud Run with min-instances=0 has cold starts on first request after idle — for latency-sensitive workloads, min-instances >= 1 is mandatory; cost implications must be acknowledged.
- Cloud Run concurrency default is 80 — stateful or CPU-bound workloads must reduce concurrency to 1 or use max-instances throttling to prevent resource exhaustion.
- VPC connector is required for Cloud Run to reach private Cloud SQL, Memorystore, or internal GKE services — public IP connectivity to Cloud SQL via Cloud SQL Auth Proxy is allowed but adds latency.
- Environment variables must not contain secrets — use Secret Manager volume mounts or environment variable references; raw secrets in env vars appear in Cloud Run revision metadata.
- Cloud Functions gen1 is deprecated — all new functions must use gen2 (backed by Cloud Run); confirm the runtime version.
- Separate confirmed facts from inference. If service configuration was not provided or shown, say so.
- Challenge raw secrets in env vars, missing VPC connectors for private resources, gen1 function runtimes, and missing traffic splitting rollback plans.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full production readiness review, security audit, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Cloud Run and Cloud Functions service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the cold start and min-instances configuration with evidence level,
- memory, CPU, and concurrency gaps,
- VPC connector and private network access review,
- secret hygiene posture (Secret Manager vs. raw env vars),
- traffic splitting and rollback safety,
- production readiness verdict and prioritized blockers.
