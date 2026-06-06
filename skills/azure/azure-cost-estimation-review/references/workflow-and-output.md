# Workflow and output contract for Azure Cost Estimation Review

## Minimal safe workflow

1. Classify estimate type: greenfield, migration, expansion, DR, nonproduction, or optimization scenario.
2. Ground the review with Microsoft Learn through the user's configured documentation MCP.
3. Build the bill-of-materials table and mark missing services.
4. Challenge assumptions: region, SKU, hours, storage growth, network egress, logs, backups, support, security, and operations.
5. Separate pricing basis: retail calculator, negotiated rate, price sheet, historical actuals, or user estimate.
6. Produce scenario ranges: minimum viable, expected, peak, DR/failover, and growth.
7. Return confidence level and required evidence for stronger precision.

## Output contract

```markdown
## Verdict
<credible estimate | conditional estimate | unreliable estimate | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Pricing/usage evidence: <estimate_review|usage_informed|rate_verified|not provided>

## Estimate risks
1. <risk> — Evidence: <docs_only|estimate_review|usage_informed|inference>

## Missing cost lines
- <omitted service or assumption>

## Confidence range
- Low / expected / peak: <if evidence supports it>

## Safe next actions
- <specific estimate improvement>
```

## Pushback triggers

Push back on single-number estimates, hidden assumptions, missing operational costs, unsupported discounts, no growth case, or calculator totals represented as committed budget truth.
