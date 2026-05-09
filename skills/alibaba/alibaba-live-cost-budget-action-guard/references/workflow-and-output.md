# Workflow and output contract

Use this reference only when executing the full live financial authority gate — budget threshold changes, Savings Plan purchases, or Reserved Instance commitments.

## Financial guard areas to check

- Budget inventory: all active budgets, current thresholds, notification targets, spend vs. threshold ratio
- Savings Plan commitments: active plans, hourly commitment amount, remaining term, utilization rate
- Reserved Instance commitments: instance type, zone/regional scope, payment option, remaining term
- Blast radius: services at risk from budget threshold reduction; suspension timeline; rollback path
- Financial authority: approver identity, written confirmation, documented approval chain

## Safe workflow

1. **Frame scope** — confirm BSS account, operator identity, and the specific financial action requested
2. **Collect evidence** — capture current budget and commitment inventory before any change; label: `live evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius of this action? what is non-refundable? what services suspend?
4. **Gate** — require explicit written confirmation from the operator covering all required confirmation statements
5. **Execute and verify** — execute the minimum scoped action; confirm in BSS; document confirmation ID

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Live Cost Budget Action Guard: <scope>
## Identity and authority confirmed
## Current inventory
## Blast radius assessment
## Confirmation received
## Execution result
## Post-action verification
```

Each section must include an evidence level label. Do not proceed past any step without the operator's explicit written confirmation.
