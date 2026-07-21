# Dependency Verification And Locking

How verification metadata and dependency locking establish reproducible, trustworthy resolution.

- Gradle dependency verification (`gradle/verification-metadata.xml`) checks checksums, and optionally PGP signatures, of every dependency on each resolution, and can run in strict mode so an unverified or mismatched artifact fails the build rather than silently resolving.
- Dependency locking records the exact resolved version of every dependency (including transitives) in a lock file so the same graph resolves in every subsequent build, and CI can be configured to fail when resolution would produce a different graph than the locked one.
- A verification-metadata entry can exempt a specific artifact from verification (`trusted-artifacts`); a broadly-scoped exemption (wildcard group or module) defeats the purpose of the file and should be scoped as narrowly as possible.

## Sources

- https://docs.gradle.org/current/userguide/dependency_verification.html
- https://docs.gradle.org/current/userguide/dependency_locking.html
