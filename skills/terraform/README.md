# 🟩 Terraform Skills

<p align="center">
  <!-- 🖼️ Add a Terraform logo to assets/logos/cloud/terraform/ and update this path -->
  <span style="font-size:3.5em">🟩</span>
</p>

This folder contains Terraform-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local Terraform skill:

- `terraform-maestro`

## Portfolio posture

Terraform skills for evidence-backed IaC review, plan safety, and guarded apply workflows across all cloud providers.

These skills are intentionally conservative:

- always review `terraform plan` output before any apply — never apply without a human-reviewed plan
- assess blast radius: count resource deletions, replacements, and modifications before approving
- check for missing `prevent_destroy` lifecycle rules on stateful resources (databases, buckets, vaults)
- verify backend state locking is enabled before any write operation
- flag remote state outputs consumed by other stacks — changes may break downstream consumers
- use official Terraform and provider documentation for resource behavior and provider version compatibility

Run `npm run validate` after changing cataloged Terraform skills.
