# Coverage Theater And Property-Based Testing

Why coverage is a floor and where property-based testing adds signal.

- Line coverage records which lines executed, not whether their outcomes were asserted, so a high percentage with weak assertions is coverage theater; branch coverage and meaningful assertions on error paths and edge cases carry the real signal.
- A fixture's scope (`function` by default, up to `session`) determines how long its state persists; a broadly-scoped fixture that mutates shared state couples tests and must be reset or narrowed to keep tests independent.
- Property-based testing (hypothesis) generates many inputs against an invariant and shrinks failing cases to a minimal example, finding edge cases that a handful of hand-written examples miss.

## Sources

- https://docs.pytest.org/en/stable/how-to/fixtures.html
- https://hypothesis.readthedocs.io/en/latest/
