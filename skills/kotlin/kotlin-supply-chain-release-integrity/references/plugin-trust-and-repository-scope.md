# Plugin Trust And Repository Scope

Why plugins must be pinned and trusted, and how repository scope prevents dependency confusion.

- A Gradle plugin executes as part of the build itself and therefore runs with the same trust as build-script code — an unpinned (dynamic/range) plugin version or a plugin sourced from an unvetted repository can introduce arbitrary code into every build that applies it.
- Gradle documents repository content filtering (restricting which repository is consulted for which group/module) as the mechanism to prevent an internal coordinate from being inadvertently resolved against a public repository — the classic dependency-confusion vector.

## Sources

- https://docs.gradle.org/current/userguide/plugins.html
