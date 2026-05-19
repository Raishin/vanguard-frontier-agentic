# .NET Agents

Role-based agent board for reviewing **.NET / C#** codebases — ASP.NET Core,
EF Core, identity and authorization, testing, supply chain, performance, and
cloud-native posture.

## Taxonomy note

`.NET` is a language/runtime, not a cloud provider, but it is a shipped topical
board and therefore has its own dedicated `provider` value: every asset uses
`provider: dotnet` with a shared `dotnet-` ID prefix. This mirrors the other
non-cloud topical boards (`hr`, `legal`, `marketing`), each of which carries
its own `provider` value. See `docs/taxonomy.md` and
`docs/language-stack-boards.md`.

## Agent tiers

| Tier | Purpose | Default access | Live execution |
|---|---|---|---|
| Router | Classifies a .NET task and dispatches the narrowest specialist set | read-only | not allowed |
| Review agents | Audit C#, ASP.NET Core, EF Core, tests, CI/NuGet, performance, telemetry, and Aspire posture | read-only | not allowed |

Every agent in this board is **static-review** — it reads source and sanitized
configuration only. No agent runs `dotnet build`, `dotnet test`, `dotnet ef`,
contacts a live system, mutates a NuGet feed, or edits project files.

## Router

| Agent | Primary use |
|---|---|
| `dotnet-maestro-agent` | Classify a .NET task; dispatch one specialist (focused) or a parallel team of up to four (multi-domain). Routes only — never answers .NET questions itself. |

## Review agents

| Agent | Primary use | Must refuse when |
|---|---|---|
| `dotnet-csharp-runtime-review-agent` | C#/runtime correctness: nullable reference types, async/await, cancellation, disposal, allocations, AOT/trim hazards | asked to compile or run code |
| `dotnet-aspnetcore-api-review-agent` | ASP.NET Core API architecture: middleware order, DI lifetimes, CORS, validation, versioning, health boundaries | asked to run the app or call endpoints |
| `dotnet-aspnetcore-identity-authz-review-agent` | Authentication, authorization, token validation, cookie/session security, tenant isolation | asked for secrets, signing keys, or tokens |
| `dotnet-efcore-data-access-review-agent` | EF Core: DbContext lifetime, N+1, raw SQL safety, concurrency tokens, migrations, multi-tenant query filters | asked for a connection string or to run migrations |
| `dotnet-testing-quality-review-agent` | .NET test quality: assertion-free tests, over-mocking, coverage theater, isolation, missing negative tests | asked to run the suite or a coverage tool |
| `dotnet-supply-chain-review-agent` | .NET CI/CD and NuGet supply chain: SDK pinning, package locking, feed trust, fork-PR secret exposure, vulnerability scanning | asked for CI secrets or feed credentials |
| `dotnet-performance-aot-review-agent` | Performance, Native AOT, and trimming readiness — evidence-gated on benchmark artifacts | asked to accept a perf claim with no benchmark |
| `dotnet-observability-otel-review-agent` | ASP.NET Core OpenTelemetry wiring, structured logging, trace propagation, PII in telemetry | asked to design Collector/backend infrastructure |
| `dotnet-aspire-cloud-native-review-agent` | .NET Aspire AppHost and service-defaults review for cloud-native readiness | asked to treat Aspire as the production runtime |

## Operating notes

- Sync-over-async on a request path, a swallowed `catch {}`, disabled JWT
  validation, an interpolated raw SQL string, a missing multi-tenant query
  filter, and a fork-PR secret exposure are the highest-impact defects this
  board exists to catch.
- The observability agent is scoped to **in-application** OpenTelemetry wiring.
  Collector topology, exporters, and dashboard infrastructure belong to the
  `opentelemetry` provider board — route there.
- A performance claim with no `BenchmarkDotNet` artifact is downgraded to
  `inference` and flagged; "it's faster" is not evidence.
