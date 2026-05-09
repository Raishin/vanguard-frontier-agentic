# Workflow and Output Contract

Use this reference when performing the full WAF Cost Optimization Pillar review or formatting the final assessment.

## Review domains

Work through these five cost optimization design principles:

1. **Cloud financial management** — Cost Explorer enabled, CUR in S3, Budgets per account/project, Cost Anomaly Detection active, tagging policy enforced
2. **Expenditure awareness** — 100% resource tagging (env, team, app, cost-center), Cost Allocation Tags in billing, charge-back / show-back reports
3. **Cost-effective resources** — Graviton vs x86 evaluation, instance family selection, S3 storage class usage, RDS instance type vs Aurora Serverless fit
4. **Usage optimization** — Auto Scaling for stateless compute, S3 Intelligent Tiering or lifecycle rules, Spot Instances for fault-tolerant workloads, dev environment auto-shutdown
5. **Rate optimization** — Savings Plans coverage (Compute > EC2 > SageMaker), Reserved Instances for database/cache, Committed Use Discounts, Hybrid Benefit (Windows/SQL)

## Safe workflow

1. **Frame scope**: accounts, Regions, top 5 services by spend, monthly total
2. **Gather evidence**: Cost Explorer by service/tag, Trusted Advisor cost checks, Compute Optimizer recommendations, current Savings Plans coverage
3. **Assess each principle**: identify gaps per principle with estimated monthly savings
4. **Prioritize findings**: by estimated monthly savings impact
5. **Quantify savings**: provide per-recommendation estimated range with confidence level
6. **Confirm before acting**: require approval for resource deletion, RI/SP purchase, or billing config changes

## Response shape

1. Scope: monthly spend, top services, current commitment coverage
2. Cost visibility and attribution assessment
3. Tagging compliance
4. Rightsizing and instance family opportunities
5. Commitment strategy (Savings Plans, RI coverage)
6. Spot and managed service adoption
7. Storage and data transfer optimization
8. Idle resource inventory
9. Prioritized savings opportunities (est. monthly $ impact)
10. Open risks and blockers
