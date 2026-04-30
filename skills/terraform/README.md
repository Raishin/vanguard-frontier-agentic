# Terraform Skills

This folder contains Terraform-focused skills curated for this marketplace.

## Local marketplace portfolio

As of **2026-04-30**, this folder contains **6** local Terraform skills:

- `terraform-reviewer`
- `terraform-repo-patch-operator`
- `terraform-live-apply-guard`
- `terraform-azurerm-set-diff-analyzer` ← Azure variant
- `terraform-oci-set-diff-analyzer` ← OCI variant
- `terraform-aws-inline-diff-analyzer` ← AWS variant

## Portfolio posture

These Terraform skills are intentionally separated into three trust tiers plus specialized provider analyzers:

- advisory review,
- repo-write execution,
- guarded live operation,
- Terraform AzureRM plan-noise analysis,
- Terraform OCI plan-noise analysis,
- Terraform AWS inline-diff analysis.

That separation matters because `terraform plan` is not `terraform apply`, repo edits are not live state mutation, CLI workspaces are not strong environment isolation, AzureRM set-order churn is not automatically a real change, OCI repeated-block churn should be treated as heuristic evidence unless corroborated, and AWS inline collection churn can hide real provider caveats rather than harmless reorder noise.

Use official Terraform docs and Context7 when Terraform CLI behavior, state locking, backend semantics, workspaces, saved plans, or approval flow matter. Use provider and service-domain docs for AzureRM, OCI, and AWS when nested resource collections are the source of noise.

The three provider-specific diff analyzers are not just statically cataloged. They are now backed by fixture plans that prove:

- an order-only or collection-reflow case,
- a real semantic change case,
- expected exit-code behavior for CI usage.
