# Devtools Exposure and CSRF-less Auth Headers

Use this reference when reviewing `connectToDevTools` configuration or a `SetContextLink`/auth-header-injection link for CSRF protection.

## What people get wrong

The naive assumption for devtools is:

> "The Apollo Client Devtools extension only does anything if someone actually opens it, so leaving `connectToDevTools` on is harmless."

Wrong. `connectToDevTools` controls whether the client *establishes the bridge* the devtools extension connects to -- it is the exposure surface, independent of whether anyone happens to have the extension open at a given moment. An unconditional `connectToDevTools: true` in a production bundle means any user (or any script running in that browser context) with the extension installed can inspect the full schema via introspection and browse the entire normalized cache, including anything a sensitive-field masking review would otherwise catch.

The naive assumption for auth headers is:

> "We attach the token as an `authorization` header, not a cookie, so CSRF doesn't apply to us."

Partially wrong. CSRF is a risk specifically when a credential is attached to a request *automatically*, without explicit action by the calling code, most commonly via a cookie the browser sends on every same-origin (or lenient SameSite) request. If the `authorization` header is the *only* mechanism carrying the credential, and nothing else in the auth flow relies on a cookie, CSRF genuinely may not apply -- but this must be confirmed by tracing the full auth flow, not assumed from the header name alone. Many real apps use a hybrid: a cookie-backed refresh mechanism or a cookie-backed session alongside header-based bearer tokens, at which point the header alone is not the whole story.

## Officially grounded rules

Apollo Client's own documentation states directly (`repo evidence` via Context7 `/apollographql/apollo-client`):

- **`connectToDevTools` explicitly enables the devtools bridge**, and Apollo Client's own developer-tooling documentation shows it being set for use in production builds as an explicit, deliberate override of the client's default (development-only) behavior -- meaning the safe default is to *not* set it to an unconditional `true`.
- **`SetContextLink` (and the older `ApolloLink`-based context-setting pattern) is the documented mechanism for attaching an `authorization` header** to every outgoing request, retrieving the token via an async lookup and merging it into `prevContext.headers`. Apollo Client's documentation demonstrates the auth-header pattern itself but does not prescribe a CSRF-token header as part of the core library -- CSRF protection is application-level responsibility layered on top of the same context-setting mechanism.

## Non-negotiable design rules

### 1. Any unconditional `connectToDevTools: true` is a finding, regardless of stated deployment target

Do not accept "this is only for our staging environment" as clearing the finding unless the review can independently verify the specific file/build path never reaches production. The construct itself -- not the current deployment claim -- is what a static review evaluates.

### 2. Trace the full auth flow before ruling a CSRF finding in or out

Do not judge a `SetContextLink` auth-header link in isolation. Check: is the token *ever* also carried by a cookie anywhere in this app's auth flow (a refresh-token cookie, a session cookie set alongside the bearer-token flow)? If yes, the header-only mutation-request pattern with no CSRF header is a genuine finding, because an attacker's cross-site form/fetch could ride the cookie's default browser-attached credential even though the header is not itself auto-attached. If the token is exclusively attached by explicit client-side code with no cookie-backed fallback anywhere in the flow, state this explicitly as the reason no CSRF finding applies -- do not silently omit the check.

### 3. A CSRF header must be on the same context-construction path as the auth header

If an anti-CSRF header exists in the codebase but is added by a different link, a different request path, or a different client instance than the one under review, that does not clear the finding for the specific `SetContextLink` construction being reviewed.

## Minimal safe implementation patterns

Devtools gated to development only (`repo evidence`, contrasted directly against Apollo Client's own production-enablement example):

```javascript
const client = new ApolloClient({
  uri: "/graphql",
  cache: new InMemoryCache(),
  connectToDevTools: process.env.NODE_ENV === 'development'
});
```

Auth header with CSRF protection attached on the same path:

```typescript
import { SetContextLink } from "@apollo/client/link/context";

const withToken = new SetContextLink(async (prevContext, operation) => {
  const token = await AsyncTokenLookup();
  const csrfToken = await getCsrfToken();
  return {
    headers: {
      ...prevContext.headers,
      authorization: `Bearer ${token}`,
      'x-csrf-token': csrfToken
    }
  };
});
```

## Adversarial checklist

Before clearing a devtools finding:

- Is `connectToDevTools` gated on an environment check that demonstrably evaluates to `false` in a production build, or is it an unconditional literal?
- If gated, does the gate reference a value the review can actually verify (a well-known bundler env substitution) rather than a custom flag whose production value is unknown?

Before clearing a CSRF finding on an auth-header link:

- Does any part of this app's auth flow rely on a cookie (refresh token, session identifier) alongside or instead of the header?
- Is there a named anti-CSRF header (e.g. `x-csrf-token`) attached on the exact same context-construction path as the `authorization` header, or only "a CSRF utility exists somewhere in this codebase"?
- Would a state-changing GraphQL mutation succeed if only the automatically-attached credential (if any) were present, with no explicit client-side header set by an attacker's cross-site request?

If any answer is unclear or reveals a gap, the finding stands at HIGH (devtools) or MEDIUM-to-HIGH (CSRF, per reachability) -- do not soften it to "worth double-checking."
