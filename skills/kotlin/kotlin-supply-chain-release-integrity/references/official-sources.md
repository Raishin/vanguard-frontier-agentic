# Official Sources

Primary Gradle dependency-trust and Kotlin publication documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://docs.gradle.org/current/userguide/dependency_verification.html
- https://docs.gradle.org/current/userguide/dependency_locking.html
- https://docs.gradle.org/current/userguide/plugins.html
- https://kotlinlang.org/docs/multiplatform-publish-lib.html

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
