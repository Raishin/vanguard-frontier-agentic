# AgentCore Gateway Integration Guide

> Version note: AgentCore tooling is evolving. Verify exact CLI syntax against the installed toolkit and current official AWS docs before production use. Do not paste secrets into commands or files.

## What people get wrong

The naive story is:

> Gateway exposes tools, so once I create a gateway I’m done.

Wrong.

Official AWS docs imply at least five separate concerns:

1. **inbound authentication** — who is allowed to call the gateway
2. **outbound authentication** — how the gateway authenticates to downstream tools
3. **target modeling** — Lambda, OpenAPI/API, Smithy/AWS-service style targets, or MCP servers
4. **policy enforcement** — Cedar-based controls over who can call what
5. **tool discovery** — including optional semantic search
6. **MCP session behavior** — session IDs, per-user scoping, timeout, streaming, and downstream target session state
7. **interactive MCP flows** — elicitation, sampling, progress notifications, and logging notifications

If you only think “endpoint + tool,” you are missing the real security model.

## Officially grounded gateway shape

AWS docs describe Gateway as:

- a managed MCP-compatible gateway
- a way to connect APIs, Lambda functions, existing services, and pre-existing MCP servers
- a service that handles both **ingress auth** and **egress auth**
- a policy-enforced surface where Cedar-based policies gate tool calls
- a stateful MCP gateway for clients and downstream MCP targets when sessions are enabled
- a streaming and interactive channel for MCP targets that use elicitation, sampling, progress, or logging messages

That is the key insight:

> Gateway is not just transport. It is identity + auth + policy + discovery + session governance.

## Non-negotiable design rules

### 1. Split inbound auth from outbound auth

Do not collapse them mentally.

- **Inbound auth** answers: who may invoke the gateway?
- **Outbound auth** answers: how does the gateway authenticate to the target system?

If you confuse those, you will either overgrant downstream access or break end-user delegation.

### 2. Do not hardcode raw secrets into examples unless there is no alternative

Official docs emphasize managed auth paths and Gateway/Identity capabilities.

So prefer:

- managed credential storage
- OAuth / JWT-based inbound flows
- clearly scoped outbound credentials

Do **not** normalize:

- pasting API keys into chat
- embedding bearer tokens in code samples casually
- pretending headers alone are a security model

### 3. Policy is first-class, not optional cleanup

Official AWS policy docs show:

- Gateway traffic can be mediated by a **policy engine**
- policies use **Cedar**
- policies control which principals can perform which actions on which resources, with conditions

If the skill recommends Gateway without also asking “what is the policy boundary?”, the skill is incomplete.

### 4. Semantic search is useful, but it changes governance

Gateway docs mention semantic tool selection / search to reduce prompt size and scale tool discovery.

That is powerful, but it also means:

- tool metadata quality matters more
- broad catalogs can cause accidental reach
- policy boundaries matter even more

Do not recommend semantic search as pure upside.

### 5. Gateway target type changes the failure mode

Different targets mean different risks:

- **Lambda** → IAM, payload schema, side effects
- **OpenAPI / API** → credential placement, scope creep, request shaping
- **Smithy / AWS service models** → AWS auth/permissions and service blast radius
- **MCP servers** → remote tool governance, protocol trust, capability sprawl

One gateway pattern does not fit all four.

### 6. MCP sessions change state and audit assumptions

Current release notes describe Gateway MCP sessions with a unique `Mcp-Session-Id`, per-authenticated-user scoping, configurable timeouts, and downstream MCP target session state.

Design implications:

- decide whether stateful sessions are required or whether stateless tool calls are safer
- set session timeout intentionally instead of inheriting defaults blindly
- bind audit logs to principal + session ID + target tool
- test what happens when downstream MCP target sessions expire or request user input
- do not reuse session IDs across users, tenants, or unrelated workflows

### 7. Streaming, elicitation, sampling, progress, and logs are not harmless extras

Newer Gateway docs/release notes describe:

- response streaming via SSE for multiple JSON-RPC messages
- elicitation pass-through for user input during tool execution
- sampling messages where MCP servers request LLM completions from the client
- progress notifications and structured logging notifications

These features increase capability and risk:

- elicitation needs UI/user-consent handling and phishing-resistant prompts
- sampling can create unexpected model-call cost and data exposure
- progress/log streams can leak sensitive tool inputs or downstream identifiers
- streaming clients must handle partial, failed, or reordered events safely

## Minimal safe implementation flow

1. Identify target type
2. Define inbound auth path
3. Define outbound auth path
4. Define Cedar policy boundary
5. Add one target only
6. Decide whether MCP sessions/streaming/elicitation/sampling are allowed
7. Verify tool schema and least-privilege behavior
8. Add semantic search only if tool catalog size justifies it

## High-risk assumptions to kill

- “Gateway automatically makes this secure”
- “Cognito exists, so auth is solved”
- “If the agent can see a tool, it should be allowed to use it”
- “Semantic search will pick the right tool”
- “OpenAPI import means safe invocation”
- “MCP server connected means MCP server trusted”

Those are lazy assumptions.

## Safe command/code verification targets

Verify against current docs and local tooling before use:

- `agentcore gateway ...`
- gateway create / target create / inspect flows
- inbound authorization requirements
- outbound authorization setup
- policy engine wiring and mode
- MCP session timeout and `Mcp-Session-Id` behavior
- SSE/streaming, elicitation, sampling, progress, and logging behavior when using MCP targets

## When to push back

Push back if the user asks for:

- one gateway with broad access to everything
- raw secret injection into headers as the default pattern
- no policy layer
- no per-target review
- semantic search over sensitive tools without policy constraints

That is not “faster.” It is reckless.
