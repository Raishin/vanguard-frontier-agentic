# Official Sources

Primary Kotlin/Java interop and migration documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://kotlinlang.org/docs/mixing-java-kotlin-intellij.html
- https://kotlinlang.org/docs/java-interop.html
- https://kotlinlang.org/docs/comparison-to-java.html
- https://developer.android.com/kotlin/add-kotlin

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
