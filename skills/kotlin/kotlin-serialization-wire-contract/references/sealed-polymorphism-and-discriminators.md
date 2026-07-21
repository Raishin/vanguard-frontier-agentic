# Sealed Polymorphism And Class Discriminators

Why closed polymorphism matters for untrusted input, and how discriminators are configured.

- A `sealed` class or interface hierarchy gives kotlinx.serialization a closed, fully enumerable set of subtypes at compile time, serialized with a class discriminator key (default `"type"`) whose value selects the concrete subtype on decode.
- The discriminator key is configurable per-`Json` instance via `classDiscriminator` or per-hierarchy via `@JsonClassDiscriminator`; a producer and consumer that disagree on the discriminator key or its registered values fail to decode correctly, and the failure surfaces at the point of type resolution rather than at compile time.
- Deserializing a polymorphic hierarchy that is `open`/`abstract` rather than `sealed` from untrusted or external input is a safety risk: the set of resolvable subtypes is not closed, so the reviewer cannot fully enumerate what the decoder is willing to instantiate from attacker-controlled input.

## Sources

- https://github.com/Kotlin/kotlinx.serialization/blob/master/docs/polymorphism.md
- https://kotlinlang.org/api/kotlinx.serialization/
