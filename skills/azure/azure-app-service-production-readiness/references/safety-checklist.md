# Safety checklist for Azure App Service Production Readiness

## Non-negotiable gates

- Never ask for publish profiles, connection strings, certificates, private keys, app secrets, tenant identifiers, subscription identifiers, or customer data.
- Do not approve production if rollback, health checks, diagnostics, managed identity, secret flow, networking, and owner/runbook evidence are missing.
- Treat app settings as sensitive. Do not echo values; reason about names and references only.
- Require explicit approval before app setting changes, slot swaps, scale changes, network changes, identity changes, backup changes, or restart operations.
- Do not equate App Service managed platform with workload resilience. Application dependencies and state stores remain the user's responsibility.

## High-risk assumptions to kill

- "Premium plan means production-ready." SKU is one input, not evidence of readiness.
- "Slots make deployment safe." Slot settings, warm-up, health, dependency compatibility, and rollback must be proven.
- "Private endpoint means no public exposure." Public access, access restrictions, DNS, reverse proxy, and SCM endpoint behavior still need review.
- "App Service backup covers the database." Linked database backup support has deprecation caveats; use native database backup/restore for state stores.
- "Key Vault references remove all secret risk." Managed identity permissions, network routing, slot settings, and reference resolution must be checked.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `sampled_read_only`: current app or plan evidence was sampled safely.
- `config_review`: IaC/app configuration was reviewed but not proven live.
- `restore_proven`: backup/restore or rollback was tested and evidence exists.
- `mutation_ready`: blast radius, rollback, and explicit approval exist.

## Minimum safe evidence

- App Service plan tier, instance count, OS, region, zone posture, and scale rules.
- Deployment slot count, swap strategy, sticky settings, warm-up path, and rollback path.
- Ingress model, public access/access restrictions, private endpoint, DNS, WAF/reverse proxy, and SCM restrictions.
- VNet integration, outbound routing, private dependencies, Key Vault references, and managed identity permissions.
- Health check, diagnostics, alerts, dashboards, backup/restore tests, and on-call ownership.
