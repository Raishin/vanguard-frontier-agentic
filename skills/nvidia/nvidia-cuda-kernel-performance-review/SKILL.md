---
name: nvidia-cuda-kernel-performance-review
description: Use this skill when reviewing CUDA C/C++ kernel sources statically against NVIDIA's published performance guidance — global-memory coalescing, shared-memory bank conflicts, warp divergence, occupancy and register pressure, stream/event concurrency, kernel launch parameter selection. Trigger when the user asks whether a `.cu` or `.cuh` file follows NVIDIA's published performance and correctness guidance, or asks for the exact `nsight-compute` or `nsight-systems` invocation to run themselves.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# NVIDIA CUDA Kernel Performance Review

## Purpose

Static review of CUDA C/C++ kernels for memory coalescing, shared-memory bank conflicts, occupancy, register pressure, and stream concurrency against NVIDIA's official CUDA Programming and Best Practices Guides. This skill is doc-anchored: it grounds review findings in NVIDIA's published documentation rather than in a certification blueprint, because no NVIDIA certification currently covers this developer-facing surface as a standalone exam objective.

## Lean operating rules

- Prefer the user's actual `.cu` and `.cuh` sources as evidence; otherwise fall back to documentation-based inference and say so.
- Separate confirmed facts from inference. If kernel launch params, register count, or shared-memory usage were not directly read from source, say so.
- Treat global-memory access patterns where adjacent threads in a warp do not access adjacent words as a high finding — coalescing is broken.
- Treat shared-memory access patterns where lanes in a warp hit the same bank with different addresses as a high finding — bank conflicts serialize the warp.
- Treat warp-divergent control flow inside hot loops as a medium finding — occupancy and instruction throughput drop.
- Treat kernel launches with thread-block sizes that are not multiples of 32 as a medium finding — warp utilization is reduced.
- Treat use of `cudaDeviceSynchronize` inside hot paths or per-batch loops as a medium finding — stream concurrency is destroyed.
- Treat absence of `__restrict__` qualifiers on non-aliasing pointer arguments as a low finding — the compiler cannot keep loads in registers.
- Always emit the exact `nsight-compute` and `nsight-systems` commands the user should run for runtime confirmation — do not execute them.

## Response minimum

Return, at minimum:
- the scoped target (kernel inventory and target architectures, global-memory access posture, shared-memory and register posture, occupancy and launch-parameter posture, stream and synchronization posture, recommended Nsight invocations) and evidence level,
- findings labelled critical / high / medium / low,
- recommended NVIDIA-tooling invocations the user should run themselves,
- safe next actions and assumptions or blockers.
