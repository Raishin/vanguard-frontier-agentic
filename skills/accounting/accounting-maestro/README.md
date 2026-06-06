# Accounting Maestro Skill

Routing logic and safety protocol for the accounting domain. Used by the `accounting-maestro-agent` to classify and dispatch accounting tasks to the correct specialist.

## What it does

- Maps accounting task types to catalog agent IDs
- Enforces routing rules: single vs. parallel dispatch, hard ceiling, live-guard gate
- Provides the safety checklist for dispatch validation
- Establishes the response shape for route/reason/mode headers

## Domain coverage

Revenue recognition → close cycle → audit evidence → consolidation (as catalog expands)
