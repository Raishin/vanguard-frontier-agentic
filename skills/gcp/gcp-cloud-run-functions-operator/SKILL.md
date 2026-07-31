---
name: gcp-cloud-run-functions-operator
description: Deploy and operate Cloud Run services, Cloud Functions gen2, Eventarc triggers, traffic splitting for progressive delivery, and cold-start optimization strategies.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.2.0"
  updated: "2026-05-09"
  category: platform
---

# GCP Cloud Run and Functions Operator

## Purpose

Act as a rigorous Cloud Run and Cloud Functions operator. Keep serverless services reliable, cost-efficient, and free of cold-start surprises or silent VPC connectivity gaps.

## Cloud Run Resource Types

Cloud Run has three distinct resource types — confirm which one the user needs before proceeding:

| Resource Type | Use When | Key Characteristics |
|---|---|---|
| **Service** | HTTP/gRPC API, event response, web app | Stateless, scale-to-zero, unique HTTPS endpoint, request-based billing |
| **Job** | Batch processing, scheduled task, data pipeline step | Runs to completion, parallelizable tasks, no persistent endpoint |
| **Worker Pool** | Pull-based consumers (Kafka, Pub/Sub pull, RabbitMQ) | Always-on, no HTTP endpoint, pulls work from queues |

## Reference Directory

| Scenario | Trigger Keywords | Reference |
|---|---|---|
| Deploy a service | HTTP, web app, API, deploy container, autoscale | [Services section](#services) |
| Run a job | batch, scheduled, cron, run to completion, data pipeline | [Jobs section](#jobs) |
| Worker pool setup | Kafka consumer, Pub/Sub pull, RabbitMQ, background worker | [Worker Pools section](#worker-pools) |
| IAM & auth | invoke, service account, ingress, unauthenticated | [Security section](#security) |
| VPC connectivity | VPC connector, egress, private IP, Cloud SQL, Memorystore | [Networking section](#networking) |
| Cost & scaling | concurrency, min-instances, max-instances, cold start | [Scaling & Cost section](#scaling--cost) |

## When to use

Use this skill for:

- Cloud Run service deployment, revision management, and traffic splitting
- Cloud Run jobs for batch and scheduled workloads
- Cloud Run worker pools for pull-based queue consumers (Kafka, Pub/Sub pull, RabbitMQ)
- Cloud Functions gen2 deployment and configuration
- Eventarc trigger design (Pub/Sub, GCS, Firestore, Audit Logs, custom sources)
- Progressive delivery via revision traffic splits (canary, blue/green)
- Cold-start analysis and minimum instances recommendations
- Concurrency tuning and CPU allocation mode (request-only vs. always-on)
- VPC connectivity (Direct VPC Egress vs. VPC connector) for private resource access

## Key Cloud Run and Functions specifics

- Cloud Run revision traffic: you can split traffic across multiple revisions (e.g., 90/10 canary) — this is the primary progressive delivery mechanism.
- Minimum instances: prevents cold starts but costs money even when idle. Use for latency-sensitive services.
- Cloud Functions gen2 runs on Cloud Run internally — same container model, same networking.
- Eventarc: event-driven triggers from Pub/Sub, GCS, Firestore, Audit Logs, and custom sources. Use instead of polling patterns.
- Concurrency: Cloud Run supports up to 1000 concurrent requests per instance. CPU is only allocated during request processing by default (not during idle).
- Always-on CPU: required for background tasks or WebSockets — set cpu="always" to keep CPU allocated between requests.
- VPC connector or Direct VPC Egress: required for Cloud Run to access private resources in a VPC. Direct VPC Egress is newer and preferred.

## Lean operating rules

- Prefer official GCP documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge missing min-instances for latency-sensitive services, CPU-only-on-request for background workloads, and missing VPC egress for private access.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Cloud Run/Functions behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
