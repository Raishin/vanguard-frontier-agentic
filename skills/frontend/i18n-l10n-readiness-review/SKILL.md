---
name: i18n-l10n-readiness-review
description: Audit frontend code for internationalization readiness — externalized ICU MessageFormat strings, Intl-based date/number/currency/plural formatting, correct lang/dir attribute propagation, and RTL-safe CSS logical properties — before translation vendor engagement, with CLDR plural-rule detail loaded only when auditing countable strings.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: compliance
---

# i18n/l10n Readiness Review

## Purpose

Teams routinely discover — after paying a translation vendor — that string concatenation
can't express another language's word order, pluralization breaks outside English's
one/other split, or RTL layouts mirror visually but not functionally. Fixing these after
translation means re-engineering already-translated strings, re-running the vendor
pipeline, and eating the schedule slip. This skill audits pluralization architecture
(ICU MessageFormat / CLDR plural categories), `Intl`-based formatting, `lang`/`dir`
attribute propagation, and RTL-safe layout so the codebase is structurally
translation-ready *before* any string reaches a translator. It does not translate
anything itself.

## When to use

Use this skill when the user asks to:

- audit a codebase for hardcoded/non-externalized user-facing strings,
- review pluralization logic against CLDR plural categories for target locales,
- review date/number/currency formatting for `Intl` API usage vs manual string-building,
- review layout/CSS for RTL-market readiness,
- assess overall i18n/l10n readiness before a new-market launch or vendor engagement.

## Lean operating rules

- First establish the target locale list, including at least one locale with more than
  two CLDR plural categories (e.g., Arabic: 6, Polish: 4, Russian: 4) if any countable
  strings exist — English's one/other split hides most pluralization bugs. If the user
  has not named target locales, ask; do not assume English-only is sufficient for a
  "readiness" review.
- Flag string concatenation for user-facing countable/orderable/gendered content as a
  structural defect, not a style nit — `"You have " + count + " items"` cannot be
  correctly localized without ICU MessageFormat (`{count, plural, ...}`) or an
  equivalent, because plural-category counts and word order both vary by locale.
- Verify date/number/currency/plural values use `Intl.DateTimeFormat`,
  `Intl.NumberFormat`, or `Intl.PluralRules` (or a library wrapping them, e.g.
  FormatJS/`react-intl`, `next-intl`, `i18next` with ICU) rather than manual string
  building, hardcoded separators, or naive `Math.round`/string concatenation for
  currency.
- Check that the `dir` attribute (and `lang`) actually reach the rendered document root
  (`<html lang dir>`) and stay in sync with the active locale, and that layout uses CSS
  logical properties (`margin-inline-start`, `padding-inline-end`, `inset-inline-start`)
  instead of physical properties (`margin-left`, `padding-right`, `left`) wherever an
  RTL-market launch is on the roadmap, even if RTL isn't shipped yet — retrofitting
  physical properties across a whole codebase later is expensive.
- Do not treat the presence of a language switcher, an `i18next`/`next-intl`
  dependency, or a `locales/` translation-file structure as proof of readiness — a
  library being installed does not mean it is used correctly everywhere. Verify the
  underlying formatting/pluralization/layout mechanics independently, file by file.
- Never author, insert, or "helpfully" translate actual copy into another language;
  that is the translation vendor's job. Only flag strings for extraction and hand off
  the inventory to whoever owns translation.
- Verify icons/imagery conveying directionality (arrows, forward/back/next controls,
  progress indicators) have a documented RTL-mirroring plan when RTL is in scope —
  logical CSS properties fix layout flow but do not auto-flip iconography.
- Treat ICU plural-category coverage as version/library-sensitive: verify the exact
  syntax and category set against current FormatJS/ECMA-402/CLDR references rather than
  assuming a fixed one/few/many split applies to every language.

## Context7 Documentation Protocol

Before making any claim about ICU MessageFormat syntax, `Intl` API behavior, or a
specific i18n library's plural/formatting API, ground it via Context7:

1. Call `resolve-library-id` for the library in scope (e.g., `formatjs`, `next-intl`,
   `react-intl`, `i18next`) or the runtime spec area (`ECMA-402`/`Intl`).
2. Call `query-docs` for the specific syntax or API behavior before citing it (plural
   category names, `selectordinal` syntax, `Intl.PluralRules` options, locale-matching
   behavior).
3. Prefer official docs (FormatJS docs, TC39 ECMA-402 spec, MDN `Intl`, Unicode CLDR)
   over memory — plural rule sets and `Intl` option names are locale- and
   spec-version-sensitive.
4. If Context7 has no coverage for a library in scope, fall back to the official docs
   listed below and mark the claim `documentation-based (uncertain — no Context7
   coverage)`.
5. Never invent ICU syntax, `Intl` constructor options, or CLDR plural-category names.
   Verified from Context7 (FormatJS docs) for this skill: the canonical CLDR plural
   categories are `zero`, `one`, `two`, `few`, `many`, `other` (not every language uses
   every category — e.g. English uses only `one`/`other`; Arabic uses all six), and ICU
   MessageFormat also supports exact-value matches via `=N` (e.g. `=0 {No items}`)
   layered alongside category matches.

## References

Load these only when needed:

- [Pluralization and ICU MessageFormat audit](references/pluralization-icu-messageformat.md)
  — use only when auditing countable/orderable strings against CLDR plural categories
  or reviewing ICU MessageFormat/`Intl.PluralRules` usage.
- [Intl formatting audit](references/intl-formatting-audit.md) — use only when
  reviewing date, number, currency, or list formatting for manual-string-building
  defects vs correct `Intl` API usage.
- [RTL and logical-property layout audit](references/rtl-logical-properties-audit.md)
  — use only when reviewing `lang`/`dir` attribute propagation, CSS logical properties,
  or directional-iconography readiness for RTL markets.

## Response minimum

Return, at minimum:

- the target locale list driving the review (including any RTL or
  multi-plural-category locale considered),
- hardcoded-string inventory with file:line for user-facing countable/orderable/
  concatenated content,
- pluralization audit against the actual CLDR categories required for the target
  locale(s), not just one/other,
- RTL/logical-CSS-property findings if RTL is in scope or roadmapped, including
  `lang`/`dir` propagation status,
- explicit statement that translation itself was not performed by this skill and that
  findings are a structural-readiness audit, not a translation deliverable.
