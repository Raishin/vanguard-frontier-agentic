# Migration Sequencing And Reversibility

How strangler-fig ordering, dependency-graph respect, and step reversibility determine a safe migration plan.

- Android's official guidance recommends adding Kotlin to an existing Java app incrementally, module by module, rather than a single rewrite, so the app keeps shipping while migration proceeds.
- A migration step should preserve the ability to revert: prefer small, independently revertible commits/PRs per module over a single broad rewrite that spans unrelated modules.
- Kotlin is documented as designed for full interoperability with Java specifically so migration can proceed incrementally — interop, not an all-at-once rewrite, is the stated compatibility goal.
- A stable, low-churn Java module carries migration risk (interop seams, retraining, review cost) without a corresponding benefit when no upcoming work touches it; migration priority should track planned churn, not blanket conversion.

## Sources

- https://developer.android.com/kotlin/add-kotlin
- https://kotlinlang.org/docs/comparison-to-java.html
