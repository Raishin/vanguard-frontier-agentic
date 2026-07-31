---
name: nvidia-tensorrt-llm-deployment-review
description: Use this skill when reviewing TensorRT or TensorRT-LLM deployment artifacts statically — ONNX/PyTorch export pipelines, precision selection (FP16/BF16/INT8/FP8/INT4), calibration cache integrity, dynamic shape profiles, custom plugin loading, engine cache and serialized engine provenance, runtime memory pool sizing. Trigger when the user asks whether a TensorRT build script, calibration pipeline, or trtexec invocation follows NVIDIA's published guidance.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# NVIDIA TensorRT-LLM Deployment Review

## Purpose

Static review of TensorRT and TensorRT-LLM deployment pipelines against NVIDIA's TensorRT Developer Guide — ONNX/PyTorch export, FP16/INT8/FP8/INT4 precision, calibration data integrity, dynamic shape profiles, plugin trust boundaries, engine cache provenance. This skill is doc-anchored: it grounds review findings in NVIDIA's published documentation rather than in a certification blueprint, because no NVIDIA certification currently covers this developer-facing surface as a standalone exam objective.

## Lean operating rules

- Prefer the user's actual TensorRT build scripts, ONNX export code, and calibration pipelines as evidence; otherwise fall back to documentation-based inference.
- Treat custom TensorRT plugins loaded from non-pinned sources or unsigned object files as a critical finding — native-code execution surface inside the inference engine.
- Treat serialized engines (`.engine`, `.plan`) distributed without sha256 verification or provenance attestation as a high finding — silent model substitution.
- Treat INT8 / FP8 calibration data containing production user traffic without redaction or retention controls as a high finding — confidentiality and PII surface.
- Treat absence of `optimization_profiles` for variable input shapes as a medium finding — builds either fail at runtime or fall back to padded inference.
- Treat hardcoded `--workspace` or `--memory-pool-size` values that exceed the deployment GPU's free memory as a medium finding — engine build will OOM in CI.
- Treat use of `--strict-types` without explicit precision tagging on every layer as a low finding — actual precision drifts from intent.
- Always emit the exact `trtexec`, `polygraphy run`, or `tensorrt_llm/build.py` commands the user should run — do not execute them.

## Response minimum

Return, at minimum:
- the scoped target (model source and export pipeline, precision selection and calibration posture, dynamic shape and profile posture, plugin and engine provenance posture, runtime memory and concurrency posture, recommended trtexec/polygraphy invocations) and evidence level,
- findings labelled critical / high / medium / low,
- recommended NVIDIA-tooling invocations the user should run themselves,
- safe next actions and assumptions or blockers.
