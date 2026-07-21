# Inline Functions, Reified Generics, And Value Classes

Why reified type parameters require inline, and when a value class boxes.

- A type parameter marked `reified` is retained at runtime and usable with `is`/`as`/`::class`, but the Kotlin compiler permits `reified` only on a type parameter of an `inline` function, because the compiler substitutes the real type at each inlined call site.
- An inline function's `noinline` parameter opts a lambda out of inlining (so it can be stored or passed on), while a `crossinline` parameter forbids non-local returns from that lambda, since its body is inlined into a different execution context.
- An `@JvmInline value class` is represented as its unboxed underlying value at a directly-typed, non-nullable call site, but the JVM requires boxing whenever the value is used as a generic type argument, assigned to an interface type, or held as a nullable `T?`.

## Sources

- https://kotlinlang.org/docs/inline-functions.html
- https://kotlinlang.org/docs/inline-classes.html
