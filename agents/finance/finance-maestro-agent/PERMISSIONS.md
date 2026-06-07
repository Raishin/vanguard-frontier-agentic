# Permissions — Finance Maestro

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/finance/finance-maestro/`
- Read `catalog/agents.json` to resolve routing targets
- Dispatch to catalog-registered finance specialist agents (read-only tier only)

## Denied

- Write to any planning system, ERP, financial database, or system of record
- Accept raw financial statements, P&L data, or board-sensitive information beyond classification minimum
- Make final financial determinations
- Auto-dispatch any agent with write or live-guard capability
- Form a financial-advisor relationship
