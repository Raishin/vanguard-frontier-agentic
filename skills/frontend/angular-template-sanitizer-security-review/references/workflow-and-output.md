# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the other two references only for the specific defect class the component or template under review actually raises.

## Prerequisites

- Read the component's imports to confirm whether `DomSanitizer` (from `@angular/platform-browser`) is in use. Do not assume a bypass call exists without confirming the import and the call site.
- Identify the Angular major version in use (`package.json` — `@angular/core`) — sanitizer internals and the NG0910 check are current-Angular behavior; note explicitly if reviewing a much older major where behavior may differ, and label the claim accordingly.

## Workflow

1. **Locate every `DomSanitizer` bypass call.** Grep for `bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, `bypassSecurityTrustResourceUrl`, `bypassSecurityTrustScript`, and `bypassSecurityTrustStyle`. For each, trace the argument backward through props/`@Input()`s, computed getters, service calls, and API responses to its origin.
2. **Locate every `[innerHTML]` binding.** For each, trace its bound expression the same way. Determine whether the origin includes user-reachable input (route params, query strings, request bodies, or a third-party API response that itself echoes user input) and whether a named sanitizer call (`DomSanitizer.sanitize(SecurityContext.HTML, ...)` or an equivalent) sits on that exact path.
3. **Locate every dynamically bound iframe security attribute.** Grep for `[attr.sandbox]`, `[sandbox]`, `[attr.allow]`, `[allow]`, `[attr.credentialless]`, `[credentialless]`, `[attr.csp]`, `[attr.referrerPolicy]`, and `[attr.fetchPriority]` on `<iframe>` elements (including via a directive's host bindings). See `references/iframe-security-attributes.md` for the decision tree.
4. **Locate every dynamic `[href]`/`[src]` binding fed by `bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl`.** Check for scheme validation (an allowlist rejecting `javascript:` and other non-`http(s)` schemes) on the path before the bypass call.
5. **Produce ranked findings** using the output contract below.

## Decision tree

- A bypass call's traced argument includes user-reachable input and no validation/sanitization occurs on that exact path before the call → **HIGH** finding, XSS or URL-injection depending on the bypass method. Do not accept "we validate elsewhere" — the trace must show the check on the specific path reviewed.
- A bypass call's traced argument is fully origin-controlled with no user-reachable input anywhere in the trace (e.g., a static, developer-authored HTML fragment with no user-submission path) → not a finding, but state this explicitly in the output rather than silently omitting the call site.
- An `[innerHTML]` binding's traced source includes user-reachable input and no sanitizer call is present on that exact path → **HIGH** finding, XSS.
- An `[innerHTML]` binding's traced source is fully origin-controlled → not a finding, stated explicitly.
- Any security-sensitive iframe attribute (`sandbox`, `allow`, `allowFullscreen`, `referrerPolicy`, `csp`, `fetchPriority`, `credentialless`) is bound dynamically (property or `attr.` binding) rather than set as a static attribute → **MEDIUM-to-HIGH** finding depending on whether the bound value can be influenced by user-reachable input, regardless of whether NG0910 is currently thrown in the environment reviewed.
- A dynamic `[href]`/`[src]` fed through a bypass call has no scheme allowlist on the path before the bypass → **MEDIUM-to-HIGH** finding depending on reachability (public unauthenticated surface vs. requiring an authenticated session to trigger).

## Output contract

Every response from this skill must return:

1. **Scope** — the component(s), template binding(s), and/or `DomSanitizer` call site(s) reviewed.
2. **Ranked findings** — each with file:line, defect category (`xss`, `url-injection`, or `iframe-sandbox-escape`), the concrete data-flow trace naming every hop from origin to sink, and a fix sketch matching Angular's documented pattern.
3. **Validation/sanitizer status per finding** — an explicit statement of whether validation or sanitization is present on the traced path; never infer one exists.
4. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`. Label structural risk findings as structural risk explicitly — do not imply confirmed exploitation without live evidence.
5. **Verdict** — approve / approve-with-notes / block.
6. **Open questions or out-of-scope items** — e.g., "confirming actual exploitation requires a live payload test in a running browser session, not static review," or "hydration-mismatch risk in this same file is out of scope — recommend `angular-ssr-hydration-review` if relevant."

## When to push back

Push back if the user asks to:

- approve a bypass call because "we validate elsewhere in the app" without a validation/sanitizer call visible on the specific traced path — that is not evidence, it is an assumption,
- treat a bound `[attr.sandbox]`/`[sandbox]` as acceptable because "NG0910 didn't fire in our environment" — a suppressed or unhit dev-mode check does not make the dynamic-binding pattern safe in production,
- downgrade an untraced `[innerHTML]` finding to informational because "it's probably fine" — this skill's default is HIGH for exactly this class of unproven claim,
- skip the review because "DomSanitizer bypass calls are always intentional, so they must already be safe" — the bypass APIs are intentional escape hatches from sanitization, not proof the argument was validated before the call.
