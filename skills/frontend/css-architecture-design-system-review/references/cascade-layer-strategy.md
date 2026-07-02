# Cascade layer strategy

Use this reference when establishing or auditing a `@layer` order for a codebase, or resolving a specificity conflict between two legitimate rules.

## What people get wrong

The naive story is:

> Cascade layers are just a naming convention — put rules in a `@layer` block and specificity stops mattering.

Half right, and the half that's wrong is dangerous. Layers change *which layer wins*, but specificity still decides the winner *within* a layer, and — critically — the precedence order **inverts** for `!important` declarations. Per MDN's cascade-layers guidance: earlier-declared layers win over later-declared layers for normal declarations, but for `!important` declarations, earlier-declared layers win over later ones too — meaning an `!important` in an early "reset" layer can beat an `!important` in a later "overrides" layer, which is the opposite of what most engineers assume ("overrides should always win"). Unlayered styles sit in an implicit final layer for normal declarations, but unlayered `!important` styles have *lower* precedence than any layered `!important` declaration. Get this backwards and you'll "fix" a cascade bug by adding `!important` to the wrong layer and watch it silently lose anyway.

## Officially grounded shape (MDN / W3C Cascade 5)

- `@layer name1, name2, name3;` declares layer order upfront, independent of where each layer's rules are later defined in the file.
- Layer order determines precedence **regardless of selector specificity** for normal (non-important) declarations — a low-specificity selector in a later layer beats a high-specificity selector in an earlier layer.
- Unlayered styles form an implicit layer that comes **last** (highest precedence) for normal declarations.
- For `!important` declarations, the precedence order is **inverted**: earlier-declared layers' important styles win over later layers' important styles, and layered important styles always beat unlayered important styles.
- Inline `style=""` importance still beats all author-layer `!important` declarations, regardless of layer order.
- `@import url(...) layer(name);` can assign an entire imported stylesheet to a layer, which is useful for third-party CSS you don't control (framework resets, component libraries) — put it in an early layer so your own rules can override it without `!important`.

## Non-negotiable design rules

### 1. Declare the full layer order in one place, upfront

Do not let layer order emerge implicitly from file-load order. A single `@layer reset, base, tokens, components, utilities, overrides;` statement (naming may vary per codebase convention) at the top of the entry stylesheet is the source of truth. If layer order is scattered or redeclared inconsistently across files, the *effective* order becomes whatever browser resolves first-seen — audit for this as a HIGH-severity finding.

### 2. `!important` inside a layer is not a free pass — verify the inversion

Before approving `!important` anywhere, confirm which layer it lives in and whether the intended "win" actually happens under the inverted precedence rule. An `!important` added to `overrides` (a late layer) to "make sure it wins" will **lose** to an `!important` already present in an earlier layer like `reset`. This is the single most common cascade-layer mistake — check it explicitly, don't assume.

### 3. Third-party/vendor CSS goes in the earliest layer

Wrap vendor resets, component-library base styles, or any CSS the team doesn't control in its own early layer (or `@import ... layer(vendor);`). This guarantees your own component/utility layers can override it with normal specificity, with no `!important` needed at all — that's the actual payoff of adopting layers.

### 4. Utilities still need to out-rank components, by layer not specificity

If the codebase uses utility classes (e.g. Tailwind-style) alongside component styles, utilities must live in a layer declared *after* the components layer. Do not rely on utility classes having higher specificity than component selectors — that's fragile and exactly the pattern layers exist to replace.

### 5. New ID-selector or `!important` usage outside an established layer is a regression, not a quick fix

Once a codebase has adopted cascade layers, any new unlayered `!important` or ID-selector styling should be treated as bypassing the system the team already invested in. Flag it and point to the correct layer.

## Specificity primer (for the parts layers don't cover)

Within a single layer (or in a codebase with no layers), specificity still resolves conflicts using the standard weight: inline style > ID selectors > class/attribute/pseudo-class selectors > type/pseudo-element selectors, with `!important` overriding all of the above except a later `!important` of equal-or-higher-weight origin. Report specificity as a 4-part tuple (inline, IDs, classes/attrs/pseudo-classes, types/pseudo-elements) when it materially matters to a finding — do not just say "too specific" without the number.

## Minimal safe implementation flow

1. Confirm whether the codebase already declares a layer order. If yes, audit new rules against it. If no, do not retrofit an entire stylesheet into layers as a side effect of a routine review — recommend it as a separate, scoped initiative and note the current specificity-conflict risk in the meantime.
2. For any flagged `!important` or ID selector, identify the specific rule it's fighting and the minimal layer-based fix (move to correct layer, or remove the escalation entirely because the real fix is layer placement).
3. Verify inverted `!important` precedence explicitly for any layer-crossing `!important` conflict before declaring a winner — do not eyeball it.
4. Note any third-party CSS that isn't isolated into its own layer as a maintainability risk.

## Adversarial checklist

Before finalizing a cascade-layer finding, answer these:

- Is the full layer order declared in exactly one place, and does every file agree with it?
- If this conflict involves `!important` on both sides, did I apply the inverted precedence rule, or did I default to "later wins" (which is wrong for important declarations)?
- Is this `!important` compensating for a missing/incorrect layer assignment, rather than a legitimate necessity (e.g., overriding inline styles from a CMS)?
- Is the specificity claim backed by an actual computed tuple, or asserted qualitatively ("too specific")?

If any answer is "not sure," lower the finding's confidence and label it `documentation-based, needs live cascade-order verification` rather than presenting it as a confirmed defect.
