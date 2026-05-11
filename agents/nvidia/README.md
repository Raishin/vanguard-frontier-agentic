# NVIDIA Agents

Role-based agents for the NVIDIA stack — CUDA, TensorRT, Triton, NIM, NeMo, NGC, DCGM, GPU Operator, AI fabric. Each agent here is bound 1:1 to the matching skill under `skills/nvidia/<id>/SKILL.md`. Agents apply judgment; skills hold the operating rules. Both must be present for the agent to be useful.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live mutation |
| --- | --- | --- | --- |
| Routing | Per-provider task router (`nvidia-maestro`) | read-only | not allowed |
| Role / advisory | Doc-anchored static reviews | read-only | not allowed |
| Live-runtime gate | Promote/block decision with signed attestation | read-only-runtime (allowlisted commands) | emits attestation only; never mutates registry or cluster |

## 🎼 Routing

| Agent | Role | What it routes |
| --- | --- | --- |
| `nvidia-maestro-agent` | `cloud-ai-platform-engineer` | Per-provider router. Classifies the task across the NVIDIA stack and dispatches to the narrowest specialist or a parallel team (max 4). Enforces a runtime-evidence gate before routing to the live promotion gatekeeper. |

## 👀 Role / advisory agents (read-only static review)

| Agent | Role(s) | Primary use |
| --- | --- | --- |
| `nvidia-ai-infrastructure-operations-agent` | `cloud-platform-engineer`, `cloud-ai-platform-engineer` | Review DGX/HGX/MGX against NVIDIA reference architectures and the AI Enterprise support matrix — driver/firmware/CUDA alignment, BMC segmentation, ECC, persistence, MIG posture. |
| `nvidia-ai-networking-fabric-review-agent` | `cloud-platform-engineer`, `cloud-ai-platform-engineer` | Review Spectrum-X / InfiniBand topology, NCCL collective tuning, RoCEv2 lossless config, congestion control, east-west isolation between training jobs. |
| `nvidia-ai-operations-day2-agent` | `cloud-platform-engineer`, `cloud-ai-platform-engineer` | Review DCGM exporter coverage, MIG lifecycle, Xid-signature-to-runbook mapping, gated driver/firmware upgrade discipline. |
| `nvidia-gpu-operator-kubernetes-hardening-agent` | `cloud-security-engineer`, `cloud-platform-engineer`, `cloud-ai-platform-engineer` | Review GPU Operator on Kubernetes — device plugin, MIG manager, NFD, time-sliced GPUs, container toolkit, securityContext, namespace tenancy. |
| `nvidia-cuda-kernel-performance-review-agent` | `cloud-ai-platform-engineer` | Doc-anchored static review of CUDA C/C++ kernels — coalescing, bank conflicts, occupancy, register pressure, stream concurrency, launch parameters. |
| `nvidia-tensorrt-llm-deployment-review-agent` | `cloud-ai-platform-engineer` | Review TensorRT / TensorRT-LLM pipelines — ONNX / PyTorch export, precision, calibration integrity, dynamic shapes, plugin trust, engine cache provenance. |
| `nvidia-triton-inference-serving-review-agent` | `cloud-ai-platform-engineer` | Review Triton deployments — model repository, dynamic batching, ensemble pipelines, custom backend trust, gRPC/HTTP auth, response cache, rate limit. |
| `nvidia-generative-ai-platform-review-agent` | `cloud-ai-platform-engineer` | Review NeMo training and customization, NIM inference microservices, model card and weights provenance, evaluation harness, guardrails. |
| `nvidia-agentic-ai-platform-review-agent` | `cloud-ai-platform-engineer` | Review agentic-AI platforms on the NVIDIA stack — NeMo Agent Toolkit, NIM-as-tool, retrieval pipelines, tool-use safety, agent memory, audit logging. |
| `nvidia-ngc-nim-supply-chain-governor-agent` | `cloud-security-engineer`, `cloud-ai-platform-engineer` | Review NGC org/team boundaries, API-key scope and rotation, NIM cosign verification, model card and weights provenance, AI Enterprise license, air-gap mirror integrity. |

## 🛡️ Live-runtime gate (allowlisted commands, signed attestation)

| Agent | Role(s) | Trust posture |
| --- | --- | --- |
| `nvidia-model-promotion-gatekeeper-agent` | `cloud-security-engineer`, `cloud-ai-platform-engineer` | `read-only-runtime`. Runs an allowlisted set of `cosign verify`, `crane digest`, `oras discover`, `grype` commands. Emits a cosign-signable attestation JSON with verdict `promote` / `block` / `manual-review`. Never mutates registry or cluster. Two harnesses by deliberate scope (claude-code + cursor); broader fan-out requires per-harness allowlist audit. |

## 📦 Install

```bash
# all NVIDIA agents (includes Maestro)
npx vfa-export-agents --provider nvidia

# the AI-platform role bundle (NVIDIA + GCP Vertex/Gemini + others)
npx vfa-export-agents --role cloud-ai-platform-engineer

# supply-chain + hardening subset only
npx vfa-export-agents --role cloud-security-engineer --provider nvidia
```

See `skills/nvidia/README.md` for the matching skill set and certification alignment notes.
