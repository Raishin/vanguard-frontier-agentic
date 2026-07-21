# Interop Boundary And Converter Governance

Platform-type null-safety debt at the interop boundary, and why J2K converter output needs review.

- Kotlin's Java-interop documentation states that a Java reference typed as a general Java type crossing into Kotlin becomes a 'platform type' (notated `T!`) for which the compiler cannot verify nullability — the boundary must be annotated or wrapped, or a runtime NPE can surface deep inside otherwise null-safe Kotlin code.
- Kotlin recognizes Java nullability annotations (e.g. JSR-305 `@Nullable`/`@Nonnull`) when present, resolving the type to nullable/non-null instead of a platform type — annotating the Java side at the boundary is the documented way to close the gap.
- IntelliJ's Java-to-Kotlin converter performs an automatic syntactic conversion but is documented as producing code that may need manual correction — its output is a starting point, not a validated result, and inferred nullability is exactly the kind of judgment that needs review before merge.

## Sources

- https://kotlinlang.org/docs/java-interop.html
- https://kotlinlang.org/docs/mixing-java-kotlin-intellij.html
