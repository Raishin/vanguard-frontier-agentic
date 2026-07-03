# Environment Variables, Image SSRF, and Rewrite/Redirect Injection

Use this reference only when the review scope includes `.env*` files, `NEXT_PUBLIC_*` usage, `images.dangerouslyAllowLocalIP` in `next.config.js`, or a rewrite/redirect destination built from dynamic input. The OWASP SSRF/open-redirect citation is loaded only when such a finding is actually present — do not cite it preemptively in a review with no SSRF/redirect surface.

## What people get wrong: `NEXT_PUBLIC_` secrets

The naive assumption is:

> "I'll just prefix it with `NEXT_PUBLIC_` so the client component I'm writing right now can read it — I'll clean it up later."

Wrong, and unrecoverable after the fact. Next.js's own documentation is direct: environment variables prefixed `NEXT_PUBLIC_` are inlined into the JavaScript bundle at build time (`documentation-based`, Next.js environment-variables guide). This is not a runtime gate that can be toggled off later — every client that has ever loaded a build containing the value has received it, in plaintext, in a bundle they can inspect via browser devtools or by fetching the JS file directly. Rotating the underlying secret afterward does not undo the exposure of the old value's existence, format, or any access it already granted before rotation.

## Officially grounded rule

Non-`NEXT_PUBLIC_` environment variables are exclusively available in the Node.js server environment and are never sent to the browser (`documentation-based`, Next.js environment-variables guide). The correct pattern is:

```txt
# .env
API_KEY=<REDACTED_secret_value>
```

```js
// Server Action or Route Handler only — never a Client Component
export async function myServerAction() {
  const key = process.env.API_KEY
  // ...
}
```

## Non-negotiable design rules

### 1. Flag by name pattern, then confirm by value shape

Any `NEXT_PUBLIC_`-prefixed variable whose name contains `KEY`, `SECRET`, `TOKEN`, `PASSWORD`, `CREDENTIAL`, or an obvious equivalent is a finding regardless of the value shown in the file — even a placeholder or rotated value in a committed `.env.example` demonstrates the naming pattern will leak the real value wherever it is actually set.

### 2. A genuinely public value under `NEXT_PUBLIC_` is not a finding

An analytics ID, a public feature-flag name, or a publishable (not secret) API key that the vendor itself documents as safe for client exposure (e.g., a Stripe *publishable* key, as opposed to a *secret* key) is the correct, intended use of the prefix. Do not manufacture a finding for a variable that is genuinely meant to be public — check the vendor's own documentation for whether a given key type is designed for client exposure before flagging it.

## What people get wrong: image optimization SSRF

The naive assumption is:

> "`dangerouslyAllowLocalIP` just lets me test with a local dev image server, it's not a real security control."

The name is the warning. Enabling `dangerouslyAllowLocalIP` allows the built-in image optimizer to fetch a `src` URL that resolves to a loopback or internal-network address. Combined with any dynamic (user- or attacker-influenced) `src` value, this becomes a classic SSRF primitive: an attacker supplies a URL pointing at an internal service (a cloud metadata endpoint, an internal admin panel, a database's HTTP interface) and the server-side image optimizer fetches it on the attacker's behalf (`documentation-based`, Next.js image-configuration guide — generally not recommended due to potential SSRF risk).

## Non-negotiable design rule

`images.dangerouslyAllowLocalIP: true` is a finding unless every dynamic `src` value reaching `<Image>` is validated against a hardcoded allowlist of safe external hostnames before it is used. `remotePatterns`/`domains` configuration that is itself permissive (a wildcard hostname pattern) does not clear this finding — the two controls are independent; a broad `remotePatterns` plus `dangerouslyAllowLocalIP: true` is worse, not better.

## What people get wrong: rewrite/redirect destination injection

The naive assumption is:

> "`NextResponse.rewrite()`/a dynamic rewrite is just routing logic, not a security-sensitive sink."

Wrong when the destination is built from user-controlled input. `NextResponse.rewrite(new URL(userControlledValue, request.url))` (or an equivalent `rewrites()` destination built from request data) lets an attacker supply a URL that the rewrite forwards the request to — either an internal service (SSRF) or an external attacker-controlled host (functionally an open redirect, since the response the client ultimately sees originates from the rewritten destination).

## Non-negotiable design rule

Any rewrite/redirect destination whose value traces back to user-controlled input (a query parameter, a header, a request body field) must pass through a hardcoded hostname allowlist check *before* the rewrite/redirect call — not merely be parsed with `new URL()` (which validates syntax, not safety) and passed straight through. Resolve the value to a local variable, check it against the allowlist, and only then call `NextResponse.rewrite()`/return the redirect. A destination built entirely from fixed, developer-authored literals is not a finding.

## OWASP grounding (load only when an SSRF/redirect finding is present)

Server-Side Request Forgery is the vulnerability class both `dangerouslyAllowLocalIP` misuse and unvalidated rewrite destinations produce: server-side code is induced to make a request to a location the attacker chose rather than the application developer, potentially reaching internal-network resources otherwise unreachable from the public internet. The OWASP SSRF reference (listed in this skill's `official_docs`) provides the vendor-neutral grounding for why this defect class is treated as HIGH severity by default — it commonly enables internal network reconnaissance, cloud metadata credential theft, or bypass of network-perimeter controls, not merely a routing inconvenience. Cite it only in the specific finding write-up for a confirmed or suspected SSRF/redirect defect, not as boilerplate in every review.

## Verification targets

- Grep every `.env*` file and any code referencing `process.env.NEXT_PUBLIC_` for a secret-shaped variable name.
- Grep `next.config.js` for `dangerouslyAllowLocalIP` and check its value; if `true`, grep the codebase for every dynamic `<Image src=` binding and trace its origin.
- Grep for `NextResponse.rewrite(` and `rewrites()` array entries; for each, trace the `destination`/URL argument's origin backward through variables to its source (literal, env var, or user-controlled request data).
- Grep for a hostname allowlist check (`.includes(`, `ALLOWED_HOSTS`, or an equivalent project-specific allowlist array) and confirm its call site sits between the user-controlled origin and the rewrite/redirect call, not merely present elsewhere in the file.
