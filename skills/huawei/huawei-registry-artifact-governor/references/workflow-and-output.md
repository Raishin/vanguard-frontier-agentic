# Workflow and output contract

Use this reference only when performing a full SWR registry governance review or supply chain security assessment for Huawei Cloud container workloads.

## Governance domains

Check these areas before giving a verdict:

- Namespace visibility: public vs. private for all SWR namespaces in scope
- VSS scanning coverage: enabled, scan-on-push configured, severity thresholds set
- Image retention: policy presence, tag retention rules, untagged image cleanup
- Cross-region synchronization: target regions, synchronization frequency, replication scope
- IAM agency permissions: CCE pull agency permission scope (least privilege vs. admin)
- Tag immutability: production repo enforcement status
- Supply chain attestation: image signing solution in use (or absence)

## Safe workflow

1. **Frame scope**
   - SWR namespace(s) and region(s) in scope:
   - Workload type and criticality:
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test posture**
   - What images are exposed publicly that should not be?
   - What CVEs are undetected because VSS scanning is not configured?
   - What storage costs accumulate because retention policies are absent?
   - What happens during a regional outage if cross-region sync is not configured?
   - What can an overpermissioned CCE pull agency do beyond pulling images?
   - What evidence is missing to confirm secure supply chain provenance?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud SWR Registry Governance: <scope>
## SWR namespace visibility and access control posture
## VSS vulnerability scanning coverage and severity thresholds
## Image retention policy and storage hygiene
## Cross-region image synchronization coverage
## IAM agency permissions for CCE image pull
## Supply chain security verdict
## Recommended hardening actions
```

Each section must include an evidence level label.
