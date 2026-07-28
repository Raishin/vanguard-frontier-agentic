# Numerical Correctness Review Checklist

The per-concern checklist applied to every numerical review.

- Money: every monetary value uses `Decimal` (from strings) or integer minor units with an explicit rounding mode — never binary `float`.
- Rounding: the rounding rule (half-up, half-even, …) is explicit via `Decimal.quantize`, not left to the default.
- Dtypes: no silent integer→float upcast on missing data; no unchecked fixed-width integer overflow.
- Missing data: `NaN` is handled explicitly with `isna`/`notna` and an explicit `skipna` choice.
- Timezones: every timestamp is timezone-aware, stored in UTC; no naive/aware mixing.
- Reproducibility: randomness is seeded via an explicit `Generator` and the seed and library versions are recorded.
