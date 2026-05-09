# Workflow and output contract

Use this reference only when performing a full IAM least-privilege audit or permission boundary review.

## IAM audit areas to check

- Fine-grained policies: action scope, resource scope, conditions, wildcard usage, admin-level policies
- System roles: which coarse-grained roles are attached and whether a narrower fine-grained policy is feasible
- Agencies: trust principal (service or account), trust condition constraints, permission scope, cross-account access paths
- SCP inventory: active SCPs at org level, deny statements, allowlist scope, member account coverage
- Enterprise project bindings: per-project role assignments, cross-project permission leakage
- MFA enforcement: privileged account MFA status, MFA exception accounts
- Access key lifecycle: unused keys, keys older than rotation schedule, keys without last-used evidence

## Safe workflow

1. **Frame scope** — confirm target account, member accounts, enterprise projects, and non-goals
2. **Collect evidence** — prefer live IAM policy export or user-provided policy JSON; label all evidence types
3. **Stress-test** — privilege escalation paths, wildcard blast radius, SCP member account impact
4. **Recommend safest action** — policy narrowing, MFA enforcement, key rotation, SCP simulation before enforcement

## Output contract

Return this structure:

```markdown
# Huawei Cloud IAM Least-Privilege Audit: <scope>
## Scope and evidence level
## Critical findings
## Policy inventory summary
## SCP coverage and member account impact
## Agency trust relationship assessment
## MFA and access key posture
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
