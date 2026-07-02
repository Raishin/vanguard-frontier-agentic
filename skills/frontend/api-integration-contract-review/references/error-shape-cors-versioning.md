# Error Shape, CORS, and Versioning

Use this reference only when reviewing error-handling code, CORS configuration, or a breaking/backward-compatible contract change with existing consumers.

## What people get wrong

> "Forwarding the upstream error makes debugging easier for us and for API consumers."

That reasoning optimizes for the wrong audience. A stack trace, an internal hostname, a database driver error message, or a raw vendor error payload (e.g., an unmodified error body from a third-party payment processor) is useful to the team operating the service — via server-side logs and traces — and actively harmful when returned to the client. It discloses internal topology, library versions, and sometimes query fragments or schema details to anyone who can call the endpoint, authenticated or not. Debuggability belongs in observability tooling, not in the HTTP response body.

> "CORS is just a browser annoyance; a wildcard origin gets people unblocked faster."

CORS is an authorization control at the browser layer, and it is meaningfully different from server-side authorization: a wildcard `Access-Control-Allow-Origin: *` says "any origin's script may read this response in the caller's browser." Combined with `Access-Control-Allow-Credentials: true` (cookies, HTTP auth, or client TLS certs sent automatically), this combination is invalid per the Fetch/CORS spec — browsers that implement the spec correctly reject wildcard-origin-plus-credentials at the browser level — and where a misconfigured server pairs a *reflected* origin (echoing back whatever `Origin` header the request sent, which is not a literal `*` but has the same effect) with credentials enabled, it functionally defeats the same-origin protections credentialed requests depend on.

## Officially grounded shape (MDN CORS)

- `Access-Control-Allow-Origin` must be a specific origin (or `null`) when the response is to be used with credentialed requests; the literal value `*` cannot be combined with `Access-Control-Allow-Credentials: true` per spec.
- A server that wants to support credentialed requests from multiple known origins must validate the incoming `Origin` header against an explicit allowlist and echo back only that validated value — not blindly reflect any `Origin` header received.
- Preflight (`OPTIONS`) responses must correctly declare `Access-Control-Allow-Methods` and `Access-Control-Allow-Headers` for the actual methods/headers the real request will use; an overly broad preflight response (allowing methods/headers never actually used) is a smaller but related over-permissioning smell worth flagging alongside a wildcard-origin finding.

## Non-negotiable design rules

### 1. Error responses are shaped at the boundary, not passed through

Every upstream/backend error must be caught and mapped to a client-facing shape (a stable error code, a safe human-readable message, no internal detail) before it reaches the client. This mapping should happen at a consistent boundary (a shared error handler / middleware), not ad hoc per route, so a new route can't accidentally skip it.

### 2. CORS is an explicit allowlist wherever credentials are involved

If the endpoint accepts cookies, HTTP auth, or client certificates (`Access-Control-Allow-Credentials: true`, or the client sets `credentials: 'include'`), the allowed-origin policy must be an explicit, reviewed allowlist — validated against the `Origin` header, not a wildcard and not an unvalidated reflection of whatever origin asked.

### 3. Breaking changes require an identified consumer list and a stated migration path

A contract change is breaking if it removes a field, renames a field, narrows a previously-accepted input shape, changes the meaning of an existing status code, or changes the error response shape in a way an existing consumer's error-handling logic depends on. Before approving a breaking change:

- Identify existing consumers (search the codebase; ask the requester about out-of-repo/cross-team consumers if the contract is public).
- Require a stated plan: an additive-only change (preferred — add a new field/route instead of changing the old one), a versioned route (`/v2/...`), or a dual-write/deprecation window with a communicated sunset date.
- "Nothing should be calling this yet" is only acceptable when the reviewer has evidence (a search result, a service registry entry, an explicit statement from the requester with context) — not assumed by default.

### 4. Additive changes still get a data-minimization pass

A new field added to an existing response is not automatically safe just because it's additive — it still needs the same field-justification check as a net-new endpoint (see `authorization-and-data-minimization.md`).

## Minimal safe review flow

1. Find every place the route handler / BFF catches an error from an upstream call, a database, or an SDK.
2. Confirm each catch site maps to a sanitized client-facing error shape — check for accidental pass-through (`res.status(500).json(err)` or equivalent is a common accidental-leak pattern).
3. Find the CORS configuration for the route. Confirm it's an explicit allowlist if credentials are enabled; flag a wildcard-plus-credentials combination as blocking.
4. For a contract change, diff the old and new response/request shape field by field and status-code by status-code.
5. Search for existing consumers of any field/status-code/error-shape being removed or changed in a breaking way.
6. If consumers exist and the change is breaking, require the stated migration plan before approval.

## Safe command/code verification targets

- Grep the route handler and its shared error-handling middleware for direct pass-through of caught error objects into the response body.
- Grep the CORS configuration (framework middleware config, reverse-proxy config, or manual header-setting code) for a literal `*` origin alongside `credentials: true` / `Access-Control-Allow-Credentials: true`.
- Search the codebase for callers of the changed endpoint/field/status-code (import references, fetch/query-key usage, generated API client usage) to build the consumer list.

## When to push back

Push back if the user says:

- "just forward the error, it's faster to debug that way" — redirect them to server-side logging/tracing instead.
- "CORS is blocking my testing, just set it to `*`" for anything that also sets credentials — a scoped allowlist (including `localhost` explicitly for local dev) is the correct fix, not a wildcard shipped to production.
- "nothing should be calling the old shape" without having checked — require the search first.
- "we'll tell people about the breaking change after we ship it" — the deprecation window has to exist before the breaking change ships, not after.
