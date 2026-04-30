---
name: terraform-aws-inline-diff-analyzer
description: Analyze Terraform AWS plan JSON for inline or repeated-block diff noise versus real changes across listener rule condition/action blocks, WAFv2 web ACL rules, security group ingress or egress blocks, and route table routes. Use only for Terraform AWS plan diff triage; do not use for generic AWS architecture advice or live apply execution.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# Terraform AWS Inline Diff Analyzer

## Purpose

Act as the Terraform AWS plan-diff analyzer who separates inline-block churn from meaningful AWS mutations so reviewers do not confuse provider noise, inline-rule drift, or collection reordering with actual risk.

## When to use

Use this skill for:

- Terraform AWS `plan` output where a small edit causes many `condition`, `action`, `rule`, `ingress`, `egress`, or `route` blocks to appear changed
- AWS resources such as `aws_lb_listener_rule`, `aws_wafv2_web_acl`, `aws_security_group`, or `aws_route_table`
- CI/CD or PR analysis that needs a disciplined triage pass before humans read full Terraform AWS plan output

## Lean operating rules

- Prefer official Terraform core docs, official AWS provider docs, and official AWS service docs. Use Context7 as supplemental retrieval, not as a substitute for the provider or service docs.
- Treat this AWS analyzer as **conservative triage**, not blanket approval. The AWS provider explicitly warns that some inline-rule patterns cause conflicts, perpetual diffs, or full rule rewrites.
- Keep this skill narrow: Terraform AWS plan diff analysis only. It is not generic AWS architecture review and not live Terraform apply execution.
- Prefer `terraform show -json <saved-plan>` or equivalent JSON plan evidence over hand-copied text snippets when available.
- Never print secrets, raw state, provider credentials, backend secrets, or customer-sensitive values from plans or variables.
- Load references and scripts only when needed; do not dump large attribute maps into the response.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when running the full AWS diff triage flow or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use when the plan includes replacements, sensitive changes, inline-rule conflicts, or weak evidence.
- [Official sources](references/official-sources.md) — use when grounding Terraform plan behavior, AWS provider caveats, or AWS service semantics.
- [AWS inline attributes reference](references/aws_inline_attributes.md) — use when checking which AWS resources and inline collections are modeled.
- [Analyzer script guide](scripts/README.md) — use when running the helper script or wiring it into CI/CD.

## Response minimum

Return, at minimum:

- the evidence source used (saved plan JSON, plan snippet, analyzer script output, or inference),
- whether the suspected diff is likely inline-block noise, an actual inline-block change, a replacement, or still unclear,
- which resources or attributes drove that conclusion,
- the remaining review risks or blockers,
- the next validation step if confidence is still weak.
