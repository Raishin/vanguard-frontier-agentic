# Scope Boundaries And Reversibility

The technical boundaries that bound how much scope can realistically be shared, and what they cost to reverse.

- expect/actual declarations bind a common declaration to a per-platform implementation under the same package name, and the compiler enforces that every expected declaration has a matching actual for each target — this is a hard boundary on what can live in commonMain.
- commonMain cannot call a platform-specific API, such as java.io.File on JVM/Android; any such usage forces the code out of commonMain into a platform source set, so a shared-code proposal that assumes otherwise will not compile as designed.
- A dependency declared in a common source set automatically propagates down to every platform source set that depends on it, so adding a dependency to commonMain must be checked against every target platform, not just the one currently being developed.
- Pure Swift is not directly consumable from Kotlin/Native; interop with iOS-side Swift code goes through an Objective-C bridge, which is a real cost the portfolio decision must account for rather than assume away.

## Sources

- https://kotlinlang.org/docs/multiplatform-expect-actual.html
- https://kotlinlang.org/docs/multiplatform-connect-to-apis.html
