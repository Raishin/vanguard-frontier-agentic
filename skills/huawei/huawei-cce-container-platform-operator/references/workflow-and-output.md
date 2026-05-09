# Workflow and output contract

Use this reference only when performing a full CCE platform review or implementation guidance.

## Container platform areas to check

- CCE cluster: version, type (Standard/Autopilot), node pool inventory, Kubernetes version lifecycle
- Node pools: version, scaling policy, drain posture, PodDisruptionBudgets
- SWR: image scan results, tag policies, registry access policy, dependency audit
- ASM: mTLS mode (STRICT/PERMISSIVE/DISABLED), VirtualService/DestinationRule inventory, namespace scope
- IEF: edge node registration status, edge application health, device twin sync state
- Workload Identity: agency-token-mount configuration vs long-lived AK/SK usage

## Safe workflow

1. **Frame scope** — confirm target cluster, enterprise project, and non-goals
2. **Collect evidence** — prefer live state; label all evidence types
3. **Stress-test** — blast radius, failure modes, missing evidence
4. **Recommend safest action** — narrow, staged, with rollback

## Output contract

Return this structure:

```markdown
# Huawei Cloud CCE Container Platform: <scope>
## Scope and evidence level
## Cluster and node pool status
## SWR image posture
## ASM service mesh policy
## IEF edge node health
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
