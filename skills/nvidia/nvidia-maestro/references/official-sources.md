# Official sources

Use this reference when grounding a routing decision in NVIDIA product documentation or verifying domain-specific behavior.

## NVIDIA general documentation

- https://docs.nvidia.com/
- https://www.nvidia.com/en-us/learn/certification/
- https://docs.nvidia.com/ai-enterprise/

## CUDA and developer toolchain

- https://docs.nvidia.com/cuda/
- https://docs.nvidia.com/cuda/cuda-c-programming-guide/
- https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/
- https://docs.nvidia.com/nsight-compute/

## TensorRT and TensorRT-LLM

- https://docs.nvidia.com/deeplearning/tensorrt/
- https://docs.nvidia.com/tensorrt-llm/

## Triton Inference Server

- https://docs.nvidia.com/deeplearning/triton-inference-server/

## NIM, NeMo, NGC

- https://docs.nvidia.com/nim/
- https://docs.nvidia.com/nemo-framework/
- https://docs.nvidia.com/ngc/

## GPU Operator and Kubernetes

- https://docs.nvidia.com/datacenter/cloud-native/gpu-operator/latest/
- https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/

## DCGM and day-2 operations

- https://docs.nvidia.com/dcgm/
- https://docs.nvidia.com/datacenter/tesla/drivers/

## Networking fabric

- https://docs.nvidia.com/networking/
- https://docs.nvidia.com/deeplearning/nccl/

## Grounding rule

Official documentation explains NVIDIA product behavior. It does not prove the user's current driver version, firmware version, NGC org boundary, AI Enterprise entitlement, cluster MIG profile, fabric topology, or live container image state. Use documentation to ground routing decisions and specialist selection, not to assert the user's live state. Always prefer user-provided sanitized evidence or read-only discovery when available.

## Using documentation for routing

When a user describes a product or scenario and you are unsure which domain or specialist to select, consult the relevant NVIDIA documentation to confirm the product category before dispatching. Do not dispatch on a guess. If the domain is ambiguous after checking documentation, ask the user one clarifying question before routing.
