# Open Redirect, Scheme Injection, and Reflected XSS Through the Router

Use this reference when the review scope includes a post-login redirect flow reading
`route.query`, a dynamic `:to`/`:href` binding, or a `v-html`/`innerHTML` sink fed by
`route.params`/`route.query`. Covers rubric items 2, 3, 4 (and their "not a finding"
counterparts 9, 10, 11, 12).

## What people get wrong

The naive assumption, said three different ways depending on which defect it excuses:

> "It's just redirecting back to where the user came from — that's a UX feature, not a
> security decision."

> "It's a router-link, Vue Router handles the URL, so it must be safe."

> "It's just the search query showing what the user typed — of course it should render as-is."

All three treat *router-adjacent* data (a `redirect` query param, a `:to` binding, a
`route.params`/`route.query` value) as inherently trusted because it arrived through routing
machinery rather than a form POST. Vue Router's own docs confirm the opposite: dynamic segments
are "exposed... as `route.params`" with no escaping step described anywhere in the dynamic-
matching guide (`repo evidence`, `/vuejs/router`) — they are raw strings lifted from the URL,
exactly as attacker-controllable as any query-string or form value.

## Officially grounded facts

- **Redirect functions receive the live route object and can echo it verbatim.** Vue Router's
  documented redirect patterns include function-based redirects that read `to.params`/`to.query`
  directly: `redirect: to => ({ path: '/search', query: { q: to.params.searchText } })` and
  `redirect: to => \`/redirected-path/${to.params.id}\`` (`repo evidence`, `/vuejs/router`,
  redirect-and-alias / extending-routes guides). The mechanism for building a redirect target
  from live route data is a first-class, documented feature — which is exactly why an
  *unvalidated* version of the same mechanism (echoing a full attacker-supplied URL instead of a
  path segment) is dangerous: the framework will not stop you.
- **`router.push` accepts a bare string path with no origin/scheme validation of its own** —
  `router.push('/users/eduardo')`, `router.push(\`/user/${username}\`)` (`repo evidence`,
  `/vuejs/router`, navigation guide). If `username` (or any interpolated value) is instead a
  full external URL or a `javascript:` string, `router.push` does not reject it — it is the
  calling code's responsibility to validate before calling.
- **`RouterLink`'s `:to` prop is commonly extended to bind directly to `:href` for external
  links, gated only by an `isExternalLink` string check** — Vue Router's own
  extending-router-link guide shows `:href="to"` used directly for values where
  `typeof props.to === 'string' && props.to.startsWith('http')` (`repo evidence`,
  `/vuejs/router`, extending-router-link guide). This is the documented pattern for handling
  external links alongside internal ones — but note precisely what it does *not* include: any
  scheme allowlist beyond the `http` prefix check. A value like `javascript:...` or
  `httpjavascript:` crafted to defeat a naive `startsWith('http')` check is not excluded by this
  documented example alone; production code needs an explicit protocol allowlist (see pattern
  below), not just an `http`-prefix string check.
- **`route.params`/`route.query` carry no built-in HTML-escaping** — confirmed by the dynamic-
  matching guide's description of params as directly exposed URL segments (`repo evidence`,
  `/vuejs/router`). Vue's *template* interpolation (`{{ }}`) auto-escapes when you render these
  values normally; the risk is exclusively when a `route.params`/`route.query` value reaches a
  `v-html` binding or a manual `.innerHTML` assignment, which bypass that auto-escaping by
  design (`documentation-based`, Vue security guide — this specific `v-html`-is-unescaped claim
  is general Vue template-compiler behavior, not a Vue-Router API, so it is grounded in the
  `official_docs` Vue security guide rather than the `/vuejs/router` library).

## Non-negotiable design rules

### 1. Open redirect: validate against a same-origin/relative-path allowlist, not a blocklist

The only acceptable fix for a `redirect`/`returnUrl` query value used in a post-auth redirect is
allowlisting: confirm the value is a relative path starting with a single `/` (reject
protocol-relative `//` and any value containing `://`), or resolve it with
`new URL(value, window.location.origin)` and compare the resolved `origin` to the app's own
origin. A denylist of "known bad" values (blocking `http://`/`https://` substrings, for example)
is not sufficient — treat a blocklist-only approach as still a finding, since it is trivially
bypassed by encoding or alternate schemes.

### 2. Scheme injection: allowlist protocols explicitly, don't rely on prefix checks alone

For any dynamic `:to`/`:href` fed by user-reachable input, require a visible allowlist check
(e.g., `['http:', 'https:', 'mailto:'].includes(new URL(value, base).protocol)`) on the exact
data-flow path. A `startsWith('http')` check (as shown in Vue Router's own external-link
extension example) is a routing-decision heuristic (is this internal or external), not a
security control — do not accept it as clearing a scheme-injection finding.

### 3. Trace `v-html`/`innerHTML` sinks fed by route data back to the URL, not just to a variable name

If a component does `<div v-html="route.query.bio">` or
`el.innerHTML = route.params.description`, the origin is the URL itself — the most directly
user-controlled input there is, requiring no stored/second-order step. Do not treat this as
lower-severity than a stored-XSS path; a reflected payload via a shared link is exploitable
immediately.

### 4. Text interpolation and non-rendering uses of route data are not the defect

`{{ route.query.q }}`, `:aria-label="route.params.id"`, passing `route.params.id` into an API
call, or using it as a `v-if` condition are all safe — Vue's default interpolation escapes, and
non-DOM-rendering uses have no injection sink at all. Only flag the specific `v-html`/`innerHTML`
sink, not every place a component touches `route.params`/`route.query`.

## Minimal safe implementation patterns

Open redirect, validated:

```js
const SAFE_REDIRECT = /^\/(?!\/)/ // single leading slash, not protocol-relative

function safeRedirectTarget(raw) {
  return typeof raw === 'string' && SAFE_REDIRECT.test(raw) ? raw : '/'
}

router.push(safeRedirectTarget(route.query.redirect))
```

Scheme-validated dynamic link:

```vue
<script setup>
const ALLOWED_SCHEMES = ['http:', 'https:', 'mailto:']

function safeHref(url) {
  try {
    const parsed = new URL(url, window.location.origin)
    return ALLOWED_SCHEMES.includes(parsed.protocol) ? parsed.href : '#'
  } catch {
    return '#'
  }
}
</script>

<template>
  <a :href="safeHref(profile.websiteUrl)">Website</a>
</template>
```

Route data into `v-html`, sanitized:

```vue
<script setup>
import DOMPurify from 'dompurify'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const safeQueryPreview = computed(() => DOMPurify.sanitize(String(route.query.q ?? '')))
</script>

<template>
  <div v-html="safeQueryPreview"></div>
</template>
```

Anti-patterns (do not approve):

```js
// Open redirect: unvalidated query value passed straight to router/window.
router.push(route.query.redirect) // or: window.location.href = route.query.returnUrl
```

```vue
<!-- Scheme injection: no protocol allowlist. -->
<router-link :to="userProvidedProfileLink">Visit</router-link>
```

```vue
<!-- Reflected XSS: route.query rendered via v-html with no sanitizer on the path. -->
<div v-html="route.query.q"></div>
```

## Adversarial checklist

- Does any code read `route.query.redirect`/`returnUrl` (or similarly named params) and pass it
  to `router.push`, `router.replace`, or `window.location` without an allowlist check?
- Is the allowlist check a same-origin/relative-path confirmation, or only a substring blocklist
  (which does not count as a fix)?
- Does any `:to`/`:href` binding's traced source include user-reachable input with no protocol
  allowlist visible on that exact path?
- Would `javascript:`, `data:`, or `vbscript:` reach any such binding unmodified?
- Does any `v-html` or manual `.innerHTML` assignment's traced source include
  `route.params`/`route.query` with no named sanitizer call on that exact path?
- Is a redirect-loop or bypass reachable by feeding a guard's own redirect target from
  unvalidated `to.query`/`to.params` (cross-reference `client-guards-and-control-flow.md`)?

## Verification targets

- Grep for `route.query.redirect`, `route.query.returnUrl`, `to.query.redirect`, and similar
  names; trace each into `router.push(`, `router.replace(`, or `window.location`.
- Grep for `:to="` and `:href="` bound to a non-literal expression; trace each backward through
  props/computed/store to its origin.
- Grep for `v-html` and `.innerHTML =` and trace each backward for any `route.params`/
  `route.query` origin.
- Grep for a scheme/protocol allowlist (`ALLOWED_SCHEMES`, `new URL(`, `.protocol ===`) near
  each dynamic link binding found above, and confirm it sits on the traced path rather than
  merely existing elsewhere in the file.
- Grep for a sanitizer import (`dompurify` or an equivalent project utility) and confirm its
  call site is on the traced `v-html`/`innerHTML` path.
