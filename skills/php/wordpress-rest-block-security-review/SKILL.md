---
name: wordpress-rest-block-security-review
description: Use this skill to review WordPress plugin, theme, REST API, and block-editor code for missing or overly permissive REST permission_callback registration, unescaped dynamic block render_callback output, missing input validation/sanitization or output escaping, and missing nonce/capability checks on state-changing requests. Use when reviewing register_rest_route() calls, dynamic block render_callback/render.php files, form/admin-ajax handlers, or any code path that takes untrusted input and produces WordPress-rendered output. Static review only; it does not execute plugin/theme code and never contacts a live, sandbox, or staging WordPress install.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-16"
  category: security
  lifecycle: experimental
---

# WordPress REST & Block Security Review

## Purpose

Review the WordPress-specific security seams that a generic PHP reviewer is likely to miss: REST routes registered without an explicit `permission_callback`, dynamic blocks whose `render_callback` emits data without escaping, input that is neither validated nor sanitized before use, and state-changing requests with no nonce or capability check. Plugins and themes are where the WordPress ecosystem's own advisory sources place the large majority of disclosed vulnerabilities, and each of these four gaps is a documented anti-pattern against WordPress's own core APIs, not a stylistic preference.

## When to use

Use this skill when the user asks to:

- review a `register_rest_route()` call or a REST controller class for a missing or overly permissive `permission_callback`,
- review a dynamic block's `render_callback` or `render.php` for unescaped output,
- review input handling (REST params, `$_POST`/`$_GET`, shortcode or block attributes) for validation/sanitization gaps, or output for missing escaping,
- review a form submission, admin-ajax action, or REST write for missing nonce or capability checks.

## When not to use

Do not use this skill for:

- generic PHP defects with no WordPress-specific dimension (raw SQL concatenation with no WordPress DB API involved, generic type-juggling, unrelated logic bugs) — use a general PHP security review for those.
- infrastructure or hosting-environment hardening (server configuration, TLS, hosting-provider WAF rules); this skill reviews source, not deployment environment.
- dependency/supply-chain review (vulnerable third-party libraries pulled in via Composer); hand that to a supply-chain reviewer.
- any live exercise of a WordPress install — executing plugin/theme code, submitting forms, calling REST endpoints, or triggering admin-ajax actions. This skill is static review only.

## Preconditions

- The plugin or theme source in scope: REST route registrations, block registration and `render_callback`/`render.php` files, form handlers, admin-ajax actions, and shortcode callbacks.
- The WordPress version the code targets, where relevant to a version-gated API such as `permission_callback` enforcement.
- Whether any REST route or block output is genuinely intended to be public, so a correctly-scoped `__return_true` or unauthenticated render is not misflagged.

## Lean operating rules

- Confirm `permission_callback` is present on every `register_rest_route()` call in scope, and that `__return_true` (or an equivalent always-true callback) is used only where the route is genuinely public.
- Trace every dynamic value in a block's `render_callback`/`render.php` from its source to its output point and confirm the matching escaping function is applied there, not merely present somewhere in the file.
- Require validation or sanitization of every untrusted input before use, favoring validation/rejection over sanitization alone where a specific check is possible.
- Require both a nonce check and a `current_user_can()` capability check on every state-changing request; neither one alone is sufficient.
- Never request, echo, store, or reproduce a secret, API key, credential, or credential-shaped string found in code; redact-and-flag it instead.
- Label every claim `repo evidence`, `documentation-based`, or `inference`.

## Context7 documentation protocol

WordPress REST route requirements, nonce/capability semantics, and block-rendering behavior are documented on developer.wordpress.org and are the only acceptable ground truth for a version or API-behavior claim. Before asserting how `register_rest_route()`, `render_callback`, nonces, or escaping/sanitizing functions behave, cite the current developer.wordpress.org page and label the claim `documentation-based`. If a Context7-indexed source is available for the API in question, prefer it (`resolve-library-id` then `query-docs`) and label the result `context7-grounded`; otherwise use the official documentation directly. Never rely on memorized API behavior for a version-gated claim.

## Workflow

1. Enumerate the REST routes, dynamic blocks, and state-changing handlers in scope from the files provided.
2. For each `register_rest_route()` call: confirm `permission_callback` is present; if it is `__return_true` or absent, confirm public-data justification or flag it. See [REST API permission_callback enforcement](references/rest-api-permission-callback.md).
3. For each dynamic block `render_callback`/`render.php`: trace every dynamic value to its output point and confirm escaping is applied there. See [Dynamic block output escaping](references/dynamic-block-output-escaping.md).
4. For each input source and output point in scope: confirm validation/sanitization on input and escaping on output, and confirm nonce plus capability checks on state-changing requests. See [Input validation, sanitization, and output escaping](references/input-sanitize-output-escape.md).
5. Emit findings with evidence tiers, concrete exploit narratives, remediation, and verification steps; hand off any non-WordPress-specific finding to the owning reviewer.

## Decision gates

- Block only on a finding with a demonstrated reachable path (unauthenticated REST access, unescaped render output, unchecked state change) — not on the mere absence of a keyword.
- Every WordPress-API version or behavior claim is `documentation-based` (or `context7-grounded` where available), never memory.
- Every finding names the specific route, block, or handler, not a general area of the codebase.
- Non-WordPress-specific findings are handed off, not adjudicated here.

## Evidence classification

Label each finding `repo evidence` (seen directly in the code), `context7-grounded` (current documentation via Context7), `documentation-based` (official developer.wordpress.org documentation fetched directly), or `inference`. Documentation describes the intended pattern; it does not prove what a specific file actually does — always tie the claim back to the file and code path observed.

## Security and privacy constraints

Static review only. Never execute plugin/theme code, submit forms, call REST endpoints, or trigger admin-ajax actions against any live, sandbox, or staging WordPress install. Never request, echo, store, or reproduce a secret, API key, database credential, or credential-shaped string found in code; treat any such string as a redact-and-flag finding.

## Escalation conditions

Escalate to incident response any evidence the gap is already reachable in a live, publicly deployed site rather than merely present in source under review. Hand off non-WordPress-specific findings (generic PHP, infrastructure, supply-chain) to the reviewer who owns that area instead of adjudicating them here.

## References

Load these only when needed:

- [REST API permission_callback enforcement](references/rest-api-permission-callback.md) — the required `permission_callback` argument, the `__return_true` public-endpoint pattern, and REST nonce/capability enforcement.
- [Dynamic block output escaping](references/dynamic-block-output-escaping.md) — how `render_callback`/`render.php` execute and what escaping is required at each output point.
- [Input validation, sanitization, and output escaping](references/input-sanitize-output-escape.md) — the validate-input/escape-output discipline, sanitizing and escaping function reference, and nonce/capability checks for state-changing requests.

## Response minimum

Return, at minimum:

- the route(s), block(s), or handler(s) in scope and, per finding, the failure class and evidence tier;
- the concrete exploit narrative (how the gap is actually reachable, not just theoretically present);
- concrete remediation naming the specific WordPress function or pattern to add, and an exact verification step;
- handoffs for any non-WordPress-specific finding, and any incident-response escalation.

## Anti-goals

- Do not expand into generic PHP review; own the WordPress-specific seam, hand off everything else.
- Do not execute plugin/theme code or contact any live, sandbox, or staging WordPress install.
- Do not echo, reproduce, or transmit any secret, credential, or credential-shaped string.
- Do not assert a WordPress API version or behavior from memory.
