# Official Sources

Primary npm publication-trust and package-inspection documentation.

Primary sources, verified 2026-08-13 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://docs.npmjs.com/generating-provenance-statements
- https://publint.dev/rules
- https://arethetypeswrong.github.io

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
