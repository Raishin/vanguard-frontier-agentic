---
name: nvidia-triton-inference-serving-review
description: Use this skill when reviewing Triton Inference Server deployments statically — `model_repository/` layout and `config.pbtxt` files, dynamic batching configuration, ensemble and BLS pipelines, custom backend (Python, C++, ONNX, OpenVINO, vLLM) trust posture, gRPC and HTTP endpoint authentication, response cache configuration, rate-limit and metrics exposure. Trigger when the user asks whether a Triton model repository or `tritonserver` invocation follows NVIDIA's published guidance and security expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# NVIDIA Triton Inference Server Review

## Purpose

Static review of Triton Inference Server deployments against NVIDIA's Triton documentation — model repository layout, dynamic batching, ensemble pipelines, custom backend trust, gRPC/HTTP authentication, model encryption at rest, response cache poisoning surface. This skill is doc-anchored: it grounds review findings in NVIDIA's published documentation rather than in a certification blueprint, because no NVIDIA certification currently covers this developer-facing surface as a standalone exam objective.

## Lean operating rules

- Prefer the user's actual `model_repository/` tree and `config.pbtxt` files as evidence; otherwise fall back to documentation-based inference.
- Treat custom Python or C++ backends loaded from non-pinned sources or without code review as a critical finding — in-process RCE.
- Treat gRPC or HTTP endpoints exposed without authentication, mTLS, or a restricted-protocol gateway as a critical finding for multi-tenant deployments.
- Treat model repository directories with world-writable permissions or a writable `--model-repository` mount as a high finding — silent model substitution.
- Treat response caching enabled across tenants without per-request cache-key partitioning as a high finding — cross-tenant cache poisoning.
- Treat ensemble or BLS pipelines that pass user-supplied tensors directly to a Python backend without input validation as a medium finding — deserialization surface.
- Treat metrics endpoints (`:8002`) exposed to the public network without scraping ACLs as a medium finding — model name and shape leakage.
- Treat dynamic batching `max_queue_delay_microseconds` left at default with latency SLOs in the millisecond range as a low finding — throughput-vs-latency tuning is wrong by default.
- Always emit the exact `tritonserver` and `perf_analyzer` commands the user should run — do not execute them.

## Response minimum

Return, at minimum:
- the scoped target (model repository layout and provenance, backend trust posture, endpoint and auth posture, batching and ensemble posture, response cache and metrics posture, recommended tritonserver/perf_analyzer invocations) and evidence level,
- findings labelled critical / high / medium / low,
- recommended NVIDIA-tooling invocations the user should run themselves,
- safe next actions and assumptions or blockers.
