# Intl Formatting Audit

Use this reference only when reviewing date, number, currency, or list formatting for
manual-string-building defects versus correct `Intl` API usage.

> Version note: `Intl` constructor options are ECMA-402-spec-version-sensitive and
> browser/runtime-support-sensitive. Verify option names (e.g., `Intl.NumberFormat`
> `style`/`currencyDisplay`/`notation`) against the current ECMA-402 spec or MDN before
> asserting a given option is safe to use, and check the project's minimum supported
> runtime for coverage gaps.

## What people get wrong

The common bad assumption is:

> "Formatting a date or currency amount is just string interpolation with the right
> separators for each locale."

That undercounts the problem. Locale-correct formatting is not just
comma-vs-period-for-decimals — it includes:

1. **Digit systems** — not every locale uses Western Arabic numerals (0-9); some use
   native digit systems, which `Intl.NumberFormat` handles automatically when correctly
   invoked and manual formatting will not.
2. **Currency symbol placement and spacing** — varies by locale independent of the
   currency itself (e.g., `$100` vs `100 $` vs `100$`); hardcoding a symbol-prefix
   pattern breaks for locales that place it differently.
3. **Date component order and calendar system** — month/day/year order is not
   universal, and some locales/config combinations use non-Gregorian calendars.
4. **Grouping separators** — thousands separators vary (`,` / `.` / space / none) and
   are not simply swappable via find-and-replace on a fixed pattern.
5. **List formatting** ("A, B, and C" vs "A, B и C") — conjunction/list-joining rules
   are locale-specific and easy to miss because they're rarely tested.

## Non-negotiable review rules

1. **Any manually-constructed date string** (`` `${month}/${day}/${year}` ``,
   string-padding a day/month) is a structural defect for any codebase with more than
   one target locale — flag it regardless of whether the current output happens to
   look correct for the developer's own locale.
2. **Any manually-formatted currency/number string** (custom thousands-separator
   insertion, hardcoded `$` prefix, `toFixed(2)` treated as "locale-safe currency
   formatting") is a defect. `toFixed(2)` produces a numeric string with no locale
   awareness at all — flag it explicitly, since it is a very common false-safe pattern.
3. **Verify the locale passed into `Intl.*Format` constructors is the actual active
   application locale**, not a hardcoded `'en-US'` left over from initial development
   or scaffolding — a codebase can use `Intl.NumberFormat` correctly and still be
   non-localized if the locale argument is hardcoded.
4. **Currency formatting must specify `currency` explicitly and correctly per
   transaction/display context** (`Intl.NumberFormat(locale, { style: 'currency',
   currency: 'USD' })`) — do not accept a numeric-only format with a manually
   concatenated currency symbol as equivalent.
5. **Check for `Intl` availability assumptions on server vs client** — if
   locale/currency data availability differs between the runtime's `Intl`
   implementation and what's expected (e.g., a minimal/embedded runtime lacking full
   CLDR data), flag it as a verification item rather than assuming full coverage.

## Minimal safe audit flow

1. Grep for manual date-building patterns (string concatenation with `/`, `-`, month
   names arrays indexed by number) and manual currency/number formatting
   (`toFixed`, manual separator insertion, hardcoded currency symbols).
2. Grep for `Intl.DateTimeFormat`, `Intl.NumberFormat`, `Intl.ListFormat`,
   `Intl.PluralRules`, `Intl.RelativeTimeFormat` usage and confirm the locale argument
   is dynamic (sourced from application locale state), not a hardcoded literal.
3. For currency displays, confirm `style: 'currency'` and an explicit `currency` code
   are present, not a generic number format with a bolted-on symbol.
4. For any library wrapping `Intl` (FormatJS, `next-intl`, `i18next`), confirm it is
   actually invoked at the display sites in scope rather than assumed present because
   it exists in `package.json`.
5. Record each finding as file:line, the specific manual-formatting pattern found, and
   which `Intl` API should replace it.

## Adversarial checklist

Before clearing formatting as "ready," answer these:

- Is the locale argument passed to every `Intl.*Format` call dynamic, or is any of them
  hardcoded to a default locale?
- Does currency formatting specify an explicit currency code per context, or is a
  single currency assumed globally?
- Are there any `toFixed()` calls being treated as currency-safe?
- Does date formatting handle non-Gregorian calendar locales if any target locale
  requires one, or is Gregorian assumed unconditionally?
- Is list-joining ("A, B, and C") formatted with `Intl.ListFormat` or hardcoded
  English conjunction rules?

If you cannot answer these, the formatting audit is incomplete — say so rather than
declaring readiness.

## When to push back

Push back if the user asks to:

- "just add a locale-to-symbol lookup table instead of `Intl.NumberFormat`" — that
  reimplements a subset of CLDR data by hand and will drift from the real locale rules
  over time.
- treat visual correctness in one locale (usually the developer's own) as proof of
  correctness for all target locales.
- skip verifying the locale argument is dynamic because "it works in dev" — dev
  environments frequently default to a single hardcoded locale, masking the defect.
