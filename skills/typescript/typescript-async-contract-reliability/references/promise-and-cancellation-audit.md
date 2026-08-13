# Promise And Cancellation Audit

How to find ignored promises, void-position async functions, and unpropagated cancellation signals.

- Node's default `--unhandled-rejections` mode is `throw`; an unhandled promise rejection is documented to terminate the process by default rather than merely log a warning.
- Node's documentation states it is not safe to resume normal operation after `'uncaughtException'` — a handler that catches it and continues serving requests operates on state Node itself does not guarantee is safe.
- `AbortController` and `AbortSignal` are documented as stable Node globals; accepting a signal at a boundary and never forwarding it to the inner asynchronous call it is meant to cancel is a plumbing gap, not a working cancellation contract.
- A `.catch(() => {})` attached purely to silence a rejection warning is not equivalent to handling the failure the rejection represents — the promise's rejection is suppressed, not resolved.
- Four typescript-eslint typed rules directly detect this domain's defects — `no-floating-promises`, `no-misused-promises`, `await-thenable`, `require-await` — but all four require type information to run; whether they are actually enabled and reachable is `typescript-static-enforcement-policy-agent`'s question, not this agent's, though this agent flags the defect instances the rules are designed to catch.

## Sources

- https://typescript-eslint.io/packages/parser/
