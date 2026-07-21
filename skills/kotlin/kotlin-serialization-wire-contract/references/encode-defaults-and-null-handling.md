# Encode Defaults And Null Handling

How encodeDefaults/explicitNulls/@EncodeDefault shape what actually appears on the wire.

- `Json { encodeDefaults }` defaults to `false`, so a property left at its default value is omitted from the encoded output entirely, not encoded as its default — any consumer assuming the field is always present must confirm this setting or the property's `@EncodeDefault` override.
- `explicitNulls` defaults to `true`, meaning an explicit `null` is both encoded and required on decode unless the property has a default value; disabling it changes an absent field's interpretation from a decode failure to an implicit null/default.
- `@EncodeDefault(EncodeDefault.Mode.ALWAYS)` or `.NEVER` on an individual property overrides the class/format-level `encodeDefaults` setting for that property alone, and is the correct mechanism when only some fields of a class must always appear on the wire.

## Sources

- https://github.com/Kotlin/kotlinx.serialization/blob/master/docs/json.md
- https://kotlinlang.org/api/kotlinx.serialization/
