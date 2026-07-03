# OWASP A01 — Broken Access Control

Use this reference only when a Server Action authorization finding is present, to ground the finding's severity and framing. Do not load this for bundle-leak-only findings.

## What people get wrong

The naive story is:

> The Server Action is protected because it only runs on the server — the client can't tamper with server-side code.

That confuses **where code executes** with **what data the code trusts**. The code executes on the server; the arguments it receives (function parameters, `FormData` fields) originate entirely from the client and are attacker-controlled, whether the caller uses the rendered form or crafts a direct request bypassing the UI. OWASP's A01:2021 — Broken Access Control category exists precisely for this class of defect: access-control decisions enforced only on the client, or derived from client-supplied data, rather than re-verified server-side against a trusted source (the session).

## Why this maps to A01, not a generic bug

OWASP Top Ten A01 (Broken Access Control) covers failures where a user can act outside their intended permissions. The Server Action pattern in this skill's scope produces exactly that failure class when:

- **Authorization uses client-supplied identity** — the action reads a `userId` or `role` field out of its own parameters/`FormData` instead of the server session, so any caller can claim to be any user or role.
- **Insecure Direct Object Reference (IDOR)** — the action re-derives the session correctly (so the caller's identity is trustworthy) but never checks that the session's user owns or may act on the specific resource ID supplied, so an authenticated user can act on another user's resource.
- **Missing function-level access control** — the action performs a privileged mutation with no authorization check at all, relying on an upstream UI/page-level gate that the direct-invocation path bypasses entirely.

All three are Broken Access Control, not merely "missing validation" or "input sanitization" — the defect is that the access-control decision was placed in the wrong trust zone.

## Non-negotiable framing rules

- **Severity floor**: any of the three patterns above is HIGH severity. Do not downgrade because the UI "normally" prevents the bad path — Server Actions are directly invokable independent of the rendered UI, and the review must assume a caller that bypasses it.
- **Evidence requirement**: cite the exact client-controlled field (parameter name or `FormData` key) being trusted, and cite the absence (or presence, for IDOR) of a resource-ownership check.
- **Do not conflate with input validation**: a missing `zod`/schema validation on `FormData` shape is a data-integrity note, not an A01 finding by itself. A01 applies specifically to the authorization *decision*, not to whether the input is well-formed.
- **Page-level checks do not transfer**: an `auth()` check in the page component that renders the form does not protect the Server Action. State this explicitly in the finding so the fix targets the action, not the page.

## Minimal safe fix pattern

Every HIGH finding from this reference should point toward the same shape of fix: re-derive identity from the server-side session inside the action itself, and, for resource-scoped mutations, verify ownership/role against that re-derived identity before performing the mutation. The fix is inside the Server Action, not in the calling page or the client form.

## When to push back

Push back if the user proposes:

- "the page already checks auth, so the action doesn't need to" — reject; each Server Action is an independent invocation surface.
- "we'll trust the role field the client sends since the form only shows it to admins" — reject; UI visibility is not an access control.
- "IDOR checks can wait, the session check is the important part" — reject; an authenticated-but-unauthorized mutation is still Broken Access Control.

Those are not shortcuts. They are the exact failure mode this reference exists to catch.
