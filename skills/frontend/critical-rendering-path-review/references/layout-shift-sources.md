# Layout-Shift Source Catalog

> Verify `LayoutShift`/`LayoutShiftAttribution` performance-entry semantics against current MDN docs via Context7 before citing exact scoring behavior — the CLS scoring window/session-gap algorithm has changed since CLS's introduction. Treat any specific numeric session-window value as `documentation-based` and re-verify rather than asserted from memory.

## What people get wrong

The common bad assumption is:

> "CLS is about images without `width`/`height` — I checked, they're all fine."

Incomplete. Unsized media is the textbook example, but it is one of at least five distinct source categories, and a page can pass a naive "all images have dimensions" check while still shipping a significant CLS regression.

## Officially grounded definition

Per MDN's CLS glossary entry and the Layout Instability API (`LayoutShift`/`LayoutShiftAttribution`), a layout shift is scored when a visible element's start position changes between two rendered frames, without that change being the direct result of a user interaction. `LayoutShiftAttribution` reports the specific DOM node(s) responsible along with `previousRect`/`currentRect`, which is the ground-truth way to attribute a shift to a specific element rather than guessing from a diff.

## Source categories to audit

### 1. Unsized media (the well-known one)

Any `<img>`, `<video>`, `<iframe>`, or `<embed>` without explicit `width`/`height` attributes (or a CSS `aspect-ratio`/explicit sized container) reserves no space before the resource loads, so its arrival pushes surrounding content. This applies equally to background images used as layout-affecting content and to responsive images (`srcset`) — the reserved box must match the *intrinsic* aspect ratio, not just have some placeholder height that doesn't match the eventual rendered size.

### 2. Web-font swap (FOIT/FOUT)

A font that loads after initial text paint and has different metrics (x-height, glyph width) than the fallback font shifts every line it affects when it swaps in. This is invisible to an "images have dimensions" check entirely. Mitigations to look for:

- `font-display: swap` (or `optional`) on `@font-face`, chosen deliberately rather than defaulted to `auto`/`block`.
- A `<link rel="preload" as="font" crossorigin>` for the LCP-critical font so it's fetched earlier and the swap window is shorter.
- Font-metric-matching fallback stacks (e.g. via `size-adjust`/`ascent-override`/`descent-override` in a matching `@font-face` fallback declaration, or a generated fallback via a tool that computes these) to minimize the visual delta when the swap happens, when the swap itself can't be eliminated.

Flag any new `@font-face` with neither a `font-display` strategy nor a metric-matched fallback as an unaddressed CLS/LCP risk.

### 3. Content injected above existing content

Any DOM insertion above the current viewport's visible content — a banner, cookie-consent bar, ad slot, or async-loaded promo — that doesn't have space reserved for it shifts everything below. This is a frequent CLS source that has nothing to do with media sizing at all. Audit: does the container reserve a fixed or `min-height` slot before the async content resolves, or does content just get prepended/inserted into flow?

### 4. Animations that trigger layout instead of compositing

CSS animations/transitions on `top`/`left`/`width`/`height`/`margin` (layout-triggering properties) rather than `transform`/`opacity` (compositor-only properties) can register as layout shifts if they affect other elements' positions, and are also a distinct main-thread-cost problem independent of CLS. Flag any newly added animation on a layout-affecting property and ask whether a `transform`-based equivalent achieves the same visual effect.

### 5. FOUC/late-applied CSS affecting already-painted content

If content paints before a stylesheet (or a `<style>` block inserted by JS) fully applies, and the styles change box dimensions, that is a layout-shift source distinct from font swap — e.g. a CSS-in-JS solution that injects styles after first paint, or a critical-CSS strategy that inlines an incomplete subset and loads the rest async without accounting for elements that need the deferred rules for correct sizing.

## Non-negotiable review rules

- Do not accept "images have width/height" as a complete CLS review — walk all five categories above.
- Any element inserted above existing viewport content without a reserved slot is a blocking finding, not a nice-to-have fix, if it's likely to occur after first paint (ads, consent banners, personalization banners, notification bars).
- New `@font-face` rules must specify a deliberate `font-display` value; flag `auto`/default (browser-dependent, commonly behaves like `block`) as unreviewed.
- New layout-triggering animations (`top`/`left`/`width`/`height`/`margin` in `@keyframes` or `transition`) should be flagged with a suggested `transform`/`opacity` alternative when the visual intent allows it.
- Distinguish shifts caused by user interaction (which are explicitly excluded from CLS scoring, e.g. an accordion the user clicked to expand) from unexpected shifts — don't flag legitimate, user-triggered layout changes as CLS problems.

## Minimal safe review flow

1. Identify every element that renders asynchronously or after a network/font/JS dependency resolves (images, fonts, injected banners, lazy content, animated elements).
2. For each, check whether visual space is reserved before it resolves (`width`/`height`, `aspect-ratio`, `min-height` container, skeleton placeholder).
3. For fonts specifically, confirm a `font-display` strategy and, for LCP-critical fonts, a preload.
4. For animations, confirm they animate compositor-only properties where visually equivalent.
5. State whether the verdict is markup/CSS-inferred (`inference`) or confirmed via a live `LayoutShift`/`LayoutShiftAttribution` observer trace (`live evidence`) — attribution data is the only way to confirm which specific element actually caused a measured shift versus which one merely looks suspicious in markup.

## When to push back

Push back if the user says:

- "it's just one image, CLS won't notice" — a single large above-the-fold image is often the single biggest CLS contributor on a page,
- "we'll add the font-display later" — that is deferring a known, cheap-to-fix CLS/LCP source indefinitely,
- "the banner only shows sometimes, so it doesn't count" — intermittent injection is still a real-user-experience regression for the users who see it, and CrUX aggregates across all real loads including that one,
- "let's just wrap it in `overflow: hidden` so the shift isn't visible" — that hides the symptom from a visual check without preventing the actual layout recalculation, and does not necessarily improve the measured CLS score.
