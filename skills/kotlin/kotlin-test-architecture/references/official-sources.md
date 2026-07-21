# Official Sources

Primary kotlinx-coroutines-test, Turbine, and Compose/Android testing documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-test/
- https://github.com/cashapp/turbine
- https://developer.android.com/develop/ui/compose/testing
- https://developer.android.com/training/testing/local-tests

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
