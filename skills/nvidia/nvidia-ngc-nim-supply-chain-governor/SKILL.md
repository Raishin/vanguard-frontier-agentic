---
name: nvidia-ngc-nim-supply-chain-governor
description: Use this skill when reviewing NVIDIA NGC and NIM supply chain posture — NGC org and team boundaries, API key scope and rotation, NIM container cosign verification against NVIDIA's published identity, model card and weights provenance, AI Enterprise entitlement posture, and air-gap mirror integrity. Trigger when the user asks whether NIM images are verified before deployment, whether NGC keys are scoped per environment, or whether the deployment is procurement-defensible for a regulated tenant.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: security
---

# NVIDIA NGC and NIM Supply Chain Governor

## Purpose

Review supply chain posture for NGC (NVIDIA GPU Cloud) registry consumption and NIM (NVIDIA Inference Microservices) deployment: NGC org/team boundaries, API key scope and rotation, NIM container signature verification, model card and weights provenance, AI Enterprise entitlement posture, and air-gap mirror integrity for sovereign deployments.

## Lean operating rules

- Prefer live evidence (`cosign verify nvcr.io/nim/...`, NGC org/team listings, key creation timestamps and scopes, AI Enterprise license metadata, mirror manifest digests) when the active client exposes it; otherwise fall back to NVIDIA NGC and NIM documentation and sanitized configuration.
- Separate confirmed facts from inference. If image signature verification, key scope, or mirror integrity was not directly queried, say so.
- Treat NGC API keys with org-wide write scope used in CI as a critical finding — a leaked key publishes attacker-controlled artifacts.
- Treat NIM containers deployed without `cosign verify` (or equivalent admission policy) against NVIDIA's published identity as a high finding — image trust is unverified.
- Treat NGC keys without rotation cadence and without environment scoping (prod / non-prod) as a high finding.
- Treat model deployment with no model card and no weights provenance attestation as a high finding for regulated tenants — lineage cannot be reconstructed for audit.
- Treat air-gap mirrors that copy by tag rather than digest as a critical finding — tags are mutable, mirror drift is silent.
- Treat AI Enterprise entitlement posture that depends on a single license server with no failover as a medium finding for production.
- Treat NIM model artifacts cached on local disk with world-readable permissions on shared hosts as a high finding — weight exfiltration path.

## Response minimum

Return, at minimum:
- the scoped target (NGC org/team, NIM image set, AI Enterprise license set) and evidence level,
- NGC key scope and rotation posture,
- NIM signature verification posture,
- model card / weights provenance posture,
- air-gap mirror integrity posture (digest-pinned vs tag-pinned),
- AI Enterprise entitlement posture,
- safe next actions and assumptions or blockers.
