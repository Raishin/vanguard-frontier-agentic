---
name: angular-template-sanitizer-security-review
description: Statically review Angular templates and components for injection via DomSanitizer bypass calls (bypassSecurityTrustHtml, bypassSecurityTrustUrl, bypassSecurityTrustResourceUrl), unsanitized [innerHTML] bindings, and dynamically bound iframe security attributes such as [attr.sandbox], grounded in Angular's own sanitizer and NG0910 documentation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-03"
  category: security
---

# Angular Template Sanitizer Security Review

## Purpose

Review Angular templates, components, and their data-flow for the documented injection classes Angular's own sanitizer architecture is built to catch, and that are only unsafe when application code deliberately or accidentally routes around it: `DomSanitizer` bypass calls (`bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, `bypassSecurityTrustResourceUrl`) fed by user-reachable input, unsanitized `[innerHTML]` bindings, and dynamically bound iframe security attributes such as `[attr.sandbox]` that Angular's own NG0910 check exists to reject. This skill exists so the review stays anchored to these documented, concrete sinks instead of drifting into a general "Angular code review" — it does not re-litigate change detection, signals architecture, or component design in every response.

## When to use

Use this skill when the user asks to:

- review whether a `bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, or `bypassSecurityTrustResourceUrl` call is safe,
- assess whether an `[innerHTML]` binding is safe,
- review an `<iframe>` embed for a dynamically bound security attribute (`[attr.sandbox]`, `[sandbox]`, `[attr.allow]`, `[attr.credentialless]`) or an NG0910 error,
- perform a pre-launch security review of Angular templates that render user- or third-party-supplied content.

Do not use this skill for:

- Angular architecture review with no security angle (signals usage, change-detection strategy, component decomposition quality) — use `angular-architecture-signals-review` instead,
- SSR/hydration-specific defects (`TransferState` leakage, hydration mismatches) — use `angular-ssr-hydration-review` instead,
- a bug that requires live traffic reproduction (a captured XSS payload in a running app, a browser-based exploit proof) to confirm exploitation — static analysis proves the structural risk, not that it has already been exploited in production.

## Context7 Documentation Protocol

- Resolve the Angular library ID with `resolve-library-id` (matched result: `/angular/angular`) before citing any sanitizer-mechanism or NG0910 claim.
- `/angular/angular` is Angular's own source-and-docs repository (high reputation, corroborated against `packages/platform-browser/src/security/dom_sanitization_service.ts`, `packages/core/src/sanitization/html_sanitizer.ts`, `packages/core/src/render3/instructions/shared.ts`, and `adev/src/content/reference/errors/NG0910.md`). Use `query-docs` against it to confirm low-level sanitizer mechanics directly from source: that `bypassSecurityTrustHtml` (and its URL/resource-URL siblings) mark a value trusted so `sanitize()` skips the HTML/URL sanitizer and returns the value unchanged, that property bindings like `[innerHTML]` accept a compiler-added sanitizer function that only risky properties receive, and that text interpolation instead calls `renderer.setValue()` on a text node — a path that never parses HTML and needs no sanitizer.
- Before flagging a `[attr.sandbox]`/`[sandbox]`/`[attr.allow]`/`[attr.credentialless]` binding, confirm Angular's own NG0910 check: these attributes are documented as required to be static (fixed at element-creation time) because they configure the iframe's security model before `src`/`srcdoc` load — a dynamic binding either throws NG0910 in dev or, if that check is suppressed, lets a runtime value weaken the sandbox.
- Read the component's imports first to confirm `DomSanitizer` (from `@angular/platform-browser`) is actually in use before assuming a bypass call exists — do not assume a project uses the bypass APIs without confirming the import and call site.
- If Context7 is unavailable, fall back to the `official_docs` URLs in this skill's `metadata.json` and label the claim `documentation-based, unverified against current release`.

## Lean operating rules

- Injection findings default to HIGH severity. This is a security-scoped skill: do not downgrade an untraced `bypassSecurityTrustHtml`/`bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl` call or an unsanitized `[innerHTML]` binding to MEDIUM just because it has not been observed exploited yet — the risk is in the structure, not in whether someone has already hit it.
- Trace every finding to a concrete file:line and a concrete data-flow path. A finding that says "this bypass call might be unsafe" or "this innerHTML might be risky" without showing the specific origin (route param, query string, request body, or a third-party API response that itself echoes user input) and the specific sink is not a valid finding — it is a guess.
- Do not approve any `bypassSecurityTrustHtml`, `bypassSecurityTrustUrl`, or `bypassSecurityTrustResourceUrl` call whose input includes user-reachable data unless the trace shows the value was validated or sanitized on that exact path before the bypass call — the bypass APIs are intentional escape hatches from Angular's compiler-driven sanitization, not accidental gaps, so their mere presence is not evidence of prior review.
- Do not approve an `[innerHTML]` binding fed by user-reachable input unless a named sanitizer call (e.g., `DomSanitizer.sanitize(SecurityContext.HTML, ...)`, or a project-specific equivalent) is visibly present on that exact data-flow path. A sanitizer import existing elsewhere in the codebase does not clear this bar — trace the specific path under review.
- Flag any `[attr.sandbox]`, `[sandbox]`, `[attr.allow]`, or `[attr.credentialless]` binding on an `<iframe>` as a finding regardless of whether NG0910 is currently thrown in the reviewed environment — a suppressed or bypassed dev-mode check does not make the underlying dynamic-security-attribute pattern safe in production.
- Check dynamic `[href]`/`[src]` bindings fed through `bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl` for scheme validation (an allowlist rejecting `javascript:` and other non-`http(s)` schemes) on the traced path before the bypass call — the bypass call itself performs no validation.
- Never execute, build, or run application code, and never send live requests, as part of this review; this is a static-review skill (Read/Grep/Glob only).
- Load only the reference needed for the concern in scope.

## References

Load these only when needed:

- [Review workflow and findings contract](references/workflow-and-output.md) — use for the step-by-step review procedure, the sanitizer-bypass/innerHTML/iframe decision tree, and the required output shape.
- [DomSanitizer bypass calls and innerHTML](references/sanitizer-bypass-and-innerhtml.md) — load only when the review scope includes a `bypassSecurityTrustHtml`/`bypassSecurityTrustUrl`/`bypassSecurityTrustResourceUrl` call or an `[innerHTML]` binding. Includes the OWASP XSS grounding reference; load that citation only when a finding is actually present.
- [Dynamic iframe security attributes](references/iframe-security-attributes.md) — load only when the review scope includes an `<iframe>` element with a bound `sandbox`, `allow`, or `credentialless` attribute, or a reported NG0910 error.

## Response minimum

Return, at minimum:

- the component(s), template binding(s), and/or `DomSanitizer` call site(s) in scope,
- ranked findings with file:line evidence, defect category (`xss`, `url-injection`, or `iframe-sandbox-escape`), the concrete data-flow trace (origin-to-sink path naming every hop), and a fix sketch matching Angular's documented pattern,
- for every bypass call or `[innerHTML]` finding, an explicit statement of whether validation or sanitization is present on the traced path — never approve on the assumption one exists elsewhere,
- evidence level per finding (`repo evidence`, `documentation-based`, or `inference`), with structural risk findings explicitly labeled as structural risk, not as confirmed-exploited,
- verdict (approve / approve-with-notes / block),
- open questions or scope the review could not cover (e.g., "confirming actual exploitation requires a live payload test in a running browser session, not static review").
