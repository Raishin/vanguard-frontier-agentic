# Third-Party Script Governance and Subresource Integrity

Use this reference only when the review scope includes a third-party/CDN `<script src>`, a tag-manager or marketing-loader script injection path, or a dynamic `document.createElement('script')` call fed by remote/untrusted config. Grounded in the MDN Subresource Integrity (SRI) guide and the MDN CSP `script-src` reference (standard-based: W3C Subresource Integrity specification; documentation-based: MDN SRI and CSP guides). This is a supply-chain concern distinct from the sink-taint review in `references/dom-xss-sink-source-taxonomy.md` — the question here is not "does tainted data reach a sink" but "can the code that *executes* on this page be silently substituted by a compromised or malicious third party."

## What people get wrong

The naive assumption is:

> "It's just an analytics/marketing snippet from a reputable CDN, and it's loaded over HTTPS — that's safe enough."

Wrong. HTTPS proves transport confidentiality and that the response came from the domain named in the URL; it proves nothing about what that domain currently serves. A compromised CDN account, a hijacked DNS record, a malicious dependency update on the vendor's side, or a MITM on a network the vendor's own infra trusts can all cause the *exact same URL* to serve attacker-controlled JavaScript that executes with full access to the page's origin — cookies, DOM, `localStorage`, everything. Subresource Integrity exists specifically to close this gap: it lets the browser verify the fetched bytes match a hash pinned by the page author, independent of which origin served them.

## Officially grounded facts (MDN Subresource Integrity + CSP)

- **SRI is a browser-enforced hash check** (standard-based, W3C SRI spec / MDN): when a `<script>` or `<link>` element carries an `integrity` attribute, the browser computes a cryptographic digest of the fetched resource and refuses to execute/apply it if the digest does not match. The `integrity` value supports SHA-256, SHA-384, or SHA-512, expressed as `<algorithm>-<base64-hash>`, and multiple space-separated hashes can be listed as fallbacks.
- **`integrity` requires a paired `crossorigin` attribute** (documentation-based, MDN SRI guide): for a cross-origin `<script src>`, the browser only performs the SRI comparison if the fetch is made in CORS mode, which requires `crossorigin="anonymous"` (or `"use-credentials"` when appropriate) on the element. An `integrity` attribute present without a `crossorigin` attribute on a cross-origin resource does not get verified as expected — the two attributes must be reviewed as a pair, not independently.
- **A broad CSP `script-src` allowlist does not substitute for SRI** (documentation-based, MDN CSP `script-src` reference): CSP's `script-src` restricts *which origins* may serve script; it says nothing about whether the *content* served by an allowed origin is the content the developer intended. `script-src https://cdn.example.com` or the broader `script-src https:` both permit the browser to execute whatever the named origin(s) currently return — if that origin is compromised, CSP does not detect it, because CSP checks origin, not content hash. Only SRI (or Trusted Types origin/content enforcement) closes that gap.
- **Dynamically created `<script>` elements do not get SRI verification for free** (documentation-based, MDN `HTMLScriptElement`): setting `script.integrity` on a script element created via `document.createElement('script')` is supported by the platform, but the far more common real-world pattern — a tag-manager or marketing loader that builds a `<script>` element and sets `.src` to a URL sourced from a remote config/API response, then appends it to the DOM — frequently omits `integrity` entirely, because the loader does not know the hash of a URL it only learns at runtime. In that shape, the safeguard has to shift from a hash check to an **origin allowlist check** performed in code before the `src` is ever assigned.
- **Trusted Types `createScriptURL` is the platform hook for enforcing that allowlist** (standard-based, W3C Trusted Types spec): when CSP's `require-trusted-types-for 'script'` is active, the platform requires any `HTMLScriptElement.src` assignment (and other script-URL sinks) to go through a `TrustedScriptURL` produced by a registered policy's `createScriptURL` callback. A policy that validates the incoming URL's origin against a fixed allowlist before returning it — and throws or rewrites otherwise — is the documented way to make a remote-config-driven script loader safe. A policy whose callback returns the input unmodified is a permissive pass-through and provides no protection, per the general Trusted Types pass-through caveat already covered in `references/trusted-types-enforcement.md`.

## Non-negotiable design rules

### 1. Every third-party/CDN `<script src>` must carry both `integrity` and `crossorigin`, or be a confirmed finding

Grep every `<script src="https://...">` (or `<link rel="...">` for stylesheets/modulepreload where applicable) pointing at a different origin than the page itself. Absence of `integrity` is a confirmed supply-chain finding regardless of the vendor's current reputation — SRI reviews the *mechanism*, not the *vendor*. `integrity` present without `crossorigin` on a cross-origin element is also a confirmed finding: the pairing is required for the check to actually run.

### 2. A dynamic `document.createElement('script')` path fed by remote config needs an origin-allowlist check, not just SRI

If a config value, API response, or tag-manager payload supplies the URL assigned to `script.src`, SRI is not the applicable control (the hash is not known ahead of time). Instead confirm the code validates the URL's origin against a fixed, statically defined allowlist before assignment — ideally enforced structurally via a Trusted Types `createScriptURL` policy under `require-trusted-types-for 'script'`, not merely as an inline `if` check that a future refactor could silently drop.

### 3. Tag-manager and marketing-loader injection is a first-class supply-chain path, not an edge case

Analytics tags, marketing pixels, and A/B-testing snippets routinely fetch a remote configuration document and then dynamically inject one or more `<script>` elements based on it. Treat this pattern as in-scope for review whenever present: identify where the config originates (first-party API vs. third-party vendor endpoint), whether that endpoint is authenticated/integrity-protected in its own right, and whether the resulting script URLs are constrained to an expected set of origins before injection.

### 4. A broad or wildcard `script-src` in CSP compounds an SRI gap — call out both, do not let one substitute for the other

If the CSP audit (see `references/csp-directive-review.md`) finds `script-src` scoped broadly (e.g., a wildcard subdomain like `https://*.cdn.example.com`, or `https:` alone) *and* the third-party scripts loaded under that policy lack `integrity`, state both findings explicitly and note they compound: CSP is not narrowing which origins can serve script, and SRI is not verifying what those origins actually return.

### 5. `'unsafe-inline'` defeats third-party script governance the same way it defeats sink review

If `'unsafe-inline'` is present in `script-src`, any injected inline `<script>` block — including one written by a compromised third-party loader — executes regardless of SRI or origin-allowlist controls applied to `<script src>` elements. Note this compounding relationship rather than treating SRI review and the `'unsafe-inline'` CSP finding as unrelated.

## Safe idiom shapes (illustrative, not a template to copy verbatim)

Static third-party script with SRI:

```html
<script
  src="https://cdn.example.com/analytics.js"
  integrity="sha384-BASE64_HASH_PLACEHOLDER"
  crossorigin="anonymous">
</script>
```

Dynamic script injection with an origin-allowlist enforced via Trusted Types:

```js
const allowedScriptOrigins = ['https://cdn.example.com'];
const scriptPolicy = trustedTypes.createPolicy('vendor-script-loader', {
  createScriptURL(url) {
    const parsed = new URL(url, location.href);
    if (!allowedScriptOrigins.includes(parsed.origin)) {
      throw new Error('Blocked script URL outside allowlist: ' + parsed.origin);
    }
    return url;
  },
});

const script = document.createElement('script');
script.src = scriptPolicy.createScriptURL(configFromRemote.scriptUrl);
script.crossOrigin = 'anonymous';
document.body.appendChild(script);
```

Anti-pattern (common in the wild — do not approve):

```html
<script src="https://cdn.example.com/analytics.js"></script>
```

```js
const script = document.createElement('script');
script.src = configFromRemote.scriptUrl; // no allowlist check, no Trusted Types policy
document.body.appendChild(script);
```

Both anti-patterns execute whatever the named origin (or config-supplied URL) currently returns, with no mechanism to detect substitution.

## Verification targets

- Grep every `<script src="http` (or `https`) across templates/rendered HTML for a cross-origin target; confirm `integrity` and `crossorigin` are both present on each match.
- Grep for `document.createElement('script')` and trace every `.src =` assignment on the resulting element back to its data source; if the source is a config object, API response, or tag-manager payload, confirm an origin-allowlist check or a Trusted Types `createScriptURL` policy sits between the untrusted value and the assignment.
- Cross-reference any CSP `script-src` finding from `references/csp-directive-review.md` — a broad allowlist there compounds with a missing-SRI finding here.
- If `require-trusted-types-for 'script'` is active, confirm the registered policy's `createScriptURL` callback actually validates origin rather than returning the input unmodified (same pass-through caveat as `references/trusted-types-enforcement.md`).

## When to push back

Push back if the user asks to:

- approve a third-party `<script src>` because "the vendor is reputable" without `integrity`/`crossorigin` present — reputation is not a substitute for a browser-enforced hash check,
- treat a broad CSP `script-src` allowlist as sufficient protection against a compromised or malicious third-party script served from an allowed origin,
- ship a tag-manager or marketing-loader integration that injects scripts from a remote-config-supplied URL with no origin-allowlist check "because it's just analytics",
- add a Trusted Types `createScriptURL` policy that returns the input URL unmodified and call the supply-chain risk mitigated.
