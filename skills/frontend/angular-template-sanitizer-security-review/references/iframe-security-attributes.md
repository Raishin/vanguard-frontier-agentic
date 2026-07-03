# Dynamic Iframe Security Attributes

Use this reference only when the review scope includes an `<iframe>` element with a bound `sandbox`, `allow`, `credentialless`, `csp`, `referrerPolicy`, or `fetchPriority` attribute, or a reported NG0910 error.

## What people get wrong

The naive assumption is:

> "Angular throws NG0910 if I bind these attributes dynamically, so if my app runs without that error, my iframe binding must be fine."

Wrong in two ways. First, the check can be suppressed or not exercised in the exact code path under review (a conditional branch not hit during development, or an older Angular version without the check). Second, even when the error is correctly thrown and "fixed" by refactoring around it, the underlying intent — a *dynamic*, potentially user-influenced sandbox/allow value — is still the actual security concern, independent of whether the specific Angular version enforces it at compile/runtime.

## Officially grounded rules

Angular's own documentation (`documentation-based`, `adev/src/content/reference/errors/NG0910.md`, confirmed via Context7 `/angular/angular`) states directly:

- **Angular throws NG0910 when it detects a binding on specific security-related `<iframe>` attributes**: `sandbox`, `allow`, `allowFullscreen`, `referrerPolicy`, `csp`, `fetchPriority`, or `credentialless`. These attributes configure the iframe's security model and must be applied before the `src`/`srcdoc` attributes load content, so Angular requires them to be static — their values fixed at element-creation time and never bound as a property (`[sandbox]="..."`) or attribute (`[attr.sandbox]="..."`) binding, including via a directive's host bindings.
- **The recommended fix is a static attribute**, e.g. `sandbox="allow-scripts"`, or Angular's `@if`/`@switch` control-flow blocks to conditionally render entire `<iframe>` elements with different static attribute values, rather than binding the attribute's value dynamically on one persistent element.

## Non-negotiable design rules

### 1. Flag the pattern regardless of whether NG0910 currently fires

A dynamically bound security-sensitive iframe attribute is the finding — not the NG0910 error message itself. If the error is suppressed (an older Angular version, a downgraded compiler check, or code that technically avoids triggering the exact check Angular implements) but the underlying dynamic-binding pattern is present, it is still a finding: the sandbox's actual value can still be influenced by whatever expression it is bound to.

### 2. Check reachability of the bound expression

A `[attr.sandbox]` bound to a hardcoded, non-configurable constant expression evaluated once at compile time is a lower-severity finding (still worth fixing for forward-compatibility with future Angular checks) than one bound to a value that traces back to user-reachable input (a query parameter, a per-tenant config value editable by end users, or similar) — the latter is a HIGH finding because an attacker-influenced value can directly weaken the iframe sandbox.

### 3. Use static attributes or conditional rendering, not dynamic binding

The correct fix is either a static attribute value or Angular's `@if`/`@switch` blocks rendering distinct `<iframe>` elements per case — not attempting to bind the attribute dynamically through a workaround (e.g., manual DOM manipulation via `ElementRef` to set the attribute outside Angular's binding system), which reintroduces the same risk outside Angular's own safety check.

## Minimal safe implementation pattern

```html
<!-- Static attribute, fixed at element-creation time. -->
<iframe sandbox="allow-scripts" [src]="trustedEmbedUrl"></iframe>
```

```html
<!-- Conditional rendering of distinct static configurations. -->
@if (isTrustedPartner) {
  <iframe sandbox="allow-scripts allow-same-origin" [src]="trustedEmbedUrl"></iframe>
} @else {
  <iframe sandbox="allow-scripts" [src]="trustedEmbedUrl"></iframe>
}
```

Anti-pattern (dynamic binding — do not approve):

```html
<!-- userSandbox may originate from a per-tenant config value; even if
     NG0910 is not currently thrown in this environment, the sandbox's
     effective value can be influenced by that input at runtime. -->
<iframe [attr.sandbox]="userSandbox" [src]="trustedEmbedUrl"></iframe>
```

## Verification targets

- Grep `<iframe>` elements for `[sandbox]`, `[attr.sandbox]`, `[allow]`, `[attr.allow]`, `[credentialless]`, `[attr.credentialless]`, `[csp]`, `[attr.csp]`, `[referrerPolicy]`, `[attr.referrerPolicy]`, `[fetchPriority]`, and `[attr.fetchPriority]`.
- For each match, trace the bound expression backward to determine whether it is a fixed compile-time constant or reachable from user-influenced input (query params, tenant config editable by end users, form state).
- Grep for `ElementRef`-based manual attribute manipulation on iframe elements, which can reintroduce the same dynamic-binding risk outside Angular's compiler-enforced static-attribute check.
