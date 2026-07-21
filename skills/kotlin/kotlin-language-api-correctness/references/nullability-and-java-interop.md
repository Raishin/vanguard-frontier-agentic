# Nullability And Java Interop

How Kotlin's null-safety interacts with Java platform types.

- A Java-sourced value with no nullability annotation is exposed to Kotlin as a platform type (`T!`), which suppresses compile-time null-safety and can NPE at the call site the same way Java would.
- Kotlin recognizes `@Nullable`/`@NotNull` (JSR-305, `org.jetbrains.annotations`, Android `androidx.annotation`) on a Java signature and maps them to `T?`/`T` respectively, restoring compile-time null-checking for that signature.
- The `!!` operator converts any nullable expression to non-null unconditionally and throws immediately if the value is null — it is not a substitute for a null check, only a deferred assertion.

## Sources

- https://kotlinlang.org/docs/null-safety.html
- https://kotlinlang.org/docs/java-interop.html
