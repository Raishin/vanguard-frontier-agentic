# Protocols, Generics, And Variance

Structural typing, generic variance soundness, and overloads.

- A `Protocol` defines a structural type: any object with the required members conforms, without an explicit base class; `@runtime_checkable` enables `isinstance` but checks only member presence, not signatures or types.
- Variance governs subtyping of generics: an immutable producer can be covariant, a consumer contravariant, but a mutable container must be invariant because it is both read and written — a covariant mutable container is unsound.
- `@overload` declares multiple call signatures for one implementation; the signatures must not overlap with conflicting returns, and the single implementation must be type-compatible with every declared overload.

## Sources

- https://typing.readthedocs.io/en/latest/spec/protocol.html
- https://peps.python.org/pep-0484/
