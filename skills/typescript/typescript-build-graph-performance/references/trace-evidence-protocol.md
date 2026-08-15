# Trace Evidence Protocol

Which measurement to request, how to read it, and the rule that no prescription issues without one.

- The measurement protocol this agent requires before any structural prescription is `--diagnostics`, `--extendedDiagnostics`, or a `--generateTrace` trace — a complaint with none of these attached gets a request for the measurement, never a guessed fix.
- TypeScript 7.0 is GA on the native Go compiler; whether `--generateTrace` and `--extendedDiagnostics` behave identically under it as they did under the classic `tsc` compiler is unverified — record which compiler binary produced any submitted trace, and do not assume parity across the two.
- TypeScript 7.0 has no stable programmatic API until 7.1, which is documented as the reason editor and framework tooling stays on TypeScript 6.0 for now — a trace from editor tooling and a trace from a CI build may therefore come from different compiler majors even on the same repository.
- A throughput or improvement claim made without `--extendedDiagnostics` or trace evidence backing it is treated as unproven and labelled as needing measurement, never asserted as an outcome.

## Sources

- https://github.com/microsoft/TypeScript/wiki/Performance
- https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/
