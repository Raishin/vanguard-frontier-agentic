# Official Sources

Primary documentation the router relies on to distinguish Kotlin domains.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://kotlinlang.org/docs/home.html
- https://developer.android.com/kotlin
- https://kotlinlang.org/docs/multiplatform.html

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
