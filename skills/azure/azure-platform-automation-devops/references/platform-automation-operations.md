# Azure Platform Automation Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating a green pipeline as proof that a platform change is safe.
- Mixing bootstrap, shared platform, and workload deployments into one uncontrolled flow.
- Skipping what-if or equivalent preview because the template compiled.
- Assuming Bicep, Terraform, or a landing-zone accelerator is automatically the right answer.
- Putting publish profiles, client secrets, or service connection secrets into repo or chat.

## Officially grounded service shape

- Microsoft Learn evidence says Bicep what-if previews predicted resource changes without making changes and works at resource group, subscription, management group, and tenant scopes.
- What-if has real limits: nested template expansion limits, short-circuiting, unevaluated expressions, and possible noise from defaulted properties.
- Microsoft Learn training for Bicep delivery emphasizes linting, preflight validation, what-if checks, manual approval steps, and post-deployment verification.
- Azure landing-zone guidance separates platform foundations from workload adoption and supports multiple implementation options rather than one universal IaC path.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident state, or production readiness.

## Non-negotiable design rules

- Classify bootstrap, platform, and workload scope before proposing a pipeline.
- Require lint, static validation, what-if or equivalent preview, human approval for risky changes, and post-deployment verification.
- Use least-privilege deployment identities scoped to the deployment boundary.
- Treat what-if output as evidence with limitations, not as absolute proof of safety.
- Refuse production rollout advice when rollback, blast radius, or approval ownership is missing.

## Minimal safe implementation flow

- Scope landing zone, subscriptions, management groups, target regions, IaC tool, and owner boundary.
- Separate bootstrap prerequisites from steady-state platform operations and workload delivery.
- Review identity, secrets, service connections, module source, state storage, and environment promotion model.
- Require lint, validation, what-if preview, approval, deployment, smoke checks, and rollback evidence.
- Return a go, no-go, or conditional-go verdict with blockers and verification targets.

## Safe verification targets

- IaC lint and schema validation pass for the exact target scope.
- What-if preview or equivalent plan is reviewed for create, modify, delete, ignore, and unevaluated-expression risks.
- Deployment identity permissions match target scope and do not require broad Owner by default.
- Secrets are stored in approved secret stores or platform-protected variables, not repo files.
- Rollback or redeploy path is tested or explicitly bounded.

## When to push back

- A single pipeline can mutate management groups, subscriptions, networking, and workloads without gates.
- The user wants to deploy without reviewing preview output.
- Secrets or tenant-specific identifiers are requested in chat.
- The plan confuses application release rollback with platform rollback.
