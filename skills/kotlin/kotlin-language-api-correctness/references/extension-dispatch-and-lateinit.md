# Extension Dispatch And Lateinit Hazards

Static extension resolution, member precedence, and lateinit use-before-init.

- Kotlin resolves an extension function call statically, using the declared (compile-time) type of the receiver expression, never its actual runtime type — an extension does not participate in the caller's polymorphism the way a member override does.
- When a class defines both a member function and an extension function with the same signature, the member function always takes precedence at every call site, regardless of import order or where the extension is declared.
- Accessing a `lateinit var` before it has been assigned throws `UninitializedPropertyAccessException`; `::property.isInitialized` performs a safe check, and `lateinit` cannot be applied to a property of a primitive type or a nullable type.

## Sources

- https://kotlinlang.org/docs/extensions.html
- https://kotlinlang.org/docs/properties.html#late-initialized-properties-and-variables
