# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `infrastructure` | DGX, HGX, MGX, reference architecture, BMC, ECC, persistence, MIG, driver, firmware, CUDA toolkit alignment, AI Enterprise support matrix, NCA-AIIO, NCP-AII |
| `networking-fabric` | Spectrum-X, InfiniBand, IB, NCCL, RoCEv2, lossless, congestion control, east-west, AR, NDR, HDR, NCP-AIN, fabric topology |
| `day2-ops` | DCGM, dcgm-exporter, MIG lifecycle, Xid, runbook, driver upgrade, firmware upgrade, fleet health, GPU monitoring, NCP-AIO |
| `kubernetes` | GPU Operator, device plugin, NFD, MIG manager, time-sliced GPU, container toolkit, securityContext, namespace tenancy, multi-tenant GPU |
| `cuda-perf` | CUDA kernel, .cu, coalescing, bank conflict, occupancy, register pressure, Nsight Compute, shared memory, stream concurrency, launch parameters |
| `inference-trt` | TensorRT, TRT, TensorRT-LLM, ONNX, precision, calibration, INT8, FP8, FP4, dynamic shapes, plugin, engine cache |
| `inference-triton` | Triton, model repository, dynamic batching, ensemble, custom backend, gRPC, response cache, rate limit |
| `genai-platform` | NeMo, NIM, generative AI, model card, weights provenance, evaluation harness, guardrails, NCA-GENL, NCA-GENM, NCP-GENL |
| `agentic-ai` | NeMo Agent Toolkit, agentic AI, NIM-as-tool, retrieval pipeline, tool-use safety, agent memory, audit log, NCP-AAI |
| `supply-chain` | NGC, NIM container, cosign, model card provenance, weights provenance, AI Enterprise license, air-gap mirror, API key scope |
| `runtime-evidence` | promote NIM to production, signed attestation, runtime-evidence verdict, promote / block / manual-review, image promotion gate |

## Full routing table

### Infrastructure (DGX / HGX / MGX)

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-ai-infrastructure-operations-agent` | infrastructure | Reviewing DGX/HGX/MGX posture against NVIDIA reference architectures and the AI Enterprise support matrix — driver/firmware/CUDA alignment, BMC segmentation, ECC, persistence, MIG posture |

### Networking fabric

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-ai-networking-fabric-review-agent` | networking-fabric | Reviewing Spectrum-X / InfiniBand topology, NCCL collective tuning, RoCEv2 lossless config, congestion control, east-west isolation between training jobs |

### Day-2 operations

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-ai-operations-day2-agent` | day2-ops | Reviewing DCGM exporter coverage, MIG lifecycle, Xid-signature-to-runbook mapping, and gated driver/firmware upgrade discipline |

### Kubernetes (GPU Operator)

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-gpu-operator-kubernetes-hardening-agent` | kubernetes | Reviewing GPU Operator on Kubernetes — device plugin, MIG manager, NFD, time-sliced GPUs, container toolkit, securityContext, namespace tenancy |

### CUDA performance

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-cuda-kernel-performance-review-agent` | cuda-perf | Doc-anchored static review of CUDA C/C++ kernels — coalescing, bank conflicts, occupancy, register pressure, stream concurrency, launch parameters |

### TensorRT / TensorRT-LLM

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-tensorrt-llm-deployment-review-agent` | inference-trt | Reviewing TensorRT / TensorRT-LLM pipelines — ONNX / PyTorch export, precision selection, calibration integrity, dynamic shapes, plugin trust boundaries, engine cache provenance |

### Triton inference serving

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-triton-inference-serving-review-agent` | inference-triton | Reviewing Triton deployments — model repository layout, dynamic batching, ensemble pipelines, custom backend trust, gRPC/HTTP auth, response cache, rate-limit and metrics endpoints |

### Generative AI platform

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-generative-ai-platform-review-agent` | genai-platform | Reviewing NeMo training and customization, NIM inference microservices, model card and weights provenance, evaluation harness, guardrails posture |

### Agentic AI platform

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-agentic-ai-platform-review-agent` | agentic-ai | Reviewing agentic-AI platforms on the NVIDIA stack — NeMo Agent Toolkit, NIM-as-tool, retrieval pipelines, tool-use safety, agent memory boundaries, audit logging |

### Supply chain

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-ngc-nim-supply-chain-governor-agent` | supply-chain | Reviewing NGC org/team boundaries, API-key scope and rotation, NIM cosign verification, model card + weights provenance, AI Enterprise license posture, air-gap mirror integrity |

### Runtime-evidence gate

| Agent | Domain(s) | Use when… |
|---|---|---|
| `nvidia-model-promotion-gatekeeper-agent` | runtime-evidence | A NIM container is moving from staging to production and you need a runtime-evidence promote / block / manual-review verdict with a cosign-signable attestation. **Requires explicit human confirmation before dispatch.** |

## Dispatch examples

### Single-domain

> "Review our DCGM exporter coverage."

```
Route: nvidia-ai-operations-day2-agent
Reason: pure day-2 operational posture review.
Mode: single
```

### Multi-domain (parallel)

> "We are bringing up a new DGX H200 cluster with Spectrum-X fabric and want a Day-1 review."

```
Route: nvidia-ai-infrastructure-operations-agent, nvidia-ai-networking-fabric-review-agent, nvidia-gpu-operator-kubernetes-hardening-agent
Reason: infrastructure posture, fabric tuning, and GPU Operator hardening all in scope.
Mode: parallel (3)
```

### Runtime-evidence gate

> "Promote `nvcr.io/nim/meta/llama-3.3-70b:1.5.0` to production."

```
Route: nvidia-model-promotion-gatekeeper-agent
Reason: runtime-evidence promotion decision with signed attestation required.
Mode: runtime-evidence-gate
```

Before dispatching the gatekeeper, surface: candidate digest, current-prod digest, expected signer identity, expected OIDC issuer, attestation TTL, rollback path. Require explicit written confirmation from the operator. Never auto-dispatch.
