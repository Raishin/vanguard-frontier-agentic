---
name: terraform-oci-set-diff-analyzer
description: Analyze Terraform OCI plan JSON for repeated-block diff noise versus real changes across route tables, load balancer resources, routing policies, path route sets, and similar OCI resources. Use only for Terraform OCI plan diff triage; do not use for generic OCI architecture advice or live apply execution.
metadata:
  author: "github: Raishin"
  version: "0.1.1"
---

# Terraform OCI Set Diff Analyzer

## Purpose

Act as the Terraform OCI plan-diff analyzer who separates repeated-block noise from real resource changes so reviewers do not confuse provider churn with meaningful OCI mutations.

## When to use

Use this skill for:

- Terraform OCI `plan` output where many repeated blocks appear changed after a small edit
- OCI route table, load balancer, routing policy, path-route, or similar resources with repeated nested blocks
- CI/CD or PR analysis that needs a faster triage pass before humans read full OCI Terraform plan output

## Lean operating rules

- Prefer official Terraform docs, official OCI provider docs, and OCI API/CLI help for route-table and load-balancer resource families. Use Context7 only as supplemental context.
- Treat this OCI analyzer as **heuristic triage**, not absolute truth. OCI provider docs do not document this pattern as explicitly as the AzureRM variant does.
- Keep this skill narrow: Terraform OCI plan diff analysis only. It is not generic OCI architecture review and not live Terraform apply execution.
- Prefer `terraform show -json <saved-plan>` or equivalent JSON plan evidence over hand-copied text snippets when available.
- Never print secrets, raw state, provider credentials, backend secrets, or customer-sensitive values from plans or variables.
- Load references and scripts only when needed; do not dump large attribute maps into the response.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when running the full OCI diff triage flow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use when the plan includes replacements, sensitive changes, or weak evidence.
- [Official sources](references/official-sources.md) — use when grounding Terraform plan behavior or OCI resource notes in official docs.
- [OCI repeated-block reference](references/oci_set_attributes.md) — use when checking which OCI resources and attributes are modeled.
- [Analyzer script guide](scripts/README.md) — use when running the helper script or wiring it into CI/CD.

## Response minimum

Return, at minimum:

- the evidence source used (saved plan JSON, plan snippet, analyzer script output, or inference),
- whether the suspected diff is likely repeated-block noise, an actual repeated-block change, a replacement, or still unclear,
- which resources or attributes drove that conclusion,
- the remaining review risks or blockers,
- the next validation step if confidence is still weak.
