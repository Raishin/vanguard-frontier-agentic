# Authorization and Data-Minimization Patterns

Use this reference only when the contract under review involves per-object access control, role-scoped fields, or a suspected Broken Object Level Authorization (BOLA) / Excessive Data Exposure concern (OWASP API Security Top 10, API1:2023 and API3:2023).

## What people get wrong

The common bad assumption is:

> "The client only asks for the objects it owns, so we don't need to check ownership server-side."

That is backwards. A client-supplied identifier — a URL path parameter, a query string, a request body field, or even a claim embedded in a bearer token that the caller can request to include — describes what the caller is *asking for*, not what the caller is *entitled to*. If the server returns whatever object matches the requested ID without independently checking it against the authenticated session, any caller can enumerate IDs and read (or in a mutating endpoint, write) other users' data. This is OWASP's #1 API risk category for a reason: it is the single most common real-world API vulnerability, and it is invisible in a UI walkthrough because the legitimate user's own ID always "just works."

A second bad assumption:

> "We'll just return the full object; the frontend only renders the fields it needs."

Returning the full object regardless of caller scope is Excessive Data Exposure. It shifts the trust boundary from the server (correct) to the client (wrong) — a browser DevTools Network tab, a proxy, or a modified client can see every field the server sent, whether or not the UI renders it. Internal fields (cost basis, internal flags, other users' identifiers embedded in nested objects, soft-deleted records) leak this way constantly.

## Non-negotiable design rules

### 1. Authorization is re-derived, not trusted

The server must independently determine "does the authenticated session have access to this object?" using the session's own identity (user ID, tenant ID, role claims from a verified token) — never by trusting that the requested ID implies ownership. Concretely: `SELECT * FROM orders WHERE id = :id AND account_id = :sessionAccountId`, not `SELECT * FROM orders WHERE id = :id` followed by returning the result unconditionally.

If the authorization check is a client-side redirect, a hidden form field, or a UI-only role gate with no matching server-side re-check, the finding is BOLA regardless of how the client behaves in the happy path.

### 2. Every field has a stated scope justification

Walk the response shape field by field. For each field, the reviewer (or the code's authorization logic) must be able to state: "this field is visible to caller scope X because Y." A field with no stated justification — "we just always send it" — is a data-minimization defect. This applies recursively to nested objects and arrays: a `user` object embedded inside an `order` response can leak another user's email or phone number even if the top-level `order` fields are all justified.

### 3. Scope-derived shape, not scope-derived filtering-after-the-fact

Prefer response shapes that are constructed per-scope (a query/serializer that only selects authorized fields for the caller's role) over constructing the full object and filtering fields out afterward in application code. Post-hoc filtering is fragile — a new field added to the full object later is exposed by default unless every filter call site is updated. Scope-derived construction fails closed; post-hoc filtering fails open.

### 4. List/collection endpoints get the same scrutiny as single-object endpoints

BOLA and data-minimization findings are not limited to `/resource/:id` endpoints. A `/resources?filter=...` list endpoint that does not scope the underlying query to the caller's authorized set — relying instead on the client to only ever request filters it "should" use — has the same defect, at higher blast radius (one request enumerates many objects instead of one).

### 5. Role/tier changes must invalidate cached authorization decisions

If authorization or field-visibility decisions are cached (a memoized permission check, a cached JWT-derived scope, a client-side cache key that doesn't include role), a role downgrade or tenant/session change must not leave stale broader access in effect. When reviewing a query-key or cache-key design for this kind of endpoint, confirm the key includes whatever identity/role dimension the response depends on — see the Context7-grounded query-key note in `SKILL.md`.

## Minimal safe review flow

1. Get the actual response shape (from code, not from a client-facing types file that may drift from what the server actually returns).
2. For each field, ask: which caller scope needs this, and where is that justified?
3. Find the authorization check in the route handler or its middleware. Confirm it reads the session's own identity, not a client-supplied value, to decide access.
4. Confirm the check runs before data is fetched/returned, not after (an after-the-fact check that still leaks partial data in an error path is still a finding).
5. For list/collection endpoints, confirm the underlying query itself is scoped, not just the individual-object path.
6. If a permission/role decision is cached, confirm the cache key includes the role/session dimension.

## Adversarial checklist

Before approving, answer these:

- If I change only the ID in the request (keeping the same session/token), do I get another caller's data? If untested and unverifiable from code alone, say so explicitly rather than assuming "probably fine."
- Is there any field in the response that exists only because "the ORM/serializer returns it by default"?
- Does a nested/embedded object carry fields belonging to a *different* principal than the top-level resource's owner?
- Is the authorization check duplicated per route, or centralized in a way that a new route can accidentally skip?
- Does a list endpoint's filter/query parameter influence which authorization scope is applied, instead of the scope being derived purely from the session?

If any answer is "unknown," the review's output must surface that as an open question, not an implicit pass.

## When to push back

Push back if the user says:

- "the frontend already filters it, so the API response doesn't matter"
- "nobody would guess another user's ID" (security by obscurity is not a mitigation)
- "we'll add the server-side check later, ship the endpoint now"
- "it's an internal-only field, the response just happens to include it"

Those are not acceptable trade-offs for a shipped contract. They are the two most common root causes (API1/API3) in the OWASP API Security Top 10.
