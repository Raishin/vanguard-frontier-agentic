# Workflow and output contract

Use this reference only when performing a full architecture design, migration assessment, or production-readiness review.

## Design domains

Check these areas before giving a verdict:

- Workload profile: compute, storage, network, data, and AI/ML requirements
- Region selection: MLPS level, data sovereignty, latency, and DR requirements
- Product selection: ECS vs CCE vs FunctionGraph, GaussDB vs RDS, OBS vs SFS vs EVS
- Enterprise-project model: resource grouping, cost allocation, and access governance
- Network topology: VPC design, AZ spread, ELB placement, hybrid connectivity
- Compliance: MLPS Level 2/3, ISO 27001, SOC 2, local regulations
- Cost: on-demand vs reserved instances, storage tiers, data transfer

## Safe workflow

1. **Frame scope**
   - Workload type and criticality:
   - Region constraints (MLPS/sovereignty):
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test design**
   - What fails if a single AZ goes down?
   - What fails if primary region is unavailable?
   - What can expose data outside sovereignty boundary?
   - What can escalate privilege across enterprise-project boundaries?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Solution Architecture: <scope>
## Workload requirements
## Region selection rationale (MLPS/sovereignty)
## Product selection
## Enterprise-project model
## Architecture topology
## Cost and compliance considerations
## Open questions
```

Each section must include an evidence level label.
