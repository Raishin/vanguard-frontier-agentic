# Official Sources

Primary Python, pandas, and numpy documentation for numerical correctness.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/tutorial/floatingpoint.html
- https://docs.python.org/3/library/decimal.html
- https://pandas.pydata.org/docs/user_guide/timeseries.html#time-zone-handling
- https://numpy.org/doc/stable/reference/random/generator.html

## Provenance notes

- docs.python.org (floating-point tutorial, decimal, math), pandas.pydata.org (time-series/time-zone handling), and numpy.org (random Generator, dtypes) are the authoritative upstreams for the claims in this skill.
- Context7 MCP was not used as a separate source for this skill: the cited semantics (IEEE-754 float representation, Decimal construction/quantize, tz-aware vs naive handling, seeded Generator) are stable across current releases and are quoted directly from the official upstream documentation, which the repository treats as authoritative. Any claim tied to a specific pandas/numpy version must still be confirmed against the user's pinned versions.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
