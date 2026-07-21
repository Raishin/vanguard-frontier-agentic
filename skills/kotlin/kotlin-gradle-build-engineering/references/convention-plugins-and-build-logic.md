# Convention Plugins And The Build-Logic Included Build

How shared build configuration should be centralized rather than duplicated.

- Gradle's documented pattern for sharing build logic across subprojects is a convention plugin authored as a precompiled script plugin inside a `build-logic` (or `buildSrc`) included build, applied by subprojects instead of duplicating repository/plugin/dependency configuration.
- Centralizing shared configuration in an included build lets Gradle compile and cache the build logic itself, avoiding the classpath and version-skew hazards of copy-pasted build-script blocks.

## Sources

- https://docs.gradle.org/current/userguide/sharing_build_logic_between_subprojects.html
