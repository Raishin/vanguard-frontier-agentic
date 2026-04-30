# OCI repeated-block attributes reference

This document explains the curated reference file `oci_set_attributes.json` used by the OCI Terraform diff analyzer.

> **Last Updated**: 2026-04-30

## Important honesty note

OCI provider documentation does **not** call out set-order noise as explicitly as the AzureRM Application Gateway docs do.

So this OCI variant is a **heuristic triage helper**, not a perfect provider-backed truth engine.

Use it to reduce review noise in common OCI Terraform plans, but keep confidence lower than the AzureRM variant unless you validate the repeated-block behavior against real plan evidence.

## Current support posture

The support list is intentionally conservative and currently focuses on OCI resources with obvious repeated nested blocks in the official Terraform Registry docs, including:

- `oci_core_route_table`
- `oci_load_balancer_load_balancer`
- `oci_load_balancer_load_balancer_routing_policy`
- `oci_load_balancer_path_route_set`
- `oci_load_balancer_rule_set`

## JSON structure

```json
{
  "resources": {
    "oci_resource_type": {
      "attribute_name": "key_attribute"
    }
  }
}
```

Nested repeated blocks can use the same `_key` style as the AzureRM variant.

## Maintenance rule

Before adding more OCI attributes:

1. verify the repeated block exists in the official OCI Terraform Registry docs,
2. confirm a stable key attribute exists,
3. cross-check the resource family against the OCI operational surface when relevant:
   - `oci network route-table --help`
   - `oci lb routing-policy --help`
   - `oci lb path-route-set --help`
   - `oci lb rule-set --help`
4. test against real `terraform show -json` evidence,
5. keep the support map conservative rather than pretending broad coverage.
