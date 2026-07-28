# Reproducibility And Numerical Stability

Seeded randomness and numerically stable summation/aggregation.

- numpy's current API creates a seeded bit generator and `Generator` (e.g. `default_rng(seed)`); results are reproducible only when the seed is fixed and recorded, and reproducibility across machines still depends on library versions and platform.
- The legacy `numpy.random` global functions share hidden global state and are discouraged for reproducible work in favor of an explicit `Generator` instance.
- Summing many floats in naive order loses precision through rounding; `math.fsum` computes an exact running sum, and pairwise/Kahan summation reduces error for large arrays — a stable algorithm should be chosen where precision matters.

## Sources

- https://numpy.org/doc/stable/reference/random/generator.html
- https://docs.python.org/3/library/math.html#math.fsum
