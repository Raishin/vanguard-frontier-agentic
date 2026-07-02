# Pluralization and ICU MessageFormat Audit

Use this reference only when auditing countable or orderable strings against CLDR
plural categories, or reviewing ICU MessageFormat / `Intl.PluralRules` usage.

> Version note: ICU MessageFormat syntax and the exact `Intl.PluralRules` option set
> are spec/library-version-sensitive. Verify plural-category coverage and syntax
> against current FormatJS docs, the TC39 ECMA-402 spec, and Unicode CLDR before
> flagging or clearing a finding.

## What people get wrong

The common bad assumption is:

> "Pluralization is `count === 1 ? singular : plural`. Ship it."

That is an English-only mental model, and it silently breaks every language that isn't
English (and even breaks English's `zero` case: "0 items" reads oddly next to "no
items" depending on style guide). Unicode CLDR defines up to **six** plural categories
per locale — `zero`, `one`, `two`, `few`, `many`, `other` — and which subset a language
uses is not predictable from English intuition:

- English: `one`, `other` only.
- Chinese, Japanese, Korean: `other` only (no grammatical number at all).
- Arabic: all six categories (`zero`, `one`, `two`, `few`, `many`, `other`).
- Polish, Russian: `one`, `few`, `many`, `other` (four categories, with nontrivial
  numeric-range rules for `few` vs `many`).
- Welsh: all six categories, with different numeric boundaries than Arabic.

A codebase hardcoded to a binary singular/plural split cannot express any of the
non-English cases correctly, no matter how good the translation is — the *structure*
is the defect, not the wording.

## Officially grounded ICU MessageFormat shape (verified via Context7 / FormatJS docs)

ICU MessageFormat's `plural` argument type matches a numeric value against plural
categories, with an escape hatch for exact-value overrides:

```
{itemCount, plural,
    =0 {You have no items.}
    one {You have # item.}
    other {You have # items.}
}
```

- `=0`, `=1`, etc. match an exact numeric value regardless of plural category — use
  this for language-independent special cases like "no items" rather than relying on
  `zero` (which many languages, including English, do not use as a grammatical
  category).
- `#` inside a plural branch is replaced with the formatted number itself; prefer it
  over re-interpolating the same variable, per the FormatJS `prefer-pound-in-plural`
  lint rule — re-interpolating `{itemCount}` inside the branch instead of `#` is a
  correctness/consistency smell to flag.
- `selectordinal` uses the same category set for ordinal forms (1st/2nd/3rd/4th in
  English maps to `one`/`two`/`few`/`other`); do not conflate `plural` and
  `selectordinal` — ordinal and cardinal plural rules differ per locale.
- `select` (non-numeric) is the correct construct for gendered or enumerated branching
  (e.g., `{gender, select, female {She} male {He} other {They}}`), not further
  string-concatenation hacks.

## Non-negotiable review rules

1. **Any string containing a runtime count and user-facing text is a plural-format
   candidate.** Concatenation (`count + " item" + (count !== 1 ? "s" : "")`) or
   template-literal interpolation of a raw count into a fixed-plurality string is a
   structural defect — flag it with file:line, not as a nit.
2. **Do not accept "we only ship English" as an exemption** if the readiness review's
   stated target locale list includes any non-English locale, or if the user is asking
   about vendor-readiness generically. Ask for the target locale list if it wasn't
   given — a plural defect invisible in English (one/other) can be a shipped-breaking
   bug in Arabic or Polish.
3. **Check that the plural/selectordinal library in use actually enforces
   locale-correct category resolution** (e.g., `Intl.PluralRules(locale).select(n)`,
   or a library — FormatJS/`react-intl`, `next-intl`, `i18next` with an ICU plugin —
   that delegates to it) rather than a hand-rolled category function. A hand-rolled
   `n === 1` check anywhere in the pipeline defeats CLDR-correct pluralization even if
   the message strings themselves use ICU syntax.
4. **Verify translators, not developers, control the plural branches per locale.** If
   the plural-branch structure is hardcoded per-language in application code instead of
   in the externalized message file, translators cannot add/remove categories for their
   locale (e.g., adding `few`/`many` branches for Polish) without a code change —
   flag this as a translation-workflow blocker, not just a formatting nit.
5. **selectordinal and plural must not be conflated** even when the surface English
   text looks similar; verify the intended semantics (count vs rank) match the ICU
   argument type used.

## Minimal safe audit flow

1. Grep for user-facing string literals interpolating a numeric variable directly
   (`` `${count} item` ``, `count + ' item'`, template literals with a count and no
   plural handling).
2. For each hit, confirm whether it flows through an ICU-aware formatter
   (`FormatMessage`/`t()` with a `plural` argument, `Intl.PluralRules`) or is raw
   concatenation.
3. For every ICU `plural`/`selectordinal` message found, confirm branch coverage
   against the CLDR categories actually required for the stated target locale list —
   not just whether `one`/`other` exist.
4. Confirm the count value passed in is a plain number (or correctly extracted from a
   formatted value) — passing an already-formatted string (`"1,000"`) into a plural
   selector breaks category resolution.
5. Record findings as file:line plus the specific CLDR category gap (e.g., "Polish
   requires `few`/`many` branches; message defines only `one`/`other`").

## Adversarial checklist

Before clearing a pluralization implementation as "ready," answer these:

- What locale(s) actually need more than `one`/`other`, and does the target locale
  list include any of them?
- Is the plural-category selection delegated to `Intl.PluralRules`/an ICU library, or
  hand-rolled?
- Can a translator add a missing CLDR category for their locale without an engineering
  change?
- Are ordinal (`selectordinal`) and cardinal (`plural`) uses distinguished correctly?
- Does any message pass a pre-formatted (comma-grouped, localized) number into the
  plural selector instead of the raw numeric value?

If you cannot answer these, the pluralization audit is incomplete — say so rather than
declaring readiness.

## When to push back

Push back if the user asks to:

- "just ship one/other, we'll fix it if a translator complains" — that defers a
  structural defect into the vendor pipeline, which is exactly the expensive rework
  this skill exists to prevent.
- treat a passing English-only test suite as proof of pluralization correctness — it
  cannot exercise categories English doesn't have.
- hardcode per-locale plural logic in application code "to save time" — that blocks
  translators from being able to fix their own locale's grammar without a release.
