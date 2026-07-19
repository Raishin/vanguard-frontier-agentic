# Official sources

## Why this matters

Every processor-specific, standard-specific, or architecture-specific claim
this skill makes traces to exactly one official, primary source — never a
tutorial, a vendor marketing page, or press coverage. This ledger is that
trace: one row per source, the exact claim(s) it grounds, the source type,
the applicable version where one exists, and the verification date. A
reviewer citing this skill's guidance should be able to follow any claim back
to the primary document that supports it, and to see at a glance which
figures are verified fact versus flagged as unverified.

## NORMATIVE: every claim in this skill must resolve to a row below

No processor behavior, standard requirement, or architecture guidance may be
asserted in this skill's other reference files without a corresponding row
here. If Context7 is unavailable for a processor lookup, the fallback is the
processor's official documentation site, fetched directly and labeled
`documentation-based` rather than `context7-grounded` — never memory.

| Official URL | Claim(s) supported in this skill | Source type | Applicable version | Last verified |
|---|---|---|---|---|
| [Stripe API reference — Idempotent requests](https://docs.stripe.com/api/idempotent_requests) | A client-generated idempotency key lets a `POST` (e.g. create charge/customer) be safely retried after a connection error without duplicating the object or repeating the operation. Stripe caches the first response — including 5xx errors — for at least 24 hours per key, and returns an error if a retry sends parameters differing from the original request. | processor-docs | Current Stripe API (fetched directly; Context7 quota was exhausted — see uncertainty note below) | 2026-07-16 |
| [Stripe — Webhook best practices](https://docs.stripe.com/webhooks/best-practices) | Endpoints may receive the same event more than once. Live-mode automatic retries continue for up to 3 days with exponential backoff; sandbox retries 3 times over a few hours; manual dashboard resend is available within 15 days and CLI resend within 30 days, and a manual resend does not cancel Stripe's own automatic retry schedule. Endpoints are disabled after 3 days of continuous failure in live mode. Events can arrive out of order. Handlers must be idempotent, e.g. by logging processed event IDs and deduping on them. | processor-docs | Current Stripe webhooks documentation | 2026-07-16 |
| [PCI SSC FAQ 1443](https://www.pcisecuritystandards.org/faqs/1443/) | SAQs are scoped by how an entity stores, processes, and/or transmits cardholder data; each SAQ type (A, A-EP, B, B-IP, C, C-VT, D, …) has specific eligibility criteria and approved system types; a merchant must confirm SAQ eligibility with its acquirer or the payment brand before self-assessing. Integration model drives the candidate SAQ: fully outsourced redirect/iframe typically maps to SAQ A; direct-post/merchant-served forms typically map to SAQ A-EP; server-side storage/processing of cardholder data maps to SAQ D. | standard | PCI DSS SAQ program (current) | 2026-07-16 |
| [PCI SSC official blog — Important Updates Announced for Merchants Validating to Self-Assessment Questionnaire A](https://blog.pcisecuritystandards.org/important-updates-announced-for-merchants-validating-to-self-assessment-questionnaire-a) | PCI DSS v4.0.1 requirements 6.4.3 (payment-page script inventory, authorization, and integrity) and 11.6.1 (change-and-tamper-detection mechanism for payment pages) became effective 31 March 2025. Effective with the January 2025 SAQ A revision, PCI SSC removed 6.4.3, 11.6.1, and 12.3.1 from the SAQ A questionnaire itself and replaced them with an eligibility criterion requiring the merchant to attest its site "is not susceptible to attacks from scripts that could affect the merchant's e-commerce system(s)." Corrects the record: the underlying PCI DSS v4.0.1 requirements remain in effect — only the SAQ A checklist scope changed, contingent on the eligibility attestation. | standard | PCI DSS v4.0.1; January 2025 SAQ A revision | 2026-07-16 |
| [AWS Well-Architected Framework — Reliability Pillar: Mitigate interaction failure with retry limits](https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/rel_mitigate_interaction_failure_limit_retries.html) | A "retry storm" occurs when retries compound across multiple stack layers under failure, saturating the service with new-plus-retried requests and reducing availability. Documented mitigation is client-side exponential backoff, jitter, and a maximum retry cap. | cloud-architecture | AWS Well-Architected Framework (current) | 2026-07-16 |
| [AWS Builders' Library — Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/) | Supplementary primary source for backoff-with-jitter retry design at revenue-critical seams; used only to reinforce the Reliability Pillar guidance above, not as a source of any additional unverified claim. | cloud-architecture | AWS Builders' Library (current) | 2026-07-16 |
| [OWASP Top 10:2021 — A01:2021 Broken Access Control](https://owasp.org/Top10/A01_2021-Broken_Access_Control/) | Grounds the general principle that enforcement decisions (authorization, business-rule checks) must not rely on client-side controls — the basis for treating the server as the only enforcement boundary for client-enforced rules (price, discount, quantity, eligibility, step-completion). | standard | OWASP Top 10:2021 | 2026-07-16 |
| [OWASP Cheat Sheet Series — Input Validation Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html) | Grounds the requirement that input validation performed client-side is a usability aid, not a security control, and must be re-implemented server-side — supports the server-side re-validation guidance for client-enforced rules. | standard | OWASP Cheat Sheet Series (current) | 2026-07-16 |
| [Baymard Institute — Cart Abandonment Rate Statistics](https://baymard.com/lists/cart-abandonment-rate) | Aggregate of 50 published studies puts average documented e-commerce cart/checkout abandonment at 70.22%. In Baymard's own study of abandonment reasons, 18% of US online shoppers abandoned an order due to a "too long/complicated checkout process" and 19% cited being required to create an account. Used only as motivating business-pain figures, not as a technical claim. | research-publisher | Baymard Institute published aggregate (current) | 2026-07-16 |

## Known uncertainty / not encoded

- **Context7 quota fallback.** Per this skill's Context7 documentation
  protocol, Stripe idempotency and webhook behavior should be resolved via
  Context7 (`resolve-library-id` then `query-docs`) first. During grounding
  for this skill, Context7 returned "Monthly quota reached" for the Stripe
  lookup. The fallback used was a direct fetch of Stripe's official
  documentation pages, and those claims are labeled `documentation-based`
  rather than `context7-grounded` throughout this skill's reference files.
  Re-attempt the Context7 path when quota is available and update the labels
  above if a discrepancy is found.
- **UNVERIFIED — false-decline dollar estimates, do not encode as fact.**
  Circulating industry estimates of e-commerce revenue lost to false payment
  declines (e.g. figures on the order of ~$81B US / ~$443B global at a
  ~1.51% false-decline rate, and an older Javelin 2014 figure contrasting
  ~$118B in false declines against ~$9B in actual fraud losses) are known
  only via secondary vendor reporting (press coverage citing PYMNTS, Datos
  Insights, Cybersource, and Javelin research) and were **not** verified
  against those primary reports for this skill. These figures must **not**
  be stated as fact anywhere in this skill's guidance or findings. If
  mentioned at all, they must be labeled explicitly as unverified,
  directionally-credible industry estimates requiring primary-source
  re-verification before use — never as a hard number backing a finding or
  a business-pain claim.

## Sources

- [Stripe API reference — Idempotent requests](https://docs.stripe.com/api/idempotent_requests)
- [Stripe — Webhook best practices](https://docs.stripe.com/webhooks/best-practices)
- [PCI SSC FAQ 1443](https://www.pcisecuritystandards.org/faqs/1443/)
- [PCI SSC official blog — Important Updates Announced for Merchants Validating to Self-Assessment Questionnaire A](https://blog.pcisecuritystandards.org/important-updates-announced-for-merchants-validating-to-self-assessment-questionnaire-a)
- [AWS Well-Architected Framework — Reliability Pillar: Mitigate interaction failure with retry limits](https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/rel_mitigate_interaction_failure_limit_retries.html)
- [AWS Builders' Library — Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)
- [OWASP Top 10:2021 — A01:2021 Broken Access Control](https://owasp.org/Top10/A01_2021-Broken_Access_Control/)
- [OWASP Cheat Sheet Series — Input Validation Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html)
- [Baymard Institute — Cart Abandonment Rate Statistics](https://baymard.com/lists/cart-abandonment-rate)

Last verified: 2026-07-16.
