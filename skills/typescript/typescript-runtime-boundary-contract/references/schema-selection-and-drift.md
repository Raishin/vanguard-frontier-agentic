# Schema Selection And Drift

How to verify which validator is installed, dialect implications, and the regenerate-and-diff drift check.

- Confirm which validator is actually installed (check `package.json`/lockfile) before asserting its behavior — `zod` and `ajv` have materially different defaults and must not be described interchangeably.
- `ajv`'s default export validates JSON Schema draft-07; validating the 2020-12 dialect (the current JSON Schema release) requires the `Ajv2020` export specifically — a schema written for 2020-12 keywords silently misbehaves under the default export.
- `zod`'s `parse()` throws on failure while `safeParse()` returns a discriminated result object; a caller that calls `safeParse` and does not branch on `success` has effectively not validated anything.
- `zod`'s `z.toJSONSchema()` throws by default on a schema containing an unrepresentable construct rather than silently producing a lossy schema — a caught-and-ignored throw here means the exported JSON Schema is missing, not merely imprecise.
- A schema and a hand-written TypeScript interface for the same shape are two maintenance points; the safer default is deriving the static type from the schema (`z.infer` or equivalent) so the two cannot diverge silently.
- Regeneration drift: a codegen-produced schema or type not regenerated in the same change as the API or database definition it mirrors is stale evidence — require proof of a wired regeneration step, not an assumption that it ran.
