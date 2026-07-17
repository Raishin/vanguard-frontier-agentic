# Dynamic block output escaping

## Why this matters

A dynamic block's render function does not run once at build time — it runs on the server for every single front-end page view that includes the block. If that render function interpolates a dynamic value (a block attribute, post meta, a query result) into its returned markup without escaping, the resulting cross-site-scripting vector is not a one-off, one-visitor bug: it fires for every visitor, on every render, for as long as the block appears on the page. This is the single highest-severity, highest-blast-radius pattern this skill reviews for.

## NORMATIVE: dynamic blocks render on the server at request time via render_callback or render.php

Per the WordPress Block Editor Handbook's "Static or Dynamic rendering" documentation (`documentation-based`): "Blocks with 'dynamic rendering' are designed to generate their content and structure in real-time when requested on the front end." The handbook identifies two supported ways to implement this: "Using the `render_callback` argument that can be passed to the `register_block_type()` function" and "Using a separate PHP file usually named `render.php`." It further states that on the front end, "the `render_callback` is used to dynamically render the markup for the block depending on the specific values on the server at the time the block is requested" — the render happens per-request, using live server-side values, not once at authoring time.

The same page's own example (the Site Title block's dynamic render) demonstrates the expected pattern directly: `esc_url()`, `esc_attr()`, and `esc_html()` are applied to dynamic values before they are placed into the returned markup. This is the documented reference pattern reviewers should compare a given `render_callback`/`render.php` against.

## Reviewer evidence criteria

For every dynamic block `render_callback` or `render.php` in scope:

- Identify every dynamic value that reaches the returned markup: block attributes (`$attributes`), post/site data fetched via WordPress functions (`get_post_meta()`, `get_the_title()`, a custom query result), and any other value not authored as static markup in the block's own template.
- For each such value, confirm the escaping function matches its output context:
  - text content inside an HTML element: `esc_html()`.
  - a value printed into an HTML attribute: `esc_attr()`.
  - a URL, including `src`/`href` attribute values: `esc_url()`.
  - content that is itself expected to contain limited, allowed HTML (e.g. rendering a subset of post content): `wp_kses_post()` rather than `esc_html()`, which would strip the allowed markup entirely.
- Flag a dynamic value that reaches output with no escaping function applied at all as a direct site-wide XSS finding — the highest-severity finding class in this skill.
- Flag escaping applied earlier in the function (e.g. sanitizing on input) but not re-applied at the actual output point, since a value can be mutated or recombined between input and output; escaping must hold at the point of output, not merely somewhere upstream.
- Do not flag static markup authored directly in the block's template with no dynamic interpolation — escaping applies to dynamic values, not to literal markup the developer wrote.

## RECOMMENDATION: how to phrase a dynamic-block escaping finding

- Name the block (registered block name) and the exact file/line in the `render_callback`/`render.php` where the unescaped value is emitted.
- Identify the dynamic value's source (attribute name, meta key, query) so the reviewer or maintainer can see exactly what an attacker-controlled or otherwise untrusted input would need to look like to exploit it.
- State the output context (HTML content / attribute / URL / limited-HTML content) and name the specific escaping function that context requires.
- Note that because dynamic block rendering executes on every front-end view, this is a standing, always-live vector once the block is published — not merely a reachable path found in review.

## Applicable versions

- Block Editor Handbook static/dynamic rendering guidance: current Block Editor Handbook (`render_callback` and `render.php` as the two supported dynamic-rendering mechanisms). Re-verify against the current handbook before relying on details beyond what is quoted here, since block-editor APIs evolve across WordPress releases.

## Sources

- [WordPress Block Editor Handbook — Static or Dynamic rendering of a block](https://developer.wordpress.org/block-editor/getting-started/fundamentals/static-dynamic-rendering/) — supports the definition of dynamic rendering, the `render_callback`/`render.php` mechanisms, the per-request server-side execution model, and the Site Title block's `esc_url()`/`esc_attr()`/`esc_html()` reference example.

Last verified: 2026-07-16.
