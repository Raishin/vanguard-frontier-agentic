# Soundness Failure Catalog

Each construct that can lie to the compiler, and the check that detects it.

- A type predicate function (`x is T`) is trusted by the compiler at every call site regardless of whether its body actually checks every field `T` requires — the check is comparing the predicate's runtime condition against every property the narrowed type adds.
- A generic type parameter that appears in both an input and an output position is checked bivariantly by default under method syntax, which can accept a supertype where a subtype was required — the check is whether the parameter is declared with function-property syntax (correctly variant) or method syntax (bivariantly checked).
- `satisfies` validates a literal against a type without changing its inferred type, so a later reassignment or spread can silently widen back to the literal's own inferred shape — the check is confirming the call site actually needs the narrower literal type `satisfies` preserves, not the wider annotated type.
- A conditional type can contain a branch that no type substitutable for its input parameter can ever select, which passes the compiler while proving nothing about that branch — the check is substituting the type parameter's real constraint and confirming every branch is reachable.
- A branded or nominal type is only as strong as its constructor function; if the brand can be attached by a bare object literal, a type assertion, or a spread of an already-branded value, the type gives no runtime guarantee — the check is confirming the branded type has no public construction path except the validating one.
- A mapped type that re-maps keys with `as` can silently drop a key whose remapped expression evaluates to `never`, leaving the resulting type with fewer properties than the source with no explicit signal in the type's shape — the check is comparing the mapped type's declared keys against the intended key set.
- Deep generic nesting or heavy conditional-type recursion is not evidence of correctness, and a type that reads simply is not automatically sound — the check is always what the type proves at its use sites, never its apparent sophistication.
