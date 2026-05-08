# Dataplane and CNI

## Step 1 — Identify the dataplane and CNI

Capture, in order:

1. **CNI plugin and version** — `kubectl -n kube-system get pods -l k8s-app=cilium -o name` (or `calico-node`, `flannel`, `aws-node`, `azure-cns`, `gke-cni`). Different CNIs have entirely different dataplanes (eBPF vs iptables vs vendor SDN), so every later question depends on this answer.
2. **kube-proxy presence and mode** — `kubectl -n kube-system get ds kube-proxy` and the `--proxy-mode` flag in its ConfigMap (`kubectl -n kube-system get cm kube-proxy -o yaml`). On Cilium kube-proxy replacement, the DaemonSet does not exist or is disabled.
3. **Routing mode** — encapsulation (VXLAN, Geneve, IPIP) vs native routing. Encapsulation works anywhere but adds 50–60 bytes of overhead and slightly higher CPU; native routing requires the underlay to route Pod CIDRs.
4. **IPAM mode** — `cluster-pool` (CNI manages CIDR), `kubernetes` (uses `node.spec.podCIDR`), `aws-eni` / `azure` / `gke` (cloud IPAM, pods get VPC IPs).
5. **Pod and Service CIDRs** — `kubectl cluster-info dump | grep -E '(cluster-cidr|service-cluster-ip-range)'` or check kube-controller-manager flags. Once set, these are very hard to change.
6. **Node MTU and overlay MTU** — `ip link show | grep mtu`, then the CNI overlay interface (`cilium_vxlan`, `flannel.1`). The overlay MTU should be `node MTU − encapsulation overhead`.

## Step 2 — Stress-test the CNI choice against actual requirements

Common decision points and the trap each one hides:

- **Cilium with kube-proxy replacement** — requires a kernel new enough for the eBPF features used (varies by version; verify with `cilium status` and the version's [system requirements page](https://docs.cilium.io/en/stable/operations/system-requirements/)). On older kernels, parts of KPR fall back to legacy paths and you lose the performance argument.
- **Calico in BGP mode** — requires the underlay to accept BGP from every node, or a route reflector. In cloud, this often means a peering VM. In a managed cluster, this is usually impossible — the cluster ends up in IPIP encapsulation, defeating the BGP choice.
- **AWS VPC CNI** — pods get VPC IPs, so subnet sizing and ENI limits per instance type bound pod density. An m5.large can hold ~30 pods because of ENI/IP limits, not memory. This is the dominant pod-density ceiling on EKS.
- **Azure CNI (legacy)** — pre-allocates pod IPs from the subnet at node join, exhausting the subnet long before the pods exist.
- **Azure CNI Overlay** — pods get a separate overlay CIDR; nodes still consume VNet IPs. Works at scale but add it as a deliberate IPAM choice, not the default.
- **GKE alias IPs** — pod range is a secondary range on the VPC; sizing is fixed per cluster and resizing requires recreation.

## Step 3 — Stress-test kube-proxy mode

| Mode | Strengths | Real failures |
|---|---|---|
| `iptables` (default) | Simple, broadly tested | Rule count grows linearly with `Services × Endpoints`. On large clusters (10k+ Services) full rule resync becomes a multi-second event; new Service propagation latency rises. |
| `ipvs` | Hash-based lookup, scales to many Services. Multiple LB algorithms. | Requires `ip_vs` kernel modules loaded on every node. Some session-affinity edge cases differ from iptables. Conntrack still drives source-IP preservation. |
| `nftables` | Modern netfilter framework; better incremental update than iptables. | Newer; not all distros have stable nftables tooling. Still relatively young in production at scale. |
| `kernelspace` (Windows) | Native Windows | Windows-only; behavior differs (no init container hostNetwork tricks, etc.). |
| Cilium kube-proxy replacement | eBPF socket-LB; bypasses iptables entirely; preserves source IP for NodePort without `externalTrafficPolicy: Local` quirks. | Requires Cilium dataplane. Some hostPort and kernel-version edge cases. Verify `cilium status` reports KPR enabled in the expected mode (`Strict`, `Probe`, etc.). |

Stress-tests the review must apply:

- Migrating from `iptables` to `ipvs` or `nftables` on a running cluster — short connectivity blip during the rollout as kube-proxy rewrites rules. Schedule like a node-by-node rollout, not a config flag flip.
- Migrating to Cilium KPR — uninstall kube-proxy *after* Cilium reports KPR healthy on every node. Removing kube-proxy first leaves Service VIPs unreachable.
- Mixed mode during rollout — half nodes on iptables, half on Cilium KPR — Service traffic to a Pod on a KPR node from a kube-proxy node may follow different return paths and confuse conntrack. Plan a fast rollout window.

## Step 4 — Stress-test IPAM and CIDR sizing

A cluster sized 10× too small for its eventual workload count is the most common architectural debt — and it is hard to fix.

Sizing stress-tests:

- **Pod CIDR size** vs **max nodes × max pods/node** — a `/16` Pod CIDR with a `/24` per node gives 256 nodes max. A `/22` per node gives 64 nodes. Many CNIs allocate a fixed-size block per node, so the right number is `nodes × pods_per_node × headroom`.
- **Service CIDR size** — Services tend to grow faster than people predict (every Helm chart adds a few). A `/16` is fine; a `/20` is risky in any cluster running a service mesh, since per-namespace mesh control planes add Services.
- **CIDR collision with on-prem or peer VPC** — pods in `10.0.0.0/8` cannot route to an on-prem `10.x.x.x` system. RFC 1918 collision checks must precede every cluster build.
- **`100.64.0.0/10` and `198.18.0.0/15`** — use carrier-grade NAT and benchmarking ranges if RFC 1918 is exhausted; cloud providers generally tolerate them.
- **IPv6 / dual-stack** — single-stack v6 only is rarely supported by ecosystem tools (registries, observability). Dual-stack is the practical choice; ensure `--service-cluster-ip-range` and `--cluster-cidr` carry both families and that EndpointSlices report both.

## Step 5 — Stress-test MTU and encapsulation

MTU mismatch is a silent failure. The TCP three-way handshake passes (small packets), then the first large response stalls forever because Path MTU Discovery (PMTUD) ICMP is dropped by a firewall.

The arithmetic the review must enforce:

- VXLAN: 50 bytes (8 VXLAN + 8 UDP + 20 IPv4 + 14 Ethernet but 14 is shared) → overlay MTU = node MTU − 50.
- Geneve: 60 bytes typical → overlay MTU = node MTU − 60.
- IPIP: 20 bytes.
- WireGuard (Cilium transparent encryption): ~60 bytes plus alignment.
- IPsec: variable, ~73 bytes worst case.

Stress-tests:

- Cloud underlay at MTU 1500 → overlay should be 1450 (VXLAN). Some installers default to 1500 on the overlay too — silent corruption on first large packet.
- Jumbo frames (MTU 9000) on AWS — works inside the same Availability Zone. Cross-AZ traffic is silently capped at 1500 by AWS, so jumbo overlay across AZs causes random stalls.
- GKE / EKS / AKS managed nodes — verify the cloud-provider MTU before trusting the CNI's auto-detection. Some installers read `eth0` MTU at startup and miss later changes.

Verification step: `kubectl run mtu-test --rm -it --image=alpine -- ping -M do -s <payload> <peer-pod-ip>`. The Don't-Fragment bit forces the kernel to fail rather than fragment, exposing the MTU ceiling.

## Step 6 — Dual-stack and IPv6

- The cluster API server, kube-controller-manager, kube-proxy, kubelet, and the CNI must all be configured for dual-stack. A partial enablement leads to confusing behavior where Services have only IPv4 addresses but Pods have both.
- `Service.spec.ipFamilies` and `ipFamilyPolicy` (`SingleStack`, `PreferDualStack`, `RequireDualStack`) are the per-Service selectors. Default depends on the cluster's primary family.
- EndpointSlices report addresses per family — verify both families are populated when `RequireDualStack` is in use.
- IPv6-only clusters interact poorly with images pulled from IPv4-only registries; verify NAT64/DNS64 or registry mirroring is in place.

## Output for this section

- CNI and version,
- kube-proxy mode (or KPR mode),
- routing mode (encapsulation type or native),
- IPAM mode and Pod / Service CIDR sizes,
- node and overlay MTU,
- dual-stack posture,
- findings on sizing headroom, mode mismatch, MTU correctness, KPR readiness,
- one-way-door warnings (CIDR resize, KPR migration) with cutover plan if a change is recommended.
