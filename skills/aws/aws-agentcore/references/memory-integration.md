# AgentCore Memory Integration Guide

> Version note: AgentCore tooling is evolving. Verify exact CLI syntax against the installed toolkit and current official AWS docs before production use. Do not paste secrets into commands or files.

## What people get wrong

The common bad assumption is:

> “Memory” is just chat history persistence.

That is incomplete.

AgentCore Memory design has at least four separate concerns:

1. **resource lifecycle** — create, wait for `ACTIVE`, inspect, delete
2. **identity model** — `actor_id` and `session_id` are not interchangeable
3. **namespace design** — what gets stored where
4. **metadata filtering** — indexed keys and retrieval filters that change what memories are visible
5. **retention / expiry / deletion** — what survives, for how long, and how you remove it

If you do not model those explicitly, you create memory bugs that look like “the AI is weird.”

## Officially grounded building blocks

AWS docs and Context7 show Memory supports:

- short-term session memory
- long-term strategies such as:
  - session summarization
  - user preference extraction
- semantic / fact extraction
- structured metadata extraction and filtering for long-term memory retrieval

Typical namespace patterns in official examples include:

- `/summaries/{actorId}/{sessionId}/`
- `/preferences/{actorId}/`
- `/facts/{actorId}/`

That pattern is not cosmetic. It is your isolation boundary.

## Non-negotiable design rules

### 1. Keep `actor_id` and `session_id` stable and intentional

- `actor_id` is the user or principal identity dimension
- `session_id` is the conversational/session boundary

If you generate random IDs carelessly on every call, memory will look “broken” because nothing reconnects.

If you reuse the same actor/session pair across unrelated contexts, memory will look “smart” while actually leaking state.

### 2. Design namespaces before writing code

Do not dump everything into one namespace.

Use separate namespaces for:

- per-session summaries
- per-user preferences
- reusable semantic facts

If you cannot explain why a memory belongs in a namespace, you are not ready to store it.

### 3. Retention is a product decision, not just an implementation detail

Official examples show `eventExpiryDuration` and long-term memory strategies.

You need to decide:

- which memories expire
- which are durable
- how deletions work
- what counts as user-requested forgetfulness

If your skill does not surface expiry/deletion, it is not production-ready.

### 4. Metadata schema is an isolation and relevance boundary

Current docs/release notes describe structured metadata filtering for long-term memory. Indexed keys can be declared when creating memory and metadata filters can narrow retrieval/listing.

Do not bolt this on casually:

- decide indexed keys before creation; changing indexed-key strategy later may require new resources or migration
- avoid indexing sensitive values unless the access model and logs are understood
- test retrieval with and without filters for same actor/session, new session, and different actor
- document whether metadata is extracted by LLM strategy, application code, or both

### 5. Wait for resource readiness

Official examples explicitly poll until the memory resource becomes `ACTIVE`.

Do not assume “create returned” means “safe to wire into the agent.”

## Minimal safe implementation flow

1. Create the memory resource
2. Wait until it is `ACTIVE`
3. Decide namespace strategy
4. Decide metadata/index/filter strategy for long-term memory
5. Wire `memory_id`, `actor_id`, and `session_id` into the agent intentionally
6. Test:
   - same actor + same session
   - same actor + new session
   - different actor
   - same actor with restrictive metadata filters
7. Verify expiry/deletion behavior before broad rollout

## Example patterns from official docs

### Create a memory resource with strategies

Official AWS examples show CLI / SDK flows with long-term strategies such as summarization, preferences, and semantic facts. Treat these as design templates, not copy-paste truth for every app.

### Strands integration shape

Official examples show Strands integration using:

- `AgentCoreMemoryConfig`
- `AgentCoreMemorySessionManager`
- optional retrieval configuration for long-term namespaces

That means your real integration surface is not just “turn memory on”; it is:

- resource ID
- actor/session mapping
- retrieval behavior
- metadata extraction and filter behavior

## Adversarial checklist

Before recommending Memory, answer these:

- What is the source of truth for `actor_id`?
- What causes a new `session_id`?
- Which namespace stores preferences vs facts vs summaries?
- Which metadata fields are indexed?
- Who can set or influence metadata values?
- What expires automatically?
- What is the deletion story?
- What happens if two applications share the same memory resource?
- What is the fallback behavior if Memory is unavailable?

If you cannot answer those, your guidance is shallow.

## Safe command/code verification targets

Verify against current docs and local tooling before use:

- `agentcore memory ...`
- memory create/get/list/delete flows
- integration package names and examples
- resource status / readiness checks

## When to push back

Push back if the user says:

- “just remember everything”
- “reuse one memory for all users”
- “we’ll figure out deletion later”
- “generate IDs automatically somehow”

Those are not shortcuts. They are design failures.
