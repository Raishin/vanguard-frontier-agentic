# OCI WAF Cost Optimization Review Operations Reference

## What people get wrong

- A forecast is a bill.
- A budget is an enforcement boundary.
- Idle-looking resources can be stopped or deleted without owner and dependency checks.
- Savings recommendations are safe without workload seasonality and reservation/commitment analysis.
- Cloud Advisor recommendations replace human ownership or change approval.

## Officially grounded service shape

- Cost Analysis, Budgets, and usage-summary APIs support cost/usage views, grouping, filtering, time windows, and budget state; they do not prove ownership or business criticality.
- Budgets can target compartments or tags; a budget alert is governance evidence, not an automatic cost-control guarantee unless an approved automation path exists.
- Cloud Advisor analyzes tenancy resources and provides cost, performance, security, and availability recommendations, but sampled API evidence did not confirm a generic Cloud Advisor CLI surface in this run.
- Usage-summary evidence can group by service, compartment, region, resource, tags, tenant, unit, and SKU dimensions when permissions and data are available.

## Non-negotiable design rules

- Separate actual spend, forecast spend, commitment/rate options, rightsizing, and deletion candidates.
- Require owner, time window, service, compartment or tag scope, business criticality, and rollback path before recommending a mutation.
- Do not propose stopping, deleting, resizing, or buying commitments without explicit approval and current utilization evidence.
- Label Cloud Advisor as documentation-grounded unless current configured-environment evidence was sampled directly.
- Never commit tenant, compartment, resource, invoice, or customer billing identifiers.

## Minimal safe implementation flow

- Define cost objective and measurement window.
- Ground Cost Analysis, Budgets, Cloud Advisor, quotas, and usage APIs in official docs.
- Use OCI API evidence through the user’s configured read-only OCI MCP for usage and budget command shape or sanitized observations.
- Rank recommendations by evidence strength, savings confidence, reversibility, and blast radius.
- Return no-regret governance fixes before destructive or commitment actions.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The user asks for deletion or commitment purchase without owner approval.
- The dataset lacks time window, tags, service scope, or forecast/actual separation.
- The request would expose billing identifiers or customer data.
