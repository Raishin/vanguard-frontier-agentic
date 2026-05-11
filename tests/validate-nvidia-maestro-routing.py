#!/usr/bin/env python3
"""Deterministic routing evaluator for `nvidia-maestro`.

For each fixture under tests/fixtures/nvidia-maestro-routing/inputs/*.json:

  1. Score every domain by counting taxonomy keywords present in the task
     (case-insensitive, word-boundary).
  2. Resolve the winning domain (or 2+ tied domains -> parallel mode).
  3. Detect runtime-evidence-gate by promotion-intent keywords.
  4. Map domain(s) -> agent(s) using the routing table.
  5. Diff route + mode against expected/<name>.json.

Regression checks:

  - Every agent referenced in the routing table exists in catalog/agents.json.
  - `nvidia-model-promotion-gatekeeper-agent` is never routed in mode 'single'
    (only via runtime-evidence-gate) — this is the live-agent auto-dispatch guard.
  - Every domain has at least one mapped agent.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INPUTS_DIR = ROOT / "tests" / "fixtures" / "nvidia-maestro-routing" / "inputs"
EXPECTED_DIR = ROOT / "tests" / "fixtures" / "nvidia-maestro-routing" / "expected"
AGENTS_CATALOG = ROOT / "catalog" / "agents.json"


# Same taxonomy as skills/nvidia/nvidia-maestro/references/workflow-and-output.md.
# Keep these in sync; the grader is the executable spec.
TAXONOMY: dict[str, list[str]] = {
    "infrastructure": [
        "DGX", "HGX", "MGX", "BMC", "ECC", "MIG",
        "driver", "firmware", "reference architecture",
        "AI Enterprise support matrix", "persistence",
    ],
    "networking-fabric": [
        "Spectrum-X", "InfiniBand", "NCCL", "RoCEv2", "lossless",
        "congestion control", "fabric topology", "east-west",
    ],
    "day2-ops": [
        "DCGM", "dcgm-exporter", "Xid", "runbook", "fleet health",
        "MIG lifecycle", "driver upgrade", "firmware upgrade",
    ],
    "kubernetes": [
        "GPU Operator", "device plugin", "MIG manager", "NFD",
        "time-sliced", "container toolkit", "securityContext",
        "namespace tenancy",
    ],
    "cuda-perf": [
        "CUDA kernel", ".cu", "coalescing", "bank conflict",
        "occupancy", "register pressure", "Nsight Compute",
        "shared memory", "stream concurrency",
    ],
    "inference-trt": [
        "TensorRT", "TensorRT-LLM", "TRT", "ONNX", "calibration",
        "INT8", "FP8", "FP4", "dynamic shapes", "engine cache",
    ],
    "inference-triton": [
        "Triton", "model repository", "dynamic batching",
        "ensemble", "custom backend", "response cache",
    ],
    "genai-platform": [
        "NeMo", "model card", "weights provenance",
        "evaluation harness", "guardrails",
        "generative AI",
    ],
    "agentic-ai": [
        "NeMo Agent Toolkit", "agentic AI", "NIM-as-tool",
        "retrieval pipeline", "tool-use safety", "agent memory",
        "audit log",
    ],
    "supply-chain": [
        "NGC", "cosign", "AI Enterprise license",
        "air-gap mirror", "API key scope",
    ],
}

DOMAIN_TO_AGENT: dict[str, str] = {
    "infrastructure": "nvidia-ai-infrastructure-operations-agent",
    "networking-fabric": "nvidia-ai-networking-fabric-review-agent",
    "day2-ops": "nvidia-ai-operations-day2-agent",
    "kubernetes": "nvidia-gpu-operator-kubernetes-hardening-agent",
    "cuda-perf": "nvidia-cuda-kernel-performance-review-agent",
    "inference-trt": "nvidia-tensorrt-llm-deployment-review-agent",
    "inference-triton": "nvidia-triton-inference-serving-review-agent",
    "genai-platform": "nvidia-generative-ai-platform-review-agent",
    "agentic-ai": "nvidia-agentic-ai-platform-review-agent",
    "supply-chain": "nvidia-ngc-nim-supply-chain-governor-agent",
}

# Promotion intent → runtime-evidence-gate (never 'single').
PROMOTION_RE = re.compile(
    r"(promote\s+\w+\s+(?:to\s+prod|to\s+production)|"
    r"production\s+promotion|"
    r"promote.*NIM|"
    r"staging[- ]to[- ]production|"
    r"runtime[- ]evidence)",
    re.IGNORECASE,
)
GATEKEEPER_AGENT = "nvidia-model-promotion-gatekeeper-agent"

# Multi-domain detection: if the second-best domain scores >= 60% of the top,
# we treat the task as multi-domain and emit parallel mode. Hard ceiling 4.
PARALLEL_THRESHOLD = 0.6
PARALLEL_CEILING = 4


def _score_domain(task: str, keywords: list[str]) -> int:
    task_lower = task.lower()
    hits = 0
    for kw in keywords:
        # Word-boundary match, case-insensitive. Some keywords contain non-word
        # chars (".cu", "TensorRT-LLM"); fall back to simple substring for those.
        if re.search(r"\W", kw):
            if kw.lower() in task_lower:
                hits += 1
        else:
            if re.search(rf"\b{re.escape(kw.lower())}\b", task_lower):
                hits += 1
    return hits


def evaluate(task: str) -> dict:
    """Return {route: [agent_ids], mode: 'single'|'parallel'|'runtime-evidence-gate'}."""
    if PROMOTION_RE.search(task):
        return {"route": [GATEKEEPER_AGENT], "mode": "runtime-evidence-gate"}

    scores = {d: _score_domain(task, kws) for d, kws in TAXONOMY.items()}
    ranked = sorted(scores.items(), key=lambda kv: (-kv[1], kv[0]))
    if ranked[0][1] == 0:
        return {"route": [], "mode": "unclassified"}

    top_score = ranked[0][1]
    winners = [d for d, s in ranked if s > 0 and s >= top_score * PARALLEL_THRESHOLD]
    winners = winners[:PARALLEL_CEILING]

    agents = sorted({DOMAIN_TO_AGENT[d] for d in winners})
    mode = "single" if len(agents) == 1 else f"parallel ({len(agents)})"
    return {"route": agents, "mode": mode}


def _regression_checks() -> list[str]:
    errors: list[str] = []
    catalog_ids = {a["id"] for a in json.loads(AGENTS_CATALOG.read_text())}
    for domain, agent_id in DOMAIN_TO_AGENT.items():
        if agent_id not in catalog_ids:
            errors.append(f"regression: domain {domain!r} maps to unknown agent {agent_id!r}")
    if GATEKEEPER_AGENT not in catalog_ids:
        errors.append(f"regression: gatekeeper agent {GATEKEEPER_AGENT!r} not in catalog")
    for domain in TAXONOMY:
        if domain not in DOMAIN_TO_AGENT:
            errors.append(f"regression: domain {domain!r} has no agent mapping")
    return errors


def main() -> int:
    if not INPUTS_DIR.is_dir():
        print(f"ERROR: inputs dir not found: {INPUTS_DIR}", file=sys.stderr)
        return 2

    failures = 0
    for err in _regression_checks():
        print(f"FAIL [regression] {err}")
        failures += 1

    inputs = sorted(INPUTS_DIR.glob("*.json"))
    if not inputs:
        print("ERROR: no fixtures found", file=sys.stderr)
        return 2

    for fp in inputs:
        fixture = json.loads(fp.read_text())
        name = fixture.get("name", fp.stem)
        expected = json.loads((EXPECTED_DIR / f"{name}.json").read_text())
        got = evaluate(fixture["task"])

        # The gatekeeper must never appear in 'single' or 'parallel' modes.
        if got["mode"] != "runtime-evidence-gate" and GATEKEEPER_AGENT in got["route"]:
            print(f"FAIL [{name}] live-agent auto-dispatch guard tripped: "
                  f"gatekeeper routed in mode={got['mode']!r}")
            failures += 1
            continue

        route_ok = set(got["route"]) == set(expected["route"])
        mode_ok = got["mode"] == expected["mode"]
        if route_ok and mode_ok:
            print(f"OK   [{name}] route={got['route']} mode={got['mode']}")
        else:
            print(f"FAIL [{name}] got route={got['route']} mode={got['mode']} | "
                  f"expected route={expected['route']} mode={expected['mode']}")
            failures += 1

    if failures:
        print(f"\n{failures} routing check(s) failed", file=sys.stderr)
        return 1
    print(f"\nOK: {len(inputs)} routing scenarios validated + regression guards passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
