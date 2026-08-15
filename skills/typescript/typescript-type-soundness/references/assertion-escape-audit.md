# Assertion And Escape-Hatch Audit

How to classify an escape hatch in published code as justified or laundering, and the artifact-scope split.

- An escape hatch (`as`, `any`, a non-null assertion `!`, `@ts-ignore`/`@ts-expect-error`) in shared or published code is justified only when the alternative is provably impossible to express in the type system and the escape is scoped as narrowly as possible — anything broader is laundering, not justification.
- `as any` used to silence a compiler error at a boundary the code itself owns (not a genuinely external, unmodelable value) is laundering: the fix is to model the value correctly, not to assert past the type checker.
- The artifact-scope split governs which agent reviews an escape hatch: an escape inside a frontend application diff belongs to `typescript-contracts-agent`; an escape inside a shared, published, or service-side type model belongs to this agent.
- A non-null assertion (`!`) on a value whose absence is only provable by an external invariant (a database constraint, an upstream contract) not visible in the reviewed source is an unverifiable assumption and must be labelled as such, not treated as safe.
- `@ts-expect-error` is the only TypeScript-team-documented compile-time assertion of an expected error, and it self-flags when no error occurs — flag any `@ts-ignore` used where `@ts-expect-error` would self-detect staleness instead.
- An escape hatch carrying a comment explaining why it is temporary, with no tracked removal path, is a laundering signal regardless of how old or well-commented it is.
