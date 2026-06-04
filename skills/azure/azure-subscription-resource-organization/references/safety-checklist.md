# Safety checklist

Use before recommending production Azure changes, access grants, security remediation, hierarchy moves, cost actions, reliability changes, or readiness conclusions for `azure-subscription-resource-organization`.

## Non-negotiables

- Do not ask for or print credentials, client secrets, certificates, private keys, access tokens, tenant IDs, subscription IDs, resource IDs, customer data, raw incident payloads, or environment-specific identifiers.
- Prefer Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior.
- Use sampled read-only Azure evidence only for current-state claims and label it as sampled evidence.
- Require explicit approval before recommending live mutation, broad access, destructive remediation, billing changes, commitment purchases, hierarchy moves, failover, failback, or alert suppression.
- Keep recommendations least-privilege, reversible where possible, and scoped to the named resource or workload.
- Separate documentation-based claims, sampled evidence, user-provided evidence, and inference.

## Component risks

- **Identity and roles:** broad privileged roles, direct user grants, wildcard custom roles, missing PIM/time-bound controls, inherited scope surprises.
- **Security posture:** stored secrets, public exposure, missing managed identities, weak Key Vault boundaries, no diagnostic coverage, untracked policy exemptions.
- **Resource organization:** flat hierarchy drift, fake isolation via resource groups, subscription sprawl, weak ownership, policy inheritance surprises.
- **Cost:** optimizing recommendations without workload context, deleting resources without owner confirmation, buying commitments before rightsizing, ignoring licensing and reliability cost tradeoffs.
- **Reliability:** vague SLOs, untested recovery, overengineered topology, missing health model, no dependency mapping, unvalidated chaos or failover assumptions.

## Evidence labels

Use `documentation-based`, `sampled read-only evidence`, `repo evidence`, `user-provided evidence`, or `inference`. Documentation alone never proves the user's live Azure environment.
