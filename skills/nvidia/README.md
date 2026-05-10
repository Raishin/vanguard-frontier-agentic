# NVIDIA Skills

Skills covering NVIDIA's certification programs (NCA / NCP) and the
developer-facing CUDA / TensorRT / Triton surface area. Two anchor
tiers, declared explicitly so consumers can see the rigor difference.

## Tier 1 — Cert-anchored (operator and architect)

Each skill aligns to one or more current NVIDIA NCA / NCP certifications.
NCA / NCP exams are proctored on Certiverse, valid 2 years, with
published blueprints and domain weightings.

- `nvidia-ai-infrastructure-operations` — NCA-AIIO, NCP-AII
- `nvidia-ai-operations-day2` — NCP-AIO
- `nvidia-ai-networking-fabric-review` — NCP-AIN
- `nvidia-generative-ai-platform-review` — NCA-GENL, NCA-GENM, NCP-GENL
- `nvidia-agentic-ai-platform-review` — NCP-AAI
- `nvidia-gpu-operator-kubernetes-hardening` — cross-cutting (no 1:1 cert)
- `nvidia-ngc-nim-supply-chain-governor` — cross-cutting (no 1:1 cert)

## Tier 2 — Doc-anchored (developer)

NVIDIA does not run an NCA or NCP exam covering CUDA kernel development,
TensorRT engine builds, or Triton model-repository hardening as a
standalone proctored credential. DLI course-completion certificates
exist but sit at a different rigor tier from NCA / NCP.

The skills below are anchored on NVIDIA's published developer
documentation rather than on a certification blueprint. They are static
review only — they never execute `nvcc`, `trtexec`, `polygraphy`,
`tritonserver`, `perf_analyzer`, `nsight-compute`, or `nsight-systems`.
They emit the recommended invocation as text for the user to run on
their own GPU host. The trust boundary stays at `Read Grep Glob`.

- `nvidia-cuda-kernel-performance-review` — CUDA C++ Programming Guide,
  CUDA C++ Best Practices Guide, Nsight Compute / Nsight Systems docs.
- `nvidia-tensorrt-llm-deployment-review` — TensorRT Developer Guide,
  TensorRT Best Practices, TensorRT-LLM documentation.
- `nvidia-triton-inference-serving-review` — Triton Inference Server
  user guide, customization guide, and inference-protocols reference.

Doc-anchored skills carry an empty `certifications: []` field. That is
the marketplace signal: any skill with `certifications: []` and provider
`nvidia` is doc-anchored, not cert-anchored.

## Out of scope (intentionally)

`NCA-ADS`, `NCP-ADS`, `NCP-OUSD`. Data science and OpenUSD are real
NVIDIA tracks but are not aligned with this repo's current
cloud-and-zero-trust focus. Add them when there is a real consumer ask,
not before. The `Physical AI` certification announced at GTC 2026 has
no published exam code as of `last_verified`; not anchored on.
