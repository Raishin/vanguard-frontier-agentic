# Tool Schema Contract Audit

How to compare a declared schema against handler behavior, field by field, including structured output.

- MCP tool fields under the current specification are `name`, `title`, `description`, `icons`, `inputSchema`, `outputSchema`, and `annotations` — a field absent from either the schema or the handler is a fidelity gap to name explicitly.
- `inputSchema` and `outputSchema` both default to JSON Schema 2020-12 when no `$schema` is present, so a schema authored against another dialect's assumptions without declaring `$schema` will be read under 2020-12 rules by any conformant client.
- `structuredContent` in a tool result is validated against the tool's declared `outputSchema` — a handler that returns `structuredContent` without keeping it in sync with `outputSchema` produces a result that fails that validation.
- A tool's `description` and other model-facing text are part of the trust surface a calling model reads; text written to influence the model's subsequent behavior rather than to document the tool is an injection vector introduced through the contract itself.
- Comparing a schema to handler behavior requires reading the handler's actual parameter destructuring and return construction, not only its type annotations, since a type can be stripped or wrong independently of the schema.
