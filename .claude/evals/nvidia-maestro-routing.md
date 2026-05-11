# EVAL: nvidia-maestro routing

## Purpose

Verify that `skills/nvidia/nvidia-maestro/SKILL.md` + its routing-table reference (`references/workflow-and-output.md`) deterministically route every NVIDIA task to the correct specialist (or parallel team, or runtime-evidence gate). No LLM in the loop — the grader is a keyword-taxonomy-driven Python evaluator so the eval is fast, free, and deterministic.

## Capability evals (12 scenarios)

| # | Scenario | Expected route | Expected mode |
|---|---|---|---|
| 01 | CUDA kernel coalescing review | `nvidia-cuda-kernel-performance-review-agent` | single |
| 02 | DCGM exporter coverage audit | `nvidia-ai-operations-day2-agent` | single |
| 03 | NIM cosign verification policy | `nvidia-ngc-nim-supply-chain-governor-agent` | single |
| 04 | Triton dynamic batching tuning | `nvidia-triton-inference-serving-review-agent` | single |
| 05 | TensorRT-LLM INT8 calibration | `nvidia-tensorrt-llm-deployment-review-agent` | single |
| 06 | GPU Operator securityContext hardening | `nvidia-gpu-operator-kubernetes-hardening-agent` | single |
| 07 | Spectrum-X NCCL tuning | `nvidia-ai-networking-fabric-review-agent` | single |
| 08 | DGX BMC + driver/firmware alignment | `nvidia-ai-infrastructure-operations-agent` | single |
| 09 | NeMo Agent Toolkit / agentic-AI review | `nvidia-agentic-ai-platform-review-agent` | single |
| 10 | NeMo generative-AI model card / guardrails review | `nvidia-generative-ai-platform-review-agent` | single |
| 11 | New DGX H200 cluster bring-up (infra + fabric + GPU Operator) | `nvidia-ai-infrastructure-operations-agent + nvidia-ai-networking-fabric-review-agent + nvidia-gpu-operator-kubernetes-hardening-agent` | parallel (3) |
| 12 | Promote a NIM container from staging to prod | `nvidia-model-promotion-gatekeeper-agent` | runtime-evidence-gate |

## Regression evals

- Routing table in `references/workflow-and-output.md` parses cleanly (every agent referenced exists in `catalog/agents.json`).
- `nvidia-model-promotion-gatekeeper-agent` is **never** routed to in mode `single` — only `runtime-evidence-gate`. Any direct single route to it is a critical regression (auto-dispatch of a live agent).
- Every domain in the taxonomy has at least one matching agent.

## Success metrics

- Capability evals: **pass^1 = 1.00** (deterministic grader, must be perfect)
- Regression evals: **pass^1 = 1.00**
- Wire into `npm run validate` as the 13th gate.

## Grader

`tests/validate-nvidia-maestro-routing.py` — keyword-overlap scorer driven by the same taxonomy table that lives in `workflow-and-output.md`. For each task description, score every domain by counting distinct taxonomy keywords present (case-insensitive, word-boundary). Highest-score domain wins; ties → the domain whose agent is listed first under that section. Multi-domain detected when ≥2 domains score > 0 with comparable strength. The gatekeeper agent is only emitted when the task explicitly contains promotion / "to production" / "promote NIM" signals.
