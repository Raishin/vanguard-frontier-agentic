# Azure App Service Production Readiness Agent Operations

> Version note: App Service deployment, health check, networking, backup, runtime, and plan capabilities change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste publish profiles, app settings with secrets, tenant or subscription identifiers, connection strings, certificates, private keys, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Equating a paid App Service Plan with production readiness.
- Deploying directly to production instead of using slots, smoke tests, warm-up, and swap rollback.
- Assuming private endpoint covers outbound traffic; Microsoft Learn distinguishes inbound private endpoint from outbound VNet integration.
- Treating Health Check as a silver bullet even when there is only one instance, an invalid path, inconsistent slot config, or an endpoint that ignores critical dependencies.
- Assuming backups restore every production dependency, network feature, identity, custom domain, TLS binding, scale rule, diagnostic setting, alert, or slot.

## Officially grounded service shape

- Microsoft recommends deployment slots for production releases on Standard tier or better; swap warms worker instances and supports rollback by swapping again.
- Continuous deployment should not target the production slot directly; deploy to a nonproduction slot and swap when validated.
- Health Check requires a valid path and is most useful with two or more instances; configuration changes can restart the app.
- Private endpoints secure inbound traffic to the app; outbound VNet integration is separate and cannot use the same subnet as the private endpoint.
- App Service backup/restore does not restore several operational dependencies, including network features, managed identities, scale, diagnostics, alerts, backups themselves, and deployment slots.
- Linked database backup support is changing; native database backup/restore should be treated as the durable recovery path for data stores.

That is the key insight:

> The agent must prove release path, health model, network direction, identity/secrets boundary, and restore evidence separately; App Service “exists” is not production readiness.

## Non-negotiable design rules

### 1. Block readiness claims without deployment slot or an explicitly accepted alternative rollback mechanism.

### 2. Treat app settings, publish profiles, certificates, and connection strings as sensitive; prefer managed identity plus Key Vault references.

### 3. Do not present backup configuration as tested recovery unless restore evidence exists.

### 4. Separate inbound private access, outbound routing, DNS, public access restrictions, and dependency connectivity evidence.

### 5. Require health endpoint semantics, scale/instance count, monitoring, alerts, and slot consistency before production approval.

## Minimal safe implementation flow

- Classify the request: deployment readiness, incident triage, networking, identity/secrets, backup/restore, monitoring, scale, or runtime review.
- Identify app, plan, slots, dependencies, identities, domains/TLS, network paths, and release pipeline.
- Ground checks in Microsoft Learn deployment, health, networking, identity, backup, security, and diagnostics docs.
- Use read-only configured-environment evidence to sample plan, slots, health, identities, app settings without secret values, network state, logs, alerts, and backups when available.
- Return blockers first, then safe next actions, rollback/restore proof needed, and residual risk.

## High-risk assumptions to kill

- A staging slot exists, therefore swap rollback and smoke tests are implemented.
- Private endpoint means outbound dependencies are private.
- A backup schedule means recovery has been tested.
- Health Check means the app and all dependencies are healthy.
- Production secrets in app settings are acceptable because App Service encrypts configuration at rest.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Plan SKU/capacity, instance count, deployment slots, slot settings, warm-up path, smoke tests, and rollback-by-swap path.
- Health Check path, response behavior, authentication exception, dependency coverage, alert rules, and restart impact.
- Managed identity, Key Vault references, sensitive app settings, publish profile exposure, TLS/custom domains, and certificate lifecycle.
- Private endpoint, public network access, access restrictions, VNet integration, DNS, and dependency reachability.
- Backup schedule, retention, restore test, non-restored settings list, linked database strategy, diagnostics, logs, and alerts.

## When to push back

- Production deploys go direct-to-prod with no slot, smoke test, or rollback mechanism.
- Secrets are embedded in app settings, workflows, code, or documentation examples.
- Private networking, health, or recovery posture is asserted without separate evidence for each boundary.
- The request asks for production readiness from documentation alone rather than sampled configured-environment evidence.
