# Permission Model: OCI Live Network Security Rule Guard

## Least-privilege IAM policy for network rule read (preflight only)

```
Allow group NetworkAuditors to read virtual-network-family in compartment <compartment>
Allow group NetworkAuditors to read vcns in compartment <compartment>
Allow group NetworkAuditors to read security-lists in compartment <compartment>
Allow group NetworkAuditors to read network-security-groups in compartment <compartment>
Allow group NetworkAuditors to read subnets in compartment <compartment>
Allow group NetworkAuditors to read db-systems in compartment <compartment>
Allow group NetworkAuditors to read autonomous-databases in compartment <compartment>
```

Read-only audit: use `inspect` or `read` verbs only. Never `manage` for auditors.

## Least-privilege IAM policy for network rule mutation (guarded operator only)

```
Allow group NetworkOperators to manage security-lists in compartment <compartment>
Allow group NetworkOperators to manage network-security-groups in compartment <compartment>
Allow group NetworkOperators to read vcns in compartment <compartment>
Allow group NetworkOperators to read subnets in compartment <compartment>
```

Do **not** grant `manage virtual-network-family` — that is broader than needed and includes VCN, route tables, internet gateways, and peering.

## Risk classification by rule type

| Rule | Risk | Reason |
|---|---|---|
| Ingress `0.0.0.0/0` any protocol | Critical | Open internet access to entire subnet |
| Ingress `0.0.0.0/0` port 22 | Critical | SSH from internet — never acceptable in production |
| Ingress `0.0.0.0/0` port 3389 | Critical | RDP from internet — never acceptable in production |
| Ingress `0.0.0.0/0` port 1521/1522 | Critical | Oracle DB from internet — data exfiltration path |
| Ingress `0.0.0.0/0` port 3306/5432 | Critical | MySQL/PostgreSQL from internet |
| Ingress from VCN CIDR, specific port | Low | Internal only — verify VCN CIDR is not transit-routed |
| Egress `0.0.0.0/0` all | Medium | Standard but verify no data-loss risk for DB subnets |
| Stateless rule on DB subnet | High | No connection tracking — asymmetric TCP risk |

## Stateful vs stateless

- **Stateful** (default, `stateless: false`): OCI tracks connection state and automatically allows return traffic. Use for all production workloads.
- **Stateless** (`stateless: true`): Higher performance, but return traffic requires an explicit rule in the opposite direction. A missing return rule silently drops responses. Only use when performance benchmarked at scale.

## Subnet criticality classification

| Subnet pattern | Classification |
|---|---|
| Hosts Autonomous DB, DB System, Exadata | Database — highest protection |
| Hosts compute instances with public IP | Public compute — ingress rules must be minimal |
| Private subnet (`prohibit-public-ip: true`) | Internal — `0.0.0.0/0` still covers all VCN-routed traffic |
| Bastion subnet | Bastion — SSH/RDP ingress from known CIDRs only |

## OCI Network Path Analyzer — preferred verification tool

Before approving a connectivity change, use Path Analyzer to simulate the traffic path:
```bash
oci network path-analyzer-test create \
  --compartment-id <COMPARTMENT_OCID> \
  --protocol-parameters '{"type":"TCP","destinationPort":<PORT>}' \
  --source-endpoint '{"type":"COMPUTE_INSTANCE","instanceId":"<INSTANCE_OCID>"}' \
  --destination-endpoint '{"type":"IP_ADDRESS","address":"<DEST_IP>"}'
```
Path Analyzer respects Security Lists, NSGs, route tables, and service gateways — use it as the final approval gate for any rule change.
