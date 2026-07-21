# Native Runtime And Swift/Objective-C Interop

Kotlin/Native memory-manager currency and the annotations that bridge Kotlin to Swift.

- Kotlin/Native's new memory manager has been the default since Kotlin 1.7.20 and the legacy memory manager was fully removed in 1.9.20; calling freeze() or relying on @FreezingIsDeprecated-flagged behavior reflects the old model, and freeze() is a hard compile error starting Kotlin 2.1.0.
- A Kotlin suspend function exported to Swift/Objective-C appears as an async function or a completion-handler-based method depending on the Kotlin/Native export configuration in use, and the exported shape should be confirmed against that configuration rather than assumed.
- @Throws(Exception::class), or a more specific exception type, declares which exceptions a Kotlin function can throw so Swift sees them as a catchable NSError; without it, a thrown Kotlin exception is not bridged and crashes the Swift caller instead.
- @ObjCName is an experimental annotation requiring explicit opt-in that customizes how a Kotlin declaration's name is mangled for Objective-C/Swift consumers; using it without the opt-in, or depending on it in a stable public API, should be flagged given its experimental status.

## Sources

- https://kotlinlang.org/docs/native-memory-manager.html
- https://kotlinlang.org/docs/native-objc-interop.html
