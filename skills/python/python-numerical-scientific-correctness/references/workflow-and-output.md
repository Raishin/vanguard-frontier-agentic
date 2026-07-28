# Review Workflow And Output Contract

The numerical-correctness review workflow and the required output shape.

## Workflow

1. Identify the numeric stack and the money/measurement paths: what is computed, in what type, and reported where.
2. Check every monetary calculation uses Decimal (from strings) or integer minor units with an explicit rounding mode.
3. Check dtypes and missing-data handling for silent integer→float upcasts, NaN comparisons, and integer overflow.
4. Check every timestamp is timezone-aware and stored in UTC, with no naive/aware mixing or DST-unsafe arithmetic.
5. Check randomness is seeded and recorded, arithmetic is numerically stable, and record every value/performance claim that needs execution or benchmark evidence.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the numeric stack assumed.
- Money/rounding, dtype/missing-data, timezone, and reproducibility/stability findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any computed-value or performance claim the user must confirm by execution/benchmark.
