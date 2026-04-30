# Permission Model: OCI Live OKE Rollout Guard

## 3-tier IAM separation

| Tier | Group/Principal | Verb | Scope |
|------|----------------|------|-------|
| Audit | `<oke-auditors>` | read | `<prod-compartment>` |
| Operator | `<oke-operators>` | read + use devops | `<prod-compartment>` |
| Admin | `<oke-admins>` | use cluster + manage node-pools | `<prod-compartment>` |
| Pipeline | `<devops-pipeline-runners>` (dynamic group) | use cluster + manage node-pools | `<prod-compartment>` |

## OKE cluster read (no deploy rights)

```
Allow group <oke-auditors> to read clusters in compartment <prod-compartment>
Allow group <oke-auditors> to read cluster-node-pools in compartment <prod-compartment>
```

## DevOps pipeline read + deployment use

```
Allow group <oke-operators> to read devops-pipelines in compartment <prod-compartment>
Allow group <oke-operators> to read devops-deployments in compartment <prod-compartment>
Allow group <oke-operators> to use devops-deployments in compartment <prod-compartment>
```

## OKE admin (use, NOT manage — cannot delete clusters)

```
Allow group <oke-admins> to use clusters in compartment <prod-compartment>
Allow group <oke-admins> to manage cluster-node-pools in compartment <prod-compartment>
```

## DevOps pipeline dynamic group

```
Allow dynamic-group <devops-pipeline-runners> to use cluster in compartment <prod-compartment>
Allow dynamic-group <devops-pipeline-runners> to manage cluster-node-pools in compartment <prod-compartment>
```

`use cluster` (not `manage cluster`) for the pipeline: `manage` grants cluster termination rights.

## Service-principal policies (required for OKE and DevOps services)

```
Allow service OKE to manage cluster-node-pools in compartment <prod-compartment>
Allow service OKE to use virtual-network-family in compartment <prod-compartment>
Allow service OKE to manage instance-family in compartment <prod-compartment>
  where target.resource.tag.Operations.OkeManaged.value = 'true'

Allow service devops to use ons-topics in compartment <prod-compartment>
Allow service devops to manage repos in compartment <prod-compartment>
Allow service devops to read secret-family in compartment <prod-compartment>
```

The `OkeManaged = 'true'` tag prevents the OKE service principal from acting on
instances outside of managed node pools.

## Do not use

```
# FORBIDDEN
Allow group <oke-operators> to manage clusters in compartment prod
Allow dynamic-group <all-instances> to manage all-resources in compartment prod
```

## Kubernetes RBAC (in-cluster, namespace-scoped)

```yaml
rules:
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch", "patch", "update"]
- apiGroups: [""]
  resources: ["pods", "pods/log", "services"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["policy"]
  resources: ["poddisruptionbudgets"]
  verbs: ["get", "list"]
```
