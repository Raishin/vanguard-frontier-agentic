# Decode Strictness And Schema Evolution

Why ignoreUnknownKeys and default-value semantics determine whether a schema change is breaking.

- `ignoreUnknownKeys` defaults to `false`; a decoder that receives a key not declared in its schema throws a `SerializationException` rather than silently ignoring it, so a consumer that must tolerate a producer's newer schema needs to opt in explicitly.
- A property with a default value is treated as optional on decode — its absence from the payload is not an error — so removing a default from an existing property, or adding a new required non-default property, changes a previously-optional or previously-absent field into a hard decode requirement and is a breaking change for any party not upgraded in lockstep.
- A property rename on a type already on the wire changes the serialized field key unless `@SerialName` is used to preserve the original key, and a producer adding a new enum constant can break a consumer whose enum decode path is not hardened against unknown values.

## Sources

- https://github.com/Kotlin/kotlinx.serialization/blob/master/docs/json.md
- https://kotlinlang.org/api/kotlinx.serialization/
