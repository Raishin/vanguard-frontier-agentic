# Official Sources — Terraform Maestro

Authoritative documentation for routing decisions and verifying IaC agent names.

---

## Agent Catalog

Verify agent IDs against this list before dispatching. Do not invent IDs not listed here.

| Agent ID | Provider | Domain |
|----------|----------|--------|
| `terraform-reviewer` | terraform | review |
| `aws-iac-change-safety-review-agent` | aws | aws-iac |
| `aws-iac-patch-executor-agent` | aws | aws-iac |
| `aws-landing-zone-governor-agent` | aws | aws-iac |
| `azure-landing-zone-architect-agent` | azure | azure-iac |
| `aws-live-iac-change-guard-agent` | aws | live-guard |
| `azure-live-arm-deployment-stack-guard-agent` | azure | live-guard |
| `oci-live-resource-manager-stack-guard-agent` | oci | live-guard |

---

## Terraform Official Docs

- Language reference: `https://developer.hashicorp.com/terraform/language`
- CLI commands: `https://developer.hashicorp.com/terraform/cli/commands`
- Plan: `https://developer.hashicorp.com/terraform/cli/commands/plan`
- Apply: `https://developer.hashicorp.com/terraform/cli/commands/apply`
- State: `https://developer.hashicorp.com/terraform/language/state`
- Modules: `https://developer.hashicorp.com/terraform/language/modules`
- Backends: `https://developer.hashicorp.com/terraform/language/settings/backends`
- Provider registry: `https://registry.terraform.io`

## AWS Provider

- AWS provider docs: `https://registry.terraform.io/providers/hashicorp/aws/latest/docs`
- CloudFormation: `https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/`
- CDK: `https://docs.aws.amazon.com/cdk/v2/guide/`
- Control Tower: `https://docs.aws.amazon.com/controltower/latest/userguide/`

## Azure Provider

- AzureRM provider: `https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs`
- ARM templates: `https://learn.microsoft.com/en-us/azure/azure-resource-manager/templates/`
- Bicep: `https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/`
- Azure Landing Zone: `https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/`

## OCI Provider

- OCI provider: `https://registry.terraform.io/providers/oracle/oci/latest/docs`
- Resource Manager: `https://docs.oracle.com/en-us/iaas/Content/ResourceManager/Concepts/resourcemanager.htm`
- OCI Terraform examples: `https://github.com/oracle-devrel/terraform-oci-oracle-cloud-foundation`

---

## Grounding Rule

Verify Terraform resource types, provider arguments, and CLI flags against official docs before routing. Do not dispatch to agent IDs not in the catalog table above.
