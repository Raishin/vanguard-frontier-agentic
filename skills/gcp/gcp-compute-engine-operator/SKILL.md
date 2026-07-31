---
name: gcp-compute-engine-operator
description: Operate GCE instances, manage Managed Instance Groups (MIGs), configure OS patch management via VM Manager, design preemptible/spot VM strategies, and manage startup/shutdown scripts.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-08"
  category: platform
---

# GCP Compute Engine Operator

## Purpose

Act as a rigorous GCP Compute Engine operator. Keep GCE instances and MIGs patched, right-sized, cost-efficient, and secure.

## When to use

Use this skill for:

- GCE instance creation, configuration, and lifecycle management
- MIG (Managed Instance Group) design for stateless and stateful workloads
- OS patch management via VM Manager (OS Config)
- Spot VM and preemptible VM strategy for batch and fault-tolerant workloads
- Machine type selection (N2/E2 general purpose, C2/C3 compute-optimized, M2/M3 memory-optimized, A2/A3 GPU)
- OS Login configuration and SSH key management
- Startup and shutdown script design

## Key GCE specifics

- MIGs support both stateless (auto-scaling, auto-healing) and stateful (instance templates with persistent disk preservation) workloads.
- VM Manager (OS Config) provides OS inventory, patch compliance, and patch jobs — requires OS Config agent (enabled by default on recent images).
- Spot VMs: no advance notice, preempted any time — use for fault-tolerant batch, not web serving.
- Machine types: general purpose (N2/E2), compute-optimized (C2/C3), memory-optimized (M2/M3), accelerator-optimized (A2/A3 for GPU).
- Custom machine types: specify vCPU and memory independently — cost-efficient for non-standard profiles.
- OS Login: SSH key management via IAM instead of metadata keys — preferred for enterprise.

## Lean operating rules

- Prefer official GCP documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad service account permissions on VMs, missing patch jobs, and overprovisioned machine types.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding GCE behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
