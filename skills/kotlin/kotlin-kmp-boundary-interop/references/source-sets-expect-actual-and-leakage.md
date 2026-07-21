# Source Sets, Expect/Actual, And Platform-API Leakage

How the source-set hierarchy, expect/actual pairing, and platform-API boundaries are enforced.

- applyDefaultHierarchyTemplate() wires up the standard Kotlin Multiplatform source-set graph, including the intermediate and platform-specific sets implied by the configured targets; a custom hierarchy that bypasses it needs its own stated justification.
- Every expect declaration must have a matching actual declaration, in the same package, for each target the project configures — the compiler enforces this pairing, and an incomplete pairing is a build-blocking defect, not a style note.
- commonMain code cannot reference a platform-specific API; any such reference either fails to compile or indicates the code needs to move behind an expect/actual boundary or into the correct platform source set.
- A dependency declared in a common source set is automatically inherited by every platform source set beneath it in the hierarchy, so its compatibility must be checked against every target it now reaches, not only the target it was written for.

## Sources

- https://kotlinlang.org/docs/multiplatform-expect-actual.html
- https://www.jetbrains.com/help/kotlin-multiplatform-dev/multiplatform-hierarchy.html
