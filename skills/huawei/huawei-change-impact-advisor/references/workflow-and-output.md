# Workflow and output contract

Use this reference only when performing a full change impact review or production-readiness assessment for a planned Huawei Cloud infrastructure change.

## Impact domains

Check these areas before giving a verdict:

- Change description: what is changing, which resources, which region and account
- Organizations SCP scope: which member accounts and Enterprise Projects are in scope
- IAM agency dependency chain: which services currently assume the agency being modified
- VPC/network topology: which subnets and peered VPCs are affected by route table changes
- GaussDB disruption window: instance class change maintenance window and retry logic status
- CCE pod eviction risk: PodDisruptionBudget coverage and eviction impact on running workloads
- Rollback plan: how to revert each change independently if needed

## Safe workflow

1. **Frame scope**
   - Change type and target resource(s):
   - Region and account context:
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the change**
   - What fails if the SCP change denies a service that a member account depends on?
   - What breaks immediately if the IAM agency is deleted?
   - What traffic is disrupted if the VPC route entry is removed?
   - What applications lack connection retry logic during a GaussDB maintenance window?
   - Which pods lack PDB protection in the CCE node pool being scaled down?
   - What evidence is missing to confirm safe change execution?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Change Impact: <scope>
## Change description and target resources
## Organizations SCP cascade scope and affected accounts
## IAM agency dependency chain impact
## VPC/network topology impact
## GaussDB and database service disruption window
## CCE node pool and application eviction risk
## Safe change sequencing and rollback plan
```

Each section must include an evidence level label.
