# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, deployed resources, current cost, vault state, app health, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/app-service/deploy-staging-slots
- https://learn.microsoft.com/azure/app-service/reference-app-settings#deployment-slots
- https://learn.microsoft.com/azure/app-service/deploy-best-practices
- https://learn.microsoft.com/azure/app-service/configure-common
- https://learn.microsoft.com/azure/app-service/overview-local-cache

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says App Service deployment slots are live apps and swaps apply target-slot settings to source instances, restart and warm source instances, then switch routing. Swap with preview pauses after target settings are applied so the operator can validate before completing or resetting. Sticky slot settings and warm-up app settings decide whether secrets, connection strings, networking-sensitive settings, and health paths behave safely during swap.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Live-operation claim: requires target, principal, approval, preflight evidence, rollback constraints, and post-action verification.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
