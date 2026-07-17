# Server-side re-validation and the client trust boundary

## Why this matters

A revenue-critical journey can look correct end-to-end in a browser or mobile
app and still be trivially bypassable, because a client is not a trusted
execution environment. Anything the client computes, displays, or gates —
price, an applied discount, an inventory check, an eligibility rule, a
required-step sequence — can be altered by a crafted request that never runs
the client's code at all. If the server does not independently re-check the
same rule, the "check" was never enforcement; it was UX. The seam this skill
owns is exactly that gap: a rule that exists in the client tier and has no
matching guard in the server tier.

## NORMATIVE: the server is the only enforcement boundary

Per OWASP Top 10:2021 A01 Broken Access Control (`documentation-based`):

> "Access control is only effective in trusted server-side code or server-less
> API, where the attacker cannot modify the access control check or
> metadata."

The same page states the default posture and the granularity access control
must operate at:

- "Except for public resources, deny by default."
- "Model access controls should enforce record ownership rather than
  accepting that the user can create, read, update, or delete any record."

Per the OWASP Input Validation Cheat Sheet, "Client-side vs Server-side
Validation" section (`documentation-based`):

> "Input validation must be implemented on the server-side before any data is
> processed by an application's functions, as any JavaScript-based input
> validation performed on the client-side can be circumvented by an attacker
> who disables JavaScript or uses a web proxy."

Read together, these are not two separate concerns: a price, discount,
quantity, or step-completion rule is both an access-control decision (is this
user/request allowed this outcome) and a validated-input decision (is this
value acceptable), and OWASP normatively requires server-side enforcement for
both. A client-side implementation of either is UX only and must not be
treated as the security or business-rule boundary.

## Rules that MUST have a server-side re-check

For any revenue-critical journey (checkout, payment submission, account
creation, login), treat each of the following as requiring an independent
server-side re-validation, regardless of what the client already checked or
displays:

- **Price** — the amount charged is computed or looked up server-side from
  the catalog/pricing source of record, never trusted from a client-supplied
  field.
- **Discount / coupon** — the coupon's validity, expiry, eligibility, and
  resulting discount amount are recomputed server-side, not accepted from a
  client-calculated total.
- **Quantity / inventory** — available stock and requested quantity are
  re-checked against server-side inventory state at submission time, not only
  validated in the client's cart UI.
- **Eligibility** — any rule gating who may purchase, enroll, or proceed
  (region, age, account tier, promo eligibility) is re-evaluated server-side
  from server-held identity/account data.
- **Authorization / entitlement** — the request is re-authorized against the
  authenticated user's actual entitlements and record ownership server-side,
  per the OWASP record-ownership principle above, not from a client-asserted
  role or ID.
- **Required-step completion** — a multi-step flow (e.g. address verification
  before payment, terms acceptance before account creation) is enforced by
  server-side state tracking which steps actually completed, not by the
  client simply choosing not to skip the UI screen.

## Reviewer evidence criteria

For each rule above that the client enforces (validates, computes, or gates
in its own code), check for a corresponding server-side re-check:

- Does the server recompute or re-look-up the value (price, discount,
  eligibility) from its own source of record, rather than accepting a
  client-supplied value for that field?
- If the server does accept a client-supplied value for such a field (e.g. an
  echoed price or discount code), does it independently validate that value
  against server-side state before acting on it, rather than trusting it
  as-is?
- For required-step flows, does the server track step completion in
  server-side state (a status flag, a state machine) and reject a request
  that skips ahead, rather than relying on the client only calling endpoints
  in the intended order?
- For authorization/entitlement, does the server check the authenticated
  identity's actual ownership/entitlement for the specific record acted on
  (not just that some valid session exists), consistent with the
  record-ownership requirement above?
- Is there any code path where a client-controlled field (hidden form field,
  request body value, mobile app local state) is used directly to determine
  a money-moving or eligibility outcome with no corresponding server lookup
  or validation?

A rule enforced only in client code, with no matching server-side re-check
reachable by a crafted request, is a client-trust bypass finding per the
skill's decision gates — regardless of whether the client-side check is well
implemented.

## RECOMMENDATION: treat client checks as UX, not defense

- Keep client-side checks — they still matter for responsiveness and honest
  users — but design and review them as UX affordances only, never as the
  place a business or security decision is actually made.
- Where a client displays a server-computed value (price, discount, remaining
  stock), prefer having the server return that value at submission time
  rather than trusting a value the client computed earlier in the session,
  since state can change between page load and submit.
- When a required-step flow exists, model it as an explicit server-side state
  machine (a status column or equivalent) rather than inferring completion
  from which endpoints were called, so a reordered or replayed client request
  cannot skip a step.

## Applicable versions

- This guidance is framework- and processor-agnostic: it applies to any
  client/server split (web, mobile, hybrid) and any payment processor,
  because the underlying principle (client code is attacker-controlled,
  server code is not) does not vary by SDK or API version.
- Whether a specific codebase actually re-validates a given rule server-side
  is an `inference` from reading that code; the existence of this guidance,
  or of comments claiming server-side validation, never proves a specific
  deployment enforces it correctly — confirm by tracing the server-side code
  path for each rule in scope.

## Sources

- [OWASP Top 10:2021 — A01 Broken Access Control](https://owasp.org/Top10/2021/A01_2021-Broken_Access_Control/index.html) — supports the trusted-server-side-code enforcement principle, deny-by-default posture, and record-ownership-level access control requirement.
- [OWASP Input Validation Cheat Sheet — Client-side vs Server-side Validation](https://cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet.html) — supports the requirement that input validation be implemented server-side because client-side JavaScript validation can be circumvented.

Last verified: 2026-07-16.
