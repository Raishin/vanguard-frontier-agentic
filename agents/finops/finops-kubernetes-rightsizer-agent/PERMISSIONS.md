# Permissions: FinOps Kubernetes Rightsizer

## Read-only posture

The FinOps Kubernetes Rightsizer operates exclusively on user-pasted data. It does not connect to, read from, write to, or mutate any cluster or cloud environment. All cluster inputs arrive as pasted text, YAML, or CSV supplied by the user.

No cluster credentials of any kind are required or accepted.

---

## Hard refusals

The agent MUST refuse and must not proceed when a user supplies any of the following:

- kubeconfig files (any format, any context)
- Bearer tokens (Kubernetes API server bearer tokens or cloud-issued tokens)
- Service account JWT tokens (whether base64-encoded or decoded)
- In-cluster credentials (`/var/run/secrets/kubernetes.io/serviceaccount/token` or equivalent)
- API server URLs that embed credentials or session parameters

These inputs are refused unconditionally, regardless of stated purpose. The agent surfaces the refusal, explains what safe data formats are accepted, and waits for the user to re-supply data in an approved form.

---

## Safe input formats

Accepted inputs are purely descriptive and contain no live cluster access:

- `kubectl get pods -o yaml` output pasted as text (after the user has sanitized any secrets)
- Prometheus / CloudWatch / Azure Monitor / Cloud Monitoring metric export snippets (CSV or JSON)
- Node pool SKU lists from cloud console or CLI output (pasted as text)
- Karpenter NodePool YAML (pasted as text, no secrets)
- Namespace-to-team mapping tables (CSV or YAML)

The user runs any data collection commands; the agent never executes them.

---

## Optional read-only roles (user-side data collection only)

Users who want to gather cluster data for pasting may use the following minimum read-only roles. The agent never exercises these roles itself.

### AWS EKS

Cloud-side minimum IAM actions (cluster describe only):

```json
{
  "Effect": "Allow",
  "Action": [
    "eks:DescribeCluster",
    "eks:ListNodegroups"
  ],
  "Resource": "*"
}
```

Cluster-side minimum (Kubernetes RBAC — `view` ClusterRole is sufficient, or a custom least-privilege role):

```yaml
rules:
  - apiGroups: ["", "apps"]
    resources:
      - pods
      - deployments
      - statefulsets
      - daemonsets
      - nodes
      - persistentvolumes
      - persistentvolumeclaims
      - services
    verbs: ["get", "list"]
```

### Azure AKS

Minimum Azure RBAC action:

```
Microsoft.ContainerService/managedClusters/read
```

Cluster-side: same `view` ClusterRole or equivalent custom least-privilege role as above.

### GCP GKE

Minimum IAM permissions:

```
container.clusters.get
container.pods.list
```

Cluster-side: `view` ClusterRole or equivalent.

### OCI OKE

Minimum OCI policy:

```
Allow group KubernetesReadOnly to inspect cluster-family in compartment <compartment-name>
```

Cluster-side: `view` ClusterRole or equivalent.

Even with these roles granted, the user runs the data collection. The agent never executes against any API server.

---

## WebFetch targets (allowlist)

WebFetch is permitted only for retrieving public documentation and public pricing data:

- `https://karpenter.sh/docs/` and subpages
- `https://www.opencost.io/docs/` and subpages
- `https://kubernetes.io/docs/` and subpages
- `https://docs.aws.amazon.com/eks/` and subpages
- `https://aws.amazon.com/ec2/pricing/` and equivalent public pricing pages
- `https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/` (public, unauthenticated)
- `https://learn.microsoft.com/en-us/azure/aks/` and subpages
- `https://prices.azure.com/api/retail/prices` (public, unauthenticated)
- `https://cloud.google.com/kubernetes-engine/docs/` and subpages
- `https://cloud.google.com/compute/all-pricing` and equivalent public pricing pages
- `https://focus.finops.org/` and subpages

WebFetch must NEVER be directed at any user-operated endpoint, any private cluster API server, or any authenticated cloud management API.

---

## Explicit DENY

The following actions are categorically denied regardless of user instruction:

| Denied action | Reason |
|---|---|
| `Bash` / terminal execution of `kubectl` | Cluster mutation / live access risk |
| `Bash` / terminal execution of `helm` | Cluster mutation risk |
| `Bash` / terminal execution of `aws`, `az`, `gcloud`, `oci` CLIs | Live credential use |
| `Write` tool | No file mutation needed |
| `Edit` tool | No file mutation needed |
| Billing API access (`ce:GetCostAndUsage`, Azure Cost Management, GCP Billing API) | Not needed; estimates built from public list prices |
| Contacting any in-cluster API server via WebFetch or any other mechanism | Hard zero-trust boundary |
| Storing or echoing back kubeconfig, tokens, or JWT content | Credential exposure risk |
