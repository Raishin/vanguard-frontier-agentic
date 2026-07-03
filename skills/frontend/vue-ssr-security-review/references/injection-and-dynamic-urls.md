# Injection: v-html and Dynamic URL Bindings

Use this reference only when the review scope includes a `v-html` usage or a dynamic `:href`/`:src` binding. The OWASP XSS citation below is loaded only when a `v-html` finding is actually present in the review — do not cite it preemptively in a review that has no `v-html` usage.

## What people get wrong

The naive assumption is:

> "Vue escapes everything by default, so if I'm using `v-html` I must already know it's a special case and I've handled it."

Partially wrong. Vue's default text interpolation (`{{ }}`) does auto-escape, and that is precisely why `v-html` exists as an explicit escape hatch — but "I chose `v-html` on purpose" is not the same as "I sanitized the content that flows into it." The recurring real-world failure is not developers being unaware `v-html` is dangerous; it is developers sanitizing at one layer (e.g., a markdown renderer) while a *different* unsanitized value reaches the same `v-html` binding through a later code change, or trusting a third-party API response as "safe" because it isn't literally user-typed input even though it echoes user-submitted content back.

## Officially grounded rules

Vue's own security best-practices guidance states directly:

- **`v-html` on untrusted content is unsafe.** Dynamically rendering arbitrary HTML on your site via `v-html` can be dangerous because it can easily lead to XSS vulnerabilities. Only use `v-html` on content you can trust to be safe, or content sanitized by a dedicated library before it reaches the binding (`documentation-based`, Vue security guide).
- **Dynamic attribute/URL bindings need scheme validation.** Vue's docs specifically call out `:href`/`:src`-style dynamic bindings as an injection surface distinct from `v-html`: unvalidated user-provided URLs bound dynamically can carry a `javascript:` (or similarly dangerous non-`http(s)`) scheme, executing script when the element is interacted with, even though no HTML markup or `v-html` was involved.
- **Template injection is a separate, related risk** for any code path that compiles user-supplied strings as Vue templates at runtime — out of scope for a typical SSR entry/template review unless the app does this explicitly (e.g., a CMS that lets users author raw Vue template syntax); flag its presence if found, but it is not the default pattern to hunt for.

The low-level mechanism, confirmed by Vue's own compiler source (`repo evidence` via Context7 `/vuejs/vue`): `v-html` compiles directly to setting the element's `innerHTML` DOM property with the bound value stringified — there is no implicit sanitization step anywhere in that compilation path. Sanitization, if it happens at all, must be applied by application code before the value reaches the binding.

## Non-negotiable design rules

### 1. Trace the full origin-to-sink path before judging a v-html binding

Do not evaluate `v-html="someVar"` in isolation. Follow `someVar` backward: is it a literal string in the template file? A prop? A computed value derived from store state? Store state populated from an API response? An API response that itself echoes a value the current user (or any user) submitted at some point? The finding depends on where that trace terminates, not on the binding syntax alone.

### 2. A sanitizer import elsewhere in the codebase does not clear a specific finding

If the trace reveals user-reachable input reaching a `v-html` binding, the only thing that clears the finding is a *named sanitizer call* (e.g., `DOMPurify.sanitize(...)`) visibly present *on that exact path* — between the untrusted origin and the binding. "This codebase has a `sanitizeHtml` utility used elsewhere" is not evidence the specific path under review calls it.

### 3. Third-party API responses are not automatically trusted

An API response is not "safe by default" just because it did not come directly from the current request's form input. If the API itself stores and echoes content that any user (not necessarily the current one) previously submitted — a comment system, a CMS with contributor accounts, a product-review feed — that response is user-reachable input for this review's purposes and needs the same sanitizer-on-path check.

### 4. Dynamic URL bindings need scheme validation, not HTML sanitization

`:href`/`:src` bindings are a distinct injection surface from `v-html` — do not conflate the two fixes. The correct control for a dynamic URL binding fed by user-reachable input is scheme validation (an allowlist accepting only `http:`/`https:`/`mailto:` as appropriate, rejecting `javascript:` and other schemes), not HTML sanitization. A `v-html`-appropriate sanitizer call does not clear a URL-injection finding and vice versa.

### 5. Origin-controlled content is not automatically a false positive to skip silently

If a `v-html` trace terminates at fully origin-controlled content (static marketing copy authored only through the app's own trusted CMS with no user-submission path anywhere upstream), it is correctly not a finding — but state this explicitly in the output rather than omitting the binding from the review entirely. A future code change could introduce a user-reachable path into the same binding, and an explicit "reviewed, not a finding, because X" record is more useful than silence.

## Minimal safe implementation pattern

```vue
<script setup>
import DOMPurify from 'dompurify'
import { computed } from 'vue'

const props = defineProps<{ rawComment: string }>()

// Sanitizer call sits directly on the path between the untrusted prop and the binding.
const safeComment = computed(() => DOMPurify.sanitize(props.rawComment))
</script>

<template>
  <div v-html="safeComment"></div>
</template>
```

Anti-pattern (untraced or missing sanitizer — do not approve):

```vue
<template>
  <!-- userBio comes from a profile API that echoes user-submitted text.
       No sanitizer call anywhere between the API response and this binding. -->
  <div v-html="userBio"></div>
</template>
```

Dynamic URL scheme validation:

```vue
<script setup>
const ALLOWED_SCHEMES = ['http:', 'https:', 'mailto:']

function safeHref(url) {
  try {
    const parsed = new URL(url, window.location.origin)
    return ALLOWED_SCHEMES.includes(parsed.protocol) ? url : '#'
  } catch {
    return '#'
  }
}
</script>

<template>
  <a :href="safeHref(userProvidedUrl)">Link</a>
</template>
```

## Adversarial checklist

Before clearing a `v-html` binding, answer these:

- What is the literal origin of the bound value — a template literal, a prop, computed state, store state, or an API response?
- Does any point along that trace involve content any user (current or otherwise) previously submitted?
- Is there a named sanitizer call visible on the exact path traced, or only "a sanitizer exists somewhere in this codebase"?
- Could a later code change (a refactor that swaps the data source, or a new field added to an existing API response) reach this same binding without re-triggering a security review?

Before clearing a dynamic `:href`/`:src` binding, answer these:

- Does the bound value's origin include user-reachable input?
- Is there scheme validation (allowlist) on the path, or only implicit trust that "URLs are just links"?
- Would a crafted `javascript:` (or `data:`, `vbscript:`) value reach this binding unmodified?

If any answer is unclear or reveals a gap, the finding is HIGH (for `v-html`) or MEDIUM-to-HIGH (for URL bindings, per reachability) — do not soften it to "worth double-checking."

## OWASP grounding (load only when a v-html finding is present)

Cross-Site Scripting (XSS) is the general vulnerability class that unsanitized `v-html` produces: an attacker-controlled or attacker-influenced string is rendered as live HTML/script in another user's browser session. The OWASP XSS reference and OWASP Top Ten (both listed in this skill's `official_docs`) provide the vendor-neutral grounding for why this defect class is treated as HIGH severity by default — it typically enables session/token theft, credential harvesting via injected forms, or full account takeover in the victim's authenticated context, not merely a cosmetic rendering issue. Cite these only in the specific finding write-up for a confirmed or suspected `v-html`/XSS defect, not as boilerplate in every review.

## Verification targets

- Grep the template scope (`.vue` files, render functions) for `v-html` and enumerate every match.
- Grep for `:href=` and `:src=` bindings (and their shorthand `v-bind:href=`/`v-bind:src=`) bound to a non-literal expression.
- For each match, grep backward through the component's props, computed properties, and any imported store/composable for the value's origin.
- Grep for a sanitizer import (`dompurify`, or an equivalent project-specific sanitizer utility) and confirm its call site is on the traced path, not merely present in the file or module.
