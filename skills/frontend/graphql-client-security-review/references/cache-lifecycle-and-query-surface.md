# Cache Lifecycle and Query-Surface Risk

Use this reference when reviewing cache clearing on logout, `typePolicies` masking of sensitive fields, or `PersistedQueryLink` allowlisting of the client's query surface.

## What people get wrong

The naive assumption is:

> "We call the logout API and redirect to `/login` -- the user is logged out, so we're done."

Wrong for a normalized-cache GraphQL client. Apollo Client's `InMemoryCache` (and equivalents) persists query results across the lifetime of the JavaScript process, not just the lifetime of a single request. If the server-side session is invalidated but the client-side cache is never told to forget what it knows, the previous user's cached data remains resolvable by whatever renders next -- a shared device, a fast re-login by a different account, or a client-side-only account switch with no full page reload. Redirecting to a login page does not clear an in-memory cache; only an explicit call does.

A parallel naive assumption applies to the query surface:

> "Our GraphQL endpoint requires authentication, so a client can't do anything malicious with it."

Also wrong. Requiring authentication limits *who* can send a query, not *what* they can send once authenticated. A legitimate, authenticated client (or a compromised/malicious one using valid credentials) can still send arbitrarily deep, alias-heavy, or otherwise expensive queries if nothing on the client or server constrains the query surface to a known-safe set.

## Officially grounded rules

Apollo Client's own networking/authentication documentation states directly (`repo evidence` via Context7 `/apollographql/apollo-client`):

- **Reset the store on logout.** The documented pattern calls `client.resetStore()` from the logout handler, immediately after the application's own logout call, specifically so the UI reflects the logged-out state and stale data from the previous session is not retained.
- **`resetStore()` clears the cache and refetches active queries; `clearStore()` clears the cache without refetching.** Choose based on whether an immediate refetch of currently-mounted queries is desired (`resetStore()`) or whether the app is about to unmount/navigate away entirely and a refetch would be wasted work (`clearStore()`). Either call clears the previous session's normalized cache entries -- the choice is about refetch behavior, not about whether the cache gets cleared.
- **`PersistedQueryLink` allowlists the client's query surface.** Chaining a `PersistedQueryLink` ahead of the transport link (e.g. `persistedQueryLink.concat(httpLink)`) restricts the operations the client can send to a build-time manifest of pre-registered, reviewed query documents, rather than allowing arbitrary ad hoc documents to reach the server.

## Non-negotiable design rules

### 1. Trace the logout path to its actual end, not to the redirect

Do not treat a call to `navigate('/login')` or `window.location.assign('/login')` as evidence the session is cleaned up. Follow the handler line by line: does it call `resetStore()`/`clearStore()` on the client instance *before or as part of* the logout flow? A redirect with no cache-clearing call leaves the cache populated for whatever renders next in that same JS process.

### 2. A cache-clearing call elsewhere in the codebase does not clear this finding

If `resetStore()` is called somewhere in the app (e.g., in a settings-reset flow unrelated to auth), that does not establish the logout handler under review clears the cache. Trace the specific handler.

### 3. Persisted-query allowlisting is a structural control, not a performance optimization

Treat the absence of `PersistedQueryLink` (or an equivalent allowlist) as a security finding, not merely a caching/performance suggestion, when the endpoint accepts arbitrary client-authored documents. The finding's severity scales with reachability: a public, unauthenticated endpoint with no allowlist is higher severity than an authenticated one, but authentication alone does not clear the finding.

### 4. Sensitive fields need field-level masking, not just cache-level access control

A GraphQL client cache has no user-level access control of its own -- anything normalized into it is readable by any code running in that JS context (including the devtools inspector, if enabled). The only client-side control for a sensitive field is a `typePolicies` (or equivalent) `read()` policy that strips the value before it is stored, or simply never requesting the field in the first place.

## Minimal safe implementation patterns

Cache clearing on logout (`repo evidence`, Apollo Client's own authentication docs):

```javascript
async function handleLogout(client) {
  await logoutAPI();
  await client.resetStore(); // clears cache, refetches active queries
  navigate('/login');
}
```

Persisted-query allowlisting on the link chain:

```javascript
import { PersistedQueryLink } from "@apollo/client/link/persisted-queries";

const persistedQueryLink = new PersistedQueryLink({
  generatePersistedQueryIdsFromManifest: () =>
    import("./persisted-query-manifest.json")
});

const client = new ApolloClient({
  link: persistedQueryLink.concat(httpLink),
  cache: new InMemoryCache()
});
```

Masking a sensitive field via `typePolicies`:

```javascript
const cache = new InMemoryCache({
  typePolicies: {
    CreditCard: {
      fields: {
        cvv: {
          read() {
            return undefined; // never persisted into the normalized cache
          }
        }
      }
    }
  }
});
```

## Adversarial checklist

Before clearing a logout/session-teardown handler:

- Does the handler call `resetStore()` or `clearStore()` (or the client-specific equivalent) on the exact path being reviewed -- not somewhere else in the codebase?
- Is the cache-clearing call ordered so it actually runs before the user could plausibly see stale cached data (before or immediately after the redirect, not in a code path that could be skipped)?
- If the app supports multiple concurrent sessions/accounts (account switching without a full logout), does switching accounts also clear or scope the cache appropriately?

Before clearing a query-surface finding:

- Is there a `PersistedQueryLink` (or equivalent) between application code and the transport link, or can arbitrary documents reach the server?
- If no allowlist exists, is the endpoint gated by anything else that meaningfully bounds query cost (server-side query complexity/depth limiting)? If the review scope is client-only, state explicitly that server-side limits are out of scope and unverified.

Before clearing a sensitive-field finding:

- Does the field have a `typePolicies` `read()` masking policy on the exact type/field being queried?
- Is the field requested by any query in scope at all -- if not, state explicitly that it was checked and found not reachable, rather than omitting it silently.
