# Data Class And Inline-Function ABI Surface

Why data class shape and inline-function bodies are part of the public ABI, not internal detail.

- A public data class's `copy()` function and `componentN()` destructuring functions are generated from the primary constructor's parameter list and order; adding, removing, or reordering a property changes `componentN` numbering and the `copy()` signature, which is a binary-compatibility event for any consumer compiled against the old shape.
- Because an inline function's body is copied into the caller's compiled bytecode at the call site rather than invoked, changing a public inline function's body does not immediately change behavior for already-compiled callers — they keep executing the old inlined logic until they are recompiled against the new library version.
- A default parameter value on a public Kotlin function is resolved by the Kotlin compiler at the call site, not encoded as a runtime default in the JVM method signature; a Java caller must supply every parameter explicitly (or call the `@JvmOverloads`-generated overload), so changing a default's value does not by itself change the Java-visible ABI shape.

## Sources

- https://kotlinlang.org/docs/java-to-kotlin-interop.html
- https://kotlinlang.org/docs/functions.html#default-arguments
