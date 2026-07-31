---
name: nvidia-ai-networking-fabric-review
description: Use this skill when reviewing NVIDIA AI networking fabrics — Spectrum-X Ethernet or InfiniBand topology, NCCL collective tuning, RoCEv2 lossless DCQCN/PFC config, adaptive routing, congestion control, and east-west isolation between training jobs. Trigger when the user asks whether the fabric is non-blocking, whether NCCL chooses the right transport, whether PFC/ECN is configured correctly, or whether the deployment meets NCP-AIN expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: networking
---

# NVIDIA AI Networking Fabric Review

## Purpose

Review NVIDIA AI fabric configuration against the NCP-AIN body of knowledge: rail-optimized topology, NCCL collective communication tuning (NCCL_TOPO, NCCL_IB_HCA, NCCL_NET_GDR_LEVEL), RoCEv2 lossless DCQCN/PFC, InfiniBand subnet manager and partitioning, adaptive routing, and tenant/job east-west isolation.

## Lean operating rules

- Prefer live evidence (`ibstat`, `ibdiagnet`, `nccl-tests` `all_reduce_perf` baselines, `ethtool -S`, switch QoS counters) when the active client exposes it; otherwise fall back to NVIDIA Spectrum-X / Quantum InfiniBand documentation and sanitized topology diagrams.
- Separate confirmed facts from inference. If NCCL transport selection, PFC posture, or partition keys were not directly queried, say so.
- Treat a non-rail-optimized topology presented as rail-optimized as a critical finding — collective performance claims are wrong.
- Treat RoCEv2 deployed without PFC + ECN (DCQCN) as a high finding — lossless behavior is not provided; goodput collapses under congestion.
- Treat default NCCL settings on multi-rail systems as a medium finding when published baselines are missing — transport selection is unverified.
- Treat shared partition keys (PKey 0x7FFF / default) across tenants on a multi-tenant InfiniBand fabric as a high finding — east-west isolation is absent.
- Treat absence of `nccl-tests` baselines stored alongside the cluster spec as a medium finding — regressions cannot be detected.
- Treat subnet manager running on a single switch with no failover as a high finding for production fabrics.

## Response minimum

Return, at minimum:
- the scoped target (fabric type, generation, scale) and evidence level,
- topology and rail posture,
- NCCL transport and tuning posture,
- lossless/PFC/ECN posture (RoCEv2) or PKey/partition posture (InfiniBand),
- tenant isolation posture,
- safe next actions and assumptions or blockers.
