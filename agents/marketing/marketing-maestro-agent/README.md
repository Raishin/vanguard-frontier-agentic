# Marketing Maestro

Domain router for marketing governance. Classifies the user's question and dispatches the narrowest specialist — or a parallel team of up to four — from the catalog.

---

## What it does

- Reads `skills/marketing/marketing-maestro/SKILL.md` to classify the incoming task.
- Routes to one or more marketing-governance specialists found in `catalog/agents.json`.
- Dispatches in parallel when two or more domains are involved (ceiling: 4 specialists).
- Synthesizes specialist outputs into a unified response.
- Produces a handoff packet for any mutating task and halts for human approval.

## What it does NOT do

- Answer marketing-governance questions directly.
- Call analytics, ad-platform, CMP, or CRM APIs.
- Accept, store, relay, or request real visitor data or credentials.
- Auto-dispatch any mutating or live-guard specialist.
- Use Bash, Edit, Write, or WebFetch.

---

## Bound skill

`skills/marketing/marketing-maestro/SKILL.md`

---

## Routing destinations (v1)

| Specialist | Domain |
|---|---|
| `marketing-consent-data-collection-review-agent` | Consent and data-collection posture (GDPR/ePrivacy/CCPA) |
| `marketing-pixel-data-leakage-review-agent` | Advertising-pixel personal-data leakage to ad networks |
| `martech-access-governance-review-agent` | Least-privilege access governance across the martech stack |

---

## Trust posture

- Read-only. No credentials or visitor data required or accepted.
- No mutation. No auto-dispatch of live-guard agents.
- All label claims as `live-evidence`, `documentation-based`, or `inference`.
- Handoff packet required before any mutating dispatch; human approval gate is non-negotiable.

---

## Full contract

See [AGENT.md](AGENT.md) for the complete canonical specification and [PERMISSIONS.md](PERMISSIONS.md) for the tool surface and credential refusal policy.
