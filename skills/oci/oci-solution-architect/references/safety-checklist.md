# Safety checklist

Use this checklist before producing guidance, requesting evidence, or proposing changes.

## Evidence labels

- `documentation-based`: grounded in official OCI documentation only.
- `sampled-api-shape`: grounded in read-only command/API help or metadata evidence.
- `sampled-current-state`: grounded in read-only observations from the configured environment.
- `user-supplied`: grounded in sanitized user evidence.
- `inference`: reasoned from incomplete evidence; never present it as fact.

## Credential and identifier boundaries

- Do not ask for credentials, tokens, private keys, API keys, config files, wallets, kubeconfigs, connection strings, customer data, or sensitive identifiers.
- Do not commit environment-specific identifiers, internal tool names, local environment labels, connector IDs, or private endpoint details.
- If the task requires live context, ask for a sanitized export or use approved read-only evidence paths.

## Mutation boundary

Require explicit approval before creating, updating, deleting, moving, scaling, rotating, patching, enabling, disabling, deleting backups, deleting images, changing retention, remediating security findings, changing access, or changing production state.

## Approval gates

- Confirm scope, owner, environment criticality, maintenance window, rollback plan, and validation target.
- Separate read-only discovery from write actions.
- Prefer least privilege and reversible changes.
- Stop if evidence conflicts or if the blast radius is unknown.

## Final-answer minimum

- State evidence level and scope.
- List blockers and unsafe assumptions.
- Give safe next actions before risky actions.
- Name what was not proved.
