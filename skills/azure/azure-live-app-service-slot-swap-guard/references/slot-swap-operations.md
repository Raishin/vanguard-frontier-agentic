# Azure App Service Slot Swap Operations

Use this reference for current, source-grounded service behavior and the hard live-operation gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating a slot as a passive artifact rather than a live app with its own hostname.
- Swapping directly to production without proving production is the target slot.
- Ignoring sticky app settings, connection strings, managed identity, VNet integration, and IP restrictions.
- Assuming swap with preview is always available even when site authentication blocks it.
- Calling rollback safe without proving immediate same-slot swap-back and health checks.

## Officially grounded service shape

Microsoft Learn evidence says App Service deployment slots are live apps and swaps apply target-slot settings to source instances, restart and warm source instances, then switch routing. Swap with preview pauses after target settings are applied so the operator can validate before completing or resetting. Sticky slot settings and warm-up app settings decide whether secrets, connection strings, networking-sensitive settings, and health paths behave safely during swap.

- Deployment slots require supported App Service plan tiers and have plan-specific limits.
- During swap, source slot is prepared with target settings before routing changes.
- Swap with preview allows validation after settings apply and before routing cutover.
- Warm-up uses root or configured warm-up paths and statuses; misconfigured rewrites can break warm-up.
- Rollback is normally an immediate swap of the same two slots, but stateful side effects still need separate handling.

## Non-negotiable design rules

- Confirm app, source slot, target slot, production target, plan tier, and active principal before any operation.
- Diff slot-specific settings and connection strings without printing values.
- Require warm-up path, expected status codes, and health criteria before completion.
- Use preview for mission-critical swaps unless a documented limitation prevents it.
- Track pending preview state; complete or reset exactly once.

## Minimal safe implementation flow

- Scope app, resource group, source slot, target slot, hostname, owners, and rollback owner.
- Collect slot config, sticky settings, route percentages, auth, warm-up settings, health endpoint, and recent activity log evidence.
- Run or request preview only after explicit approval and preflight evidence.
- Validate source slot under target settings, then either complete or reset.
- Verify production health and document rollback posture after cutover.

## High-risk assumptions to kill

- A successful slot swap does not prove dependency, identity, or network readiness; it proves only that App Service completed its documented routing/configuration sequence.
- Swap with preview is not universally available; site authentication on either slot blocks that path, so the guard must choose reset/swap controls based on documented constraints.
- Sticky settings are not optional hygiene. Missing slot-specific secrets, connection strings, event bindings, or networking settings can turn a clean platform swap into a production incident.
- Rollback by swapping back does not undo database migrations, queue consumers, external callbacks, or other stateful side effects.
- Warm-up success based on the root path is weak evidence unless the app owner has mapped it to real readiness.

## Safe command/code verification targets

- Check source/target slot names, production target, traffic percentages, and pending swap state before issuing preview, swap, or reset.
- Verify sticky app settings and connection strings by name only; never print values.
- Confirm warm-up path/status settings and recent activity-log entries for slot-swap operations.
- Validate source-slot health after preview and production health after completion with app-owner-approved endpoints.
- Keep reset and same-slot swap-back commands prepared before cutover, with stateful side effects listed separately.

## Safe verification targets

- Production is target slot and source slot is healthy before preview.
- Sticky settings cover secrets, connection strings, event bindings, auth-sensitive and network-sensitive settings.
- Warm-up path and statuses match real readiness, not just HTTP 200 from a shallow page.
- Activity log and app logs can identify swap failures.
- Rollback command and success criteria are available before action.

## When to push back

- The requested operation skips preview and health checks for a critical app.
- Settings diff includes unknown secret or network behavior.
- Authentication, warm-up, or local-cache behavior makes preview unsafe or unsupported.
- No owner accepts stateful side effects outside slot content/config.
