# App Service production operations

## What people get wrong

- They treat App Service as production-ready because it is PaaS. The workload still needs health, scaling, network, identity, release, and recovery design.
- They trust slots without checking warm-up, sticky settings, dependency compatibility, and rollback drills.
- They confuse VNet integration with private ingress. VNet integration controls outbound access; private endpoint and access restrictions control inbound exposure.
- They put secrets in app settings instead of using managed identity and Key Vault references.
- They configure backup but never prove restore, especially for state stores outside the app.

## Officially grounded service shape

Microsoft Learn frames App Service readiness around Well-Architected pillars. Production web apps need appropriate plan tier and instance count, health checks, diagnostics, HTTPS/TLS, managed identity, private networking where needed, deployment slots for release safety, autoscale, zone or regional resilience where required, and clear backup/restore ownership.

## Non-negotiable design rules

1. Use multiple instances for production workloads unless the business explicitly accepts single-instance risk.
2. Enable health check with a path that proves critical dependency readiness without leaking sensitive data.
3. Use managed identity and Key Vault references for secrets; never normalize pasted secrets.
4. Validate inbound and outbound networking separately.
5. Use staging slots for risky deployments and prove swap/rollback behavior.
6. Use native database backup/restore for linked state stores; do not rely on App Service backup as a database recovery strategy.
7. Enable diagnostics, metrics, alerts, and operator runbooks before go-live.

## Minimal safe implementation flow

1. Inventory app, plan, OS, region, tier, instances, deployment model, and dependencies.
2. Verify ingress: public endpoint, access restrictions, private endpoint, DNS, Front Door/Application Gateway, WAF, and SCM exposure.
3. Verify outbound: VNet integration, route-all, DNS, private endpoints, Key Vault, database, storage, registry, and firewall paths.
4. Verify identity and secrets: system/user-assigned identity, Key Vault role or access policy, slot-specific references, and no embedded secret values.
5. Verify release: staging slot, warm-up path, swap with preview if needed, sticky settings, rollback, and smoke tests.
6. Verify resilience: health check, auto-heal, autoscale, zone/multiregion posture, backup, restore test, and dependency retry/circuit breakers.
7. Deliver go/no-go with blockers.

## High-risk assumptions to kill

- App Service being managed PaaS does not prove production readiness; plan sizing, instance count, health checks, release safety, and dependency recovery still matter.
- A staging slot is not a rollback plan unless sticky settings, warm-up path, smoke tests, and swap reversal are rehearsed.
- VNet integration is outbound connectivity only; it does not provide private inbound access to the app.
- App Service backup does not restore every surrounding dependency, network feature, identity, alert, or deployment slot.
- A green health endpoint is weak evidence if it does not represent critical dependency readiness or if it leaks sensitive internals.

## Safe command/code verification targets

- Inspect IaC for plan SKU, worker count, zone redundancy, Always On, health check path, autoscale rules, diagnostics, and backup configuration.
- Review deployment automation for staging-slot deployment, smoke testing, swap operation, rollback operation, and tagged container/image provenance.
- Check app configuration for Key Vault references, managed identity use, slot-specific settings, no embedded secrets, and SCM access restrictions.
- Validate templates distinguish private endpoint inbound access from VNet integration outbound access and include private DNS where required.
- Review restore/runbook evidence for app content, configuration, custom domains, TLS, identities, networking, databases, and alerts instead of assuming one backup covers all.

## Safe verification targets

- Plan SKU/tier, worker count, autoscale rules, Always On, health check, and auto-heal.
- Slot list, slot settings, traffic routing, swap behavior, and rollback evidence.
- Public network setting, access restrictions, private endpoints, DNS, reverse proxy, and SCM restrictions.
- Managed identity, Key Vault reference resolution, network-restricted vault access, and config separation.
- Diagnostic settings, App Insights, alerts, log retention, backup schedule, and restore test results.

## When to push back

Push back on production launch if rollback is manual guesswork, health check is absent, public access is unexplained, secrets are embedded, linked database backup is assumed, or no one owns alerts and restore drills.
