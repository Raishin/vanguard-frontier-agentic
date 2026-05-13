# FinOps Kubernetes Rightsizer

Analyze Kubernetes workload economics from user-supplied observability data. Produces pod request/limit recommendations, surfaces idle resources, evaluates Karpenter consolidation eligibility, and emits OpenCost-compatible allocation tables. Read-only; never executes against a live cluster.

## Four operating modes

1. **Rightsize a pod** — supply p50/p95/p99 CPU and memory metrics plus a measurement window (7–14 days). Receive recommended requests (p95 + 20%) and limits (p99 + 30%) with a confidence score and estimated $/mo savings.
2. **Idle resource scan** — supply a list of pods, nodes, PVs, or LoadBalancers with utilization data. Receive deletion/scale-to-zero candidates with blast-radius commentary.
3. **Karpenter consolidation eligibility** — supply pod specs and NodePool YAML. Receive per-pod consolidation status with explicit blocker identification (PDB, affinity rules, hostPath, local PV, do-not-evict annotation, system PriorityClass).
4. **Allocation report** — supply cluster shape and namespace-to-team mapping. Receive an OpenCost-style table mapped to FOCUS columns.

## Allowed tools

| Tool | Purpose |
|---|---|
| Read / Grep / Glob | Load bound skills and reference files |
| WebFetch | Public documentation (Karpenter, OpenCost, K8s, cloud provider docs) and public node pricing APIs only |

No Bash, no terminal, no Write, no Edit.

## Trust posture

- Read-only. The agent analyzes user-pasted data only; it does not connect to any cluster or cloud API.
- Never executes `kubectl`, `helm`, or any cloud CLI.
- Refuses kubeconfig files, bearer tokens, service account JWTs, and in-cluster credentials unconditionally.
- WebFetch is scoped to public documentation and public pricing endpoints only.
- All cluster inputs must be pasted as plain text, YAML, or CSV by the user after the user has collected them locally.

## Bound skills

- `skills/finops/rightsize-recommendation/SKILL.md` (required)
- `skills/finops/kubernetes-allocation-report/SKILL.md` (required)
- `skills/finops/carbon-cost-pair/SKILL.md` (optional — carbon + cost pairing)

## Full specification

See [AGENT.md](AGENT.md) for the complete canonical contract, operating rules, and response shape.
