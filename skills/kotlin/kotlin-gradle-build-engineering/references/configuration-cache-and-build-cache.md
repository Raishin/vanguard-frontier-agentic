# Configuration Cache And Build Cache

How execution-time project access breaks the configuration cache, and what the build cache requires.

- The Gradle configuration cache serializes the task graph after the configuration phase and, on a cache hit, skips configuration entirely on the next build — but a task that reads `Project`, `Task.project`, or other live build-model state at execution time breaks that serialization and invalidates the cache.
- Gradle's documented fix is to capture configuration-time inputs via the `Provider`/`Property` lazy-configuration APIs and pass them into the task, rather than resolving them from `project` inside a task action.
- The build cache requires a task be declared `@CacheableTask` with complete `@Input`/`@InputFiles`/`@OutputDirectory`/`@OutputFile` annotations on every property affecting output; missing annotations cause either a false cache hit (stale output) or a permanent cache miss.
- Cached task output must be relocatable and reproducible — free of absolute paths, timestamps, or machine-specific values — to be shared safely across machines or a remote build-cache node.

## Sources

- https://docs.gradle.org/current/userguide/configuration_cache.html
- https://docs.gradle.org/current/userguide/build_cache.html
