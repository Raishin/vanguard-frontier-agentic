# JVM-Facing Surface Annotations

How @JvmOverloads/@JvmStatic/@JvmName shape the Java-visible ABI.

- `@JvmOverloads` generates one Java-callable overload for each parameter that has a default value, each overload dropping trailing defaulted parameters; adding a new defaulted parameter anywhere but last, or reordering/removing an existing one, changes the generated overloads' signatures and can produce a runtime error for Java callers compiled against the old set.
- `@JvmStatic` on a member of a Kotlin `object` or a class's `companion object` generates a genuine static method for Java callers in addition to the instance method on the singleton; removing the annotation removes that static entry point from the Java-visible surface.
- `@JvmName` changes the name a declaration compiles to under the JVM, most often to avoid a platform signature clash (for example a property getter colliding with a same-named function); changing or removing a `@JvmName` value is a rename of the Java-visible member and breaks existing Java source and binary callers.

## Sources

- https://kotlinlang.org/docs/java-to-kotlin-interop.html
- https://kotlinlang.org/docs/whatsnew1420.html
