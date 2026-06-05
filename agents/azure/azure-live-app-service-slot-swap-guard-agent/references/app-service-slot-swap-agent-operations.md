# Azure Live App Service Slot Swap Guard Agent Operations

> Version note: Azure App Service slot behavior, swap limitations, permissions, and warmup controls change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, publish profiles, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating a slot swap as a simple traffic flip instead of a configuration apply, restart, warmup, and routing operation.
- Forgetting that some settings swap while others stay slot-specific, causing production to run with unintended configuration.
- Skipping swap with preview for mission-critical apps even though it lets the source run with target settings before completion.
- Assuming warmup is healthy because the root path returned anything, while the real health endpoint is untested.
- Completing a pending swap without reset and immediate re-swap rollback options.

## Officially grounded service shape

- App Service deployment slots are live apps with their own host names, available on supported App Service plan tiers.
- During a swap, App Service applies target-slot settings to source-slot instances, restarts the source, waits for availability, warms instances, and then switches routing.
- Swap with preview pauses after the first phase so operators can validate the source slot with target-slot settings before completing or resetting.
- Some settings are swapped and some are not; slot-specific settings, identities, custom domains, TLS settings, scale settings, diagnostic settings, IP restrictions, Always On, and VNet integration need explicit review.
- Custom warmup can use application initialization or app settings such as `WEBSITE_SWAP_WARMUP_PING_PATH` and `WEBSITE_SWAP_WARMUP_PING_STATUSES`; bad warmup rules can stop a swap.

That is the key insight:

> The agent is a live production gate. It must prove the swap target, sticky-settings diff, warmup evidence, preview state, approval, and rollback path before allowing final completion.

## Non-negotiable design rules

### 1. Never complete a production swap without target-slot confirmation, preview evidence where supported, approval, and rollback posture.

### 2. Treat sticky settings, connection strings, managed identity, VNet integration, auth, custom domains, and TLS as production-risk boundaries.

### 3. Block final commit when warmup or health probes are missing, broad, or contradictory.

### 4. Prefer read-only slot inventory, config diff, activity log, health checks, and preview state before mutation.

### 5. Label configured-environment observations as sampled and bounded to the app, slot pair, and time window.

## Minimal safe implementation flow

- Confirm app, source slot, production target slot, desired action, approval state, rollback owner, and health endpoint.
- Ground swap behavior and limitations in Microsoft Learn App Service deployment slot guidance.
- Collect read-only evidence for slot list, app settings stickiness, connection strings, auth, identity, networking, warmup settings, current routing, and recent swap activity.
- Decide: preview, complete, reset, re-swap, or block; if action is live, require explicit human approval.
- Verify post-action production health, activity log status, routing, and open risks.

## High-risk assumptions to kill

- Production is the target because the slot name looks familiar.
- Sticky settings are correct because the app starts.
- A 200 on `/` proves the workload dependency path is healthy.
- Swap with preview is available in every app configuration.
- Documentation proves this app's slot state, approval, or rollback readiness.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- App identity, slot inventory, source/target pair, App Service plan tier, and production target designation.
- App settings, connection strings, slot-setting flags, auth, managed identity, VNet integration, custom domains, TLS, IP restrictions, diagnostic settings, and scale settings.
- Warmup path, expected statuses, application initialization, root-path fallback behavior, recent restarts, and activity log swap events.
- Pending preview state, reset command target, complete command target, immediate rollback command target, and post-swap health checks.
- Approval record, impact summary, maintenance window, rollback owner, and verification evidence.

## When to push back

- The source slot, target slot, approval state, or rollback owner is ambiguous.
- Sticky-setting or warmup evidence is missing for production-impacting apps.
- The user wants to paste publish profiles, connection strings, tokens, certificates, or raw environment dumps.
- The requested action would complete or reset a live swap without evidence and explicit approval.
