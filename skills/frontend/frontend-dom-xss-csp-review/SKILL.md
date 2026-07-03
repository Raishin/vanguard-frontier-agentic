---
name: frontend-dom-xss-csp-review
description: Review frontend source for DOM XSS sinks (innerHTML, dangerouslySetInnerHTML, v-html, document.write, eval-class APIs), verify actual attacker-reachable taint flow, and audit Content-Security-Policy and Trusted Types enforcement for real bypasses rather than header-presence checks, with framework-specific sink guidance loaded progressively.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: security
---

# Frontend DOM XSS & CSP Review

## Purpose

DOM XSS and CSP misconfiguration remain the dominant client-side attack surface (OWASP A03: Injection and A05: Security Misconfiguration). Most reviews stop at grep-for-`innerHTML` or "a CSP header exists, ship it." Neither proves anything: a sink match without confirmed taint is noise, and a CSP header with `unsafe-inline`, a wildcard `script-src`, or a permissive Trusted Types default policy provides false confidence while remaining fully bypassable. This skill exists so the review stays anchored to confirmed source-to-sink taint flow and directive-level CSP/Trusted Types analysis instead of drifting into either a shallow pattern-match report or a full framework-architecture review.

## When to use

Use this skill when the user asks to:

- review code touching `innerHTML`, `outerHTML`, `dangerouslySetInnerHTML`, `v-html`, `document.write`/`document.writeln`, `eval`, `Function()`, or `setTimeout`/`setInterval` called with a string argument,
- audit or design a Content-Security-Policy header or `<meta>` tag,
- roll out or review Trusted Types enforcement (`Content-Security-Policy: require-trusted-types-for 'script'`, `trusted-types` directive, or a `TrustedTypePolicyFactory.createPolicy` call),
- review third-party script inclusion, `<script src>` origins, or subresource integrity for supply-chain/injection risk,
- triage a reported XSS finding, bug-bounty submission, or `postMessage`-based injection report.

Do not use this skill for:

- server-side template injection or SSRF review with no DOM-sink or CSP component — those are different vulnerability classes outside this skill's scope,
- confirming a finding is actually exploited in production — that requires live penetration testing or a captured exploit, which this skill explicitly does not perform (see Non-negotiables below),
- general component-architecture or state-management review with no security angle — use the relevant framework-architecture skill instead.

## Context7 Documentation Protocol

- Resolve each in-scope framework's library ID with `resolve-library-id` before citing any sink-specific or sanitizer-specific claim (React: `/websites/react_dev_reference`; Vue: `/websites/vuejs_guide`; Angular: `/websites/angular_dev` or `/websites/angular_dev_guide`).
- Read `package.json` first to confirm the actual framework and major version in use — sink APIs and sanitizer defaults changed across major versions (e.g., Angular's `DomSanitizer.bypassSecurityTrust*` methods, React's `dangerouslySetInnerHTML` contract, Vue's automatic template escaping vs. explicit `v-html`). Do not apply one framework's sink semantics to another framework's codebase.
- For CSP and Trusted Types directive semantics, Context7 does not reliably surface a maintained CSP-specific library; ground every directive claim in the `official_docs` URLs in this skill's `metadata.json` (MDN CSP header reference, W3C Trusted Types spec) and label it `documentation-based`.
- OpenTelemetry browser instrumentation (`/websites/opentelemetry_io`) has no built-in CSP-violation-report receiver or native `report-to`/`report-uri` ingestion pipeline as of the current docs — if a user asks how to observe CSP violations, state this gap explicitly (`documentation-based, gap confirmed via Context7`) rather than inventing an integration; a custom collector endpoint receiving the browser's native CSP report POST is the documented pattern, not an OpenTelemetry-specific feature.
- If Context7 is unavailable for a library in scope, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- First trace whether the sink actually receives attacker-influenceable data (URL params, query strings, API responses that render user-submitted or third-party content, `postMessage`, `localStorage`/`sessionStorage` written by another origin or execution context, `document.referrer`, `window.name`) before flagging it as a blocker. A sink match with no confirmed taint path is a lower-severity pattern-only observation, not a finding — state the distinction explicitly in every response.
- Never treat CSP header presence alone as a pass. Parse the actual directive values: flag `unsafe-inline`, `unsafe-eval`, a missing `object-src 'none'`, a missing `base-uri`, a wildcard or overly broad `script-src` (e.g., `https:` alone, `*`), and `strict-dynamic` combined with a static nonce/hash misconfiguration that defeats its purpose.
- Check the Trusted Types default policy's actual transformation logic, not just whether the API is referenced. A default policy whose `createHTML`/`createScript`/`createScriptURL` callback returns the input unmodified (a permissive pass-through) defeats the enforcement even though `trustedTypes.createPolicy('default', ...)` is present in the code.
- Use framework-current sink and escape-hatch APIs verified against the project's actual framework version from `package.json`, not assumed from memory or from a different framework's conventions (React `dangerouslySetInnerHTML`, Angular `DomSanitizer.bypassSecurityTrust*`, Vue `v-html`/`innerHTML` render-function binding).
- Check every `postMessage` event listener (`window.addEventListener('message', ...)`) for explicit `event.origin` validation before treating the handler as safe; an unchecked origin turns any cross-origin `postMessage` sender into an untraced taint source.
- Verify every `<script src>` targeting a third-party/CDN origin carries a paired `integrity` hash and `crossorigin="anonymous"` attribute (Subresource Integrity); a `<script>` tag loading from an external origin with `integrity` absent is a confirmed supply-chain finding regardless of whether the origin is currently trustworthy. Also treat any dynamic script-injection path (`document.createElement('script')` followed by `script.src = <value>`) as needing the same scrutiny: if `<value>` derives from an untrusted/remote config (a tag-manager loader, analytics config endpoint, or any API response), confirm it is checked against an origin allowlist — ideally via a Trusted Types `createScriptURL` policy — before treating the injection path as safe, since SRI does not apply to dynamically assigned `src` values at all.
- Never write, generate, or execute a working exploit payload against a live, staging, or any networked environment. Confirm taint exclusively via static analysis, code reading, and (when available) offline/local reproduction that sends no traffic to a real target — this is a static-review skill (Read/Grep/Glob/local-only Bash).
- Never print, log, or reproduce a discovered secret, token, session identifier, or credential-shaped string in findings output; flag its presence and location, and redact the value itself.
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the taint-confirmation decision tree, and the required output shape.
- [DOM XSS sink and source taxonomy](references/dom-xss-sink-source-taxonomy.md) — load only when the review scope includes a specific sink (`innerHTML`, `dangerouslySetInnerHTML`, `v-html`, `document.write`, `eval`-class API) and needs source-to-sink classification, grounded in the OWASP DOM-Based XSS Prevention Cheat Sheet.
- [CSP directive review](references/csp-directive-review.md) — load only when auditing or authoring an actual CSP header/meta value, directive by directive, grounded in the MDN CSP header reference.
- [Trusted Types enforcement review](references/trusted-types-enforcement.md) — load only when the review scope includes Trusted Types policy design or `require-trusted-types-for` enforcement mode, grounded in the W3C Trusted Types specification.
- [Third-party script governance and Subresource Integrity](references/third-party-script-governance.md) — load only when the review scope includes third-party/CDN `<script src>` inclusion, tag-manager/marketing-loader script injection, or a dynamic `document.createElement('script')` path fed by remote config, grounded in the MDN Subresource Integrity guide and the MDN CSP `script-src` reference.

## Response minimum

Return, at minimum:

- the sink(s) and/or CSP/Trusted Types surface in scope, with confirmed-taint vs. pattern-only status stated explicitly for every sink finding,
- the exact OWASP category id (e.g., A03:2021-Injection) for each confirmed finding,
- CSP/Trusted Types directive-level gap analysis — never a header-presence-only verdict,
- for any third-party/CDN `<script src>` or dynamic script-injection path in scope, an explicit SRI gap analysis (`integrity`/`crossorigin` present-and-matched, or absent and flagged) plus origin-allowlist status for any config-driven `src` value,
- framework-correct remediation code (sanitizer call, Trusted Types policy definition, or specific CSP directive change) matching the project's confirmed framework/version,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`),
- an explicit statement that no live exploit was executed, plus what remains to be manually confirmed (e.g., "confirming this is exploitable in production requires a controlled penetration test, not static review").
