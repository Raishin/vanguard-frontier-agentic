# Workflow and output contract

1. Confirm the input is Terraform AzureRM plan evidence, ideally JSON from `terraform show -json <saved-plan>`.
2. Identify whether the suspicious churn is concentrated in known AzureRM set-type attributes.
3. Prefer deterministic analyzer output from `scripts/analyze_plan.py` when JSON evidence is available.
4. For `azurerm_application_gateway`, explicitly separate nested collection churn from real routing-graph changes affecting listeners, rewrite-rule sets, request-routing rules, or URL path maps.
5. For `azurerm_network_security_group` and `azurerm_route_table`, explicitly separate order churn from real rule-priority, address-scope, or next-hop changes.
6. Separate categories clearly: order-only set churn, actual set additions/removals/modifications, replacements, and unrelated non-set changes.
7. Return a compact conclusion with evidence, caveats, and next review steps.

## Output shape

1. Verdict
2. Evidence source
3. Resources and attributes involved
4. Real-change versus order-only analysis
5. Risks or caveats
6. Next validation step
