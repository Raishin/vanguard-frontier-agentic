# DomSanitizer Bypass Calls and [innerHTML] Bindings

Use this reference only when the review scope includes a `bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, or `bypassSecurityTrustResourceUrl` call, or an `[innerHTML]` binding. The OWASP XSS citation below is loaded only when a finding is actually present — do not cite it preemptively in a review that has no finding.

## What people get wrong

The naive assumption is:

> "I called `bypassSecurityTrustHtml`, so I must have already decided this content is safe — the method name itself is my proof of review."

Wrong. The bypass method's name documents *intent* to skip sanitization, not that the argument was actually checked. The recurring real-world failure is a bypass call added to silence Angular's sanitizer error during development, with the "we'll add validation later" step never landing, or a bypass call whose argument source changes in a later refactor (a static string swapped for a computed value fed by an API response) without anyone re-reviewing the call site.

## Officially grounded rules

Angular's own source (`repo evidence` via Context7 `/angular/angular`, `packages/platform-browser/src/security/dom_sanitization_service.ts`) confirms the mechanism directly:

- **`bypassSecurityTrustHtml()` disables Angular's XSS sanitization for the value it wraps.** It marks a string as trusted `SafeHtml`, which causes `sanitize()` to skip the HTML sanitizer entirely and return the value as-is. `bypassSecurityTrustUrl()` and `bypassSecurityTrustResourceUrl()` do the equivalent for URL and resource-URL contexts.
- **Property bindings like `[innerHTML]` accept a compiler-added sanitizer function.** `setDomProperty()` (in `packages/core/src/render3/instructions/shared.ts`) only calls a sanitizer when one is present — "it is assumed that the sanitizer is only added when the compiler determines that the property is risky." Without a sanitizer function reaching that call (which happens whenever a `Safe*` value from a bypass call is passed, since `sanitize()` unwraps it unchanged), the raw value is set directly via `renderer.setProperty()`.
- **Text interpolation is a structurally different, safer path.** `updateTextNode()` calls `renderer.setValue()` on a text node — a text node never parses HTML, so interpolated content cannot execute as markup or script regardless of its contents. This is why interpolation needs no sanitizer at all, and why it is the default recommendation whenever raw HTML rendering is not actually required.
- **`_sanitizeHtml()` (`packages/core/src/sanitization/html_sanitizer.ts`) is the actual sanitizer** invoked when no bypass has occurred: it parses the HTML into an inert DOM tree, strips disallowed elements/attributes against an allowlist, and repeats parsing to catch mutation-XSS (mXSS) auto-correction attacks. This is the protection a bypass call routes around entirely.

## Non-negotiable design rules

### 1. Trace the full origin-to-sink path before judging a bypass call or [innerHTML] binding

Do not evaluate `sanitizer.bypassSecurityTrustHtml(someVar)` or `[innerHTML]="someVar"` in isolation. Follow `someVar` backward: is it a literal string in the component? An `@Input()`? A value derived from a service call? An API response that itself echoes content any user (not necessarily the current one) previously submitted? The finding depends on where that trace terminates, not on the call/binding syntax alone.

### 2. The bypass call's existence is not evidence the argument was validated

A `bypassSecurityTrustHtml`/`bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl` call is an intentional escape hatch by design — its presence proves a developer wanted to skip sanitization, not that they checked the value first. Only a validation or sanitization step visibly present *before* the bypass call on that exact path clears the finding.

### 3. A sanitizer call elsewhere in the codebase does not clear a specific [innerHTML] finding

If the trace reveals user-reachable input reaching an `[innerHTML]` binding, the only thing that clears the finding is a named sanitizer call (e.g., `DomSanitizer.sanitize(SecurityContext.HTML, ...)`, or a project-specific equivalent such as DOMPurify) visibly present on that exact path. "This codebase has a sanitize utility used elsewhere" is not evidence the specific path under review calls it.

### 4. Third-party API responses are not automatically trusted

An API response is not "safe by default" just because it did not come directly from the current request's form input. If the API itself stores and echoes content that any user previously submitted (a comment system, a CMS with contributor accounts, a product-review feed), that response is user-reachable input for this review's purposes.

### 5. URL-context bypasses need scheme validation, not HTML sanitization

`bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl` are a distinct injection surface from `bypassSecurityTrustHtml` — do not conflate the fixes. The correct control before a URL-context bypass is scheme validation (an allowlist accepting only `http:`/`https:`/`mailto:` as appropriate, rejecting `javascript:` and other schemes), not HTML sanitization.

## Minimal safe implementation patterns

```ts
import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-comment',
  // Text interpolation never sets innerHTML and needs no sanitizer.
  template: `<div>{{ userComment }}</div>`,
})
export class CommentComponent {
  @Input() userComment = '';
}
```

```html
<!-- A named sanitizer call sits directly on the path between the
     untrusted field and the binding. -->
<div [innerHTML]="sanitizer.sanitize(userBio)"></div>
```

Anti-pattern (untraced bypass call — do not approve):

```ts
// userComment comes from a profile API that echoes user-submitted text.
// No validation anywhere between the API response and this bypass call.
this.trustedComment = this.sanitizer.bypassSecurityTrustHtml(this.userComment);
```

## Adversarial checklist

Before clearing a bypass call or `[innerHTML]` binding, answer these:

- What is the literal origin of the bound value — a template literal, an `@Input()`, a service call result, or an API response?
- Does any point along that trace involve content any user (current or otherwise) previously submitted?
- Is there a named sanitizer or validation call visible on the exact path traced, or only "we sanitize somewhere in this codebase"?
- Could a later code change (a refactor that swaps the data source, or a new field added to an existing API response) reach this same call or binding without re-triggering a security review?

If any answer is unclear or reveals a gap, the finding is HIGH — do not soften it to "worth double-checking."

## OWASP grounding (load only when a finding is present)

Cross-Site Scripting (XSS) is the general vulnerability class that an unvalidated bypass call or unsanitized `[innerHTML]` binding produces: an attacker-controlled or attacker-influenced string is rendered as live HTML/script in another user's browser session. The OWASP XSS reference and OWASP Top Ten (both listed in this skill's `official_docs`) provide the vendor-neutral grounding for why this defect class is treated as HIGH severity by default — it typically enables session/token theft, credential harvesting via injected forms, or full account takeover in the victim's authenticated context. Cite these only in the specific finding write-up for a confirmed or suspected defect, not as boilerplate in every review.

## Verification targets

- Grep component and template source for `bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, `bypassSecurityTrustResourceUrl`, `bypassSecurityTrustScript`, and `bypassSecurityTrustStyle`.
- Grep templates for `[innerHTML]` and enumerate every match.
- For each match, grep backward through the component's `@Input()`s, service injections, and any imported store/composable for the value's origin.
- Grep for a `DomSanitizer.sanitize(` call (or an equivalent project-specific sanitizer utility) and confirm its call site is on the traced path, not merely present in the file or module.
