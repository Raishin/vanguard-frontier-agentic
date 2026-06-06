# Permissions — Accounting Maestro

## Execution Tier

`read-only-runtime`

## Allowed

- Read skill and reference files from `skills/accounting/accounting-maestro/`
- Read `catalog/agents.json` to resolve routing targets
- Dispatch to catalog-registered accounting specialist agents (read-only tier only)

## Denied

- Write to any ledger, ERP, accounting system, or file system record
- Access to financial databases, trial balance exports, or ERP APIs
- Accept or relay customer names, contract amounts, or PII
- Make final accounting determinations
- Auto-dispatch any agent with write or live-guard capability
- Form an accountant-client relationship
