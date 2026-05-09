# Workflow and output contract

Use this reference only when performing a full compute operations review, scaling audit, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Instance family, type, and sizing relative to workload requirements
- Auto Scaling group health checks, scaling rules, and cooldown configuration
- Spot/preemptible usage, interruption handling, and fallback strategy
- Image lineage, custom image versioning, and patch compliance
- Cloud Assistant job history and O&M automation coverage
- Placement group configuration for fault domain distribution

## Safe workflow

1. **Frame scope**
   - Workload and environment (dev/staging/prod):
   - Business criticality and owner:
   - Compliance and patch requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live ECS console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What happens when a spot instance is reclaimed?
   - What Auto Scaling health checks are missing?
   - What images are unpatched or unversioned?
   - What Cloud Assistant jobs have failed or are unmonitored?
4. **Recommend the smallest safe action**
   - Prefer staged rollout and image versioning before mass replacement.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud ECS Compute Review: <scope>
## Executive verdict
- Status: HEALTHY / ATTENTION NEEDED / ACTION REQUIRED
- Biggest risk:
- Evidence level:
## Instance inventory and type review
| Instance ID | Family | Type | State | Finding |
|---|---|---|---|---|
## Auto Scaling configuration
- Scaling group:
- Health check type:
- Scaling rules:
- Findings:
## Patch and image compliance
- Current image:
- Last patched:
- Compliance status:
## Spot and preemptible assessment
- Spot usage:
- Interruption handling:
- Fallback strategy:
## Cloud Assistant job status
- Recent jobs:
- Failures:
## Recommendations
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Open risks
- <risk or explicit none>
```
