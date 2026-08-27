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

# NetSuite maestro (routes across 25 NetSuite specialists)
npx vfa-export-agents --platform claude-code --agents netsuite-maestro-agent --repo .

# Legal maestro (routes across 13 Legal specialists)
npx vfa-export-agents --platform claude-code --agents legal-maestro-agent --repo .

# SAP maestro (routes SAP S/4HANA, BTP, Integration Suite, security/GRC specialists)
npx vfa-export-agents --platform claude-code --agents sap-maestro-agent --repo .

# Microsoft maestro (top-level router across all M365 / D365 / Power Platform / Copilot / Fabric specialists)
npx vfa-export-agents --platform claude-code --agents microsoft-maestro-agent --repo .

# Or install a focused sub-maestro for one Microsoft estate:
npx vfa-export-agents --platform claude-code --agents m365-maestro-agent --repo .              # Microsoft 365
npx vfa-export-agents --platform claude-code --agents d365-maestro-agent --repo .              # Dynamics 365
npx vfa-export-agents --platform claude-code --agents power-platform-maestro-agent --repo .    # Power Platform
npx vfa-export-agents --platform claude-code --agents copilot-governance-maestro-agent --repo . # Copilot governance
```

> **Tip:** install `microsoft-maestro-agent` for one entry point across the whole Microsoft estate, or a sub-maestro (`m365`, `d365`, `power-platform`, `copilot-governance`) when your work stays within one product family. The top-level Microsoft maestro deflects cross-cloud requests (AWS/Azure-IaaS/GCP) back to the right provider maestro instead of guessing.

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

You: "Review our SuiteScript for injection risks"
→ NetSuite Maestro routes to: netsuite-suitescript-secure-code-review-agent

You: "Review our ABAP custom code against clean core before the S/4HANA upgrade"
→ SAP Maestro routes to: sap-clean-core-debt-reviewer-agent (single specialist)

You: "Inventory our BTP subaccounts and entitlements for audit" (read-only)
→ SAP Maestro routes to: sap-live-readonly-landscape-discovery-agent

You: "Import transport SAPK-90021 into PRD tonight"
→ SAP Maestro HOLDS at live-guard gate — sap-guarded-transport-import-operator-agent
   requires: named approver + change ticket + dry-run output + SoD check + rollback plan
   The maestro never auto-dispatches this agent; explicit human confirmation is mandatory.

You: "Are we ready to turn on Microsoft 365 Copilot without oversharing?"
→ Microsoft Maestro → M365 Maestro routes to: m365-copilot-readiness-governance-agent

You: "Design a least-privilege Dataverse security model for our new D365 rollout"
→ Microsoft Maestro → D365 Maestro routes to: power-platform-governance-dataverse-security-agent
```

The maestro **never auto-dispatches** to live-guard agents. Any live mutation requires explicit human confirmation.

### Applying the maestro pattern to adjacent suites (M365 & D365)

The SAP board's three-tier model — advisory / read-only-live / guarded-mutating-live,
with a non-negotiable live-guard gate on all mutations — is a reusable architecture
pattern. The examples below illustrate how the same model would apply to Microsoft 365
(M365) and Dynamics 365 (D365) boards. **These providers are not yet shipped in this
catalog.** The examples are suggested usage patterns for teams who later add M365 or
D365 boards following the SAP board's structure.

#### Microsoft 365 (M365)

**Advisory (no live access):**

```
Operator: "Review our Exchange Online transport rules for data-loss exposure"
→ M365 Maestro routes to: m365-exchange-transport-rule-reviewer-agent (advisory)

Operator: "Assess our Purview DLP policy posture against GDPR obligations"
→ M365 Maestro routes to: m365-purview-dlp-compliance-advisor-agent (advisory)
```

**Read-only live (discovery, no mutation):**

```
Operator: "Inventory Entra ID conditional-access policies and privileged role assignments for audit"
→ M365 Maestro routes to: m365-live-readonly-entra-identity-discovery-agent
  (read-only; connects with least-privilege Graph API reader credentials)
```

**Guarded mutating live (held at gate):**

```
Operator: "Apply the new SharePoint external-sharing policy across all site collections"
→ M365 Maestro HOLDS at live-guard gate — m365-guarded-sharepoint-policy-operator-agent
  requires: named approver + target tenant confirmation + what-if/dry-run output +
            rollback snapshot + post-change verification
```

#### Dynamics 365 (D365)

**Advisory (no live access):**

```
Operator: "Review D365 Finance & Operations security roles for segregation-of-duties conflicts"
→ D365 Maestro routes to: d365-fo-security-role-sod-reviewer-agent (advisory)

Operator: "Assess our D365 CE (CRM) field-level security model for over-permissioned profiles"
→ D365 Maestro routes to: d365-ce-field-security-advisor-agent (advisory)
```

**Read-only live (discovery, no mutation):**

```
Operator: "Inventory D365 CE environments, security roles, and business unit hierarchy for audit"
→ D365 Maestro routes to: d365-live-readonly-environment-discovery-agent
  (read-only; least-privilege Dataverse API access, no write permissions)
```

**Guarded mutating live (held at gate):**

```
Operator: "Deploy the updated managed solution to the D365 F&O production environment"
→ D365 Maestro HOLDS at live-guard gate — d365-guarded-solution-deployment-operator-agent
  requires: named approver + target environment confirmation + dry-run import log +
            rollback restore point + post-deployment smoke-test plan
```

**Invariants that hold across all three suites (SAP, M365, D365):**

- Discovery and mutation are never combined in a single agent.
- Live mutations always require a named approver, explicit target confirmation, a
  dry-run or what-if output, a defined rollback path, and post-change verification.
- The maestro never auto-dispatches a guarded-mutating-live agent.

These are suggested usage patterns for teams who add M365 or D365 boards following
the SAP board's structure.

---

## Microsoft 365 & Dynamics 365 — Maestro Usage Patterns

The Microsoft estate is large, so it ships a **two-level maestro tree**: a
top-level `microsoft-maestro-agent` that classifies intent into a product
family, then a sub-maestro (`m365`, `d365`, `power-platform`,
`copilot-governance`) that routes to the specialist. You talk to the maestro
in plain language — describe the *outcome* you want, not the agent name.

### How to phrase a request

Good prompts name the **estate + the artifact or risk**. The maestro keys off
both:

```
You: "Harden Conditional Access and close legacy-auth gaps in our tenant"
→ m365-identity-zero-trust-agent

You: "Map our Purview sensitivity labels and DLP to where data actually lives"
→ m365-purview-data-security-compliance-agent

You: "Review Intune compliance + app-protection policies before we go BYOD"
→ m365-intune-endpoint-management-agent

You: "Tighten external sharing and guest access across Teams and SharePoint"
→ m365-teams-collaboration-governance-agent  /  m365-exchange-sharepoint-information-governance-agent

You: "Stand up Success by Design governance for our D365 F&O program"
→ d365-success-by-design-governance-agent

You: "Find segregation-of-duties conflicts in our D365 finance security roles"
→ d365-security-sod-governance-agent

You: "Plan the data migration and cutover for our legacy ERP into D365"
→ d365-data-migration-cutover-agent

You: "Review our Power Platform ALM pipelines and environment strategy"
→ power-platform-alm-pipelines-agent

You: "Govern the Copilot Studio agents our makers are building"
→ copilot-studio-agent-governance-alm-agent

You: "Review our Microsoft Fabric medallion + Direct Lake design"
→ fabric-data-engineering-agent  /  fabric-analytics-engineering-agent
```

### End-to-end workflow examples

**1) "Get us ready for Microsoft 365 Copilot."**

```
You: "We want to enable M365 Copilot for 500 users. What do we fix first?"

Microsoft Maestro → Copilot Governance Maestro →
  • m365-copilot-readiness-governance-agent  → oversharing / SharePoint access sprawl, label coverage
  • m365-purview-data-security-compliance-agent → sensitivity labels + DLP gaps feeding Copilot
  • m365-identity-zero-trust-agent → Conditional Access + least-privilege admin roles

Each returns a structured verdict (blockers + safe_next_actions). You remediate,
then re-run. No tenant change is made by the agents — they review and propose.
```

**2) "Stand up a new D365 implementation safely."**

```
You: "We're starting a D365 Finance & Operations implementation."

Microsoft Maestro → D365 Maestro →
  • d365-success-by-design-governance-agent → program/workstream governance gates
  • power-platform-governance-dataverse-security-agent → least-privilege Dataverse security roles
  • d365-security-sod-governance-agent → SoD conflict matrix for finance roles
  • d365-data-migration-cutover-agent → migration + cutover runbook and rollback
```

**3) "Audit-evidence and value tracking for the board."**

```
You: "Show our M365/D365 control posture and value realization for the steering committee."

Microsoft Maestro →
  • microsoft-business-impact-value-realization-agent → value/KPI mapping
  • cross-functional protocol: audit-evidence-mapping → control → evidence trace
```

### Role-based install for a whole Microsoft team

Instead of installing maestros one by one, install a curated role:

```bash
# Full M365/D365/Power Platform/Copilot/Fabric advisory board (maestros + specialists)
npx vfa-export-agents --platform claude-code --role microsoft-365-d365-platform-advisor --repo .

# Security & compliance focus (identity, Purview, Defender, + the live-guards)
npx vfa-export-agents --platform claude-code --role microsoft-security-compliance-engineer --repo .

# Collaboration & endpoint admin (Teams, Exchange/SharePoint, Intune)
npx vfa-export-agents --platform claude-code --role microsoft-collaboration-endpoint-admin --repo .

# Data & analytics (Fabric, Power BI, + Databricks/Snowflake on Azure)
npx vfa-export-agents --platform claude-code --role microsoft-data-analytics-engineer --repo .
```

### Databricks board roles

The cloud-neutral Databricks board ships nine curated roles. Each installs five
to six agents, not the whole board — pick the one whose decisions you actually
own. See [The Databricks Board](../databricks-board/) for the full contract.

```bash
# Platform: account/workspace topology, identity boundary, bundles, reliability, cost
npx vfa-export-agents --platform claude-code --role databricks-platform-engineer --repo .

# Governance & security: Unity Catalog privileges, identity, masking, residency
npx vfa-export-agents --platform claude-code --role databricks-governance-security-engineer --repo .

# Data engineering: Lakeflow pipelines, streaming recovery, data quality
npx vfa-export-agents --platform claude-code --role databricks-data-engineer --repo .

# Analytics: SQL warehouse performance, AI/BI Genie, metric views
npx vfa-export-agents --platform claude-code --role databricks-analytics-engineer --repo .

# GenAI: agent framework, Databricks AI Search retrieval, MLflow evaluation
npx vfa-export-agents --platform claude-code --role databricks-genai-engineer --repo .

# FinOps: billing system tables, tag attribution, budgets, value realization
npx vfa-export-agents --platform claude-code --role databricks-finops-engineer --repo .
```

Also available: `databricks-solution-architect`, `databricks-ml-engineer`, and
`databricks-sre`. The older `azure-databricks-platform-engineer` role remains for
the Azure-specific `-at-azure` agents.

### When a request leaves the Microsoft estate

The Microsoft maestro **deflects** rather than guesses. "Review my S3 bucket
policy" or "harden my EKS cluster" is handed back so you can route it to the
`aws-maestro-agent` / `kubernetes-maestro-agent`. Azure *IaaS* (VMs, NSGs,
Key Vault) belongs to the `azure` provider maestro; the Microsoft maestro owns
the **M365/D365/Power Platform/Copilot/Fabric** SaaS layer.

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

# NetSuite platform advisor — ERP architecture, finance, compliance, and integration review
npx vfa-export-agents --platform claude-code --role netsuite-platform-advisor --repo .

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

### Microsoft 365 & Dynamics 365: Least-Privilege for Live Agents

The Microsoft live-guards come in two tiers. **Phase A** (`read-only-runtime`)
discovers posture; **Phase B** (`mutating-runtime`) executes one approved,
reversible change. Both are gate-only and never auto-dispatched.

#### D365 / Dataverse — use the DATA PLANE, not the management path

> **Critical:** a service principal added through the Power Platform *management*
> path is treated as a Power Platform Administrator and **cannot be
> least-privileged**. Always bind the app user to a **custom security role on
> the Dataverse data plane** instead.

- **`d365-live-security-role-guard-agent` (Phase A, read-only):** app user bound
  to a custom security role with **Read** on only the tables it inspects (e.g.
  `role`, `roleprivileges`, `systemuser`, `teammembership`). **Denied:** System
  Administrator, System Customizer, and any Write/Create/Delete.
- **`d365-live-record-field-update-guard-agent` (Phase B, mutating):** same app
  user, plus **Write (`prvWrite`) on the ONE in-scope table** — nothing else.
  Each run still requires a written approval token (table + record GUID + named
  fields), a PREFLIGHT GET diff, and an inverse-PATCH rollback. **Denied:** bulk,
  wildcard, `DELETE`, `ownerid` changes, and any security-role edits.

```
# Conceptual: a minimal custom Dataverse security role for the Phase B field-update guard
Role: VFA-FieldUpdate-Guard (data-plane, environment-scoped)
  account            : Read, Write          # the ONE in-scope table only
  (no other tables)  : —
  Denied             : System Administrator, System Customizer, Delete,
                       bulk operations, ownership reassignment
Auth: certificate or managed identity (never a long-lived client secret)
Env vars (names only): DATAVERSE_CLIENT_ID, DATAVERSE_ENV_URL
```

#### M365 / Microsoft Graph — app-only, least scope, admin-consented

- **`m365-live-identity-posture-guard-agent` (Phase A, read-only):** application
  permissions only — prefer `User.ReadBasic.All` over `User.Read.All`, plus the
  narrowest directory/policy *read* scopes. **No write scopes.** Certificate or
  managed identity; never a long-lived client secret.
- **`m365-live-sensitivity-label-apply-guard-agent` (Phase B, mutating):**
  `assignSensitivityLabel` on a driveItem is a **protected, metered** Graph API
  whose documented least-privileged *application* permission is
  **`Files.ReadWrite.All`** (higher-privileged alternative: `Sites.ReadWrite.All`).
  Graph exposes **no** per-item or `Sites.Selected` application scope for this
  API, so blast radius is constrained *outside* the grant — via an app-only
  access policy / RSC or a `Sites.Selected` site-level grant — **plus** the
  guard's one-item approval token, PREFLIGHT current-label capture, and
  re-apply-prior-label rollback. **Denied:** `Sites.ReadWrite.All`,
  `Sites.FullControl.All`, `Directory.ReadWrite.All`,
  `InformationProtectionPolicy.ReadWrite.All`, and bulk labeling.

```
# Conceptual: app registration for the Phase B sensitivity-label guard
Application permissions (admin-consented):
  InformationProtectionPolicy.Read.All      # read/verify the label catalog
  Files.ReadWrite.All                       # documented least-priv APP perm for this API
Compensating controls (no per-item app scope exists for this API):
  app-only access policy / RSC, or Sites.Selected site-level grant
  + one-item written approval token + PREFLIGHT diff + idempotency-keyed attestation
Auth: certificate or managed identity   Env vars (names only): GRAPH_CLIENT_ID, GRAPH_TENANT_ID
```

> See [M365/D365 live-agent IAM least-privilege research](../research/live-agent-iam-least-privilege/) for the full contract and why the Dataverse management path is forbidden.

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
- [Salesforce Portfolio](../salesforce-portfolio/) — Design rationale, routing matrix, and red-team scenarios for the Salesforce provider
- [NetSuite Portfolio](../netsuite-portfolio/) — Design rationale, routing matrix, and red-team scenarios for the NetSuite ERP provider
- [Execution Tiers](../execution-tiers/) — Tier contract including the `mutating-runtime` Phase B live-guard tier
- [M365 field research](../research/m365-field-research/) · [D365 field research](../research/d365-field-research/) — grounding for the Microsoft board
- [M365/D365 live-agent IAM least-privilege](../research/live-agent-iam-least-privilege/) — least-privilege contract for the Microsoft live-guards
- [Databricks & Snowflake on Azure](../research/databricks-snowflake-at-azure/) — grounding for the `-at-azure` data-platform agents
