# Marketing-Governance Provider — Roadmap

Status: planning · Last verified: 2026-05-17

This document records the shipped v1 of the `marketing` provider and the
board-vetted candidate pipeline for v2. It exists so that future additions
stay scoped to the same thesis and do not drift into generative marketing
content.

## Thesis

The `marketing` provider covers the **marketing technology compliance and
security surface** — not generic marketing content. Every asset is a
**static-review** skill: it ingests a sanitized config or artifact and emits
severity-labelled findings with an evidence basis. Generative advisor skills
(copywriting, campaign design, SEO content) are explicitly out of scope —
they do not fit the repository's review modality, zero-trust stance, or the
closed skill `category` enum.

## Shipped — v1

| Skill / agent | Category | Reviews |
|---|---|---|
| `marketing-consent-data-collection-review` | compliance | CMP banner, tag-manager container, Consent Mode, cookie policy |
| `marketing-pixel-data-leakage-review` | security | Advertising pixels and conversion-event payloads |
| `martech-access-governance-review` | security | OAuth grants, API keys, CRM and marketing-automation roles |
| `marketing-maestro` | ai | Per-domain router across the three reviews above |

Each skill ships a 1:1 companion agent across all seven harnesses. The
maestro adds a `marketing-governance-reviewer` install role and CI-validated
routing fixtures under `tests/fixtures/marketing-maestro-routing/`.

## Candidate pipeline — v2

The ten candidates below were generated from current regulatory and martech
research and filtered through a five-persona adversarial review (CFO, CISO,
Chief Privacy Officer, Head of Marketing Ops, Product VP). Each survivor
passed all five. They are ranked by business value.

| Rank | Proposed skill id | Category | Artifact reviewed | Primary standard |
|---|---|---|---|---|
| 1 | `martech-gpc-signal-honoring-review` | compliance | Tag-manager container + CMP opt-out config | CPPA enforcement sweep (2025), CA AB 566 |
| 2 | `email-sender-authentication-review` | compliance | DNS TXT records (SPF/DKIM/DMARC/BIMI) | Google/Yahoo bulk-sender rules, PCI DSS v4.0 |
| 3 | `programmatic-supply-chain-integrity-review` | finops | ads.txt / app-ads.txt + sellers.json | IAB Tech Lab ads.txt 1.1, MRC IVT guidelines |
| 4 | `ai-advertising-targeting-fairness-review` | ai | Ad-platform audience targeting config | FTC AI bias guidance, Fair Housing Act, EU AI Act Art. 5 |
| 5 | `eu-ai-act-marketing-system-review` | compliance | AI system description card | EU AI Act Arts. 5, 6, 14, Annex III |
| 6 | `lookalike-audience-upload-compliance-review` | data | Audience upload field-mapping + consent basis | GDPR Art. 5, CCPA/CPRA §1798.100 |
| 7 | `marketing-email-list-retention-review` | compliance | CRM/ESP list segment metadata + retention policy | GDPR Arts. 5 & 17, CASL §6 |
| 8 | `influencer-disclosure-compliance-review` | compliance | Influencer campaign audit pack | FTC 16 CFR Part 255 Endorsement Guides |
| 9 | `marketing-conversion-flow-dark-pattern-review` | compliance | UX flow spec / annotated wireframe | FTC Negative Option Rule, CPRA §1798.140(l) |
| 10 | `analytics-data-minimization-review` | data | GA4 / analytics platform config export | GDPR Art. 5(1)(c)(e), EU DPA enforcement |

### Board-rejected candidates and why

- **CAN-SPAM / CASL commercial email review** — overlaps the shipped
  consent-data-collection skill; most ESPs auto-enforce; thin at enterprise grade.
- **Marketing-site accessibility (WCAG/ADA/EAA) review** — outside the
  security/compliance/data risk surface; applies to all digital products, not
  marketing governance specifically.
- **Cookie-wall / "pay or consent" model review** — legally unsettled
  (EDPB Opinion 08/2024 is guidance, not enforcement); too niche for
  Fortune-500 marketing ops.
- **Ad-spend cross-border transfer review** — insufficient differentiation
  from existing cloud-compliance agents.
- **Martech vendor DPA gap review** — wrong primary user; legal and
  procurement own DPA artifacts, not marketing ops.

## Build contract for v2 candidates

Any candidate promoted to a shipped skill must:

- Map to an existing `category` enum value — no new enum entries.
- Be a static review that emits severity-labelled findings, not a generative advisor.
- Review a concrete, pasteable, sanitized artifact.
- Ship a 1:1 companion agent across all seven harnesses.
- Register in `marketing-maestro`'s routing table and the
  `marketing-governance-reviewer` install role.
- Pass `npm run validate` (all gates) including regenerated maestro routing fixtures.
