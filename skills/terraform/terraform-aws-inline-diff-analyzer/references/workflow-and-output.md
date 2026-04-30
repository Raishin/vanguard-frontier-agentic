# Workflow and output contract

1. Confirm the input is Terraform AWS plan evidence, ideally JSON from `terraform show -json <saved-plan>`.
2. Identify whether the suspicious churn is concentrated in currently modeled AWS inline or repeated-block attributes.
3. Prefer deterministic analyzer output from `scripts/analyze_plan.py` when JSON evidence is available.
4. For `aws_security_group` and `aws_route_table`, explicitly check whether the configuration mixes inline blocks with standalone Terraform rule resources. If yes, surface that as a provider-conflict risk, not mere plan noise.
5. For `aws_wafv2_web_acl`, separate likely order churn from real rule-content changes and explicitly mention the provider-documented inline rule limitations.
6. For `aws_lb_listener_rule`, separate condition or action reflow from real listener-rule semantic changes such as priority, host/path/query logic, or authentication behavior.
7. Return a compact conclusion with evidence, caveats, and next review steps.

## Output shape

1. Verdict
2. Evidence source
3. Resources and attributes involved
4. Real-change versus inline-noise analysis
5. Risks or caveats
6. Next validation step
