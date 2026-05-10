---
name: nvidia-ai-infrastructure-operations
description: Use this skill when reviewing NVIDIA AI infrastructure deployments — DGX, HGX, MGX systems, GPU server install posture, BMC and out-of-band exposure, BIOS/firmware levels, vGPU host configuration, and rack-scale power/cooling/networking readiness. Trigger when the user asks whether a GPU host is provisioned per NVIDIA reference architecture, whether the BMC is segmented, whether driver/firmware versions match the AI Enterprise support matrix, or whether the deployment is in scope for NCA-AIIO or NCP-AII certification expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# NVIDIA AI Infrastructure Operations Review

## Purpose

Review NVIDIA GPU infrastructure deployments (DGX, HGX, MGX, certified OEM systems) against NVIDIA reference architectures and the NCA-AIIO / NCP-AII certification body of knowledge. Anchor judgments on driver + firmware + CUDA toolkit + AI Enterprise support matrix alignment, BMC/iDRAC/iLO segmentation, and host-level GPU configuration (persistence mode, ECC, MIG capability, vGPU).

## Lean operating rules

- Prefer live evidence (`nvidia-smi`, `nvidia-smi -q`, `dmidecode`, `ipmitool lan print`, `dcgmi diag`) when the active client exposes it; otherwise fall back to NVIDIA Enterprise Support documentation, sanitized topology diagrams, and the AI Enterprise compatibility matrix.
- Separate confirmed facts from inference. If BMC network segmentation, firmware level, or driver-toolkit match was not directly queried, say so.
- Treat a BMC / iDRAC / iLO interface reachable from a tenant or workload network as a critical finding. GPU hosts hold model weights and tenant data; OOB compromise is total compromise.
- Treat driver / CUDA / cuDNN versions outside the published NVIDIA AI Enterprise support matrix as a high finding — silent ABI breakage and unsupported workloads.
- Treat ECC disabled on production GPUs as a high finding for training workloads (silent corruption of weights or gradients).
- Treat persistence mode disabled on long-running inference hosts as a medium finding (driver re-init latency at first call).
- Treat MIG-capable GPUs running in default whole-GPU mode in a multi-tenant cluster as a medium finding — partitioning is the isolation primitive.
- Treat absent or unverified firmware bundle (HGX baseboard, NVSwitch, BMC) as a high finding for any deployment with regulated or high-value workloads.

## References

Load these only when needed:
- NVIDIA AI Enterprise support matrix
- DGX/HGX system user guides for the deployed generation
- NCA-AIIO and NCP-AII exam blueprints

## Response minimum

Return, at minimum:
- the scoped target (host class, generation, AI Enterprise version) and evidence level,
- driver / CUDA / cuDNN / firmware posture vs the support matrix,
- BMC / OOB segmentation posture,
- ECC / persistence / MIG posture per GPU,
- the safest next actions and any assumptions or blockers.
