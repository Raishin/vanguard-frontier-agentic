# Trust-Boundary Validation Patterns

Use this reference when auditing external-data ingestion points — API responses, `JSON.parse`, `postMessage`, URL/query-param parsing, form input, environment variables — for whether a declared TypeScript type is actually backed by a runtime check.

## What people get wrong

The naive story is:

> I typed the response as `User`, so `response.data` is a `User`.

Wrong, and dangerously so. TypeScript types are fully erased at compile time — the emitted JavaScript contains no trace of the type annotation. A cast like `const user = response.data as User` or a generic call like `fetch<User>(url)` where the generic only annotates the return type performs **zero runtime work**. If the actual payload is malformed, missing a field, or actively malicious (an attacker-controlled API, a compromised third-party SDK, a crafted `postMessage`), the code proceeds as if it received a valid `User` and every downstream consumer inherits that false confidence.

This is not a hypothetical edge case — it is the default behavior of `JSON.parse`, which is typed to return `any` specifically because the compiler cannot know the shape of arbitrary parsed JSON. Any annotation applied after that point (`JSON.parse(raw) as Shape`, or assigning the `any` result to a `Shape`-typed variable) is the developer asserting a guarantee the compiler cannot verify and runtime does not enforce.

## Officially grounded shape

- `JSON.parse` returns `any`. Assigning that `any` to a typed variable, or asserting it with `as`, produces no runtime check — `typescript-eslint`'s `no-unsafe-assignment`, `no-unsafe-member-access`, and `no-unsafe-return` rules specifically exist to flag this exact pattern (an `any` from `JSON.parse` flowing into a typed context) because it is common enough to warrant dedicated tooling.
- `no-explicit-any` and the `no-unsafe-*` family are part of `recommended-type-checked` / `strict-type-checked` shared configs — if the diff introduces this pattern in a repo that has those configs active, it should already surface as a lint failure; if it does not surface, either the config isn't active on that file or the pattern is disguised (e.g. via an intermediate `unknown` cast chained through `as`).
- A type assertion (`as T`) does not perform a structural check; it only suppresses the compiler's own inference. `T!` (non-null assertion) similarly performs no runtime check — it only silences the compiler's null/undefined narrowing.
- The compiler-sound way to narrow an `unknown` or `any` value to a specific type is a **type guard function** (`function isUser(x: unknown): x is User`) that performs actual runtime property/shape checks, or a schema-validation library (e.g. Zod, io-ts, ArkType — verify current API via `query-docs`/official docs for whichever the repo already uses; do not introduce a new one without the user's decision) whose `.parse()`/`.safeParse()` throws or returns a discriminated result on shape mismatch.

## Non-negotiable design rules

1. **Every trust boundary needs a paired runtime check, not just a type annotation.** Trust boundaries include: HTTP/fetch response bodies, WebSocket messages, `postMessage` payloads, URL/query-string parsing, `localStorage`/`sessionStorage` reads, environment variables consumed at runtime, file uploads, and any third-party SDK callback whose payload originates outside this codebase's own type-checked code.
2. **A cast is not a check.** `as T`, `<T>value`, and `!` all compile away. None of them are acceptable as the sole guarantee for external data. Flag them at trust boundaries even if the code "usually works" — the point is what happens on the malformed/adversarial input, not the happy path.
3. **`unknown` is the correct type for genuinely-unvalidated external input**, not `any`. `unknown` forces every consumer to narrow before use; `any` opts the value (and everything derived from it) out of type checking entirely, often silently propagating far beyond the original ingestion point.
4. **Validation must match the type it claims to back.** A partial validator (checks `id` and `name` exist but the type declares five required fields) is a false sense of security — flag a type/validator shape mismatch as a defect, not just validator absence.
5. **Distinguish internal-boundary types from trust-boundary types.** Not every `any`/assertion in a codebase needs a schema validator — internal function calls between already-typechecked code do not cross a trust boundary. Scope the audit to data that originates outside this codebase's own compiled/typechecked surface.

## Minimal safe audit flow

1. Identify every point in the diff where external data enters: fetch/axios/SDK response handling, `JSON.parse` calls, `postMessage` listeners, `URLSearchParams`/`location.search` reads, `process.env`/`import.meta.env` reads used in logic (not just build-time constants), form-field extraction.
2. For each, trace whether the value passes through a runtime validator (schema library, hand-written type guard with actual property checks) before being treated as its declared type, or whether a bare assertion/cast/generic-only-annotation is the only "check."
3. Where a validator exists, confirm its declared shape matches the TypeScript type it's meant to back — a validator checking fewer fields than the type declares is a gap, not full coverage.
4. Where no validator exists, name the concrete blast radius: what downstream code trusts this value's shape, and what happens if a field is missing/wrong-typed/malicious (e.g. `undefined.toUpperCase()` throwing, or a numeric field actually being an attacker-supplied string flowing into a template literal or SQL-adjacent context).

## Adversarial checklist

Before approving a trust-boundary type as sound, answer:

- What is the actual runtime function/library call that validates this data, by name — not "it's typed as `User`"?
- What happens if a required field is absent? Does the validator reject it, or does the type only claim it's required while runtime silently proceeds with `undefined`?
- What happens if a field has the wrong primitive type (a string where a number is declared)? Coercion, rejection, or silent pass-through?
- Is the validator's schema kept in sync with the TypeScript type by construction (e.g. `z.infer<typeof schema>` generates the type from the validator) or are they two independently hand-maintained declarations that can drift?
- If this is a third-party SDK response, does the SDK's own published types (if any) constitute a runtime guarantee, or are they — like all TypeScript types — compile-time only and equally unenforced by the SDK itself?

If these cannot be answered concretely, the trust-boundary claim is unverified — report it as a finding, not a pass.

## High-risk assumptions to kill

- "It's typed, so it's validated" — types are erased; only executed code validates.
- "The SDK's TypeScript types mean the SDK enforces the shape" — a third-party SDK's `.d.ts` file is exactly as unenforced at runtime as first-party code's types.
- "We control the API, so the response is always well-formed" — deploy skew (client ahead of/behind the API), a compromised or misconfigured upstream, or a partial outage returning an error body with a 200 status all violate this assumption in practice.
- "`as unknown as T` is safer than `as T`" — it silences the double-assertion protection the compiler otherwise gives when the two types are structurally too different to be a plausible mistake; it typically appears specifically to bypass that protection, which is a stronger signal something is being forced through.

## Verification targets

- `git diff` or `Grep` for `JSON.parse(`, ` as `, and `!` immediately following a value derived from `fetch`, `axios`, `postMessage`, `URLSearchParams`, `process.env`, or a named SDK client.
- Presence and shape of a schema-validation library call (`.parse(`, `.safeParse(`, or a hand-written `is`-suffixed type-guard function) between the raw external value and its first typed use.
- If a schema library generates the TypeScript type (e.g. `z.infer<...>`), confirm the type in the diff is actually derived that way rather than hand-declared separately, which would let the two drift.

## When to push back

Push back if the user asks to:

- add a type annotation to an external-data-handling function as the fix for a bug, without adding a runtime check — the type change alone does not prevent the bug's root cause,
- suppress a `no-unsafe-assignment`/`no-explicit-any` lint finding at a trust boundary with an inline disable comment instead of adding validation,
- treat "the SDK is typed" as sufficient justification to skip validating that SDK's response shape.
