# CSP Directive Review

Use this reference only when auditing or authoring an actual Content-Security-Policy header or `<meta http-equiv="Content-Security-Policy">` value. Grounded in the MDN CSP header reference. Do not treat header presence as a pass — every directive must be parsed individually.

## What people get wrong

The naive assumption is:

> "The response has a `Content-Security-Policy` header, so CSP is handled."

Wrong. A CSP header is a set of independent directives, each restricting a different resource category, and a permissive value in any single directive can defeat the protection the other directives appear to provide. The recurring real failure mode: a policy that looks comprehensive (many directives listed) but includes `'unsafe-inline'` or a wildcard in `script-src`, which alone reduces the script-injection protection to near zero regardless of how strict the other directives are.

## Officially grounded directive semantics (MDN)

- **`script-src`** — controls valid sources for `<script>` elements, inline event handlers, and `javascript:` URLs. `'unsafe-inline'` allows inline `<script>` blocks and inline event-handler attributes, which defeats the primary purpose of CSP against DOM XSS: an injected `<script>` tag or `onerror` attribute would otherwise be blocked by the browser, but with `'unsafe-inline'` present it executes normally. `'unsafe-eval'` permits `eval()`, `Function()`, and string-argument `setTimeout`/`setInterval` to execute — combining this with an `eval`-class sink finding from `references/dom-xss-sink-source-taxonomy.md` means CSP provides zero mitigation for that specific finding.
- **`strict-dynamic`** — when present, propagates trust from a script loaded with a valid nonce or hash to the scripts it dynamically inserts, and (per spec behavior) causes host-source and scheme-source expressions in `script-src` to be ignored by browsers that support it. Effective only when paired with a per-response, unpredictable nonce or a hash allowlist; a static, hardcoded nonce value defeats it because an attacker can read and reuse the same nonce.
- **`object-src`** — controls `<object>`, `<embed>`, and `<applet>` sources. MDN explicitly recommends `object-src 'none'` as a baseline hardening step for most applications, because plugin content can execute script outside the page's normal script-loading path and is rarely needed by modern applications.
- **`base-uri`** — restricts URLs usable in a `<base>` element. A missing `base-uri` directive lets an attacker who can inject any HTML (even in a context otherwise well-protected by `script-src`) insert a `<base href="https://attacker.example/">` tag that silently rewrites all relative URLs on the page, redirecting script/resource loads to an attacker-controlled origin.
- **`default-src`** — the fallback for any resource-type directive not explicitly specified. A restrictive `default-src` (e.g., `'self'`) is not sufficient on its own if `script-src` is separately specified with a permissive value — explicit directives always override `default-src` for their resource type, so `default-src` alone does not prove `script-src` is safe.
- **`frame-ancestors`** — controls which origins may embed the page in a `<frame>`/`<iframe>`/`<object>`. Distinct from clickjacking-only `X-Frame-Options`; `frame-ancestors 'none'` or a specific allowlist is the CSP-native replacement and takes precedence when both are present.
- **`require-trusted-types-for`** and **`trusted-types`** — CSP-level enforcement hooks for the Trusted Types API; see `references/trusted-types-enforcement.md` for the full review of these two directives specifically.

## Non-negotiable design rules

### 1. Parse every directive value individually — do not grade the policy as a whole

A policy with 8 well-configured directives and 1 permissive `script-src 'unsafe-inline'` is not "mostly good." State the single permissive directive as its own confirmed finding at the severity that directive's gap implies, independent of how strict the others are.

### 2. `'unsafe-inline'` in `script-src` is a confirmed finding whenever DOM XSS sinks are also in scope

If the review scope includes both a CSP audit and a DOM-sink review, and the CSP has `'unsafe-inline'` in `script-src`, note explicitly that CSP provides no mitigation for any confirmed HTML-context sink finding in the same review — the two findings compound rather than one substituting for the other.

### 3. A wildcard or overly broad `script-src` source is equivalent to no restriction for practical purposes

`script-src https:` (any HTTPS origin) or `script-src *` allows loading and executing script from effectively any external origin, including an attacker-controlled one reachable over HTTPS. Treat this the same severity as `'unsafe-inline'` for a script-injection finding.

### 4. `strict-dynamic` correctness depends on nonce generation, not directive presence

Do not clear a `strict-dynamic` configuration as a pass without confirming the nonce is generated fresh per response (typically server-side, per-request) rather than hardcoded in a template or CSP meta tag. A static nonce in source is trivially readable and reusable by an attacker, providing no protection despite `strict-dynamic` being present.

### 5. CSP is a defense-in-depth layer, not a substitute for fixing a confirmed DOM XSS sink

Never recommend "add CSP" as the sole remediation for a confirmed unsanitized sink finding from `references/dom-xss-sink-source-taxonomy.md`. The sink itself must be fixed (sanitizer call or removal of the dynamic-code-execution pattern); CSP reduces blast radius if a sink is missed elsewhere, it does not replace fixing the one found.

## Minimal safe policy shape (illustrative, not a template to copy verbatim)

```
Content-Security-Policy:
  default-src 'self';
  script-src 'self' 'nonce-{PER-RESPONSE-RANDOM-VALUE}' 'strict-dynamic';
  object-src 'none';
  base-uri 'self';
  frame-ancestors 'none';
  require-trusted-types-for 'script';
```

Anti-pattern (common in the wild — do not approve):

```
Content-Security-Policy: default-src * 'unsafe-inline' 'unsafe-eval';
```

This is functionally equivalent to no CSP for script-injection purposes: it permits inline scripts, `eval`-class execution, and loading script from any origin.

## Verification targets

- Locate the effective policy: grep server/middleware config, framework security headers config, or the rendered HTML `<meta http-equiv="Content-Security-Policy">` tag.
- Split the policy string on `;` and parse each directive-value pair independently; flag every occurrence of `'unsafe-inline'`, `'unsafe-eval'`, a bare wildcard `*`, or a scheme-only source (`https:`) in `script-src` or `default-src`.
- Confirm `object-src 'none'` and `base-uri` are present; if absent, this is a confirmed finding even if no other directive is misconfigured.
- If `strict-dynamic` is present, grep the server-side code that renders the nonce value to confirm it is generated per-request (e.g., via a cryptographically random value), not a fixed string.
- If a CSP `report-to`/`report-uri` endpoint is configured, note that browser-native CSP violation reports are a distinct mechanism from typical observability instrumentation (`documentation-based, gap confirmed via Context7`: OpenTelemetry's browser SDK/instrumentation packages have no built-in CSP-violation-report receiver as of current docs) — a custom collector endpoint must ingest the browser's native report POST; do not assume an existing OpenTelemetry pipeline captures these without a purpose-built receiver.

## When to push back

Push back if the user asks to:

- treat CSP header presence as sufficient without directive-level review,
- add `'unsafe-inline'` "temporarily" to unblock a deploy without a tracked follow-up to remove it — this is a confirmed finding regardless of stated intent to revisit it,
- rely on `X-Frame-Options` alone in a CSP review scope when `frame-ancestors` is unset — the two are not equivalent and `frame-ancestors` should be reviewed explicitly,
- treat "we added CSP" as the fix for a confirmed unsanitized DOM XSS sink instead of fixing the sink directly.
