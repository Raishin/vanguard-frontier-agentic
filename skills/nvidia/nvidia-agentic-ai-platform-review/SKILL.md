---
name: nvidia-agentic-ai-platform-review
description: Use this skill when reviewing agentic-AI platforms built on the NVIDIA stack — NeMo Agent Toolkit, NIM-as-tool patterns, retrieval-augmented generation pipelines, tool-call safety, agent memory boundaries, and per-tenant audit logging. Trigger when the user asks whether agent tool calls are sandboxed, whether agent memory is tenant-scoped, whether tool definitions are signed, or whether the deployment meets NCP-AAI expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: ai
---

# NVIDIA Agentic AI Platform Review

## Purpose

Review agentic-AI platforms built on the NVIDIA stack against the NCP-AAI body of knowledge: NeMo Agent Toolkit configuration, NIM microservices used as tools, retrieval/RAG pipelines, tool-call sandboxing and approval flows, agent memory partitioning, and per-tenant audit logging.

## Lean operating rules

- Prefer live evidence (NeMo Agent Toolkit configs, tool registry manifests, retrieval pipeline definitions, audit log samples) when the active client exposes it; otherwise fall back to NVIDIA NeMo Agent Toolkit documentation and sanitized agent configurations.
- Separate confirmed facts from inference. If tool-call sandbox state, memory partitioning, or audit coverage was not directly queried, say so.
- Treat agent tool definitions loaded from unsigned, mutable sources at runtime as a critical finding — a tampered tool spec is prompt injection at platform scale.
- Treat agent memory shared across tenants without explicit partition keys as a critical finding — cross-tenant memory bleed.
- Treat tool calls with side effects (write, delete, send) executed without an approval gate or per-tool allowlist as a high finding for production agents.
- Treat retrieval pipelines that ingest user-provided URLs without an egress allowlist as a high finding — SSRF and data-exfil path.
- Treat absence of per-call audit log (agent id, tool, inputs, outputs, latency, tenant) as a high finding for any regulated workload.
- Treat unbounded tool-call loops with no max-turn, max-tokens, or budget cap as a medium finding — cost and reliability risk.

## Response minimum

Return, at minimum:
- the scoped target (agent platform, NeMo Agent Toolkit version) and evidence level,
- tool registry signing and load posture,
- memory partitioning posture,
- tool-call approval and sandbox posture,
- retrieval/egress posture,
- audit log coverage,
- safe next actions and assumptions or blockers.
