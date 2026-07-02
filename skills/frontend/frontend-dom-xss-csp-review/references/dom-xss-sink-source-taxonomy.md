# DOM XSS Sink and Source Taxonomy

Use this reference only when the review scope includes a specific sink match (`innerHTML`, `dangerouslySetInnerHTML`, `v-html`, `document.write`, an `eval`-class API, or a dynamic attribute/URL binding). Grounded in the OWASP DOM-Based XSS Prevention Cheat Sheet's source/sink model.

## What people get wrong

The naive assumption is:

> "I found `innerHTML` in a grep, so this is an XSS finding."

Wrong. OWASP's own DOM XSS model requires two confirmed elements, not one: a **source** (attacker-reachable input) and a **sink** (a DOM API that can execute the value as code or markup), connected by an actual data-flow path with no sanitization in between. A sink with no reachable source is not exploitable. A source with no sink it reaches is not exploitable either. The recurring real failure mode is treating the sink match alone as the finding, or treating "we sanitize somewhere" as proof the specific traced path is clean.

## Officially grounded sink classes (OWASP DOM XSS Cheat Sheet)

- **HTML-context sinks** — assignment to `innerHTML`, `outerHTML`, `insertAdjacentHTML`, jQuery's `.html()`, React's `dangerouslySetInnerHTML`, Vue's `v-html` or an `innerHTML` render-function/JSX binding, Angular's `[innerHTML]` binding without going through `DomSanitizer`. These parse the assigned string as HTML/DOM, so any embedded `<script>`, event-handler attribute (`onerror`, `onload`), or `javascript:` URL in an unsanitized value executes.
- **JavaScript-execution sinks** — `eval()`, `new Function(...)`, `setTimeout(string, ...)`, `setInterval(string, ...)`, `execScript` (legacy IE). These execute the string argument directly as code; there is no markup step to sanitize around, so any confirmed attacker-reachable string reaching these is a direct code-execution finding.
- **URL-context sinks** — assignment to `location`, `location.href`, `location.replace()`, `window.open()`, dynamic `<a href>`/`<script src>`/`<iframe src>` bindings. A `javascript:` or `data:` scheme in an unvalidated attacker-reachable value executes on interaction (click, navigation, or load, depending on the element).
- **Document-write sinks** — `document.write()`, `document.writeln()`. Parses the argument as HTML into the document at call time; same risk class as HTML-context sinks but with the added hazard of executing during initial page parse, before most runtime sanitization layers are wired up.
- **jQuery/legacy DOM sinks** — `.html()`, `.append()`, `.prepend()`, `.after()`, `.before()`, `.replaceWith()` when passed a string (these route through the same HTML-parsing path as `innerHTML`); `.attr()` when setting an event-handler or `href`/`src` attribute from an attacker-reachable value.

## Officially grounded source classes (OWASP DOM XSS Cheat Sheet)

Treat all of the following as attacker-reachable sources unless proven otherwise for the specific application:

- `location.*` (`href`, `search`, `hash`, `pathname`) — URL components are fully attacker-controlled; a victim can be sent a crafted link.
- `document.referrer` — controlled by whatever page linked to the current one.
- `document.cookie` — if the application itself writes attacker-influenceable values into cookies (e.g., echoing a query param into a cookie), reading it back is a source.
- `window.name` — persists across navigations within a tab and is attacker-settable by a page that opens or navigates the target window.
- `postMessage` payloads (`event.data`) — attacker-reachable from any origin unless the receiving handler validates `event.origin`.
- `localStorage`/`sessionStorage` — attacker-reachable if written by another script/extension in the same origin, or if the value itself originated from one of the sources above and was persisted.
- API/network responses that echo user-submitted or third-party-submitted content (a comment system, a CMS with contributor accounts, a product-review feed, any endpoint whose stored data originated from user input at some point upstream) — not automatically trusted just because the immediate call site is "just an API response."

Not attacker-reachable by default: literal strings in source files, values from the application's own build-time configuration with no runtime mutation path, and content authored exclusively through a trusted internal CMS with no public or user-submission path anywhere upstream.

## Non-negotiable design rules

### 1. A sink match without a completed source trace is a pattern-only observation, not a finding

State this distinction explicitly in every response. Do not let a sink count (e.g., "found 12 uses of `dangerouslySetInnerHTML`") stand in for 12 findings.

### 2. Trace through every intermediate transform, not just the assignment site

A value can pass through a template-string concatenation, a markdown renderer, a `JSON.parse`, or a component-prop chain before reaching the sink. Each hop must be checked: does it neutralize the attacker-controlled portion, or does it pass it through unmodified (or re-introduce it, e.g., a markdown renderer that itself allows raw HTML passthrough)?

### 3. A sanitizer call must sit on the exact traced path

"This codebase imports DOMPurify" is not evidence for a specific finding. The sanitizer call must be reachable on the specific hop-by-hop path between the confirmed source and the confirmed sink.

### 4. JavaScript-execution sinks (`eval`, `Function()`, string-arg `setTimeout`/`setInterval`) have no sanitization escape hatch

Unlike HTML-context sinks, there is no "sanitize the string first" pattern that makes passing attacker-reachable data to these safe while still executing it as code. The only safe fix is removing the dynamic-code-execution pattern entirely (e.g., replacing `setTimeout(userString, 1000)` with `setTimeout(() => knownFunction(parsedArgs), 1000)`).

### 5. Do not conflate framework auto-escaping with sink safety

React's JSX text interpolation, Vue's `{{ }}` interpolation, and Angular's default property binding all auto-escape by design — but this protection applies only to the framework's normal rendering path, not to any of the explicit escape-hatch sinks listed above. Confirming "the component mostly uses JSX text nodes" does not clear a `dangerouslySetInnerHTML` call three lines later.

## Verification targets

- Grep for each sink pattern listed above across the review scope (`innerHTML`, `outerHTML`, `insertAdjacentHTML`, `dangerouslySetInnerHTML`, `v-html`, `document.write`, `document.writeln`, `eval(`, `new Function(`, string-argument `setTimeout`/`setInterval`, jQuery `.html()`/`.append()`-family calls).
- For each match, grep backward through the enclosing function/component for the variable's assignment, prop origin, or API-response parsing site.
- Grep for a sanitizer import (`dompurify`, `sanitize-html`, or an equivalent project-specific utility) and confirm the call site sits on the traced path, not merely present in the file or module.
- Grep for `addEventListener('message'` / `.onmessage` and confirm an `event.origin` check exists inside every handler whose `event.data` reaches a sink in scope.

## Adversarial checklist

Before clearing a sink as not a finding, answer these:

- What is the literal origin of the value reaching the sink — a source-file literal, a prop, a computed value, store state, or an API response?
- Does any hop in that trace involve content any user (current or otherwise) previously submitted, or any of the attacker-reachable sources listed above?
- Is there a named sanitizer or Trusted-Types transform visible on the exact traced path, or only "a sanitizer exists somewhere in this codebase"?
- For a JavaScript-execution sink specifically: is there any dynamic-code-execution path at all reachable from attacker-controlled input, regardless of sanitization — because sanitization does not clear this sink class?

If any answer is unclear, the finding is confirmed and defaults to HIGH — do not soften it to "worth double-checking."
