# DNS and Service Discovery

## Step 1 — Identify the in-cluster DNS topology

Capture:

- CoreDNS deployment shape: `kubectl -n kube-system get deploy coredns -o wide`. Replica count, resources, anti-affinity.
- CoreDNS Corefile: `kubectl -n kube-system get cm coredns -o yaml`. Plugins enabled, forward target, cache TTL, autopath, log/errors.
- NodeLocal DNSCache presence: `kubectl -n kube-system get ds node-local-dns` (or `nodelocaldns`). Listening IP (typically a link-local address like `169.254.20.10`), upstream target, kube-proxy mode coupling.
- kubelet `--cluster-dns` flag — does it point at the CoreDNS Service IP, or at the NodeLocal DNSCache link-local IP?
- A pod's `/etc/resolv.conf`: `kubectl exec <pod> -- cat /etc/resolv.conf`. Note the `search`, `nameserver`, and `options ndots:` lines.

## Step 2 — Stress-test the Corefile

A canonical Corefile for a Kubernetes cluster typically uses these plugins in order:

```
.:53 {
    errors
    health { lameduck 5s }
    ready
    kubernetes cluster.local in-addr.arpa ip6.arpa {
        pods insecure          # consider: pods verified
        fallthrough in-addr.arpa ip6.arpa
        ttl 30
    }
    prometheus :9153
    forward . /etc/resolv.conf {
        max_concurrent 1000
    }
    cache 30
    loop
    reload
    loadbalance
}
```

Stress-tests:

- `pods insecure` returns A records for any pod IP without verifying a pod exists in that namespace. `pods verified` validates against pod existence in the same namespace; higher memory cost but a tighter security posture.
- `forward . /etc/resolv.conf` follows the node's resolv.conf, which may point to the cloud DNS (169.254.169.254 on AWS, 168.63.129.16 on Azure). If pods talk to external services, every miss escapes the cluster — set `max_concurrent` deliberately to bound load on the upstream.
- Missing `cache` plugin — every query hits upstream, including repeated queries for the same name within the TTL. The cache plugin is required for any cluster with non-trivial DNS load.
- `cache` TTL larger than the upstream record's TTL — stale records persist past the authoritative source's update window. 30 seconds is a typical compromise.
- `autopath` plugin — server-side search-path completion. Reduces the `ndots:5` round-trip cost (see Step 4) but requires `pods verified`, costs more memory, and complicates debugging because client-side lookups no longer match what reaches CoreDNS.
- `loop` plugin — detects forwarding loops at startup. Without it, a forward to the cluster's own resolv.conf can loop until the deployment crashes. Always keep `loop` enabled.
- `reload` plugin — picks up Corefile changes without a restart. Without it, a ConfigMap edit is not honored until the pod is recreated.

## Step 3 — Stress-test CoreDNS scaling

CoreDNS is the single most overlooked source of cluster-wide latency. Defaults from `kubeadm` and many installers are not production-sized.

- Replica count — small clusters get 2 replicas by default. A cluster with 1000 pods doing 100 QPS per pod sees 100k QPS; two replicas with stock resources will become a queue.
- Resource requests / limits — many installers default to a tight `100m CPU / 70Mi memory` request that is fine for 100 pods and a CPU throttle for 10000.
- Pod anti-affinity — every CoreDNS replica must be on a different node. The default deployments usually have this; verify after migration to a new cluster.
- PodDisruptionBudget — `minAvailable: 1` is the absolute floor; a stricter `maxUnavailable: 1` plus PDB on the DaemonSet's underlying nodes is safer during cluster autoscaler events.
- `cluster-proportional-autoscaler` ([repository](https://github.com/kubernetes-sigs/cluster-proportional-autoscaler)) is the standard way to scale CoreDNS replicas with cluster size.

## Step 4 — The `ndots:5` and search-path tail-latency trap

The default `dnsPolicy: ClusterFirst` injects an `options ndots:5` line into every pod's `/etc/resolv.conf` along with a search list like:

```
search default.svc.cluster.local svc.cluster.local cluster.local
options ndots:5
```

The `ndots:5` directive means: any name with fewer than 5 dots is treated as relative and tried against every search-list entry first.

For a query like `api.example.com` (3 dots) the resolver issues:

1. `api.example.com.default.svc.cluster.local.` — NXDOMAIN
2. `api.example.com.svc.cluster.local.` — NXDOMAIN
3. `api.example.com.cluster.local.` — NXDOMAIN
4. `api.example.com.` — finally the real query

For external services, that is **4× the DNS load** and 3× the chance of dropped UDP packets causing a 5-second timeout.

Mitigations the review must consider:

- Lower `options ndots:1` per-pod via `dnsConfig.options` for workloads that overwhelmingly resolve external hostnames. Cluster-internal Service names still resolve because they are absolute when written in the canonical `service.namespace.svc.cluster.local` form.
- Use `ExternalName` Services so that `mysvc.namespace.svc.cluster.local` is the canonical reference, avoiding search-list expansion entirely.
- Enable `autopath` in CoreDNS — completes the search path server-side, requires `pods verified` and more memory.
- Always pair this fix with NodeLocal DNSCache so the search-list expansion cost stays on the node.

## Step 5 — NodeLocal DNSCache deep-dive

Per the [Kubernetes docs](https://kubernetes.io/docs/tasks/administer-cluster/nodelocaldns/), NodeLocal DNSCache runs as a DaemonSet listening on a link-local IP. It solves three production problems:

1. **conntrack overhead** — every UDP DNS query allocates a conntrack entry that lives 30 seconds; high-DNS-QPS pods exhaust conntrack tables and trigger random drops elsewhere. NodeLocal DNSCache upgrades to TCP for upstream traffic, where entries are removed on connection close.
2. **5-second timeout amplification** — a dropped UDP packet triggers the resolver's retry timer (3 × 10s on glibc, with extra hops). Local cache hits remove most of this.
3. **DNAT bypass** — pods talking to the link-local IP skip the iptables/IPVS Service VIP rewrite entirely.

Operational risks the review must call out:

- **OOMKill is a node-wide DNS outage.** When the local cache pod is killed, the iptables rules that redirected DNS traffic to the cache stay in place pointing at an unhealthy pod until the new pod is ready. Set memory limits with headroom (default cache is 10k entries ≈ 30 MB; 100 MB request and 200 MB limit is a safer baseline) and monitor `coredns_dns_cache_size_entries`.
- **PodDisruptionBudget** for the DaemonSet — node drains during cluster autoscaler events should not all evict the local cache simultaneously.
- **kubelet `--cluster-dns` flag must be updated** if the kube-proxy mode is IPVS — see the linked docs for the exact rewrite. iptables mode tolerates either the cache IP or the kube-dns Service IP.
- **IPv6** — the link-local address must be enclosed in brackets (`[fd00::1]:53`).
- **Cilium kube-proxy replacement** — Cilium's `socketLB` honors the per-pod redirect to the link-local IP only with the right options; verify with Cilium's NodeLocal DNSCache integration page for the version in use.

## Step 6 — ExternalDNS and the boundary

`ExternalDNS` runs in the cluster but operates on cloud DNS (Route 53, Cloud DNS, Azure DNS, OCI DNS) — it is the bridge between Kubernetes Service / Ingress / Gateway and externally reachable hostnames.

Architecture review checks:

- The IAM/role binding ExternalDNS uses must be scoped to the specific hosted zone(s) and the record types it actually creates (typically A, AAAA, CNAME, TXT). A `*` permission is over-scoped.
- Ownership records (TXT) prevent two clusters fighting over the same hostname. Without them, two ExternalDNS deployments will continually overwrite each other's records — silent ping-pong.
- TTL: the default may be high (300s); for blue/green or canary cutovers, a low TTL on the routed records is required, set per-resource via the `external-dns.alpha.kubernetes.io/ttl` annotation.

This boundary is also where this skill's scope ends — DNS *outside* the cluster is the AWS / Azure / OCI network architect's territory.

## Output for this section

- CoreDNS topology, Corefile critical-plugin checks (cache, loop, reload, kubernetes plugin mode),
- replica count and autoscaler posture,
- `ndots:5` exposure for the workloads in scope and the per-pod `dnsConfig.options` plan,
- NodeLocal DNSCache presence, memory headroom, PDB, and OOM exposure,
- ExternalDNS scope, ownership-TXT posture, TTL hygiene,
- findings, severity, and the next-step delegate (cloud-network-architect agent for hosted-zone scoping).
