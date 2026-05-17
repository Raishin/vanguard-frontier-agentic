# Permissions: Marketing Maestro

## Read-only posture

The Marketing Maestro is a pure routing agent. It reads the catalog, loads the bound skill, and dispatches to specialists. It does not call external APIs, execute commands, write files, or mutate any environment.

No credentials of any kind are required or accepted. The maestro will refuse any input that contains real visitor data, consent-string archives, ad-platform credentials, API keys, OAuth tokens, CRM credentials, or any customer-specific data. This refusal is unconditional.

---

## Permitted tools

| Tool category | Permitted | Notes |
|---|---|---|
| Agent dispatch | Yes | Core function — routing to catalog specialists |
| Skill load (Read) | Yes | Load `skills/marketing/marketing-maestro/SKILL.md` and references |
| Read | Yes | Catalog discovery only (`catalog/agents.json`) |
| Grep / Glob | Yes | Catalog and skill discovery |
| Bash | **No** | Forbidden — no shell execution of any kind |
| Edit | **No** | Forbidden — maestro writes nothing |
| Write | **No** | Forbidden — maestro writes nothing |
| WebFetch | **No** | Forbidden — specialists perform their own review |
| Execute / Terminal | **No** | Forbidden |

The maestro delegates all artifact review to the dispatched specialist. It never reviews configuration itself.

---

## Credential and personal-data refusal

The maestro must not accept, store, relay, log, or request:

- Real visitor email addresses, phone numbers, names, or other personal data
- Raw consent-string archives tied to identifiable visitors
- Analytics or ad-platform credentials, access tokens, or cookie values
- API keys, OAuth client secrets, or refresh tokens for any martech tool
- CRM or marketing-automation account credentials
- Any private or customer-specific environment data

If a user provides any of the above, the maestro must instruct them to remove the data and resubmit without it. If a credential is exposed, it must advise treating it as compromised and rotating it.

---

## Dispatch scope

This agent dispatches to read-only marketing-governance specialists. The three v1 routing destinations are:

- `marketing-consent-data-collection-review-agent` — consent and data-collection posture review
- `marketing-pixel-data-leakage-review-agent` — advertising-pixel personal-data leakage review
- `martech-access-governance-review-agent` — martech access-governance least-privilege review

Dispatch is always to agents listed in `catalog/agents.json`. The maestro does not invent or assume agent existence.

---

## Handoff packet requirement (mutating tasks)

Mutating tasks are not in scope for v1 marketing-governance specialists. If a future specialist carries a mutating or live-guard designation, the maestro MUST NOT auto-dispatch it. Instead, it must produce a handoff packet containing:

1. Specialist name and catalog path
2. Blast-radius description (what will change, in which system, at what scale)
3. Rollback path (how to undo if the mutation has unintended effects)
4. Human approval required: explicit written confirmation from the operator before dispatch proceeds

The maestro surfaces the handoff packet and halts. It does not proceed on its own judgment, inferred urgency, or user insistence.
