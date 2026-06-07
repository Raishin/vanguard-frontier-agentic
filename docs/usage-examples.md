---
layout: default
title: "Usage Examples"
permalink: /docs/usage-examples/
---

# 🧭 Usage Examples

Real-world patterns for using maestro agents, live-guard agents, and role-based installs across providers.

---

## Maestro Agents — Entry Points

Every provider ships a **maestro agent** — the routing entry point that classifies your request and dispatches to the correct specialist. You never invoke specialists directly; the maestro handles selection.

### Install a maestro

```bash
# AWS maestro (routes across 47 AWS specialists)
npx vfa-export-agents --platform claude-code --agents aws-maestro-agent --repo .

# Kubernetes maestro (routes across 16 K8s specialists)
npx vfa-export-agents --platform claude-code --agents kubernetes-maestro-agent --repo .

# OCI maestro (routes across 39 OCI specialists)
npx vfa-export-agents --platform codex --agents oci-maestro-agent --repo .

# Salesforce maestro (routes across 30 Salesforce specialists)
npx vfa-export-agents --platform cursor --agents salesforce-maestro-agent --repo .

# Legal maestro (routes across 13 Legal specialists)
npx vfa-export-agents --platform claude-code --agents legal-maestro-agent --repo .
```

### How routing works

Once installed, you talk to the maestro naturally:

```
You: "Review my S3 bucket policies for public access"
→ AWS Maestro routes to: aws-s3-security-agent (companion skill: aws-s3-security)

You: "Check if my Kubernetes RBAC roles are over-scoped"
→ Kubernetes Maestro routes to: kubernetes-rbac-review-agent

You: "Review this employment termination package"
→ Legal Maestro routes to: legal-employment-law-risk-specialist-agent

You: "Audit our Salesforce field-level security"
→ Salesforce Maestro routes to: salesforce-security-identity-access-agent
```

The maestro **never auto-dispatches** to live-guard agents. Any live mutation requires explicit human confirmation.

---

## Role-Based Installs — Get a Curated Set

Roles install a curated team of agents for a practitioner function:

```bash
# Cloud security engineer — IAM, RBAC, network security across all providers
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --repo .

# Cloud FinOps — cost optimization, budgets, reserved instances
npx vfa-export-agents --platform claude-code --role cloud-finops-analyst --repo .

# Kubernetes network engineer — CNI, service mesh, network policies
npx vfa-export-agents --platform claude-code --role kubernetes-network-engineer --repo .

# Legal + HR risk reviewer — employment law, investigations, compliance
npx vfa-export-agents --platform claude-code --role legal-hr-risk-reviewer --repo .

# Salesforce portfolio architect — full CRM platform review
npx vfa-export-agents --platform claude-code --role salesforce-portfolio-architect --repo .

# .NET application review engineer
npx vfa-export-agents --platform claude-code --role dotnet-application-review-engineer --repo .
```

### Scope a role to one provider

```bash
# Only AWS agents for the cloud-security-engineer role
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --provider aws --repo .

# Only Azure agents for cloud-devops-engineer
npx vfa-export-agents --platform copilot --role cloud-devops-engineer --provider azure --repo .

# Only OCI agents for cloud-platform-engineer
npx vfa-export-agents --platform codex --role cloud-platform-engineer --provider oci --repo .
```

---

## Live-Guard Agents — Least-Privilege Access

Live-guard agents can interact with real infrastructure. They require **explicit setup** of least-privilege access before use.

### The 5-Layer Defense Model

| Layer | Enforces | Bypassable by LLM? |
|-------|----------|---------------------|
| L1 — Prompt rules | Refusal triggers, mandatory pre-mutation checks | Yes |
| L2 — Tool permissions | `allowed-tools` limits what the harness mounts | Partially |
| L3 — Cloud/Cluster RBAC | Principal cannot exceed granted permissions | **No** |
| L4 — Admission control | Kyverno / OPA validates even allowed operations | No |
| L5 — Audit + alert | Detect and surface any mutation attempt | No |

**L3 is the authoritative defense.** The prompt is advisory. The RBAC binding is enforcement.

---

### AWS: Least-Privilege IAM for Live Agents

The AWS live-guard agents (`aws-live-deployment-guarded-operator-agent`, `aws-live-iac-change-guard-agent`, etc.) need an IAM role or user with minimal permissions scoped to exactly what the agent does.

#### Example: IAM Policy for `aws-live-deployment-guarded-operator-agent`

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "ReadOnlyBaseline",
      "Effect": "Allow",
      "Action": [
        "sts:GetCallerIdentity",
        "cloudformation:DescribeStacks",
        "cloudformation:DescribeChangeSet",
        "cloudformation:ListChangeSets",
        "codepipeline:GetPipelineState",
        "codepipeline:ListPipelineExecutions",
        "ssm:GetCalendarState"
      ],
      "Resource": "*"
    },
    {
      "Sid": "DeploymentMutations",
      "Effect": "Allow",
      "Action": [
        "cloudformation:ExecuteChangeSet",
        "codepipeline:EnableStageTransition",
        "codepipeline:DisableStageTransition",
        "codepipeline:PutApprovalResult"
      ],
      "Resource": [
        "arn:aws:cloudformation:*:ACCOUNT_ID:stack/YOUR-STACK-PREFIX-*",
        "arn:aws:codepipeline:*:ACCOUNT_ID:YOUR-PIPELINE-NAME"
      ]
    }
  ]
}
```

**Key principles:**
- Read-only actions are broad (describe, list, get) — they reveal state without changing it
- Mutation actions are resource-scoped by ARN pattern — cannot affect other stacks/pipelines
- No `*` on mutation actions
- No `iam:*`, `organizations:*`, or `s3:DeleteBucket` ever

#### Example: IAM Policy for `aws-live-serverless-release-guard-agent`

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "LambdaReadOnly",
      "Effect": "Allow",
      "Action": [
        "lambda:GetFunction",
        "lambda:GetFunctionConfiguration",
        "lambda:ListAliases",
        "lambda:ListVersionsByFunction",
        "lambda:GetAlias"
      ],
      "Resource": "arn:aws:lambda:*:ACCOUNT_ID:function:YOUR-PREFIX-*"
    },
    {
      "Sid": "LambdaDeployMutations",
      "Effect": "Allow",
      "Action": [
        "lambda:UpdateAlias",
        "lambda:PublishVersion"
      ],
      "Resource": "arn:aws:lambda:*:ACCOUNT_ID:function:YOUR-PREFIX-*"
    }
  ]
}
```

---

### Kubernetes: ServiceAccount + RBAC for Live Agents

Kubernetes live-guard agents need a `ServiceAccount` with a scoped `Role` (namespace) or `ClusterRole`.

#### Example: RBAC for `kubernetes-rbac-review-agent` (read-only)

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: vfa-rbac-review
  namespace: vanguard-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: vfa-rbac-review
rules:
  - apiGroups: ["rbac.authorization.k8s.io"]
    resources: ["roles", "clusterroles", "rolebindings", "clusterrolebindings"]
    verbs: ["get", "list"]
  - apiGroups: [""]
    resources: ["serviceaccounts", "namespaces"]
    verbs: ["get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: vfa-rbac-review
subjects:
  - kind: ServiceAccount
    name: vfa-rbac-review
    namespace: vanguard-system
roleRef:
  kind: ClusterRole
  name: vfa-rbac-review
  apiGroup: rbac.authorization.k8s.io
```

**What this allows:** Read RBAC objects cluster-wide. Cannot create, update, or delete anything.

#### Example: RBAC for a Cilium live-guard (namespace-scoped mutation)

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: vfa-cilium-guard
  namespace: kube-system
rules:
  - apiGroups: ["cilium.io"]
    resources: ["ciliumnetworkpolicies"]
    verbs: ["get", "list", "patch"]
    # No "delete", no "create" — only patch existing policies
  - apiGroups: [""]
    resources: ["pods"]
    verbs: ["get", "list"]
    # Read-only on pods for status correlation
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: vfa-cilium-guard
  namespace: kube-system
subjects:
  - kind: ServiceAccount
    name: vfa-cilium-guard
    namespace: vanguard-system
roleRef:
  kind: Role
  name: vfa-cilium-guard
  apiGroup: rbac.authorization.k8s.io
```

**What this allows:** Read pods and Cilium policies in `kube-system`. Patch (update) existing Cilium policies only. Cannot delete policies, cannot affect other namespaces.

---

### OCI: IAM Policy for Live Agents

```hcl
# Least-privilege policy for OCI live infrastructure review
resource "oci_identity_policy" "vfa_live_review" {
  compartment_id = var.tenancy_ocid
  name           = "vfa-live-infrastructure-review"
  description    = "Read-only access for VFA live-guard agents"
  statements = [
    "Allow group VFA-LiveAgents to inspect all-resources in compartment ${var.compartment_name}",
    "Allow group VFA-LiveAgents to read instances in compartment ${var.compartment_name}",
    "Allow group VFA-LiveAgents to read vcns in compartment ${var.compartment_name}",
    "Allow group VFA-LiveAgents to read subnets in compartment ${var.compartment_name}",
    "Allow group VFA-LiveAgents to read security-lists in compartment ${var.compartment_name}",
  ]
}
```

**Key:** `inspect` gives metadata only. `read` gives full object data but no mutation. No `manage` or `use` verbs.

---

### Azure: Custom Role for Live Agents

```json
{
  "Name": "VFA Live Review Reader",
  "Description": "Read-only access for VFA live-guard agents",
  "Actions": [
    "Microsoft.Compute/virtualMachines/read",
    "Microsoft.Network/networkSecurityGroups/read",
    "Microsoft.Network/virtualNetworks/read",
    "Microsoft.Authorization/roleAssignments/read",
    "Microsoft.Authorization/roleDefinitions/read",
    "Microsoft.Resources/deployments/read"
  ],
  "NotActions": [],
  "AssignableScopes": ["/subscriptions/YOUR-SUBSCRIPTION-ID/resourceGroups/YOUR-RG"]
}
```

**Key:** Only `/read` actions. Scoped to one resource group. No write, delete, or `*/write` actions.

---

## Structured Verdict Response

Every live-guard and review agent produces a structured response with 5 required fields:

```json
{
  "verdict": "blocked",
  "evidence_level": "verified",
  "blockers": [
    "IAM role has sts:AssumeRole with Resource:* — must scope to specific role ARNs",
    "No MFA condition on the assume-role trust policy"
  ],
  "safe_next_actions": [
    "Add Condition: aws:MultiFactorAuthPresent = true",
    "Replace Resource:* with arn:aws:iam::ACCOUNT:role/specific-role-name",
    "Re-run review after scoping changes"
  ],
  "open_questions": [
    "Is cross-account access intentional? If yes, which accounts?"
  ]
}
```

| Field | Meaning |
|-------|---------|
| `verdict` | `approved`, `blocked`, or `needs-review` |
| `evidence_level` | `verified` (live state confirmed), `partial` (stale snapshot), `assumed` (no capture) |
| `blockers` | Named violations that must be fixed |
| `safe_next_actions` | Ordered remediation or verification steps |
| `open_questions` | Ambiguities requiring human input |

---

## Pre-Flight Checklist for Live Agents

Before using any live-guard agent:

1. **Create a dedicated principal** — ServiceAccount (K8s), IAM role (AWS), custom role (Azure), policy group (OCI)
2. **Scope to minimum verbs** — Read-only for review agents, minimal mutation for guard agents
3. **Scope to minimum resources** — Namespace, resource group, compartment, or ARN pattern
4. **Never use cluster-admin / AdministratorAccess / Owner** — The agent inherits whatever the principal can do
5. **Test with dry-run first** — The agent's prompt enforces preview-before-mutate, but RBAC is the real defense
6. **Enable audit logging** — CloudTrail (AWS), Kubernetes audit log, Azure Activity Log, OCI Audit
7. **Set up alerts** — Any mutation by the agent principal should trigger a notification

---

## Further Reading

- [Least-Privilege RBAC](../least-privilege-rbac/) — Full Kubernetes RBAC contract for live agents
- [Evidence Output Spec](../evidence-output-spec/) — Complete response shape documentation
- [CI/CD Enforcement Pattern](../ci-cd-enforcement-pattern/) — Run agents in pipelines without developer opt-in
