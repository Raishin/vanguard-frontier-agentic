# Sink Hardening Catalog

> Static review only. Anchor findings to OWASP deserialization / XXE guidance (see the skill's `official_docs`), not to a single library version. A finding is CRITICAL only when the parsed/deserialized data can be **attacker-controllable**; state the trust boundary you assumed and mark it `inference` when the source does not prove it. A "this version is patched" claim belongs to the supply-chain agent, not here.

## 1. Java native serialization (`ObjectInputStream`)

**Dangerous:** `new ObjectInputStream(in).readObject()` (or `readUnshared`, RMI, JMX, or any native deserialization) where `in` carries attacker-controllable bytes and there is no `ObjectInputFilter`. Gadget chains in libraries on the classpath turn this into remote code execution regardless of your own classes.

**Safe:**
- Prefer not to use native serialization for untrusted data at all — use a data format (JSON/protobuf) with explicit types.
- If unavoidable, install a strict **allow-list** `ObjectInputFilter` (JEP 290): `ObjectInputFilter.Config` / `setObjectInputFilter` permitting only the exact expected classes and rejecting everything else, with depth/array/ref limits.

**Do not accept as a fix:** a **blocklist** of known-bad gadget classes (bypassable as new chains appear), or catching the resulting exception.

## 2. SnakeYAML

**Dangerous:** `new Yaml()` or `new Yaml(new Constructor(SomeType.class))` (the bare `Constructor`) parsing untrusted YAML. It can instantiate arbitrary types via `!!` tags — the class-of CVE-2022-1471 RCE.

**Safe:** `new Yaml(new SafeConstructor(new LoaderOptions()))`, or SnakeYAML 2.x with `LoaderOptions` plus an explicit tag inspector limiting allowed tags; for typed binding, a `Constructor` restricted to an allow-list of expected types.

**Do not accept as a fix:** a SnakeYAML version bump **alone** without `SafeConstructor`/tag restriction — the safe API must be used, not just present.

## 3. Jackson polymorphic deserialization

**Dangerous:** `enableDefaultTyping()` / `activateDefaultTyping(...)` on an `ObjectMapper`, or `@JsonTypeInfo(use = Id.CLASS)` / `Id.MINIMAL_CLASS`, on data that can be attacker-controllable and without a restrictive `PolymorphicTypeValidator`. Attackers supply a type id that instantiates a gadget.

**Safe:** avoid default typing entirely where possible; where polymorphism is required, register a **whitelist** `PolymorphicTypeValidator` (`BasicPolymorphicTypeValidator.builder().allowIfSubType(...)`) permitting only the known base types, or use explicit `@JsonSubTypes` with logical names (not class names).

**Do not accept as a fix:** relying on `jackson-databind` being "recent enough" — default typing without a validator is unsafe by design, not merely by CVE.

## 4. XML external entities (XXE)

**Dangerous:** any of `DocumentBuilderFactory`, `SAXParserFactory`, `XMLInputFactory` (StAX), `TransformerFactory`, `SchemaFactory`, `Validator`, or JAXB `Unmarshaller` created with defaults and fed untrusted XML. Enables file read, SSRF, and DoS.

**Safe (per factory):**
- Best: `dbf.setFeature("http://apache.org/xml/features/disallow-doctype-decl", true)` — rejects DTDs outright.
- Otherwise disable external entities and DTD loading: `external-general-entities` = false, `external-parameter-entities` = false, `http://apache.org/xml/features/nonvalidating/load-external-dtd` = false, and set `XMLConstants.FEATURE_SECURE_PROCESSING` = true.
- StAX: `XMLInputFactory.setProperty(XMLInputFactory.SUPPORT_DTD, false)` and `IS_SUPPORTING_EXTERNAL_ENTITIES, false`.
- `TransformerFactory`/`SchemaFactory`: set `ACCESS_EXTERNAL_DTD` and `ACCESS_EXTERNAL_STYLESHEET`/`ACCESS_EXTERNAL_SCHEMA` to the empty string.

**Do not accept as a fix:** hardening one factory while another parser in the same path is left default.

## 5. Other reflective / expression sinks

- **Reflection from input:** `Class.forName(userValue)` / constructor invocation on an attacker-supplied class name → allow-list the permissible types.
- **XStream:** without a type-permission allow-list (`XStream.setupDefaultSecurity` / explicit `allowTypes`), treat as native-serialization-class RCE.
- **Expression languages:** SpEL (`SpelExpressionParser`), OGNL, MVEL evaluated over user input → do not evaluate untrusted expressions; use a non-evaluating parser or a strict context.
- **`java.beans.XMLDecoder`** on untrusted XML → treat as RCE; replace with a safe format.

## 6. Denial of service

Even a parser hardened against RCE can be a DoS vector: no entity-expansion limit (billion laughs), no input-size or nesting bound. Flag missing limits on any deserializer/parser reading untrusted input as HIGH.

## Trust-boundary rule (applies to every sink above)

CRITICAL requires attacker-controllability: request body, message payload (Kafka/JMS), uploaded file, external API response, or any value crossing a trust boundary. If the source proves the data is internal/trusted, downgrade to a hardening recommendation and record the assumption. If the boundary is unclear, mark the finding `inference` and ask the user to confirm the source of the data.
