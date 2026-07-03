# Cache-Control and Vary Response Headers

Load this reference when reviewing a Route Handler's (`route.ts`) or `getServerSideProps`'s response headers for a response derived from per-user data. Includes the MDN grounding reference for `Vary`; cite it only when a `Vary` finding is actually present.

## `Cache-Control: public` on per-user data

Any Route Handler or `getServerSideProps` call that sets `Cache-Control: public` (or a directive implying public cacheability, like a bare `s-maxage` with no `private`) authorizes every shared cache between the origin and the client -- CDN edge nodes, corporate proxies, ISP caches -- to store the response body and replay it to a *different* client that requests the same URL. If that body is derived from `cookies()`, a session token, or any other per-user request context, this is a direct cross-user data-exposure path, not a cosmetic caching choice.

```ts
// DANGEROUS: CDN may store and replay this session-derived body to any
// subsequent requester of the same URL.
export async function GET() {
  const session = (await cookies()).get('session')?.value
  const userData = await db.users.findBySession(session)
  return new Response(JSON.stringify(userData), {
    headers: { 'Cache-Control': 'public, max-age=300' },
  })
}
```

The documented fix is `Cache-Control: private`, which tells shared caches not to store the response at all -- only the requesting user's own browser may cache it locally. Next.js's own dynamically-rendered pages (including Draft Mode) set `private, no-cache, no-store, max-age=0, must-revalidate` by default for exactly this reason; an explicit `public` override on a per-user response works against that default protection.

```ts
// SAFE: shared caches must not store this response.
return new Response(JSON.stringify(userData), {
  headers: { 'Cache-Control': 'private, max-age=300' },
})
```

## `Vary` and cache-key selection

`Vary` tells a shared cache which request headers it must fold into the cache key -- two requests that differ only in a header *not* listed in `Vary` are treated as cache-equivalent and may share a cached response. If a response is `Cache-Control: public` (or otherwise CDN-cacheable) and is derived from a session cookie, but `Vary` does not include `Cookie` (or whatever header actually carries the session identifier), the CDN has no signal that two different users' requests need separate cache entries -- it may serve User A's cached response to User B simply because every other varied header (e.g. `Accept-Encoding`) happened to match.

```ts
// DANGEROUS: Cookie is absent from Vary, so the CDN cache-keys only on
// Accept-Encoding and may conflate two different users' requests.
return new Response(JSON.stringify(userData), {
  headers: {
    'Cache-Control': 'public, max-age=300',
    Vary: 'Accept-Encoding',
  },
})
```

```ts
// SAFE: Cookie is included, so the CDN keys its cache on the session value.
return new Response(JSON.stringify(userData), {
  headers: {
    'Cache-Control': 'public, max-age=300',
    Vary: 'Cookie, Accept-Encoding',
  },
})
```

This is standard HTTP caching semantics (see MDN's `Vary` reference in `official_docs`), not a Next.js-specific API -- ground this specific claim against the MDN URL rather than Context7 against `/vercel/next.js`, which does not reliably document general `Vary` mechanics.

## Reading the trace correctly

A `Vary` gap only matters when the response is otherwise cacheable by a shared cache (`Cache-Control: public`, or a `s-maxage`/CDN-level caching directive). If the response already sets `Cache-Control: private` (or relies on Next.js's dynamic-rendering default), the `Vary` value is moot -- do not raise a `Vary`-only finding on a response that is never stored by a shared cache in the first place. When both are wrong (public `Cache-Control` and an incomplete `Vary`), report them together as one `header-cache-bleed` finding with both header values named, not as two separate findings for the same response.
