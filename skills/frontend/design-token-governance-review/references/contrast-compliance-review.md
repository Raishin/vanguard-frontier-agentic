# Contrast Compliance Review

Use this reference when computing or verifying resolved contrast ratios for token pairings (text-on-background, UI-component/border/icon/focus-indicator) against WCAG success criteria 1.4.3 and 1.4.11, across every theme variant a token set ships.

> Version note: Storybook addon-a11y capabilities and axe-core detection coverage evolve. Verify tool behavior against installed version and current Context7 docs before asserting coverage.

## What people get wrong

The naive story is:

> "We ran the tokens through an automated a11y check once, so contrast is handled."

Wrong, for at least three reasons:

1. **One theme is not all themes.** A token pairing that passes in light mode has no automatic bearing on the same semantic pairing resolved in dark mode or high-contrast mode — each theme resolves the same token *names* to different literal values, and each resolution must be checked independently.
2. **Automated tooling has documented, partial coverage.** Storybook's `@storybook/addon-a11y` is built on axe-core and is documented (verified via Context7, `/storybookjs/storybook`) as automatically catching up to approximately 57% of WCAG issues. A clean addon-a11y run is evidence of "no automatically detectable violation in the states exercised," not evidence of full WCAG conformance.
3. **1.4.3 and 1.4.11 are different success criteria with different scopes and thresholds.** Treating them as interchangeable produces both false passes (applying the lower non-text threshold to body text) and false alarms (applying the text threshold to a decorative icon that WCAG doesn't require to meet either).

## Officially grounded thresholds (WCAG 2.1)

- **1.4.3 Contrast (Minimum):** normal text and images of text must have a contrast ratio of at least **4.5:1** against its background; large-scale text (per WCAG's definition of large text) needs at least **3:1**. This applies to text-on-background token pairings — body copy, labels, placeholder text if it conveys required information, etc.
- **1.4.11 Non-text Contrast:** visual information required to identify UI components and states (borders on input fields, focus indicators, icons that convey meaning, graphical objects required to understand content) must have a contrast ratio of at least **3:1** against adjacent color(s). This applies to a different set of token pairings than 1.4.3 — component borders, focus rings, icon-on-background — and uses a flat 3:1 threshold regardless of size.
- Both criteria are about **resolved, rendered** contrast — the actual sRGB values that end up on screen after every token reference and theme override is applied — not about the token's declared or "intended" relationship.

## Non-negotiable design rules

### 1. Compute the ratio; do not eyeball it or trust the token name

Resolve both sides of the pairing (foreground token, background token) to final hex/RGB values for the theme variant under review, then compute the WCAG relative-luminance contrast ratio. A name like `--text-on-surface` does not guarantee the resolved pair clears any threshold — that is exactly the kind of drift this skill exists to catch after a rebrand or a single token-value edit.

### 2. Route each pairing to the correct success criterion

Before computing anything, classify the pairing: is it text-on-background (1.4.3) or a UI-component/graphical-object pairing (1.4.11)? Do not apply 4.5:1 to a focus-indicator token pairing or 3:1 to body text — cite the criterion you're actually applying, and the correct threshold for it.

### 3. Every theme variant is a separate finding set

Light, dark, and high-contrast (or any other shipped theme) must each be evaluated independently against the tokens that variant resolves to. A pass in one variant must never be reported as if it covers another. If a theme variant is missing from what was provided, say so explicitly as an evidence gap rather than assuming parity.

### 4. Automated-tool evidence is a floor, not a ceiling

If the user supplies Storybook `addon-a11y` (or equivalent axe-core-based) results, treat a "no violations" result as partial coverage evidence only, and say so using the ~57% figure grounded via Context7. Do not upgrade a clean automated run to "WCAG 1.4.3/1.4.11 compliant" without independently computing the ratio for the specific pairings in scope.

### 5. State-dependent tokens need their own check

Hover, focus, active, and disabled states often resolve to different token values than the resting state. A resting-state pass does not imply a focus-state pass — focus indicators are explicitly in scope for 1.4.11 and are a common place for contrast regressions to hide.

## Minimal safe review flow

1. Enumerate the token pairings in scope and classify each as text-on-background (1.4.3) or UI-component/graphical (1.4.11).
2. For each theme variant provided, resolve both sides of each pairing to final color values (following token references/aliases through to their literal value for that theme).
3. Compute the WCAG relative-luminance contrast ratio for each resolved pairing.
4. Compare against the correct threshold for the criterion (4.5:1 normal text / 3:1 large text for 1.4.3; 3:1 for 1.4.11), and record pass/fail per pairing per theme variant.
5. If any automated-tool evidence (addon-a11y/axe-core) was supplied, report it as supplementary partial-coverage evidence, not as the basis for the pass/fail verdict on the specific pairings you computed.
6. Flag any state (hover/focus/active/disabled) or theme variant that was not supplied as an explicit evidence gap, not a silent pass.

## Adversarial checklist

Before reporting a contrast finding, answer these:

- Did you resolve every token reference/alias to a literal color for the specific theme variant, or did you compute against an intermediate/aliased value?
- Is this pairing actually text, or a UI component/icon/border — and does your cited threshold match that classification?
- Did you check this pairing in every theme variant supplied, or only the default one?
- Did you check the focus-visible state separately, given it is explicitly a 1.4.11 concern and often uses a different token than the resting border?
- If you're relying on an automated tool result, did you disclose its documented partial-coverage limitation rather than presenting it as a conformance determination?

If you cannot answer those, the contrast finding is not ready to report.

## High-risk assumptions to kill

- "The design system says this pairing is accessible" — design-system documentation can be stale relative to the current resolved token values; compute, don't cite intent.
- "It passed axe-core in Storybook, so it's WCAG compliant" — axe-core-class tooling is documented as catching a minority of WCAG issues; contrast math specifically is one of the more reliably automatable checks, but a "pass" scope is still limited to states/variants actually exercised in the stories run.
- "Dark mode uses the same relationships as light mode, just inverted, so it's fine" — inversion does not preserve ratios; each variant's resolved values must be computed independently.
- "It's just a decorative icon, ignore it" — if the icon is required to identify a UI component or convey state (not purely decorative), 1.4.11 applies; verify which case it is before dismissing it.

## Safe verification targets

- The resolved (post-reference, post-theme-override) hex/RGB value for each side of each pairing under review, for each theme variant.
- The computed contrast ratio and the specific criterion (1.4.3 vs 1.4.11) and threshold it was checked against.
- Any automated-tool report supplied by the user (Storybook a11y-addon panel output, CI a11y job output), labeled as partial-coverage supplementary evidence.

## When to push back

Push back if the user asks you to:

- report a pairing as "compliant" based on the token name or design intent alone, with no resolved-value computation,
- treat a single theme variant's result as representative of all shipped variants,
- treat a clean automated a11y-addon run as a full WCAG conformance statement,
- skip focus/hover/active state checks because "the resting state passed."

Those are not shortcuts. They are exactly the kind of gap this review exists to close.
