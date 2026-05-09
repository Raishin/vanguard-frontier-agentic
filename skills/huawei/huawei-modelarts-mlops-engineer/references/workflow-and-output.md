# Workflow and output contract

Use this reference only when performing a full MLOps review or AI/ML platform implementation guidance.

## MLOps areas to check

- Training jobs: GPU vs Ascend NPU flavor, resource quota, timeout, dedicated pool vs on-demand, cost per run
- Framework alignment: MindSpore for Ascend, PyTorch/TensorFlow for GPU — misalignment causes runtime errors
- Pangu deployments: endpoint inventory, rate limiting configuration, scaling policy, inference cost
- AI Gallery: model version lifecycle, sharing policy, consumer dependency audit
- MLOps pipeline: data prep stage, training stage, evaluation gate (metric threshold), deployment approval, monitoring hook
- Cost governance: dedicated pool utilization, on-demand job cost history, quota enforcement

## Safe workflow

1. **Frame scope** — confirm training workload type (GPU/NPU), model deployment requirements, and non-goals
2. **Collect evidence** — prefer live job logs and cost history; label all evidence types
3. **Stress-test** — hung job cost risk, OOM failure patterns, rate limit blast radius, pipeline gate gaps
4. **Recommend safest action** — quota enforcement, staged deployment, evaluation gate configuration

## Output contract

Return this structure:

```markdown
# Huawei Cloud ModelArts MLOps: <scope>
## Scope and evidence level
## Training job inventory and cost governance
## GPU vs Ascend NPU framework alignment
## Pangu deployment posture
## MLOps pipeline evaluation gates
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
