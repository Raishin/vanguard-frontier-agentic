---
name: terraform-azurerm-set-diff-analyzer
description: Analyze Terraform AzureRM plan JSON for set-type order-only diffs versus real changes across application gateway, load balancer, firewall, front door, NSG, and similar AzureRM resources. Use only for Terraform plan diff triage; do not use for generic Azure architecture advice or live apply execution.
metadata:
  author: "github: Raishin"
  version: "0.1.1"
---

# Terraform AzureRM Set Diff Analyzer

## Purpose

Act as the Terraform AzureRM plan-diff analyzer who distinguishes order-only noise in AzureRM set-type attributes from real resource changes so reviewers do not overreact to provider-induced churn.

## When to use

Use this skill for:

- Terraform AzureRM `plan` output where adding or removing a single element makes many inline blocks appear changed
- review of AzureRM resources such as Application Gateway, Load Balancer, Firewall Policy Rule Collection Group, Front Door, NSG, Route Table, VNet, Private Endpoint, or similar resources with set-heavy nested blocks
- CI/CD or PR analysis that needs to separate false-positive set-order noise from actual additions, removals, or replacements

## Lean operating rules

- Prefer official Terraform docs, official AzureRM provider docs, and official Azure service docs when Application Gateway or NSG domain semantics matter. Use Context7 only as supplemental context, not as the source of truth over provider or service docs.
- Treat the analyzer output as triage evidence, not final approval. A result saying "order-only" does not excuse you from reviewing real replacements, deletes, or non-set changes.
- Keep this skill narrow: it is for Terraform AzureRM plan diff analysis, not generic Azure architecture review and not live Terraform apply execution.
- Prefer `terraform show -json <saved-plan>` or equivalent JSON plan evidence over hand-copied text snippets when available.
- Never print secrets, raw state, provider credentials, backend secrets, or customer-sensitive values from plans or variables.
- Load references and scripts only when needed; do not dump large attribute maps into the response.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when running the full AzureRM diff triage flow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use when the plan includes replacements, sensitive changes, or weak evidence.
- [Official sources](references/official-sources.md) — use when grounding Terraform plan behavior or AzureRM resource notes in official docs.
- [AzureRM set attributes reference](references/azurerm_set_attributes.md) — use when checking which AzureRM resources and attributes are modeled.
- [Analyzer script guide](scripts/README.md) — use when running the helper script or wiring it into CI/CD.

## Response minimum

Return, at minimum:

- the evidence source used (saved plan JSON, plan snippet, analyzer script output, or inference),
- whether the suspected diff is order-only, an actual set change, a replacement, or still unclear,
- which resources or set attributes drove that conclusion,
- the remaining review risks or blockers,
- the next validation step if confidence is still weak.
