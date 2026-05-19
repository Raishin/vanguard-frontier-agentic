# Language and Stack Boards

Language- and stack-specific agent boards (`.NET`, `legal`, `hr`, `marketing`) are not cloud providers. They use `provider: generic` with a shared ID prefix and dedicated directories, enabling clean discovery and future faceting by language or domain without schema changes.

## What are language/stack boards?

Language and stack boards are curated collections of agents and skills organized around a specific language (e.g., `.NET`, `Go`, `Python`), platform (e.g., `Kubernetes`), or business domain (e.g., `legal`, `hr`, `marketing`). Unlike provider boards, which map to cloud platforms, language/stack boards cut across all platforms and focus on specialized expertise in a particular vertical.

**Key difference from provider boards:**
- Provider boards (e.g., `aws`, `azure`, `gcp`) cover cloud-platform-specific operations across multiple domains.
- Language/stack boards cover domain-specific expertise that applies universally (across multiple platforms or languages).

**Example:** The `.NET` board reviews C# code, ASP.NET Core architecture, and EF Core patterns regardless of whether the application runs on Azure, AWS, on-premises, or in a container. The `legal` board advises on compliance, licensing, and policy across all technical stacks.

See `docs/taxonomy.md` for the broader provider and asset-type taxonomy.

## Why `provider: generic`?

Language and stack boards do not introduce new provider enum values. Instead, they use:
- `provider: generic` in all agent and skill metadata
- A shared ID prefix (e.g., `dotnet-*`, `legal-*`, `hr-*`)
- Dedicated asset directories (`agents/dotnet/`, `skills/dotnet/`, etc.)

This pattern avoids schema proliferation and keeps the provider enum focused on cloud platforms. A future language/stack faceting axis (a marketplace feature to group boards by language/domain) can cleanly adopt these assets without any ID or metadata changes.

**Rationale:**
1. Providers are cloud platforms; language/stacks are orthogonal dimensions.
2. Schema stability: no new enum values needed.
3. Future-proof: the assets are already structured for a faceting axis.
4. Consistency: mirrors the existing non-cloud boards (`hr`, `qa`, `legal`, `marketing`), establishing a pattern.

## The four language/stack boards

| Board | Type | Agents | Purpose |
|-------|------|--------|---------|
| `.NET` | Language | 10 | C#, ASP.NET Core, EF Core, testing, supply chain, performance, and cloud-native posture review |
| `legal` | Domain | TBD | Compliance, licensing, regulatory, and legal risk review |
| `hr` | Domain | TBD | HR policy, hiring, and organizational structure review |
| `marketing` | Domain | TBD | Marketing strategy, content, and brand alignment review |

### `.NET` board

**Location:** `agents/dotnet/`, `skills/dotnet/`  
**ID prefix:** `dotnet-*`  
**Install role:** `dotnet-application-review-engineer` (10 agents + 10 skills)

**Agents:**
- `dotnet-maestro-agent` — Router; classifies tasks and dispatches specialists
- `dotnet-csharp-runtime-review-agent` — C# language, async/await, nullability, disposal
- `dotnet-aspnetcore-api-review-agent` — API architecture, middleware, DI, CORS, versioning
- `dotnet-aspnetcore-identity-authz-review-agent` — Authentication, authorization, token validation
- `dotnet-efcore-data-access-review-agent` — EF Core, DbContext, N+1, migrations, query filters
- `dotnet-testing-quality-review-agent` — Test design, mocking, coverage, flakiness
- `dotnet-supply-chain-review-agent` — NuGet, CI, SDK pinning, lock files
- `dotnet-performance-aot-review-agent` — Performance, Native AOT, trimming, benchmarks
- `dotnet-observability-otel-review-agent` — OpenTelemetry wiring, structured logging
- `dotnet-aspire-cloud-native-review-agent` — .NET Aspire, service composition

Each agent is `execution_tier: static-review` — reads source and configuration only; never builds, runs, or mutates code.

### `legal` board

**Location:** `agents/legal/`, `skills/legal/`  
**ID prefix:** `legal-*`  
**Install role:** TBD  
**Agents:** TBD

Scope: Licensing, regulatory compliance, terms and conditions, data protection, and legal risk assessment across codebases, infrastructure, and process.

### `hr` board

**Location:** `agents/hr/`, `skills/hr/`  
**ID prefix:** `hr-*`  
**Install role:** TBD  
**Agents:** TBD

Scope: HR policy, hiring practices, organizational structure, and people operations review.

### `marketing` board

**Location:** `agents/marketing/`, `skills/marketing/`  
**ID prefix:** `marketing-*`  
**Install role:** TBD  
**Agents:** TBD

Scope: Marketing strategy, content, brand alignment, and campaign evaluation.

## How to use language/stack boards

### Discovery

Browse `catalog/install-roles.json` for install roles keyed to business functions. For `.NET`, search for `dotnet-application-review-engineer`:

```json
{
  "id": "dotnet-application-review-engineer",
  "label": ".NET Application Review Engineer",
  "description": "Static review of .NET applications: C# and runtime correctness, ASP.NET Core API architecture, ...",
  "agents": ["dotnet-csharp-runtime-review-agent", "dotnet-aspnetcore-api-review-agent", ...],
  "skills": ["dotnet-csharp-runtime-review", "dotnet-aspnetcore-api-review", ...]
}
```

Install all agents and skills in the role, or pick specific agents for targeted reviews.

### Routing

Language/stack boards often include a maestro router that classifies a task and dispatches the narrowest specialist. For `.NET`:

```
User: "Review our EF Core DbContext for N+1 queries and missing query filters."

Maestro routing:
  Route: dotnet-efcore-data-access-review-agent
  Reason: Task is EF Core specific — data-access domain only.
  Mode: single
```

For multi-domain tasks, the maestro dispatches a parallel team (max 4 specialists).

### Invocation

**In Claude Code:**
Invoke a language/stack agent by its ID. The maestro routes to specialists automatically:

```
@dotnet-maestro-agent
Review my ASP.NET Core API for middleware order, dependency injection lifetimes, and CORS policy.
```

Or invoke a specialist directly:

```
@dotnet-aspnetcore-api-review-agent
Is my middleware order correct? I have logging, CORS, auth, then routing.
```

**In other harnesses (Copilot, Cursor, etc.):**
Language/stack agents are available via the same naming convention as provider agents. Select the agent by name and invoke via the harness's chat interface.

## Adding a new language/stack board

### Step 1: Create directories and structure

```bash
mkdir -p agents/<prefix>
mkdir -p skills/<prefix>
```

Example: `agents/golang/`, `skills/golang/`.

### Step 2: Define agents and companion skills

For each agent:
1. Create `agents/<prefix>/<agent-id>/AGENT.md` (canonical contract)
2. Create `agents/<prefix>/<agent-id>/metadata.json`
3. Create 7 harness adapters under `agents/<prefix>/<agent-id>/harnesses/`
4. Create `skills/<prefix>/<skill-id>/SKILL.md`
5. Create `skills/<prefix>/<skill-id>/metadata.json`

### Step 3: Set metadata

- `provider: generic`
- `execution_tier: static-review` (or `read-only-runtime` for runtime-scoped agents)
- `allowed-tools: Read Grep Glob` (or extend as needed)
- `lifecycle: experimental` (until stable)
- ID prefix consistent across the board (e.g., `golang-*`)

### Step 4: Add an install role

Edit `catalog/install-roles.json` and add a new role grouping the agents and skills:

```json
{
  "id": "golang-code-review-engineer",
  "label": "Go Code Review Engineer",
  "description": "...",
  "agents": ["golang-concurrency-review-agent", "golang-error-handling-review-agent", ...],
  "skills": ["golang-concurrency-review", "golang-error-handling-review", ...]
}
```

### Step 5: Validate and catalog

1. Run `npm run validate` — all gates must pass.
2. Run `npm run manifest:write` to refresh `catalog/skill-manifest.json`.
3. Run `npm run asset-integrity:write` to refresh the integrity manifest.
4. Commit and push.

## Trust posture

All language/stack board agents are **static-review** or equivalent read-only tiers:
- They read source files, configuration, and logs only.
- They never compile, build, test, run, migrate, or contact live systems.
- They never request secrets, connection strings, or credentials.
- They refuse tasks that require mutation or live execution.

This isolation ensures language/stack boards can be invoked safely on any codebase without risk of unintended side effects.

## Cross-reference

- `docs/taxonomy.md` — Provider and asset-type taxonomy
- `catalog/install-roles.json` — Role-to-agent mapping for discovery
- `.NET` board README: `agents/dotnet/README.md`
