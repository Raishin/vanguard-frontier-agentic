# REST API permission_callback enforcement

## Why this matters

A REST route is reachable by anyone who can send an HTTP request to it, regardless of whether it appears in a menu, an admin screen, or any UI at all. `register_rest_route()` treats the permissions callback as a first-class, separate argument from the main callback for exactly this reason: the main callback should handle the resource, and the permissions callback should decide who is allowed to reach it. Omitting that argument, or setting it to an always-true callback on a route that is not genuinely public, leaves the route open to anyone who finds it.

## NORMATIVE: permission_callback is a required, checked argument

Per the WordPress REST API Handbook (`documentation-based`), `register_rest_route()` currently supports two callback arguments — `callback` and `permission_callback` — and describes their roles explicitly: "The main callback should handle the interaction with the resource. The permissions callback should handle what users have access to the endpoint." The handbook further states plainly that permissions callbacks "are extremely important for security with the WordPress REST API," and that any endpoint exposing data that should not be displayed publicly needs one registered.

Per the `register_rest_route()` function reference changelog (`documentation-based`), version **5.5.0** added a `_doing_it_wrong()` notice for the case where the required `permission_callback` argument is not provided. The notice text itself states the expectation directly: "The REST API route definition for %1$s is missing the required %2$s argument. For REST API routes that are intended to be public, use %3$s as the permission callback." That message names `__return_true` as the sanctioned pattern specifically for routes "intended to be public" — using it on a route that is not intended to be public is the documented anti-pattern this skill flags, not an inference.

## Reviewer evidence criteria

Treat each of the following as a finding when reviewing a `register_rest_route()` call or a `WP_REST_Controller` subclass:

- No `permission_callback` key present in the route's args array at all.
- `permission_callback` set to `__return_true`, an inline closure that always returns `true`, or any other always-true callback, on a route that reads non-public data, accepts a state-changing HTTP method (`POST`/`PUT`/`PATCH`/`DELETE`), or otherwise is not genuinely intended for anonymous, public access.
- `permission_callback` present but checking a condition unrelated to authorization for the operation performed (e.g. checking that a parameter is set, rather than checking the requesting user's identity or capability).
- A permissions callback that itself trusts client-supplied data (e.g. a request parameter) to decide authorization, rather than checking the authenticated user or a server-side capability.

A `permission_callback` correctly scoped to a genuinely public, read-only, non-sensitive route (e.g. `__return_true` on a route that only returns already-public content) is not a finding — confirm the public-data justification before flagging.

## NORMATIVE: REST authentication relies on nonces and capability checks together

Per the WordPress REST API Handbook's authentication documentation (`documentation-based`): "Cookie authentication is the standard authentication method included with WordPress" and "the REST API includes a technique called nonces to avoid CSRF issues." The nonce is transmitted as "the `_wpnonce` data parameter (either POST data or in the query for GET requests), or via the `X-WP-Nonce` header," created against "the action set to `wp_rest`." The handbook also states that "if no nonce is provided the API will set the current user to 0, turning the request into an unauthenticated request" — so a missing or invalid nonce does not error out; it silently demotes the request to an anonymous one, which is itself a review-relevant fact when a route's `permission_callback` assumes an authenticated user.

The same documentation states that beyond nonce verification (which the API performs automatically for you), "the current user must have the appropriate capability to perform the action being performed" — nonce verification proves the request came from an expected context, it does not by itself prove the user is authorized. A `permission_callback` must still perform its own capability check (e.g. `current_user_can()`) for any route that is not intended to be fully public.

## RECOMMENDATION: how to phrase a permission_callback finding

- Name the route (path and HTTP method) and the exact file/line where `register_rest_route()` is called.
- State whether `permission_callback` is absent or set to an always-true callback, and whether the route was intended to be public — cite the code that suggests intent (e.g. handling of user-specific or write-capable data) rather than assuming.
- Give the concrete remediation: add a `permission_callback` that checks `current_user_can()` against the specific capability the operation requires, or confirm and document that the route is intentionally public.
- Note, where relevant, whether the endpoint also depends on nonce-based cookie authentication and whether a missing/invalid nonce would silently demote the request to anonymous rather than fail closed.

## Applicable versions

- `permission_callback` enforcement via `_doing_it_wrong()` notice: WordPress 5.5.0 and later.
- REST cookie authentication and the `wp_rest` nonce action: current WordPress REST API Handbook, re-verify against the handbook before relying on details beyond what is quoted here.

## Sources

- [WordPress REST API Handbook — Routes and Endpoints](https://developer.wordpress.org/rest-api/extending-the-rest-api/routes-and-endpoints/) — supports the `callback`/`permission_callback` argument split, the description of what each callback does, and the handbook's own framing of permissions callbacks as extremely important for security.
- [WordPress REST API Handbook — Authentication](https://developer.wordpress.org/rest-api/using-the-rest-api/authentication/) — supports cookie authentication as the standard method, the `wp_rest` nonce action, the `X-WP-Nonce`/`_wpnonce` transport mechanism, the silent demotion to user 0 on a missing/invalid nonce, and the requirement for a capability check beyond nonce verification.
- [WordPress function reference — register_rest_route()](https://developer.wordpress.org/reference/functions/register_rest_route/) — supports the 5.5.0 changelog entry adding a `_doing_it_wrong()` notice for a missing required `permission_callback` argument, and the notice's own text naming `__return_true` as the pattern for routes intended to be public.

Last verified: 2026-07-16.
