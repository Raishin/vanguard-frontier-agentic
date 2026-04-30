# AWS inline or repeated-block attributes reference

This document explains the curated reference file `aws_inline_attributes.json` used by the Terraform AWS inline diff analyzer.

> **Last Updated**: 2026-04-30

## Important honesty note

This AWS variant is **not** a pure Set-type analyzer.

Why:

- some AWS resources use repeated inline blocks where order or provider flattening can create noisy plans,
- some AWS resources have provider-documented drift caveats,
- some inline patterns are better modeled with standalone Terraform resources instead of inline blocks.

So this skill is a **conservative inline-diff triage helper**, not a blanket "ignore the churn" engine.

## Current support posture

The support map is intentionally conservative and currently focuses on AWS resources where the provider docs or service docs make repeated block review important:

- `aws_lb_listener_rule`
- `aws_route_table`
- `aws_security_group`
- `aws_wafv2_web_acl`

## JSON structure

```json
{
  "resources": {
    "aws_resource_type": {
      "attribute_name": "key_attribute"
    }
  }
}
```

Nested or repeated blocks can use the same `_key` style as the AzureRM and OCI variants. `null` means the analyzer falls back to comparing the full object when there is no stable key.

## Provider-specific caveats

- `aws_security_group` inline `ingress` and `egress` blocks should **not** be mixed with standalone security-group rule resources; the provider docs warn that this can create conflicts and perpetual diffs.
- `aws_route_table` inline `route` blocks should **not** be mixed with standalone `aws_route` resources for the same route table; the provider docs warn that this causes conflicts and overwrite behavior.
- `aws_wafv2_web_acl` inline `rule` blocks have provider-documented limitations, including unpredictable ordering and cases where editing one rule can appear to recreate many rules.
- `aws_lb_listener_rule` conditions and actions are nested collections; a real review must still check priority, host/path/query semantics, and whether a rule meaningfully changed.

## Maintenance rule

Before adding more AWS attributes:

1. verify the repeated or inline block exists in the official Terraform AWS provider docs,
2. verify the AWS service docs explain the domain semantics well enough to review the change safely,
3. confirm a stable key attribute exists or accept a conservative `null` fallback,
4. test against real `terraform show -json` evidence,
5. keep the support map conservative rather than pretending broad coverage.
