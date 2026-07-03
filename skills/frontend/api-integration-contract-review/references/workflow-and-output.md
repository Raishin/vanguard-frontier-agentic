# Contract Review Workflow and Verdict Contract

Use this reference for the step-by-step review procedure and the required output shape for every API integration contract review. Load the other two references only when the specific concern (authorization/data-minimization, or error-shape/CORS/versioning) is actually in scope for the endpoint under review.

## Workflow

1. **Identify the contract surface.** Is this a direct frontend-to-backend call, or a BFF route handler aggregating/proxying one or more upstream calls? Note every upstream system the response depends on.
2. **List every field in the response.** For each field, state which caller role(s)/scope(s) need it and why. Any field without a stated justification is a data-minimization finding.
3. **Trace the authorization path.** Find where the object/resource being requested is checked against the authenticated session. Confirm this check happens server-side, using the session's own identity — not merely by trusting a client-supplied ID as proof of ownership.
4. **Inspect error handling.** Find where upstream/backend errors are caught and confirm they are mapped to a sanitized client-facing shape before being returned, not forwarded as-is.
5. **Check CORS configuration.** Confirm the allowed-origin policy is an explicit allowlist (not a wildcard) whenever `Access-Control-Allow-Credentials: true` is set, and confirm the policy matches the actual trust boundary (not a blanket allowance for convenience during development left in place).
6. **For contract changes, identify existing consumers.** Search the codebase (and ask the user for out-of-repo consumers if the contract is public/cross-team) for callers of the changed field/status-code/error-shape. If any exist and the change is breaking, require a stated deprecation window (dual-write, versioned route, or additive-only change) before approval.
7. **Verify version-sensitive framework claims via Context7** (see `SKILL.md`'s Context7 Documentation Protocol) before relying on any claim about caching defaults, route-handler behavior, or query-key serialization.
8. **Issue a verdict** using the decision tree below.

## Decision tree

- Any response field lacks a stated authorization justification → **block** (excessive data exposure).
- Object-level authorization is not independently re-derived server-side from the authenticated session → **block** (BOLA risk; escalate as a security finding, not a style note).
- Error responses forward upstream detail (stack traces, internal hostnames, DB/vendor error bodies) → **block**.
- CORS is wildcard origin combined with credentialed requests → **block**.
- A breaking contract change has identified existing consumers with no deprecation plan → **block-with-conditions**, naming the required plan.
- All of the above are resolved, and any version-sensitive framework claim is Context7-verified or flagged unverified with the flag surfaced to the requester → **approve**, or **approve-with-conditions** if only non-blocking conditions (e.g., an unverified Context7 claim pending confirmation) remain.

## Output contract

Every response from this skill must include:

1. The endpoint/route handler and its consumer(s), stated explicitly.
2. A per-field data-minimization table or list: field → justified for which caller scope, or flagged unjustified.
3. The authorization enforcement mechanism found in the code, and whether it is independently server-verified (yes/no, with the file/line evidence).
4. Error-shape and CORS findings, each labeled by severity (blocking / non-blocking).
5. The versioning/deprecation status for any breaking change, including the consumer list found.
6. The verdict: approve / approve-with-conditions / block, with every unresolved condition listed explicitly (not implied).
7. Every version-sensitive framework claim labeled `Context7-verified` or `documentation-based — unverified this session`.
8. Open questions the review could not resolve from available evidence (e.g., "cannot confirm out-of-repo consumers without a service registry").

A response missing any of these eight elements is an incomplete review, not a shorter one.
