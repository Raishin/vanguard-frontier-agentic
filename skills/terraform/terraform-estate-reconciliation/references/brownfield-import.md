# Brownfield Adoption And Import

How to bring unmanaged infrastructure under management without a destroy, and the gate that proves it worked.

- An `import` block is declarative and appears in a plan before it touches state, which makes it reviewable in a pull request and repeatable in a pipeline — unlike the imperative import command, which mutates state directly with no preview and no review trail.
- The `identity` argument addresses a remote object by a set of attributes and is the modern form, while the legacy `id` argument takes a single provider-assigned string; which one a resource accepts is a property of the resource type and must come from the provider's own documentation.
- The verification gate for any import is a no-op plan afterwards. A plan that still proposes changes means the configuration does not describe the object as it actually exists, and applying it modifies infrastructure that was working before the adoption began.
- `-generate-config-out` writes configuration derived from the object's current attributes; it does not produce module structure, variables, naming conventions, or policy defaults, so its output is a starting point for authoring rather than an artifact to commit.
- On OpenTofu, configuration generation is marked experimental and cannot currently be combined with `for_each` on import blocks, so a bulk adoption strategy that relies on both at once is available on one engine and not the other.
- Importing resources in dependency order — those with no dependents first — keeps each no-op check attributable; a single bulk import whose plan is not a no-op gives no way to tell which of the imported resources caused the difference.
- An import writes to state and therefore carries the same preconditions as any other state mutation: a verified restorable copy, and a lock held for the duration.
