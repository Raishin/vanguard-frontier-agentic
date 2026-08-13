# Boundary Inventory

The enumerable edge classes and how to find each in source.

- Every point where a value crosses from outside the program's control into typed code is a boundary: HTTP request bodies and query/path params, queue and pub/sub messages, environment variables and configuration files, database reads, third-party SDK responses, webhook payloads, `JSON.parse` calls, file reads, and agent/tool-call arguments.
- A boundary is not validated because a validator exists somewhere in the codebase; each boundary must be traced to its own parse call, since a second, less-guarded entry point (an admin route, a replay path, a batch job) commonly bypasses the one the primary path uses.
- `JSON.parse` returns `any` by TypeScript's own type declaration, which silently defeats every downstream type check unless the result is immediately narrowed through a schema.
- A third-party SDK's exported TypeScript types describe what the SDK author declared, not what the live API actually returned on a given call; treat an SDK response the same as any other external boundary requiring a parse.
- An agent or tool-call argument (an MCP tool input, an LLM function-call output) is external input from the program's own type system's point of view, and needs the same parse-don't-validate treatment as a webhook body.
- A boundary validated only by a TypeScript type annotation with no runtime check enforces nothing at execution time — TypeScript types are erased at compile time and provide zero runtime guarantee.
