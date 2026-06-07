# Safety checklist

Use before recommending production Azure changes, access grants, network connectivity changes, deployment automation, resilience claims, or incident conclusions for `azure-private-endpoint-adoption-planner`.

## Non-negotiables

- Do not ask for or print credentials, client secrets, certificates, private keys, access tokens, tenant IDs, subscription IDs, resource IDs, customer data, raw incident payloads, or environment-specific identifiers.
- Prefer Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior.
- Use sampled read-only Azure evidence only for current-state claims and label it as sampled evidence.
- Require explicit approval before recommending live mutation, broad access, destructive remediation, production deployment, DNS changes, failover, failback, or alert suppression.
- Keep recommendations least-privilege, reversible where possible, and scoped to the named resource or workload.
- Separate documentation-based claims, sampled evidence, user-provided evidence, and inference.

## Component risks

- **Identity and RBAC:** broad privileged roles, direct user grants, wildcard custom roles, missing PIM/time-bound controls, inherited scope surprises.
- **Automation and IaC:** missing preview, unreviewed delete/modify changes, overbroad deployment identities, unsafe secret handling, no rollback path.
- **Networking and Private Link:** DNS misconfiguration, duplicate private DNS zones, missing VNet links, resolver/forwarder gaps, route surprises, broken application connectivity.
- **Resilience and BCDR:** fantasy RTO/RPO, untested restore, undocumented failback, inaccessible DR assets, hidden single-region dependencies.
- **Health triage:** false provider attribution, unsupported resource health, ignored activity-log changes, sensitive incident payload exposure, broad remediation before blast-radius evidence.

## Evidence labels

Use `documentation-based`, `sampled read-only evidence`, `repo evidence`, `user-provided evidence`, or `inference`. Documentation alone never proves the user's live Azure environment.
