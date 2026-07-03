---
name: pci-payment-ui-security-review
description: Statically review payment-page frontend code for PCI-DSS-relevant defects in the browser/DOM slice only — raw PAN collection in self-controlled inputs instead of Stripe hosted fields, card data persisted client-side or to analytics, raw card data POSTed to a first-party endpoint, and third-party scripts loaded without Subresource Integrity — grounded in Stripe's own tokenization docs and PCI-DSS v4 script-security requirements.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# PCI Payment UI Security Review

## Purpose

Review payment-collection frontend code — checkout pages, card-entry forms, and any script loaded on a page that collects cardholder data — for the frontend-specific defect classes that keep raw cardholder data out of the DOM and out of merchant-controlled JavaScript: raw PAN/CVV collection in a self-controlled `<input>` instead of Stripe's iframe-isolated hosted fields, card data persistence to `localStorage`/`sessionStorage`/analytics/logs, raw card data POSTed to a first-party endpoint instead of tokenized first, and third-party scripts loaded on the payment page without Subresource Integrity (SRI). This skill exists so the review stays anchored to the frontend tokenization boundary and script-integrity surface instead of drifting into a general "payments code review."

## When to use

Use this skill when the user asks to:

- review a checkout page or card-entry form for PCI-relevant frontend risk,
- assess whether card-number, expiry, or CVV fields are collected safely (hosted fields vs. raw `<input>`),
- audit which scripts load on a payment-collection page and whether they carry Subresource Integrity,
- assess SAQ-A scope reduction claims for a page that outsources card collection to Stripe Elements/Payment Element,
- perform a pre-launch frontend security review of a Stripe-based payment integration.

Do not use this skill for:

- a full PCI-DSS compliance audit — this skill covers **only the frontend slice** (client-side script inventory/integrity, hosted-field isolation, tokenization boundary). Server-side cardholder-data-environment (CDE) segmentation, network firewalling, key management, encryption-at-rest, access control, and the remaining PCI-DSS requirement families are out of scope; route those to a dedicated infrastructure/compliance review,
- a non-payment page with no card-data collection or third-party script surface — general frontend security review is a different scope,
- a bug that requires live traffic capture (a proxy/network trace confirming what actually left the browser) to confirm exploitation — static analysis proves the structural risk (the code path that *would* expose or persist raw PAN/CVV), not that raw card data has already left the browser in production.

## Context7 Documentation Protocol

- Resolve the Stripe library ID with `resolve-library-id` before citing any Stripe Elements/Payment Element or tokenization-mechanism claim; use `query-docs` to corroborate the specific API behavior (e.g., `stripe.createToken(cardElement)`, `stripe.confirmPayment()`, `stripe.confirmCardPayment()`) rather than relying on memory.
- Label every Stripe-API behavioral claim `documentation-based` (or `repo evidence` if confirmed directly against fetched Context7 doc content) — do not assert current Stripe API shape from training memory alone.
- Label every PCI-DSS requirement claim (6.4.3, 11.6.1, or the general SAQ-A scope-reduction rationale) `standard-based` — this skill does not verify a merchant's actual PCI compliance status or attestation; it evaluates whether the frontend code matches the documented pattern the standard and Stripe's architecture rely on.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.
- Read `package.json`/script tags first to confirm which Stripe integration is in use (`@stripe/stripe-js` + Elements, `Payment Element`, or a legacy/custom integration) — the safe idiom and API names differ; do not assume Elements is in use without confirming.

## Lean operating rules

- Raw PAN/CVV collection outside an iframe-isolated hosted field, and any client-side persistence of raw cardholder data, default to HIGH severity. This is a security-scoped skill: do not downgrade a raw-PAN-in-DOM finding to MEDIUM because "it's only in a dev/staging build" — the structural pattern is the risk, not its current deployment target.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this form might expose card data" without showing the specific `<input>` element, the specific `.value` read, or the specific POST body construction is not a valid finding — it is a guess.
- Do not flag `<CardNumberElement />`, `<CardExpiryElement />`, `<CardCVCElement />`, or `<PaymentElement />` usage as a defect — these are Stripe's iframe-rendered hosted fields; the PAN/CVV they collect stays inside iframes controlled by Stripe and is never accessible to merchant JavaScript. Only a self-controlled `<input>` (e.g., `<input type="text" id="cardnumber" />`) that application JS reads via `.value` is the raw-pan-input defect.
- Treat any `localStorage`, `sessionStorage`, in-memory app store, or analytics/logging call that could carry a PAN, CVV, or full cardholder data payload as a HIGH finding regardless of whether the value is currently populated in the traced code path — check the shape of the object being persisted, not just its current runtime value.
- Treat a first-party POST (`fetch`/`XMLHttpRequest`/form submit to a first-party endpoint) carrying raw card fields (PAN, CVV, expiry) as a HIGH finding. The safe pattern tokenizes first via the Stripe API (`stripe.createToken(cardElement)` or the Payment Element confirmation flow) and POSTs only the resulting token or PaymentIntent/PaymentMethod ID to the first-party endpoint.
- Treat any third-party `<script src="...">` loaded on a payment-collection page without an `integrity` attribute (Subresource Integrity, e.g. a `sha256-` hash) as a finding — cite PCI-DSS v4 requirement 6.4.3 (script inventory, script authorization, and integrity control) as the standard-based grounding. Do not accept "we trust this vendor" as a substitute for a visible integrity attribute or equivalent cryptographic verification.
- Stripe's hosted fields (`CardNumberElement`, `CardExpiryElement`, `CardCVCElement`, `PaymentElement`) render inside sandboxed iframes controlled by Stripe, not merchant code — the browser's same-origin policy and the iframe sandbox boundary are what keep merchant JavaScript from ever reading raw PAN/CVV values. A missing-iframe-isolation finding applies when card fields are collected via manual `<input>` elements with no sandboxed hosted-field equivalent in use.
- When reviewing broader payment-page change-detection posture, note PCI-DSS v4 requirement 11.6.1 (automated change-detection/alerting for unauthorized script modification on payment pages) as a standard-based expectation — but do not fabricate a finding about a change-detection mechanism you cannot observe in the frontend code under review; note it as an open question instead.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the defect decision tree, and the required output shape.
- [Hosted fields and tokenization boundary](references/hosted-fields-and-tokenization.md) — load when reviewing card-entry form markup, `raw-pan-input`, `card-data-persistence`, or `self-posted-card-data` defect classes.
- [Script integrity and SAQ-A scope reduction](references/script-integrity-and-scope.md) — load when reviewing third-party script tags for Subresource Integrity, or assessing whether a page's use of hosted fields plausibly supports SAQ-A scope-reduction claims.

## Response minimum

Return, at minimum:

- the payment-collection page(s), card-entry form markup, script tags, and/or client-side persistence/network calls in scope,
- ranked findings with file:line evidence, defect category (`raw-pan-input`, `card-data-persistence`, `self-posted-card-data`, `unsigned-third-party-script`, or `missing-iframe-isolation`), the concrete data-flow trace (the raw `<input>`/`.value` read, the `localStorage`/analytics call, the POST body construction, or the missing `integrity` attribute), and a fix sketch matching Stripe's documented idiom (`CardNumberElement`, `PaymentElement`, `stripe.createToken`, or an `integrity`/SRI attribute),
- for every finding involving PAN/CVV/cardholder data exposure, an explicit statement of whether the value stays inside a Stripe-controlled iframe (safe) or is reachable by merchant JavaScript (unsafe) — never approve on the assumption isolation exists without tracing it,
- evidence level per finding (`repo evidence`, `documentation-based`, `standard-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited or confirmed non-compliant,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover — explicitly restate that this is **not a full PCI-DSS audit**: server-side cardholder-data-environment segmentation, key management, network controls, and non-script requirement families are out of scope and require a dedicated compliance/infrastructure review.
