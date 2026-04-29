# Workflow and output contract

Use this reference for full write-capable AWS patch work.

## Workflow

1. Classify the repo-side correction.
2. Confirm the target files and blast radius.
3. Make the smallest reversible edit.
4. Run local validators or syntax checks.
5. Report exact files changed, validation results, and rollback path.

## Guardrails

- Repo write access is allowed.
- Live AWS mutation is out of scope by default.
- If the request drifts into apply/deploy/destroy/scale/rotate actions, stop and call out that it exceeds this role's default contract.
