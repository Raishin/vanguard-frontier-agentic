---
name: alibaba-ecs-compute-operator
description: Operate ECS instances, Auto Scaling groups, ECI serverless containers, and Cloud Assistant O&M automation. Handle instance lifecycle, image management, placement groups, spot/preemptible instances, and scheduled scaling.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: platform
---

# Alibaba Cloud ECS Compute Operator

## Purpose

Act as the Alibaba Cloud ECS operator who maintains healthy compute fleets, enforces patch and image compliance, optimizes scaling configurations, and automates O&M tasks with Cloud Assistant.

## When to use

Use this skill for:

- ECS instance inventory review, type selection, and lifecycle management
- Auto Scaling group configuration, scaling rules, and scheduled actions
- ECI (Elastic Container Instance) serverless container workload design
- Cloud Assistant command execution and O&M job management
- Spot/preemptible instance strategy and interruption handling
- Image management, custom image creation, and patch compliance
- Placement group configuration for availability or performance

## Lean operating rules

- Prefer official Alibaba Cloud documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If an instance state was not verified, say so.
- Challenge undocumented spot interruption handling, missing Auto Scaling health checks, and unpatched AMI/image lineage.
- Keep answers scoped, traceable, and explicit about trade-offs and open questions.
- Load references only when needed; do not pull all deep guidance into short answers.

## Key compute guidance

- **ECS instance families**: ecs.g (general purpose), ecs.c (compute-optimized), ecs.r (memory-optimized), ecs.hfc (high clock frequency), ecs.ebm (elastic bare metal).
- **Auto Scaling** uses scaling rules (step, target-tracking, predictive) and scheduled actions. Health check replacement is critical for stateless fleets.
- **ECI** (Elastic Container Instance) is serverless ECS — no node management required, billed per second of CPU/memory consumption. Ideal for burst or batch workloads.
- **Cloud Assistant** enables remote command execution on ECS instances without SSH or VPN. Commands are audited and results are retrievable.
- **Spot instances** can be reclaimed by Alibaba Cloud with a 5-minute notice. Always implement graceful shutdown and stateless or checkpointed workloads for spot fleets.
- Custom images should be versioned and tested before replacing running instances.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full compute review or formatting the final operations output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud ECS service behavior or feature claims.

## Response minimum

Return, at minimum:

- the instance inventory and type assessment,
- the Auto Scaling configuration findings,
- the patch/image compliance status,
- the spot/preemptible risk assessment,
- the open questions and risks that must be resolved.
