# Official Sources

Primary Gradle and Kotlin build-engineering documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://docs.gradle.org/current/userguide/configuration_cache.html
- https://docs.gradle.org/current/userguide/build_cache.html
- https://kotlinlang.org/docs/ksp-overview.html
- https://docs.gradle.org/current/userguide/sharing_build_logic_between_subprojects.html

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
