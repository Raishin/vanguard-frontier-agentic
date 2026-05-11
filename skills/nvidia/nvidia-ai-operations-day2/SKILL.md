---
name: nvidia-ai-operations-day2
description: Use this skill when reviewing day-2 operations of NVIDIA GPU fleets — DCGM exporter and DCGM-Diag posture, GPU telemetry into Prometheus/Grafana, MIG partitioning lifecycle, GPU health and Xid error response, fleet upgrade paths, and incident response for GPU-failure modes. Trigger when the user asks how a GPU fleet is monitored, whether DCGM is wired, whether Xid signatures map to runbooks, or whether the deployment meets NCP-AIO certification expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: observability
---

# NVIDIA AI Operations (Day-2) Review

## Purpose

Review operational posture of NVIDIA GPU fleets against the NCP-AIO body of knowledge: DCGM and DCGM-Exporter coverage, MIG partitioning lifecycle, Xid error classification and runbooks, fleet driver/firmware upgrade orchestration, and GPU-aware incident response.

## Lean operating rules

- Prefer live evidence (`dcgmi diag`, `dcgmi health`, `nvidia-smi --query-gpu=...`, Prometheus DCGM metrics) when the active client exposes it; otherwise fall back to NVIDIA DCGM documentation, sanitized dashboards, and Xid reference tables.
- Separate confirmed facts from inference. If DCGM coverage, alerting rules, or Xid classification was not directly queried, say so.
- Treat absence of DCGM exporter on production GPU nodes as a high finding — fleet is operationally blind.
- Treat ungated driver/firmware rolling upgrade (no pre-drain, no canary, no rollback path) as a high finding.
- Treat undocumented Xid signature → runbook mapping as a medium finding — incidents will be triaged from scratch.
- Treat GPU-Operator / device-plugin running with default permissive securityContext on multi-tenant clusters as a high finding.
- Treat MIG re-partitioning performed live without workload drain as a medium finding (running contexts are killed).
- Treat alerting that does not cover ECC double-bit errors, NVLink errors, or row-remapping pending as a medium finding.

## Response minimum

Return, at minimum:
- the scoped target (cluster/fleet, exporter version) and evidence level,
- DCGM exporter coverage and alert posture,
- MIG partitioning lifecycle posture,
- Xid → runbook coverage,
- driver/firmware upgrade discipline,
- safe next actions and assumptions or blockers.
