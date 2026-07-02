# Review Workflow and Findings Contract

Use this reference for the step-by-step review procedure and the required output shape. Load the sink-taxonomy, CSP, and Trusted Types references only for the specific concern the code under review actually raises.

## Prerequisites

- Read `package.json` to confirm the framework(s) and major version(s) in scope. Sink APIs, sanitizer defaults, and CSP/Trusted Types tooling (e.g., a framework's CSP-nonce middleware) differ by framework and version — do not apply one framework's semantics to another's codebase.
- Identify whether the review scope is sink-focused (specific file/component), CSP-focused (a header/meta value or server config emitting one), Trusted Types-focused, or all three. Scope the review to what was actually asked; do not expand a single-sink review into a full CSP audit unless requested.

## Workflow

1. **Enumerate every candidate sink in scope.** Grep for `innerHTML`, `outerHTML`, `dangerouslySetInnerHTML`, `v-html`, `document.write`, `document.writeln`, `eval(`, `new Function(`, and `setTimeout`/`setInterval` calls whose first argument is a string literal built from a variable (not a function reference). See `references/dom-xss-sink-source-taxonomy.md`.
2. **Trace each candidate sink's data source backward.** Follow the value through props, variables, function returns, and API responses to its origin. Classify the origin as attacker-reachable (URL/query params, `postMessage`, request bodies, third-party API responses that echo user or third-party input, `document.referrer`, `window.name`, `localStorage`/`sessionStorage` writable by another origin or script context) or fully origin-controlled (a literal in source, a value from the app's own trusted build-time config with no runtime mutation path).
3. **For each attacker-reachable sink, check for a sanitizer call on the exact traced path.** A sanitizer import or call present elsewhere in the codebase does not clear the finding — it must sit between the untrusted origin and the sink on the specific path reviewed.
4. **If the review scope includes CSP, locate the effective policy.** Find the `Content-Security-Policy` HTTP header, the `<meta http-equiv="Content-Security-Policy">` tag, or the server/framework config that emits either. Parse directive-by-directive per `references/csp-directive-review.md`.
5. **If the review scope includes Trusted Types, locate the enforcement mode and default policy.** Check for `require-trusted-types-for 'script'` and `trusted-types` directives, and read every `trustedTypes.createPolicy(...)` call's callback bodies per `references/trusted-types-enforcement.md`.
6. **Check every `postMessage` listener in scope for origin validation.** An `addEventListener('message', handler)` with no `event.origin` check inside `handler` is itself an untraced taint source for any sink it feeds.
7. **Produce ranked findings** using the output contract below.

## Decision tree (taint confirmation)

- Sink match, traced data source is attacker-reachable, no sanitizer/Trusted-Types call on the exact path → **confirmed finding**, severity per sink class (script-execution sinks — `eval`, `Function()`, `innerHTML`/`dangerouslySetInnerHTML`/`v-html` with script-capable markup, `document.write` — default HIGH; URL/attribute-injection sinks depend on reachability, MEDIUM-to-HIGH).
- Sink match, traced data source is attacker-reachable, but a named sanitizer call (e.g., DOMPurify) or a Trusted Types policy transformation visibly sits on the exact path → **not a finding**; state this explicitly with the sanitizer/policy name and location rather than omitting the sink from the review.
- Sink match, traced data source is fully origin-controlled with no attacker-reachable input anywhere in the trace → **not a finding**, but record it explicitly ("reviewed, not a finding, because X") rather than silently dropping it — a later code change could introduce a reachable path into the same sink.
- Sink match, trace could not be completed within the review scope (e.g., the origin is in a third-party dependency not under review, or requires runtime state unavailable statically) → **pattern-only observation**, not a confirmed finding; state exactly what would be needed to complete the trace.
- CSP directive parsed and found permissive (`unsafe-inline`, `unsafe-eval`, missing `object-src 'none'`, missing `base-uri`, wildcard `script-src`) → **confirmed finding** regardless of whether a sink finding co-occurs; CSP gaps are findings in their own right.
- Trusted Types default policy callback returns input unmodified or is absent while `require-trusted-types-for 'script'` is not set → **confirmed finding**; enforcement is not active even if policy-creation code exists.

## Output contract

Every response from this skill must return:

1. **Scope** — the file(s)/sink(s), CSP surface, and/or Trusted Types surface actually reviewed.
2. **Ranked findings** — each with file:line (or CSP directive name / policy name), defect category (`dom-xss-sink`, `postmessage-origin`, `csp-directive-gap`, or `trusted-types-gap`), the concrete source-to-sink trace or directive-level gap, and a fix sketch matching the confirmed framework's documented pattern.
3. **Confirmed-taint vs. pattern-only status** for every sink finding — never presented as equivalent to a confirmed finding.
4. **Sanitizer/Trusted-Types status per sink finding** — an explicit statement of whether a sanitizer or Trusted Types transform is present on the traced path; never inferred.
5. **OWASP category id** for every confirmed finding (e.g., A03:2021-Injection, A05:2021-Security Misconfiguration).
6. **Evidence level per finding** — `repo evidence`, `documentation-based`, or `inference`.
7. **Verdict** — approve / approve-with-notes / block.
8. **Explicit no-live-exploit statement** — confirm no exploit payload was run against any live or staging target, and state what remains to be manually confirmed (e.g., "confirming exploitability in production requires a controlled, authorized penetration test").
9. **Open questions or out-of-scope items** not covered by this review pass.

## When to push back

Push back if the user asks to:

- treat a sink match as a confirmed finding without a completed source-to-sink trace — a grep hit is not a finding,
- clear a sink finding because "we sanitize elsewhere in the app" with no sanitizer call visible on the specific traced path,
- treat CSP header presence alone as sufficient without directive-level parsing,
- run an exploit payload against a live or staging system to "prove" a finding — this skill does not perform live penetration testing; confirm via static trace and recommend authorized live testing separately if needed,
- downgrade a confirmed script-execution sink finding to informational because "it's probably fine" — this skill's default for confirmed script-execution sinks is HIGH.
