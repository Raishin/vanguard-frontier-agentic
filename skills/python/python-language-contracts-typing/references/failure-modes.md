# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- An `Any` returned from a core helper erases type checking across the whole call graph, so a wrong-typed value reaches production unflagged.
- A route annotated with a model but not validated at runtime accepts a malformed payload that the annotation implied was impossible.
- A covariant mutable container allows an unsound assignment that corrupts shared state.
- A mutable default argument accumulates state across requests and leaks data between callers.
- A silently widened return type (now `| None`) breaks every typed consumer that did not expect `None`.
