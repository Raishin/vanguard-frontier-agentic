---
name: nvidia-generative-ai-platform-review
description: Use this skill when reviewing NVIDIA generative-AI platforms — NeMo training and customization pipelines, NIM inference microservices, NeMo Guardrails, model card and weights provenance, evaluation/eval-harness posture, and tenant data isolation. Trigger when the user asks whether NIM containers are correctly verified before deployment, whether NeMo Guardrails are configured, or whether the deployment meets NCA-GENL, NCA-GENM, or NCP-GENL expectations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: ai
---

# NVIDIA Generative AI Platform Review

## Purpose

Review NVIDIA generative-AI platform posture against the NCA-GENL, NCA-GENM, and NCP-GENL bodies of knowledge: NeMo training/customization pipelines, NIM inference microservice deployment, NeMo Guardrails configuration, model card and weights provenance, evaluation harness coverage, and tenant data isolation in multi-tenant inference.

## Lean operating rules

- Prefer live evidence (`cosign verify nvcr.io/nim/...`, NIM `/v1/health/ready`, NeMo Guardrails config files, model card YAML, eval harness reports) when the active client exposes it; otherwise fall back to NVIDIA NeMo / NIM documentation and sanitized configuration.
- Separate confirmed facts from inference. If NIM image signature verification, guardrails state, or eval coverage was not directly queried, say so.
- Treat NIM container pulled from `nvcr.io` without `cosign verify` against NVIDIA's published key/identity as a high finding — image trust is unverified.
- Treat absence of model card or weights provenance attestation for production-served models as a high finding — lineage cannot be reconstructed.
- Treat NeMo Guardrails absent or bypassable for an externally exposed LLM endpoint as a critical finding for any regulated workload.
- Treat evaluation harness covering only loss/perplexity but no safety, jailbreak, or hallucination evaluation as a high finding for customer-facing LLMs.
- Treat training data with no documented PII handling, no opt-out path, and no retention bound as a critical finding for regulated tenants.
- Treat NIM inference shared across tenants without per-tenant request scoping as a high finding — cross-tenant context leakage path.

## Response minimum

Return, at minimum:
- the scoped target (NeMo pipeline, NIM microservice, model class) and evidence level,
- NIM image signature verification posture,
- guardrails posture,
- model card / weights provenance posture,
- eval harness coverage,
- tenant isolation posture,
- safe next actions and assumptions or blockers.
