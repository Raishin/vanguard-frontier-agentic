# Protocol Version And Error Contract

Version negotiation, error classification, cancellation, and the current revision's departures from its predecessor.

- The MCP specification revision 2026-07-28 removed the `initialize` handshake and protocol sessions entirely, replacing session-based negotiation with a per-request version declaration.
- Every request under the current revision carries `_meta.io.modelcontextprotocol/protocolVersion`; a server or client encountering a mismatched version returns JSON-RPC error code `-32022`.
- The current specification requires servers implement `server/discover` for tool discovery.
- A protocol-level failure (transport, negotiation, malformed request) is returned as a JSON-RPC `error`; a tool-execution failure (the tool ran but the operation failed) is returned as `result.isError: true` — conflating the two removes the caller's ability to distinguish a retryable transport fault from a logic failure.
- Cancellation semantics are transport-dependent; accepting a cancellation signal at the protocol layer without propagating it to the underlying operation leaves work running after the caller believes it stopped.
- The TypeScript SDK split into `@modelcontextprotocol/server` and `@modelcontextprotocol/client` at version 2.0.0; `@modelcontextprotocol/sdk` is the legacy 1.x line, at 1.30.0, and the two should not be mixed in one server without an identified reason.
