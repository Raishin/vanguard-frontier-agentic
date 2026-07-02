# Responsive strategy decision guide

Use this reference when deciding container query vs. media query, or auditing reflow/resize-text compliance at 400%/200% zoom.

## What people get wrong

The naive story is:

> Container queries are the new best practice — replace media queries with them wherever possible.

Wrong. Container queries and media queries answer different questions and are not interchangeable. A media query answers "how big is the viewport (or does the user prefer reduced motion / dark mode / etc.)?" A container query answers "how big is *this specific containing element*, regardless of viewport?" A component meant to be reusable inside a narrow sidebar *and* a wide main-content area needs a container query — a media query cannot express that, because the viewport is the same in both placements. Conversely, a page-level layout shift (e.g. switching a top-nav to a hamburger menu) is inherently viewport-driven and belongs in a media query — wrapping the whole page in a container just to query it is unnecessary indirection.

## Officially grounded shape (MDN CSS Containment / Container Queries)

- `container-type: inline-size;` (or `size`) on an ancestor establishes a **query container**; `container-name` optionally scopes which `@container` rules target it.
- `@container (width >= 400px) { ... }` and named variants `@container excerpt (width >= 400px) { ... }` share the same comparison syntax and logical operators (`and`, `or`, `not`) as media queries.
- Container **style** queries (`@container style(--flag: value)`) also exist for querying a custom-property value on the container, distinct from size queries — verify which variant (`size` vs `style` vs both) the codebase/browser target actually needs before recommending one.
- A container query cannot query the size of the element it's applied to relative to itself (no self-referential sizing loop) — the `container-type` must be set on an **ancestor**, and the query targets descendants of that ancestor.
- Setting `container-type: size` or `inline-size` on an element applies layout/size containment to it, which affects how that element's children are sized (e.g. percentage heights) — this is a real layout side effect, not a no-op flag, and must be accounted for, not just bolted on to "enable container queries."

## Non-negotiable design rules

### 1. Ask "does this depend on its container, or on the viewport?" before choosing

If the same component needs different styling depending on *where it's placed* (sidebar vs. full-width), that's a container-size dependency → container query. If the same component always needs the same styling regardless of placement, but that styling should change based on *screen size*, that's a viewport dependency → media query. Do not default to container queries just because they are newer; recommending one when the actual dependency is viewport-based is a correctness bug, not a stylistic choice.

### 2. Verify the containment side effect before approving `container-type`

Applying `container-type: size` or `inline-size` establishes containment that can change how a container's children resolve percentage-based dimensions and can affect the container's own intrinsic sizing behavior. Flag any new `container-type` addition to an element whose children rely on percentage heights or intrinsic sizing without a check that behavior is still correct.

### 3. Do not recommend "just add both" as a hedge

Applying both a media query and a container query to the same rule for the "same" breakpoint without a distinct rationale for each is a maintainability trap — two systems now need to be kept in sync for one visual outcome. If both are genuinely needed (e.g., viewport-level layout change *and* the component independently needs to adapt to a narrower container within that layout), state the distinct rationale for each explicitly.

### 4. Media queries remain correct — and required — for preference-based and non-size media features

`prefers-reduced-motion`, `prefers-color-scheme`, `prefers-contrast`, `forced-colors`, and print media all have no container-query equivalent; container queries only address size/style-of-container, not user/device preference signals. Never suggest replacing a preference-based media query with a container query.

## WCAG 2.2 reflow and resize-text audit

- **1.4.10 Reflow**: content must be usable without horizontal scrolling (except content requiring 2D layout, like data tables or images) at a 320 CSS-px equivalent viewport width (i.e., 400% zoom on a 1280px-wide viewport). Flag any fixed-width container, fixed `min-width` wider than ~320px on primary content, or `overflow-x` forced on the body/main content region as a reflow-failure risk.
- **1.4.4 Resize text**: text must remain readable and functional when resized up to 200% without loss of content or functionality, without requiring assistive technology. Flag any font-size defined in `px` on root/body text without a corresponding relative-unit (`rem`/`em`/`%`) sizing strategy, and flag any fixed-height text container that would clip text at 200% zoom.
- Fixed viewport units (`vh`/`vw`) used for text container height, combined with fixed-size text, is a common reflow+resize-text double failure — flag it as one finding with both WCAG criteria cited, not two separate notes.
- These are static-review flags based on markup/CSS inspection, not a substitute for live zoom/reflow testing in a real browser — always label the finding `documentation-based risk, requires live verification at 400%/200% zoom`.

## Minimal safe implementation flow

1. For each new/changed responsive rule, identify what the styling actually depends on: container size, viewport size, or a user/device preference.
2. Match the dependency to the correct query type per the rules above.
3. If `container-type` is newly applied, check descendant elements for percentage-based sizing that containment could affect.
4. Scan for fixed-width primary-content containers, fixed-px text sizing, and fixed-height text containers as reflow/resize-text risk flags.
5. Report all size-based container-query/media-query findings and all WCAG 1.4.10/1.4.4 flags as `documentation-based risk` pending live-browser verification — never assert a WCAG pass/fail from static review alone.

## Adversarial checklist

Before finalizing a responsive-strategy finding, answer these:

- Is the actual styling dependency container-size, viewport-size, or a user/device preference — confirmed, not assumed?
- If recommending `container-type`, did I check whether it changes containment behavior for existing percentage-sized children?
- Am I recommending both a media query and a container query for the same visual outcome without a distinct rationale for each?
- Have I mislabeled a static-review flag as a confirmed WCAG pass/fail instead of a risk pending live verification?

If any answer is "not sure," lower the finding's confidence and label it `documentation-based, requires live verification` rather than presenting it as a confirmed defect.
