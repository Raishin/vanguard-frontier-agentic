# Terraform AWS Inline Diff Analyzer Script

A Python script that analyzes Terraform plan JSON and identifies likely inline or repeated-block diff noise in selected AWS provider resources.

## Overview

This script helps reviewers distinguish likely collection churn from meaningful changes in selected AWS resources such as:

- `aws_lb_listener_rule`
- `aws_security_group`
- `aws_route_table`
- `aws_wafv2_web_acl`

It is useful for PR triage, CI evidence, and skill-assisted review.

## Important honesty note

This script is **conservative triage**, not final approval.

Why:

- some supported AWS resources have provider-documented inline rule conflicts,
- some have service-domain ordering semantics that still need human review,
- WAFv2 inline rules have explicit provider limitations.

## Prerequisites

- Python 3.8 or higher
- No additional packages required (uses only standard library)

## Usage

### Basic usage

```bash
# Read from file
python analyze_plan.py plan.json

# Read from stdin
terraform show -json plan.tfplan | python analyze_plan.py
```

### Exit codes (with `--exit-code`)

| Code | Meaning |
|------|---------|
| 0 | No changes, or likely inline/order-only changes |
| 1 | Actual inline or repeated-block changes |
| 2 | Resource replacement |
| 3 | Error |

## Output interpretation

| Category | Meaning | Recommended action |
|----------|---------|-------------------|
| 🟢 Likely inline noise | Collection churn with no clear semantic delta | Still review provider caveats |
| 🟡 Actual change | Inline block added/removed/modified | Review resource semantics carefully |
| 🔴 Resource replacement | delete + create | Check downtime or control-plane impact |

## Special cautions

- If a security group uses inline `ingress` or `egress`, check whether the repo also manages rules with standalone rule resources.
- If a route table uses inline `route` blocks, check whether the repo also manages routes with standalone `aws_route` resources.
- If the resource is `aws_wafv2_web_acl`, do not treat "many rules changed" as harmless by default; provider docs explicitly warn about inline rule limitations.

## Custom attribute definitions

By default, uses `references/aws_inline_attributes.json`, but you can specify a custom definition file:

```bash
python analyze_plan.py plan.json --attributes /path/to/custom_attributes.json
```

See `references/aws_inline_attributes.md` for the definition-file format and provider caveats.

## Related documentation

- [SKILL.md](../SKILL.md) - Usage as an agent skill
- [aws_inline_attributes.md](../references/aws_inline_attributes.md) - Attribute definition reference
