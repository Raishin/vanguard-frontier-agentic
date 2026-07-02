# Review workflow and findings contract

Use this reference for the full boundary-decision procedure, the existing-BFF audit method, and the required output shape.

## What people get wrong

The naive story is:

> A BFF is just a place to smoosh a couple of API responses together before sending them to the client. If the client is already logged in, the BFF can just pass that along.

Wrong, on both halves.

- **Aggregation-placement is not a style preference.** Whether a need should be solved server-side (BFF) or client-side (parallel calls from the browser) is a trust-boundary and topology question first, and a latency/convenience question second. Getting it backward either forces the browser to hold credentials for internal-only services, or grows a BFF into an unbounded pile of feature-specific routes with no consolidation.
- **"The client is logged in" is not the same as "the BFF has verified this request."** A BFF sits between an untrusted client and one or more backends that may trust the BFF implicitly (mTLS, network-level trust, service credentials). If the BFF forwards a client-supplied claim or token instead of re-establishing its own trusted identity for the call, it has not added a trust boundary — it has just added a network hop with the same trust problem.

The review has to operate at two levels: the **placement decision** (should this be a BFF route or client composition, and does a route already exist for this need) and the **trust-boundary audit** (does the BFF re-verify, or does it pass through) — and a clean placement decision does not excuse a failed trust-boundary audit, or vice versa.

## Workflow — new aggregation need (placement decision)

1. **Enumerate the backends the feature needs data from.**
   - List each backend service, its authorization model (session cookie, API key, mTLS, service token), and its current reachability from the browser (already CORS-exposed and public, or internal-only).

2. **Apply the default-toward-BFF test.**
   - Two or more backends with different auth models or error shapes → default toward BFF aggregation.
   - All backends are already browser-reachable, already public, and share a trust level with the client → client-side composition is a legitimate option.
   - Any backend involved is internal-only and not intended to be browser-reachable → BFF aggregation is required; client-side composition would mean exposing that backend directly, which is a bigger regression than the placement question itself.

3. **Search for an existing BFF route covering an overlapping need.**
   - Grep the BFF/route-handler directory for routes touching the same backend services or a similar response shape. A new route that re-implements existing aggregation logic instead of extending it is a duplication finding, not a placement finding — call it out separately.

4. **If a Route Handler is the proposed implementation, confirm the Next.js major version before making caching claims.**
   - See the Context7 Documentation Protocol in SKILL.md. Route Handler `GET` caching defaults changed between Next.js 14 and 15; do not claim a BFF route "reduces backend load through caching" without confirming the version and the explicit `dynamic`/`revalidate`/`fetchCache` configuration in the route itself.

## Workflow — existing BFF audit (trust-boundary + scope audit)

1. **Enumerate every BFF route in scope.**
   - For each, identify: which backend(s) it calls, what authorization input it uses to gate each backend call, and what shape it returns to the client.

2. **Classify each route's authorization source.**
   - **Re-authenticated (correct):** the route derives the caller's identity from a server-side session/auth mechanism the BFF itself verifies (a validated session cookie, a re-issued short-lived downstream token minted by the BFF after its own verification), independent of any claim the client attached to the request body or headers.
   - **Pass-through (defect):** the route reads a role, user ID, permission flag, or `Authorization` bearer value directly from the incoming client request and forwards it — or a value derived from it — to a backend call without the BFF independently verifying it first.
   - **Missing (defect):** the route makes an authorization-sensitive backend call with no authorization check at all in the BFF layer, relying solely on the backend to reject unauthorized calls (which, if the backend trusts the BFF network-level, means the backend performs no check either).

3. **Check for topology leakage in each route's response and error handling.**
   - Does an error response include a backend hostname, internal service name, stack trace, or backend-specific error code/shape passed through verbatim? If the client can distinguish "billing-service returned 503" from "usage-service returned 500," the BFF is not shaping the response, it is proxying it.

4. **Check for duplicated or drifted aggregation logic across routes.**
   - If two or more BFF routes independently implement near-identical aggregation of the same backends (with copy-pasted or subtly diverging logic), flag it as a maintenance/drift finding — the fix is consolidation, not a rewrite of either route in isolation.

5. **Produce ranked findings.**
   - Order by blast radius: pass-through authorization and credential-forwarding findings first (HIGH), then topology leakage (MEDIUM/HIGH depending on sensitivity of what leaked), then missing-consolidation/duplication findings (MEDIUM/LOW).

## Decision tree

- Feature needs data from 2+ backends with different auth models or error shapes → **BFF aggregation required.** Check for an existing overlapping route before creating a new one.
- An existing BFF route already covers this need → **extend it; do not create a duplicate.**
- Client-side composition would require the browser to hold credentials for, or call directly, an internal-only backend → **block; require BFF aggregation.**
- A BFF route reads a client-supplied authorization claim and uses it directly to gate a backend call → **HIGH: pass-through authorization.** Load `references/owasp-api-trust-boundary.md` and frame the finding per that reference.
- A BFF route forwards a client-supplied bearer token/credential straight to a backend instead of re-authenticating and minting its own downstream credential (and this is not an explicitly documented token-exchange design) → **HIGH: credential pass-through.**
- A BFF response exposes internal-only hostnames, service names, or backend-specific error shapes to the client → **MEDIUM-to-HIGH: topology leak,** severity scaled by sensitivity of what is exposed.
- Two or more BFF routes independently re-implement the same aggregation → **MEDIUM: duplicated aggregation logic; recommend consolidation.**

## Output contract

Return:

1. Boundary decision: BFF aggregation or client-side composition, with the backend count/auth-model/reachability reasoning that drove it
2. For new/extended BFF routes: scope statement and result of the existing-route overlap search
3. Per-route trust-boundary table (for audits): route | authorization source (re-authenticated / pass-through / missing) | credential-forwarding check | topology-leak check
4. Ranked findings, each with:
   - file:line evidence
   - risk class (pass-through-authorization / credential-forwarding / topology-leak / duplicated-aggregation)
   - concrete fix, scoped to the narrowest sufficient change
   - severity (HIGH / MEDIUM / LOW)
   - evidence level (`repo evidence`, `documentation-based`, `inference`)
5. Next.js major version confirmed, if a Route Handler is the proposed or reviewed implementation, or explicitly noted as unconfirmed
6. Verdict: approve / approve-with-notes / block
7. Open questions or explicitly out-of-scope items (e.g. field-level contract details deferred to `api-integration-contract-review`, client-side cache design deferred to `state-management-decision-review`)

## Validation gates

- No duplicated BFF aggregation logic for the same need is approved without a consolidation recommendation.
- Every pass-through-authorization finding identifies exactly which client-supplied value (header, body field, query param) is being trusted in place of BFF-side re-verification.
- No BFF route that exposes an internal-only backend's hostname or service-specific error shape verbatim is approved without a topology-leak finding.
- No new BFF route is approved without first checking for an existing overlapping route.
- No finding is downgraded to a style note for "it's just an internal tool" reasoning — the security-notes hard gate in `metadata.json` applies regardless of perceived internal-only exposure, since internal-only assumptions are exactly what erode over a system's lifetime.

## Common failure modes

- Treating "the BFF runs on the server" as sufficient trust justification, without checking whether it actually re-verifies the caller or just relays what it received.
- Approving a new BFF route without grepping for an existing route already covering the same backend combination.
- Letting error responses pass through backend-specific shapes/codes/hostnames unshaped, because "it's easier to just forward the error."
- Recommending client-side composition to "keep it simple" when one of the backends involved is not meant to be browser-reachable.
- Missing that two BFF routes have quietly diverged while implementing nominally the same aggregation, because each was reviewed in isolation.

## Adversarial checklist

Before finalizing a finding, answer these:

- Does this BFF route re-verify the caller's identity itself, or does it just forward whatever authorization claim/token the client attached?
- Is there already a BFF route serving this need that should be extended instead of duplicated?
- Does the route's response or error handling leak an internal backend hostname, service name, or service-specific error shape?
- Would client-side composition require exposing an internal-only backend directly to the browser?
- Is this aggregation logic duplicated, with possible drift, across more than one BFF route?
- If a Route Handler caching claim is being made, has the Next.js major version actually been confirmed rather than assumed?

If any answer is "not sure," lower the finding's confidence and label the evidence level accordingly — do not present it as a confirmed defect, except for pass-through-authorization and credential-forwarding findings with clear file:line evidence, which stay HIGH regardless.
