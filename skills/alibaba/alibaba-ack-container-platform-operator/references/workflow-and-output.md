# Workflow and output contract

Use this reference only when performing a full cluster operations review, upgrade assessment, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- ACK cluster type, Kubernetes version, and upgrade path
- Node pool configuration, auto-repair, and capacity
- ACR registry tier, image vulnerability scan results, and image signing
- ASM service mesh version, mTLS enforcement, and policy coverage
- OIDC workload identity configuration and pod-level RAM role assignments
- Resource quotas, LimitRange, and namespace isolation

## Safe workflow

1. **Frame scope**
   - Cluster name, type, region, and environment:
   - Business criticality and owner:
   - Compliance requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live ACK console or kubectl/API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What workloads use RAM access keys instead of OIDC?
   - What images have unresolved critical CVEs?
   - What node pools are on end-of-support Kubernetes versions?
   - What ASM policies are missing mTLS enforcement?
4. **Recommend the smallest safe action**
   - Prefer canary or blue-green upgrade strategies before full cluster upgrade.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud ACK Platform Review: <scope>
## Executive verdict
- Status: HEALTHY / ATTENTION NEEDED / ACTION REQUIRED
- Biggest risk:
- Evidence level:
## Cluster type and version
- Cluster type:
- Kubernetes version:
- Upgrade path:
## Node pool inventory
| Pool name | Node count | Instance type | Version | Health |
|---|---|---|---|---|
## ACR registry and image scan status
- Registry tier:
- Vulnerability findings:
- Image signing:
## ASM service mesh health
- ASM version:
- mTLS enforcement:
- Policy gaps:
## OIDC workload identity
- OIDC provider configured:
- Workloads using OIDC vs. RAM keys:
## Recommendations
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Open risks
- <risk or explicit none>
```
