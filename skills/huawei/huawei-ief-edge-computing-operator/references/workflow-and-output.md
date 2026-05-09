# Workflow and output contract

Use this reference only when performing a full IEF edge computing review or implementation guidance.

## IEF areas to check

- Edge nodes: registration status, connectivity to cloud, hardware specification, node group assignment
- Edge applications: workload inventory per node, container image versions, health status, resource limits
- Device twins: device inventory, desired vs reported state divergence, sync delay
- Cloud-edge messaging: bus configuration, message throughput, queued message backlog
- Offline operation: expected offline duration tolerance, reconnect reconciliation behavior, local state persistence
- Security: node certificate validity, IEF agent version, access policy for edge-to-cloud communication

## Safe workflow

1. **Frame scope** — confirm target edge nodes, geographic distribution, offline risk tolerance, and non-goals
2. **Collect evidence** — prefer live node status and device twin state; label all evidence types
3. **Stress-test** — offline reconnect conflicts, application update blast radius for remote nodes, device twin sync failure modes
4. **Recommend safest action** — staged application updates, rollback plan for remote nodes, reconnect conflict resolution procedure

## Output contract

Return this structure:

```markdown
# Huawei Cloud IEF Edge Computing: <scope>
## Scope and evidence level
## Edge node inventory and connectivity
## Edge application deployment health
## Device twin sync posture
## Offline operation and reconnect plan
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
