# Workflow and output contract

1. Confirm the input is Terraform OCI plan evidence, ideally JSON from `terraform show -json <saved-plan>`.
2. Identify whether the suspicious churn is concentrated in currently modeled OCI repeated-block attributes.
3. Prefer deterministic analyzer output from `scripts/analyze_plan.py` when JSON evidence is available.
4. For `oci_core_route_table`, explicitly separate route-rule churn from real destination or network-entity changes.
5. For load-balancer routing policies, path-route sets, and rule sets, explicitly separate collection reflow from real ordered rule, path-match, backend, or action changes.
6. Separate categories clearly: likely order-only or repeated-block noise, actual additions/removals/modifications, replacements, and unrelated non-repeated-block changes.
7. Return a compact conclusion with evidence, caveats, and next review steps.

## Output shape

1. Verdict
2. Evidence source
3. Resources and attributes involved
4. Real-change versus repeated-block-noise analysis
5. Risks or caveats
6. Next validation step
