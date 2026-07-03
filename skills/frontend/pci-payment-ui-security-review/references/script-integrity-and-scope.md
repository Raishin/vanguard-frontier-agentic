# Script Integrity and SAQ-A Scope Reduction

Use this reference when reviewing third-party script tags loaded on a payment-collection page for Subresource Integrity, or when assessing whether a page's architecture plausibly supports an SAQ-A scope-reduction claim.

## What people get wrong

The naive assumption is:

> "This third-party script is from a well-known vendor's CDN over HTTPS, so it's safe to load on the checkout page."

Wrong for PCI-DSS purposes. HTTPS protects the script in transit from the CDN to the browser; it does not protect against the CDN itself being compromised, the vendor's build pipeline being compromised, or a supply-chain attack that swaps the script's content while its URL stays identical (the class of attack behind real-world Magecart-style card-skimming incidents). Subresource Integrity closes exactly this gap: it lets the browser refuse to execute a script whose fetched content does not match a cryptographic hash pinned at authorization time.

## PCI-DSS v4 grounding (standard-based)

**6.4.3 — Script Inventory, Authorization & Integrity Control:**

- All scripts loaded on payment collection pages must be inventoried and approved.
- Scripts must be authorized before loading (script authorization).
- Integrity verification is required via Subresource Integrity (SRI) or an equivalent cryptographic verification mechanism (e.g., a SHA-256 hash).

**11.6.1 — Change Detection on Payment Pages:**

- Automated change-detection/alerting mechanisms are required for unauthorized code additions or modifications to payment pages.
- This is intended to detect malicious script injections, DOM tampering, and compromise attempts in near-real time, independent of the SRI control above.

Both of these are standard requirement citations (`standard-based`) — this skill can confirm from frontend code whether SRI is present on a given script tag, but it cannot confirm from static frontend review alone whether a merchant has an operating script-inventory/authorization *process* or a functioning 11.6.1 change-detection deployment. Note any such gap as an open question rather than asserting non-compliance.

## Defect class: `unsigned-third-party-script`

Third-party script loaded without Subresource Integrity.

- **Dangerous:**
  ```html
  <script src="https://cdn.example.com/analytics.js"></script>
  ```
  No `integrity` attribute means the browser will execute whatever content the CDN serves at request time, with no cryptographic check against what was reviewed/authorized.
- **Safe:**
  ```html
  <script src="https://cdn.example.com/analytics.js"
          integrity="sha256-C6CB9UYIS9UJeqinPHWTHVqh/E1uhG5Twh76tviuROE="
          crossorigin="anonymous"></script>
  ```
  The `integrity` attribute pins a SHA-256 (or SHA-384/SHA-512) hash; the browser refuses to execute the script if the fetched bytes don't match. `crossorigin="anonymous"` is required alongside `integrity` for cross-origin script resources so the browser can perform the integrity check.
- **Verification targets:** Grep every `<script src="...">` tag rendered on a payment-collection page (including scripts injected via a tag-manager snippet, if visible in source) for the presence of an `integrity` attribute. A same-origin/first-party script (served from the merchant's own domain) is lower risk but still benefits from SRI if served via a CDN in front of first-party assets; the primary finding target is third-party origins.
- A missing `integrity` attribute on a third-party script on a payment page is a HIGH finding regardless of the vendor's reputation — reputation is not a substitute for script authorization and integrity verification under 6.4.3.

## SAQ-A scope reduction (standard-based)

SAQ-A is the PCI-DSS self-assessment questionnaire for merchants that have **fully outsourced** all cardholder data collection and processing to a PCI-validated third party (e.g., Stripe), such that the merchant's own systems never store, process, or transmit cardholder data and never touch it even transiently in the browser.

For a payment page's frontend architecture to plausibly support an SAQ-A claim:

- Every card field (PAN, expiry, CVV) must be collected exclusively via Stripe hosted fields (`CardNumberElement`/`CardExpiryElement`/`CardCVCElement`) or the unified `PaymentElement` — no manual `<input>` anywhere in the flow (would otherwise be a `raw-pan-input`/`missing-iframe-isolation` finding).
- No client-side persistence of raw cardholder data anywhere in the flow (would otherwise be a `card-data-persistence` finding).
- No first-party POST of raw card fields anywhere in the flow — only tokens/PaymentIntent/PaymentMethod IDs cross to the first-party server (would otherwise be a `self-posted-card-data` finding).

This skill can confirm the frontend evidence for or against these three conditions. It **cannot** confirm the merchant's actual SAQ-A eligibility determination or attestation — that is a compliance/assessor decision that also depends on the merchant's payment page hosting model (e.g., whether the payment page itself is served from PCI-relevant infrastructure) and other SAQ-A eligibility criteria outside this skill's frontend-only scope. Label any SAQ-A-adjacent conclusion `standard-based` and explicitly note the full-audit exclusion.

## Adversarial checklist

Before clearing a payment page's script surface:

- Does every third-party `<script src="...">` on the page carry an `integrity` attribute with a `sha256-` (or stronger) hash and a `crossorigin` attribute?
- Is there any script loaded dynamically (via `document.createElement('script')` or a tag-manager container) that bypasses static `integrity` attribution entirely? Flag this as a distinct, harder-to-verify risk — dynamically injected scripts cannot carry a static `integrity` attribute in the same way, and the review should note this as an open question requiring the tag-manager's own vendor controls to be checked separately.
- If asked about SAQ-A: do all three frontend conditions above hold, with no raw-PAN, persistence, or self-posted-data findings anywhere in the reviewed flow?
- Is there any observable client-side change-detection or integrity-monitoring code on the page (relevant to 11.6.1), or is this an open question requiring server/ops-side confirmation?
