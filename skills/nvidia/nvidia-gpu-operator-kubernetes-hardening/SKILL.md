---
name: nvidia-gpu-operator-kubernetes-hardening
description: Use this skill when reviewing NVIDIA GPU Operator deployments on Kubernetes — device plugin, MIG manager, NFD labels, time-sliced GPU configuration, container toolkit, securityContext posture, namespace tenancy, and admission policy coverage. Trigger when the user asks whether GPUs are being shared safely across tenants, whether MIG profiles are enforced, or whether the GPU Operator is deployed per NVIDIA hardening guidance.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# NVIDIA GPU Operator on Kubernetes Hardening

## Purpose

Review NVIDIA GPU Operator deployments on Kubernetes against NVIDIA documentation and Kubernetes pod-security hardening: device plugin posture, MIG manager configuration, time-sliced GPU configuration, NFD label usage, container toolkit isolation, securityContext posture for GPU workloads, and admission policy enforcement of GPU resource requests.

## Lean operating rules

- Prefer live evidence (`kubectl -n gpu-operator get pods`, `kubectl get clusterpolicy`, `kubectl get nodes -L nvidia.com/gpu.product`, MIG profile annotations) when the active client exposes it; otherwise fall back to NVIDIA GPU Operator documentation and sanitized manifests.
- Separate confirmed facts from inference. If MIG strategy, time-slicing config, or admission posture was not directly queried, say so.
- Treat GPU workload pods running with `privileged: true` outside of the GPU Operator's own DaemonSets as a critical finding — privilege creep across tenant workloads.
- Treat MIG-capable nodes running in `single` strategy on multi-tenant clusters when `mixed` is required as a high finding — partition diversity is impossible.
- Treat time-sliced GPU configuration shared across tenants without namespace-scoped admission as a high finding — noisy-neighbor and side-channel risk.
- Treat absence of an admission policy (Kyverno / OPA / ValidatingAdmissionPolicy) gating `nvidia.com/gpu` requests by namespace as a high finding for multi-tenant clusters.
- Treat default GPU Operator deployment with public, unsigned image pulls as a medium finding — add image-verification policy.
- Treat node-feature-discovery labels absent on GPU nodes as a low finding — scheduling reliability is reduced.

## Response minimum

Return, at minimum:
- the scoped target (cluster, GPU Operator version, MIG strategy) and evidence level,
- device plugin and toolkit posture,
- MIG / time-slicing posture,
- admission policy posture for GPU resources,
- namespace tenancy posture,
- image verification posture,
- safe next actions and assumptions or blockers.
