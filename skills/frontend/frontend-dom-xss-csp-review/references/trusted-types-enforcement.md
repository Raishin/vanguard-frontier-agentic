# Trusted Types Enforcement Review

Use this reference only when the review scope includes Trusted Types policy design or `require-trusted-types-for`/`trusted-types` CSP directive enforcement. Grounded in the W3C Trusted Types specification.

## What people get wrong

The naive assumption is:

> "The code calls `trustedTypes.createPolicy(...)`, so Trusted Types is protecting this application."

Wrong. Trusted Types is an opt-in enforcement mechanism with two independent conditions that must both hold: (1) the CSP `require-trusted-types-for 'script'` directive must actually be set so the browser enforces the requirement at all, and (2) every policy's `createHTML`/`createScript`/`createScriptURL` callback must perform a real transformation (sanitize or reject), not pass the input through unmodified. Code that creates a policy but never sets the enforcing CSP directive provides no protection — Trusted Types objects become optional convenience wrappers, and raw strings can still reach `innerHTML` and other sinks unchecked. A policy whose callback returns its input unchanged provides no protection either, even with enforcement active.

## Officially grounded rules (W3C Trusted Types spec)

- **Trusted Types only restricts assignment to specific "injection sink" DOM APIs** (e.g., `Element.innerHTML`, `Element.outerHTML`, `Document.write`, `Range.createContextualFragment`, `eval`-family via `TrustedScript`, and `<script src>`/similar via `TrustedScriptURL`) when `require-trusted-types-for 'script'` is set. Without that CSP directive, these sinks continue to accept plain strings exactly as before — Trusted Types objects can still be created and used, but they are not required.
- **The `trusted-types` CSP directive restricts which named policies may be created**, and whether an unnamed/default policy or duplicate policy names are permitted. A missing `trusted-types` directive combined with `require-trusted-types-for 'script'` still enforces the sink restriction, but allows any script running on the page (including an injected one, before enforcement blocks further injection) to create its own policy — restricting allowed policy names closes this gap.
- **A policy named `'default'` is special**: per spec, when a string is assigned directly to a Trusted-Types-guarded sink (bypassing an explicit policy call), the browser invokes the `'default'` policy if one is registered, implicitly converting the string. This is a compatibility escape hatch, not a security boundary — a permissive `'default'` policy that echoes its input unmodified reintroduces exactly the unrestricted-sink behavior Trusted Types is meant to close.
- **`createHTML`, `createScript`, and `createScriptURL` are ordinary JavaScript callbacks** with no built-in sanitization. The spec does not provide a default sanitizer; the application must supply one (e.g., calling DOMPurify inside `createHTML`) or reject/throw for unsafe input. A callback that simply returns its argument is spec-compliant but provides zero security value.

## Non-negotiable design rules

### 1. Confirm the enforcing CSP directive is actually set, not just that policy-creation code exists

Grep the effective CSP (see `references/csp-directive-review.md` for locating it) for `require-trusted-types-for 'script'`. If absent, every `trustedTypes.createPolicy` call in the codebase is inert for enforcement purposes — state this as a confirmed finding, not as "Trusted Types is partially implemented."

### 2. Read every policy callback body, not just the `createPolicy` call site

For each `createHTML`/`createScript`/`createScriptURL` callback, confirm it performs a real transformation: a sanitizer call (e.g., DOMPurify), a strict allowlist check with rejection/throw on mismatch, or an equivalent safe transform. A callback body of `(input) => input` or equivalent pass-through is a confirmed finding regardless of enforcement being active.

### 3. Treat a permissive `'default'` policy as high severity

Because the `'default'` policy silently intercepts any direct string assignment to a guarded sink, a permissive `'default'` policy re-opens every sink in the application at once — it has broader blast radius than a single permissive named policy used in one call site. Flag this distinctly and at higher severity than a single permissive named-policy finding.

### 4. Restrict allowed policy names via the `trusted-types` directive when feasible

If the `require-trusted-types-for` directive is set but the `trusted-types` directive is absent or overly permissive (e.g., no allowlist), note this as a gap: any script that executes before full lockdown (or a supply-chain-compromised dependency) can register its own arbitrarily permissive policy.

### 5. Do not treat "uses a framework with built-in Trusted Types support" as sufficient without confirming the actual runtime configuration

Some frameworks and bundlers offer opt-in Trusted Types integration, but it must be explicitly enabled and configured (policy name, sanitizer wiring) in the project's actual config — its mere availability in the framework does not mean the reviewed application has turned it on. Confirm via the project's actual build/runtime configuration, not the framework's general capability.

## Minimal safe implementation pattern

```javascript
// Enforcing CSP directive (see csp-directive-review.md):
// Content-Security-Policy: require-trusted-types-for 'script'; trusted-types app-html;

import DOMPurify from 'dompurify';

const htmlPolicy = trustedTypes.createPolicy('app-html', {
  createHTML: (input) => DOMPurify.sanitize(input),
  createScript: () => { throw new Error('script creation not permitted by this policy'); },
  createScriptURL: (input) => {
    const allowed = ['https://cdn.trusted-example.com/'];
    if (!allowed.some((prefix) => input.startsWith(prefix))) {
      throw new Error('script URL not on allowlist');
    }
    return input;
  },
});

element.innerHTML = htmlPolicy.createHTML(untrustedInput);
```

Anti-pattern (policy exists but provides no protection — do not approve):

```javascript
// No 'require-trusted-types-for' directive set anywhere — this policy is never enforced.
const policy = trustedTypes.createPolicy('default', {
  createHTML: (input) => input, // pass-through: zero sanitization
});
```

## Adversarial checklist

Before clearing Trusted Types as properly enforced, answer these:

- Is `require-trusted-types-for 'script'` actually present in the effective CSP (header or meta tag), or does only application code reference the Trusted Types API?
- For every `createPolicy` call in scope, does the `createHTML`/`createScript`/`createScriptURL` callback perform a real sanitize-or-reject transform, or does it return its input unmodified?
- Is there a `'default'` policy registered? If so, is its transform equally strict as the named policies, given its broader implicit-invocation blast radius?
- Is the `trusted-types` directive present to restrict which policy names may be created, or can any script on the page register an arbitrarily permissive policy?
- Does this confirmed enforcement actually cover the specific sink(s) flagged in a co-occurring `references/dom-xss-sink-source-taxonomy.md` finding, or is the policy scoped to a different part of the application?

If any answer reveals a gap, state it as a confirmed finding — do not describe partial implementation as "Trusted Types is in place."

## Verification targets

- Grep the effective CSP for `require-trusted-types-for` and `trusted-types` directive values.
- Grep the codebase for `trustedTypes.createPolicy` and read every callback function body in full.
- Grep specifically for a policy named `'default'` and treat any match as requiring the same scrutiny as every named policy, plus the broader-blast-radius note above.
- Cross-reference confirmed Trusted-Types-guarded sinks against the sink list in `references/dom-xss-sink-source-taxonomy.md` to confirm enforcement scope actually matches the sinks the review is concerned with.

## When to push back

Push back if the user asks to:

- treat the presence of `trustedTypes.createPolicy` calls as sufficient without confirming the enforcing CSP directive is set,
- approve a policy callback that passes input through unmodified because "we'll add sanitization later,"
- register a permissive `'default'` policy as a convenience to avoid updating call sites — this is a confirmed high-severity finding given its implicit, page-wide invocation,
- skip verifying the `trusted-types` directive's policy-name allowlist because "only our own code creates policies" — a supply-chain-compromised dependency is exactly the scenario this allowlist defends against.
