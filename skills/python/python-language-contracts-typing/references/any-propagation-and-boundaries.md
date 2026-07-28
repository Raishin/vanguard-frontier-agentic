# Any Propagation And Public Boundaries

How Any erases safety and where runtime validation must sit.

- `Any` is compatible with every type in both directions, so a checker performs no verification on a value typed `Any`; the effect propagates to everything derived from it.
- An untyped third-party import or a function with no return annotation introduces implicit `Any`, which is why a strict configuration flags untyped defs and disallows implicit `Any` at boundaries.
- Static types are erased at runtime (PEP 484 gradual typing): the annotation is not enforced when the program runs, so untrusted input at a boundary requires explicit runtime validation in addition to the type.

## Sources

- https://docs.python.org/3/library/typing.html
- https://mypy.readthedocs.io/en/stable/dynamic_typing.html
