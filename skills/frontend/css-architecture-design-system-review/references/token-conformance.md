# Design token conformance patterns

Use this reference when auditing hardcoded-value drift against an existing custom-property token system, including token-tier (primitive vs. semantic vs. component) conventions.

## What people get wrong

The naive story is:

> If a codebase has CSS custom properties, any hex code or `px` value found in a stylesheet is a violation — just swap in the nearest-looking token.

Wrong on two counts. First, not every hardcoded value is drift — a one-off value with no semantic meaning (e.g. a decorative `box-shadow` blur radius unique to one component) may legitimately not belong in the token system. Second, "nearest-looking" is the wrong selection criterion: swapping a hardcoded `#3B82F6` for whatever token happens to render the closest shade of blue, without checking whether that token is semantically named for a *different* purpose (e.g. `--color-danger` vs `--color-brand-primary` happening to be visually similar), silently couples unrelated UI concerns and will drift the moment either token's value changes for its actual intended purpose.

## Officially grounded shape (W3C CSS Custom Properties / MDN)

- CSS custom properties (`--token-name: value;`) are just inherited, cascade-participating properties — they carry no built-in type, tier, or namespace semantics. Any tiering convention (primitive/semantic/component) is a *codebase convention*, not a CSS-spec-enforced structure. Verify the codebase's actual convention before applying a generic three-tier assumption.
- `var(--token-name, fallback)` resolves to `fallback` only if the custom property is unset or invalid at computed-value time — an empty-string or whitespace-only fallback is valid and will not error, which can silently produce unstyled output. Flag fallback values used to paper over a token that should exist but doesn't.
- Custom properties are visible to and overridable at any DOM node via the cascade — a component-scoped token override (e.g. `.card { --spacing-gap: 8px; }`) is a normal and often correct pattern, not automatically drift.

## Non-negotiable design rules

### 1. Establish which token tiers actually exist in this codebase before auditing

Common conventions: **primitive** (raw values: `--blue-500: #3B82F6`), **semantic** (purpose-bound: `--color-brand-primary: var(--blue-500)`), **component** (scoped: `--button-bg: var(--color-brand-primary)`). Not every codebase has all three tiers. Read the existing token file(s) first — do not assume a tier structure the codebase hasn't adopted.

### 2. Flag hardcoded values only when a token exists for that exact purpose

If a semantic token like `--spacing-md` exists and a new rule hardcodes `16px` where `--spacing-md` resolves to the same value, that's drift — flag it and name the token. If no token covers that specific purpose (e.g. a genuinely one-off decorative value), do not force a token substitution; note it as "no matching token — token-system gap or legitimately one-off" and let a human decide whether to add a token.

### 3. Recommend the correct tier, not just the nearest value

When a hardcoded value should become a token, point to the most specific applicable tier — component tier if a component-scoped token already exists for that role, semantic tier if not, primitive tier only as a last resort (raw primitives should rarely be referenced directly from component styles; that's what semantic tokens are for). Recommending a raw primitive when a semantic token already covers the case reintroduces exactly the coupling problem tokens exist to prevent.

### 4. Treat `var()` fallback values as a signal, not noise

A `var(--token, <hardcoded-fallback>)` pattern used defensively across many call sites for the *same* token is a sign the token itself may be missing from some build/theme context — flag it as a build/theming risk, not a per-call-site style nitpick.

### 5. Do not flag legitimate component-scoped overrides as drift

`.card--compact { --card-padding: var(--spacing-sm); }` is normal token-system usage (a component redefining a token it owns, still referencing the token system). Only flag when the override value is hardcoded raw instead of referencing another token, or when it silently diverges from the token's documented purpose.

## Minimal safe implementation flow

1. Locate and read the codebase's token definition file(s) (commonly `:root { --... }` blocks, a `tokens.css`, or a design-tokens JSON/YAML source feeding generated CSS).
2. Determine the tier convention in use, if any.
3. Grep the diff/file under review for raw color (`#`, `rgb(`, `hsl(`), spacing (`px`, `rem` outside of token definitions), and typography (font-family, font-size literals) values.
4. For each hit, check whether an existing token resolves to the same or functionally equivalent value and purpose. Report a match, a "gap" (no token exists), or confirm it's an intentional one-off.
5. Never invent a token name that doesn't exist in the codebase and present it as if it does — only recommend using tokens that are confirmed present via Read/Grep.

## Adversarial checklist

Before finalizing a token-conformance finding, answer these:

- Does a token actually exist for this exact purpose, confirmed via Read/Grep — not assumed from naming convention alone?
- Am I recommending the most specific applicable tier (component > semantic > primitive), or just the first token that visually matches?
- Could this hardcoded value be a legitimate one-off rather than drift?
- Is this `var()` fallback masking a real token-availability gap in some theme/build context?

If any answer is "not sure," report it as "no matching token found — confirm with design-system owner" rather than asserting a specific token substitution as fact.
