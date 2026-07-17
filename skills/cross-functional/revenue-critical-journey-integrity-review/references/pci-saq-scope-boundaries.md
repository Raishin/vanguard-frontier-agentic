# PCI DSS SAQ scope boundaries

## Why this matters

Which Self-Assessment Questionnaire (SAQ) applies to a merchant is driven by
how the code actually stores, processes, and transmits cardholder data — not
by what the merchant intends or believes. A redirect/hosted-page integration,
a direct-post custom form, and server-side card storage are three different
risk postures with three different SAQ types, and treating them as
interchangeable is a scope-misjudgment finding in its own right. This skill
gives an advisory opinion on which SAQ the integration model in the code
points to; it never issues a compliance determination.

## NORMATIVE: SAQ eligibility is scoped by integration model, and must be confirmed with the acquirer

Per PCI SSC FAQ 1443 (`documentation-based`): SAQs are scoped by how an entity
stores, processes, and/or transmits cardholder data, and each SAQ type (A,
A-EP, B, B-IP, C, C-VT, D, …) defines specific eligibility criteria and
approved system types. A merchant must confirm SAQ eligibility with its
acquirer or the payment brand before self-assessing against a given SAQ — the
eligibility criteria in the SAQ itself are necessary but not sufficient; the
acquirer/brand relationship is the actual authority.

This agent's SAQ-scope opinion is **advisory input to that confirmation
process**, not a substitute for it, and never a compliance attestation.

## Reviewer evidence criteria: mapping integration model to candidate SAQ

Form the opinion only against the payment integration model actually present
in the code, not against what the merchant states elsewhere:

- **Fully outsourced redirect or iframe (hosted payment page)** — the
  merchant's page never receives, touches, or transmits cardholder data; the
  browser is redirected to, or an iframe is served entirely from, the payment
  processor's domain. Evidence: no card-data form fields rendered by
  merchant-controlled code; the only merchant-side artifact is a link/redirect
  or an iframe `src` pointing at the processor's hosted page. **Typically maps
  to SAQ A.**
- **Direct-post / merchant-served payment form** — the merchant's own page
  renders the card-data input fields (even if the submission target or an
  embedded field library posts the values onward without the merchant's
  server persisting them), so cardholder data is present in the merchant's
  browser context. Evidence: merchant-authored HTML/JS renders PAN/CVV/expiry
  input elements, or a client-side script the merchant's page loads has the
  ability to observe or intercept those fields before submission. **Typically
  maps to SAQ A-EP.**
- **Server-side storage or processing of cardholder data** — the merchant's
  backend receives, stores, transmits, or processes the PAN (or full track
  data) directly, rather than only ever exchanging tokens/references with the
  processor. Evidence: server-side code paths, database columns, logs, or API
  payloads that carry raw PAN/CVV. **Maps to SAQ D.**

Any of these mappings is an `inference` from the code as reviewed — it is not
proof that the live, deployed system matches the code path examined, and it
is not proof the merchant's actual production configuration is what the
repository suggests.

## NORMATIVE: the 31 March 2025 changes and what they did — and did not — change

Per PCI SSC's official blog post "Important Updates Announced for Merchants
Validating to Self-Assessment Questionnaire A" (`documentation-based`):

- PCI DSS v4.0.1 requirements **6.4.3** (payment-page script inventory,
  authorization, and integrity) and **11.6.1** (a change-and-tamper-detection
  mechanism for payment pages) became effective **31 March 2025**. These are
  requirements of the PCI DSS standard itself, not of any one SAQ.
- Effective with the **January 2025 SAQ A revision**, PCI SSC removed
  requirements 6.4.3, 11.6.1, and 12.3.1 from the SAQ A questionnaire's own
  checklist and replaced them with a new SAQ A **eligibility criterion**: the
  merchant must attest that its site "is not susceptible to attacks from
  scripts that could affect the merchant's e-commerce system(s)."

**CORRECT THE RECORD:** a claim that "SAQ A merchants don't need script
security" is wrong at the standard level. The underlying PCI DSS v4.0.1
requirements 6.4.3 and 11.6.1 remain in effect and unchanged by this revision
— what changed is narrower: those two requirements were removed from the SAQ
A questionnaire's checklist of items to individually validate, and replaced
with an eligibility attestation that the merchant's site is not susceptible to
script-based attacks in the first place. A merchant who cannot truthfully make
that attestation is not eligible for SAQ A at all, regardless of having an
outsourced redirect/iframe integration. This is exactly the kind of
architecture-level scope judgment this agent should surface as a finding when
a reviewed integration's script exposure looks inconsistent with an SAQ A
eligibility claim.

## RECOMMENDATION: how to document the integration model for a scope opinion

- State the integration model observed (redirect / iframe / direct-post /
  server-side) with the specific file(s) or code path(s) as evidence, not a
  general description of the vendor's product.
- Name the candidate SAQ and label the statement advisory, e.g.: "Based on the
  hosted-redirect integration observed in `checkout/pay.js`, this points to
  SAQ A eligibility, pending confirmation with the acquirer and pending the
  merchant's script-security eligibility attestation; this is advisory input,
  not a compliance determination."
- Flag, as a distinct finding, any case where the code shows script inclusion
  or third-party tag/pixel loading on a payment page in a redirect/iframe
  integration otherwise claimed as SAQ A-eligible — this bears directly on the
  SAQ A eligibility attestation above, independent of any A-EP/D question.
- Do not state or imply that passing this review satisfies 6.4.3 or 11.6.1;
  those are PCI DSS requirements validated through the merchant's actual PCI
  assessment process, not through this skill.

## Applicable versions

- PCI DSS v4.0.1.
- The January 2025 SAQ A revision (reflecting the 31 March 2025 effective date
  for requirements 6.4.3 and 11.6.1 across PCI DSS v4.0.1 generally).
- SAQ eligibility criteria and questionnaire content are revised by PCI SSC
  periodically; re-verify against the current PCI SSC document library before
  relying on a specific SAQ's checklist contents beyond what is stated here.

## Sources

- [PCI SSC FAQ 1443 — SAQ scoping and eligibility](https://www.pcisecuritystandards.org/faqs/1443/) — supports the integration-model-to-SAQ mapping (redirect/iframe → SAQ A, direct-post → SAQ A-EP, server-side storage/processing → SAQ D) and the requirement to confirm eligibility with the acquirer/payment brand.
- [PCI SSC official blog — Important Updates Announced for Merchants Validating to Self-Assessment Questionnaire A](https://blog.pcisecuritystandards.org/important-updates-announced-for-merchants-validating-to-self-assessment-questionnaire-a) — supports the 31 March 2025 effective date for PCI DSS v4.0.1 requirements 6.4.3 and 11.6.1, the January 2025 SAQ A revision removing 6.4.3/11.6.1/12.3.1 from the SAQ A checklist, and the replacement script-security eligibility criterion.

Last verified: 2026-07-16.
