# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the payment page under review actually raises.

## Prerequisites

- Confirm the page under review actually collects or handles cardholder data: a checkout form, a saved-card management page, or any page that loads Stripe.js or renders card-entry fields. If no such page exists in scope, this skill does not apply.
- Identify the Stripe integration shape in use: `@stripe/stripe-js` + `Elements` (`CardNumberElement`/`CardExpiryElement`/`CardCVCElement`), the unified `Payment Element`, or a legacy/custom card-collection integration. The safe idiom and the specific API calls to verify differ by shape.
- Remember the scope boundary throughout: this is a **frontend-only** PCI-DSS review. Do not extend findings into server-side cardholder-data-environment segmentation, network firewalling, key management, or encryption-at-rest — those require a dedicated infrastructure/compliance review and are out of scope here.

## Workflow

1. **Locate every card-entry surface.** For each checkout/payment form, identify whether card number, expiry, and CVV are collected via Stripe hosted fields (`CardNumberElement`, `CardExpiryElement`, `CardCVCElement`, or `PaymentElement`) or via manual `<input>` elements. See `references/hosted-fields-and-tokenization.md` for the `raw-pan-input` and `missing-iframe-isolation` decision tree.
2. **Trace client-side persistence and analytics calls.** Grep for `localStorage.setItem`, `sessionStorage.setItem`, in-memory app `store` writes, and analytics/logging calls (`analytics.track`, `console.log`, error-reporting SDK calls) reachable from the payment form's state. For each, inspect the shape of the persisted/logged object for PAN, CVV, or other cardholder data fields. See `references/hosted-fields-and-tokenization.md` for the `card-data-persistence` decision tree.
3. **Trace network calls from the payment form.** For each `fetch`/`XMLHttpRequest`/form submission triggered by the payment form, determine whether raw card fields (PAN, CVV, expiry) are POSTed directly to a first-party endpoint, or whether a Stripe tokenization call (`stripe.createToken(cardElement)`, `stripe.confirmPayment()`, `stripe.confirmCardPayment()`) runs first and only the resulting token/PaymentIntent/PaymentMethod ID is POSTed. See `references/hosted-fields-and-tokenization.md` for the `self-posted-card-data` decision tree.
4. **Enumerate every script tag loaded on the payment page.** For each third-party `<script src="...">`, check for an `integrity` attribute (Subresource Integrity) and a `crossorigin` attribute. See `references/script-integrity-and-scope.md` for the `unsigned-third-party-script` decision tree.
5. **Assess SAQ-A scope-reduction plausibility (if asked).** If the user asks about SAQ-A eligibility, check whether card data collection is fully outsourced to Stripe hosted fields/Payment Element with no raw card data ever touching the merchant's own JavaScript or servers. See `references/script-integrity-and-scope.md`.
6. **Produce ranked findings** using the output contract below.

## Decision tree

- Card number/expiry/CVV collected via a manual `<input>` element (e.g., `<input type="text" id="cardnumber" />`) with application JS reading `.value` → **HIGH** finding, `raw-pan-input` and `missing-iframe-isolation`. The safe pattern is `<CardNumberElement />`/`<PaymentElement />`.
- Card number/expiry/CVV collected via `<CardNumberElement />`, `<CardExpiryElement />`, `<CardCVCElement />`, or `<PaymentElement />` → not a finding; these are Stripe's iframe-rendered hosted fields, and the browser's same-origin policy prevents merchant JS from reading iframe content.
- A `localStorage`/`sessionStorage`/store/analytics call persists an object containing PAN, CVV, or full card data (even if named generically, e.g. `paymentInfo`) → **HIGH** finding, `card-data-persistence`. The safe pattern persists only a tokenized `paymentMethod` ID.
- A `localStorage`/`sessionStorage`/store/analytics call persists only a Stripe token, PaymentIntent ID, or PaymentMethod ID (no raw card fields) → not a finding.
- A first-party POST body is constructed from raw card fields (`pan`, `cardNumber`, `cvv`, `exp`) without a preceding Stripe tokenization call → **HIGH** finding, `self-posted-card-data`.
- A first-party POST body is constructed from a Stripe token (`token.id`) or PaymentIntent/PaymentMethod confirmation result, with the tokenization call (`stripe.createToken`, `stripe.confirmPayment`, `stripe.confirmCardPayment`) preceding it on the same path → not a finding.
- A third-party `<script src="...">` on the payment page has no `integrity` attribute → **HIGH** finding, `unsigned-third-party-script` (PCI-DSS v4 6.4.3, standard-based).
- A third-party `<script src="..." integrity="sha256-..." crossorigin="anonymous">` on the payment page → not a finding.
- User asks about SAQ-A scope reduction and card collection is fully outsourced to Stripe hosted fields/Payment Element with no raw card data touching merchant JS or servers → plausible SAQ-A alignment, label `standard-based`, and note this skill does not verify the merchant's full SAQ-A questionnaire or attestation status.
- User asks about SAQ-A scope reduction and any `raw-pan-input`, `card-data-persistence`, or `self-posted-card-data` finding is present → SAQ-A eligibility is not plausible on the frontend evidence reviewed; flag the specific defect blocking it.

## Output contract

Every response from this skill must return:

1. **Scope** — the payment-collection page(s), card-entry form markup, script tags, and/or client-side persistence/network calls reviewed.
2. **Ranked findings** — each with file:line, defect category (`raw-pan-input` / `card-data-persistence` / `self-posted-card-data` / `unsigned-third-party-script` / `missing-iframe-isolation`), the concrete data-flow trace (the raw input read, the persistence/analytics call, the POST body construction, or the missing integrity attribute), and a fix sketch matching Stripe's documented pattern.
3. **Iframe-isolation status per PAN/CVV-handling finding** — an explicit statement of whether the value stays inside a Stripe-controlled iframe (safe) or is reachable by merchant JavaScript (unsafe); never infer isolation exists without tracing it.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, `standard-based`, or `inference`. Label structural risk findings as structural risk — do not imply confirmed exploitation or confirmed non-compliance without live evidence (e.g., a captured network trace, an actual PCI assessor finding).
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — always restate explicitly that this is not a full PCI-DSS audit: server-side CDE segmentation, network controls, key management, and non-script requirement families are out of scope.

## When to push back

Push back if the user asks to:

- approve a raw `<input>` card field because "we sanitize/mask it before sending" — masking display does not change that raw PAN/CVV briefly exists in merchant-readable DOM/JS state; only Stripe's iframe-isolated hosted fields keep it out of merchant JS entirely,
- treat a third-party script as safe because "it's from a reputable vendor" without a visible `integrity` attribute — vendor reputation is not equivalent cryptographic verification under PCI-DSS v4 6.4.3,
- skip the client-side persistence check because "we only store it in memory, not localStorage" — an in-memory app `store` holding raw PAN/CVV is still a `card-data-persistence` finding if it is reachable by other application code, logging, or a crash-reporting SDK,
- call a page "SAQ-A eligible" solely because it uses Stripe.js somewhere on the page — SAQ-A eligibility requires that raw card data never reaches merchant-controlled JS or servers at all; a page that uses Stripe.js for one flow but also POSTs raw card fields elsewhere does not qualify,
- treat this review as a full PCI-DSS compliance sign-off — this skill covers only the frontend script/tokenization slice; state the full-audit exclusion explicitly in every response.
