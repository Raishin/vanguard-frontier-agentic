# Official Sources

Primary Kotlin Multiplatform and Kotlin/Native interop documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://kotlinlang.org/docs/native-memory-manager.html
- https://kotlinlang.org/docs/multiplatform-expect-actual.html
- https://kotlinlang.org/docs/native-objc-interop.html
- https://www.jetbrains.com/help/kotlin-multiplatform-dev/multiplatform-hierarchy.html

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
