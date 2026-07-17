# Java Agents

Role-based agent board for adversarial, evidence-first static review of **Java /
JVM** enterprise applications — JDK lifecycle and upgrade risk, JPA/Hibernate
data-access performance, and untrusted-deserialization/parser security, with more
specialists landing on the same branch.

## Taxonomy note

`java` is a language/runtime, not a cloud provider, but it is a shipped topical
board and therefore has its own dedicated `provider` value: every asset uses
`provider: java` with a shared `java-` ID prefix. This mirrors the other
non-cloud topical boards (`dotnet`, `hr`, `legal`, `marketing`), each of which
carries its own `provider` value. See `docs/taxonomy.md` and
`docs/language-stack-boards.md`.

## Agent tiers

| Tier | Purpose | Default access | Live execution |
|---|---|---|---|
| Router | Classifies a Java/JVM task and dispatches the narrowest specialist set | read-only | not allowed |
| Review agents | Audit JDK lifecycle, ORM data access, and deserialization/parser security | read-only | not allowed |

Every agent in this board is **static-review** — it reads source and sanitized
configuration only. No agent runs a build, executes tests, opens a database or
broker connection, invokes a JDK, contacts a live system, or edits project files.

## Router

| Agent | Primary use |
|---|---|
| `java-maestro-agent` | Classify a Java/JVM task; dispatch one specialist (focused) or a parallel team of up to four (multi-domain). Routes only — never answers Java questions itself. |

## Review agents

| Agent | Primary use | Must refuse when |
|---|---|---|
| `java-jdk-lifecycle-and-upgrade-agent` | JDK vendor/version identification, support and license-boundary exposure, language/API upgrade blockers, phased upgrade planning | asked to assert a vendor lifecycle date from memory, run a build, or invoke a JDK |
| `java-jpa-hibernate-performance-agent` | JPA/Hibernate fetch strategy: N+1, JOIN FETCH vs @EntityGraph vs @BatchSize vs DTO, pagination-with-fetch, open-in-view, HikariCP pool sizing | asked for a connection string or to run a query/migration |
| `java-deserialization-and-parser-security-agent` | Untrusted deserialization and parser RCE: ObjectInputStream gadget chains, SnakeYAML Constructor, Jackson default typing, XML XXE | asked to deserialize or execute a sample payload |

## Operating notes

- A vendor lifecycle date stated from memory, an unfiltered `ObjectInputStream`
  on request data, a SnakeYAML bare `Constructor`, Jackson default typing without
  a `PolymorphicTypeValidator`, an unhardened XML parser, an N+1 on a request
  path, and pagination over a collection `JOIN FETCH` are the highest-impact
  defects this board exists to catch.
- The board is **static-review only**. Production mutations (deploy, migrate,
  rollout, key/secret changes) and live telemetry (GC pauses, p99) are out of
  tier — the maestro hands them to the named human owner or the appropriate
  provider/observability board.
- JDK **commercial/license** exposure (Oracle per-employee subscription, app-server
  licensing) is a portfolio decision that belongs to the forthcoming
  `java-application-server-exit-agent`, not the technical lifecycle agent.
- The board is being delivered in tranches; the routing table in
  `skills/java/java-maestro/SKILL.md` lists only the specialists currently
  shipped in `catalog/agents.json`.
