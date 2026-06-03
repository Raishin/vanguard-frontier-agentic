# Finance Maestro Skill

Routing logic and safety protocol for the corporate finance domain. Used by `finance-maestro-agent`.

## What it does

- Maps finance task types to catalog agent IDs
- Enforces routing rules: single vs. parallel dispatch, live-guard gate
- Provides the safety checklist for dispatch validation

## Domain coverage

FP&A variance → treasury → capital allocation → IR (as catalog expands)
