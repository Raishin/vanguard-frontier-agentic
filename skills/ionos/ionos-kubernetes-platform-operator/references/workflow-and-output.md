# Workflow and output contract

Use this reference only when performing a full IONOS managed Kubernetes cluster review or upgrade safety assessment.

## Review domains

Check these areas before giving a verdict:

- Cluster health and readiness: control-plane version, node pool status, and API server accessibility
- Node pool configuration: instance type, core and memory sizing, autoscaling min/max bounds, and LAN attachment
- Workload placement: affinity and anti-affinity rules, node selectors, and toleration coverage
- PodDisruptionBudget coverage: all production Deployments and StatefulSets must have PDB definitions before scale-down or upgrade
- Control-plane upgrade safety: upgrade is irreversible — confirm k8s version compatibility, deprecated API usage, and node pool drain behavior
- kubeconfig scope: is the generated kubeconfig scoped to the minimum required permissions?
- GDPR data residency: cluster datacenter region matches declared processing location

## Safe workflow

1. **Frame scope**
   - Cluster name and datacenter region:
   - Declared GDPR processing location:
   - Operation type: health review, scale-down, upgrade, LAN change, kubeconfig rotation, PDB audit
   - Business criticality and owner:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live IONOS Kubernetes API or `kubectl` output if available.
   - Otherwise inspect Terraform IaC, user-provided sanitized cluster state, or official IONOS docs.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What workloads are disrupted if a node pool scale-down evicts a pod without PDB protection?
   - What breaks if a control-plane upgrade fails mid-way? Is rollback possible?
   - What GDPR constraint applies to the cluster region?
   - What deprecated Kubernetes APIs are in use that would break after an upgrade?
   - What is the kubeconfig scope — is it narrower than cluster-admin?
4. **Recommend the smallest safe action**
   - Require PDB coverage confirmation before any scale-down or upgrade recommendation.
   - For control-plane upgrades, always require a confirmed rollback plan (upgrades are irreversible on IONOS managed K8s).
   - Prefer incremental node pool changes over simultaneous multi-pool modifications.

## Output contract

Return this structure:

```markdown
# IONOS Managed Kubernetes Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Cluster region:
- GDPR residency: CONFIRMED / MISMATCH / UNKNOWN
- Confirmed:
- Unknown:
- Out of scope:
## Cluster health
- Control-plane version:
- Node pool status:
- API server accessible: yes / no / unknown
## PDB coverage
| Workload | PDB defined | Min available | Status |
|---|---|---|---|
## Upgrade or scale-down safety verdict
- Pre-conditions met: yes / no / partial
- Rollback path: <documented or absent>
## Findings
| Severity | Domain | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Evidence gaps
- <gap or explicit none>
## Residual risk
- <risk or explicit none>
```
