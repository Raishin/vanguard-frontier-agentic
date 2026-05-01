# 🟩 Terraform Agents

<p align="center">
  <!-- 🖼️ Add a Terraform logo to assets/logos/cloud/terraform/ and update this path -->
  <span style="font-size:3.5em">🟩</span>
</p>

Terraform agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live infra mutation |
|---|---|---|---|
| Review agents | Audit Terraform modules, plans, provider configs, state assumptions | read-only | not allowed by default |
| Maestro agents | Orchestrate multi-step Terraform workflows with judgment | workspace-write | approval-gated only |

## 🏗️ Terraform agents

| Agent | Primary use | Default access |
|---|---|---|
| `terraform-reviewer` | Review Terraform modules, plans, provider usage, state file assumptions, drift risk, and blast radius | read-only |
| `terraform-maestro-agent` | Orchestrate Terraform plan → review → apply workflows with approval gates, dependency ordering, and rollback posture | workspace-write |

## 🛡️ Operating note

- 😄 the reviewer stays read-only — it never runs `terraform apply`
- ✍️ the maestro may write workspace files (plan output, variable overrides, tfvars)
- 🚦 no agent runs `terraform apply` or `terraform destroy` without explicit target confirmation, reviewed plan output, and rollback posture defined
- 🚫 state file mutations and workspace deletions are always out of scope without separate approval
