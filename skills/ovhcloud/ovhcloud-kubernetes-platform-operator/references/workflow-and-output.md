# Workflow and output contract

Use this reference only when performing a full cluster review, upgrade planning pass, node pool assessment, or security posture audit for OVHcloud Managed Kubernetes.

## Review domains

Check these areas before giving a verdict:

- Cluster version, available upgrade path, and API deprecation risks in the target version
- Node pool flavor selection, autoscaling bounds (min/max), and current utilization posture
- PodDisruptionBudgets (PDB): presence, minAvailable / maxUnavailable settings, and workload coverage
- Drain and reschedule readiness: taint, toleration, and node affinity configuration
- RBAC: ClusterRoleBindings and RoleBindings scope, service account privileges, and wildcard permission usage
- Network policies: default-deny posture, ingress/egress rules, and cross-namespace exposure
- Terraform IaC for `ovh_cloud_project_kube` and `ovh_cloud_project_kube_nodepool`: version pinning, node pool lifecycle policies

## Safe workflow

1. **Frame scope**
   - Project ID, cluster name, and region:
   - Current and target Kubernetes version:
   - Business criticality and maintenance window:
   - Required outcome (upgrade, scale, security review, node pool change):
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live OVHcloud MCK API or `kubectl` read-only evidence if available.
   - Otherwise inspect repository `ovh_cloud_project_kube` resources, sanitized cluster config, or official OVHcloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What workloads lack a PDB and would be disrupted by a node drain?
   - What version upgrade carries API deprecations or breaking add-on compatibility?
   - What RBAC binding grants cluster-wide write or escalation paths?
   - What network policy gap exposes backend services across namespaces?
   - What evidence is missing that prevents confident upgrade or operational sign-off?
4. **Recommend the smallest safe action**
   - Prefer blue-green node pool rotation over in-place forced replacement.
   - Prefer upgrade in a staging cluster before production.
   - If PDBs are absent or drain verification is unconfirmed, stop and say so.

## Output contract

Return this structure:

```markdown
# OVHcloud Managed Kubernetes Review: <cluster name or scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## PDB and drain readiness
- PDB coverage: <assessed or unknown>
- Drain verification: <confirmed / unconfirmed / not applicable>
## Residual risk
- <risk or explicit none>
```
