# Type-Contract Test Matrix

Compile-time assertion patterns and the consumer configuration set that proves a contract.

- Vitest's `expectTypeOf` and `assertType` are compile-time-only assertions: they produce no runtime check and only execute as part of Vitest's `--typecheck` mode — a repository that ships these assertions without a documented `--typecheck` CI step has a type-test suite that never actually runs.
- `@ts-expect-error` is the only TypeScript-team-documented compile-error assertion, and it self-flags: the directive itself produces a compiler error if the expected error does not occur on the following line, so a stale or now-passing assertion is caught rather than silently going stale.
- A consumer compilation matrix must include the configuration that resembles the largest actual consumer, not merely a convenient default `tsconfig.json` — a matrix built only from the publisher's own configuration proves nothing about a consumer on a different `moduleResolution` or `target`.
- A type-level test that asserts what the current implementation happens to infer, rather than what the declared contract promises, passes straight through a contract-breaking regression: the test and the regression change together.

## Sources

- https://vitest.dev/guide/testing-types
