# OWASP API Security — trust-boundary risks

Use this reference only when a BFF pass-through-authorization, credential-forwarding, or topology-leak finding is present, to ground the finding's OWASP API Security Top 10 classification and severity framing. Do not load this for placement-decision-only or duplication-only findings.

## What people get wrong

The naive story is:

> The BFF sits on the server, so anything it does is "backend" and therefore trusted. If the client already sent a valid-looking token or role claim, the BFF can just pass it along to keep things simple.

That confuses **where the BFF runs** with **what it verifies**. A BFF's value as a trust boundary comes entirely from the fact that it terminates the client's untrusted request and re-establishes a verified identity before talking to backends that may extend it a higher level of implicit trust (network-level trust, service credentials, mTLS). A BFF that relays a client-supplied claim or token unmodified has not added a trust boundary — it has added a network hop that carries the exact same unverified trust forward, while looking, to anyone reading the architecture diagram, like a security control exists there.

## Why this maps to the OWASP API Security Top 10, not a generic bug

The OWASP API Security Top 10 exists because API-shaped systems fail differently from traditional web apps — trust decisions happen at machine-to-machine boundaries with no UI to visually obscure the shortcut. The BFF failure modes in this skill's scope map onto specific categories:

- **Pass-through authorization** — the BFF reads a role, permission flag, or user identifier directly from client-supplied input (a header, body field, or query parameter) and uses it to gate a backend call, instead of re-deriving that identity from its own verified session. This is a **Broken Object Property Level Authorization / Broken Function Level Authorization** failure: the client dictates its own privilege level, and the BFF enforces nothing.
- **Credential forwarding** — the BFF forwards a client-supplied bearer token, API key, or session artifact straight to a downstream backend instead of re-authenticating and minting its own downstream credential scoped to the verified identity. This is a **Broken Authentication** failure at the BFF layer: the BFF has no authentication step of its own, it is a transparent relay wearing an authentication-shaped label.
- **Topology leakage** — the BFF's responses or error handling expose internal-only backend hostnames, service names, or backend-specific error codes/shapes verbatim to the client. This is a **Security Misconfiguration / excessive data exposure** failure: the BFF's shaping responsibility — the entire reason to interpose a server-side layer instead of letting the client call backends directly — has been skipped, and the client now has a map of internal architecture it should never see.

All three are trust-boundary defects, not merely "missing error handling" or "verbose logging" — the defect is that a boundary presented as a security control performs none of the verification or shaping that justifies calling it one.

## Non-negotiable framing rules

- **Severity floor**: pass-through authorization and credential forwarding are HIGH severity, regardless of how the client-supplied value was obtained (even if it originated from a legitimate prior login) — the defect is that the BFF trusts it without independent verification. Topology leakage is MEDIUM-to-HIGH depending on the sensitivity of what leaked (an internal hostname is lower severity than a leaked service-account error revealing valid credentials or query structure).
- **Evidence requirement**: cite the exact client-controlled value (header name, body field, query param) the BFF trusts in place of its own verification, or the exact response/error path that exposes internal topology.
- **Do not conflate with input validation**: a BFF route that fails to validate the *shape* of a request body is a data-integrity note, not a trust-boundary finding by itself. These categories apply specifically to authorization/authentication *decisions* and *response shaping*, not to whether input is well-formed.
- **Token-exchange is not automatically a defect**: a BFF that receives a client token and performs an explicit, documented token-exchange step (validating the client token, then requesting a new downstream-scoped token from an authorization server) is re-verifying, not passing through. The distinguishing question is always: did the BFF perform its own verification step, or did it just relay the value forward unchanged?

## Minimal safe fix pattern

Every HIGH finding from this reference should point toward the same shape of fix: the BFF independently verifies the caller (session validation, token introspection, or a documented token-exchange call) and derives the identity/role used for backend authorization from that verification step — never from a value the client attached to the request. For topology leakage, the fix is a response-shaping layer that maps backend-specific errors to a generic, client-safe error taxonomy before the response leaves the BFF.

## When to push back

Push back if the user proposes:

- "the token was already validated on the frontend, so the BFF doesn't need to check it again" — reject; frontend validation is not enforceable and does not constitute a server-side trust boundary.
- "we'll just forward the Authorization header, it's simpler than minting a new one" — reject unless this is an explicit, documented token-exchange design with its own verification step, not a bare relay.
- "the error message is only useful for debugging, it's fine if it shows the service name" — reject; a debugging convenience shipped to production clients is exactly the kind of topology leak this reference exists to catch.

Those are not shortcuts. They are the exact failure mode this reference exists to catch.
