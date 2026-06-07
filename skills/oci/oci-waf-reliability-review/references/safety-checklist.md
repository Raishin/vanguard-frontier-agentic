# Safety Checklist

## Non-negotiable gates

- [ ] Scope is explicit: service, workload, region class, compartment boundary, and evidence window are defined without exposing identifiers.
- [ ] Official OCI documentation is used for service behavior and caveats.
- [ ] OCI API evidence through the user’s configured read-only OCI MCP is labeled as sampled evidence.
- [ ] Secrets and identifiers are absent from prompts, notes, committed docs, and examples.
- [ ] Mutations require explicit user approval and a rollback or recovery path.
- [ ] Findings separate fact, inference, unknowns, and recommended action.

## High-risk mutation boundary

Stop and require explicit approval before any action that creates, updates, deletes, enables, disables, rotates, attaches, detaches, escalates, fails over, resizes, purchases commitments, changes access, changes retention, or sends data to a support channel.

## Credential boundary

Never ask the user to paste credentials, tokens, tenant or tenancy identifiers, subscription-like identifiers, compartment identifiers, resource identifiers, support identifiers, customer data, private keys, wallets, fingerprints, connection strings, kubeconfigs, or config contents. Ask for a sanitized description or permission to inspect through already configured read-only tooling instead.
