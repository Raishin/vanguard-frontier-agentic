# Official Sources

Primary Kotlin language and Java-interop documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://kotlinlang.org/docs/null-safety.html
- https://kotlinlang.org/docs/java-interop.html
- https://kotlinlang.org/docs/inline-functions.html
- https://kotlinlang.org/docs/inline-classes.html

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
