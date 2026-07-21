# Official Sources

Primary OWASP MASVS/MASTG and Android security documentation.

Primary sources, verified 2026-07-21 against official documentation and cross-checked via the Context7 MCP where a version-sensitive claim was encoded:

- https://mas.owasp.org/MASVS/
- https://developer.android.com/privacy-and-security/security-tips
- https://developer.android.com/training/articles/security-config
- https://developer.android.com/guide/topics/manifest/manifest-intro

## Grounding rule

Documentation explains language, framework, and platform behaviour in general. It does not prove the version, target, build configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the build files or source confirm it.
