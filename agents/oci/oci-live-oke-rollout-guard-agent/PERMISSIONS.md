# Permissions: OCI Live OKE Rollout Guard

# OCI IAM policy for OKE rollout guard

## Identity model preference

1. DevOps Service pipeline with explicit approval stage — human must approve before deploy
2. OKE cluster RBAC (Kubernetes-native) for in-cluster operations, not IAM only
3. Separate read-only and deploy-operator groups at compartment scope

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

## OKE admin for rollback (use, NOT manage — cannot delete clusters)

```
Allow group <oke-admins> to use clusters in compartment <prod-compartment>
Allow group <oke-admins> to manage cluster-node-pools in compartment <prod-compartment>
```

## DevOps service dynamic group (pipeline automation)

```
Allow dynamic-group <devops-pipeline-runners> to use cluster in compartment <prod-compartment>
Allow dynamic-group <devops-pipeline-runners> to manage cluster-node-pools in compartment <prod-compartment>
```

`use cluster` (not `manage cluster`) for the pipeline dynamic group: `manage` grants
cluster termination rights, which must never be automated. Node pool management
(`manage cluster-node-pools`) covers rolling updates, scaling, and version upgrades
without exposing cluster deletion.

## Service-principal policies (OKE + DevOps services)

OCI is policy-based IAM: the OKE control plane and the DevOps pipeline service
each need their own `Allow service ...` grants. Without these, node pool scaling
and pipeline execution fail with `NotAuthorized` even when human operators are
correctly scoped.

```
Allow service OKE to manage cluster-node-pools in compartment <prod-compartment>
Allow service OKE to use virtual-network-family in compartment <prod-compartment>
Allow service OKE to manage instance-family in compartment <prod-compartment>
  where target.resource.tag.Operations.OkeManaged.value = 'true'

Allow service devops to use ons-topics in compartment <prod-compartment>
Allow service devops to manage repos in compartment <prod-compartment>
Allow service devops to read secret-family in compartment <prod-compartment>
```

The `OkeManaged = 'true'` tag condition prevents OKE from acting on instances
that are not part of a managed node pool — an extra least-privilege guard on
the service principal itself.

## Do not use

```
# FORBIDDEN
Allow group <oke-operators> to manage clusters in compartment prod
  # "manage" allows cluster termination — use "use" for operators
Allow dynamic-group <all-instances> to manage all-resources in compartment prod
```

## Kubernetes RBAC (in-cluster)

Bind the OKE operator's OCID to a namespace-scoped Role, not ClusterRole:

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

