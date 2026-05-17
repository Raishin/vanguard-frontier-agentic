# Marketing Maestro

A routing skill that classifies marketing-governance review tasks and dispatches them to the narrowest available specialist. Maestro never answers questions directly; it classifies domains, selects agents, and synthesizes outputs.

## Allowed tools

`Agent` `Skill` `Read` `Grep` `Glob`

## Usage

**Single domain:** Provide a task with a clear governance signal (e.g., "Do my analytics tags fire before the consent banner is accepted?"). Maestro routes to `marketing-consent-data-collection-review-agent`.

**Multi-domain:** Provide a task spanning two or more domains (e.g., "Audit our consent banner and check whether our pixels leak email addresses"). Maestro routes to `marketing-consent-data-collection-review-agent` and `marketing-pixel-data-leakage-review-agent` in parallel.

## Specialists (v1)

| Agent ID | Domain |
|---|---|
| `marketing-consent-data-collection-review-agent` | Consent and data-collection posture (GDPR/ePrivacy/CCPA) |
| `marketing-pixel-data-leakage-review-agent` | Advertising-pixel personal-data leakage to ad networks |
| `martech-access-governance-review-agent` | Least-privilege access governance across the martech stack |

## Trust posture

Read-only. No live-guard agents exist in v1. Mutation requests are refused and escalated to a human operator. No real visitor data, credentials, API keys, or tenant data accepted at any point in the routing chain.

See [SKILL.md](SKILL.md) for the full routing protocol and response shape.
