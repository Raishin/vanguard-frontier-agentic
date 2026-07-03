# Consent and PII in Events

Use this reference when reviewing whether tracking calls are properly consent-gated and whether event payloads leak personally identifiable information (PII).

## What people get wrong

The naive story is:

> "We have a cookie consent banner, so our tracking is compliant."

Wrong. A consent banner existing somewhere on the page proves nothing about whether any *specific* tracking call actually respects the user's choice. Compliance is a property of the call site, not the page.

## Officially grounded shape

Per web.dev's Permissions API guidance (`permissions-best-practices`), browser permission state is queried per capability via `navigator.permissions.query({name: ...})`, returns a `state` of `granted` / `denied` / `prompt`, and — critically — **permission grants are scoped per-origin**: a grant on one origin does not transfer to a different origin or subdomain. This same non-transferability logic applies to consent state in analytics/marketing tooling: a consent decision recorded for one property/origin should not be silently assumed to apply to another origin or a cross-subdomain deployment without an explicit, documented consent-propagation mechanism (e.g. a shared consent cookie deliberately scoped for that purpose).

Google's Consent Mode (referenced by GA4 tooling) works by adjusting or blocking specific measurement signals based on granted/denied consent state, rather than by a single all-or-nothing toggle — treat "consent" as a set of distinct signals (e.g. analytics storage, ad storage, ad personalization) each with independent gating, not one binary flag, unless the reviewed implementation is verified via Context7/current docs to behave otherwise.

## Non-negotiable design rules

1. **Consent must be checked at the call site of the tracking function**, immediately before or as a guard condition on the SDK call itself — not only at page load in a separate initialization block that the actual event-firing code doesn't reference. If the tracking call does not read live consent state (or a consent-derived flag) at the point it fires, treat the call as unverified for compliance regardless of banner presence.
2. **Default state before consent is granted must be "no non-essential tracking"**, not "track now and revoke later" — later revocation does not undo data already collected and transmitted.
3. **Consent scope is per-origin/per-property by default.** Any claim that a consent decision on one domain covers a subdomain, related property, or downstream data-sharing partner must point to an explicit propagation mechanism (documented shared cookie, server-side consent state sync) — never assume it transfers implicitly.
4. **Every event-schema field must be classified**: essential/functional (may be exempt from consent gating under most frameworks), or analytics/marketing (must be consent-gated). Do not let a field's classification be assumed from its name alone — check what it actually contains.
5. **PII must never appear in event payloads in plaintext** — this includes free-text search/input fields, full email addresses, exact geolocation (lat/long or address-level), payment instrument details, and raw URLs containing query-string parameters that carry any of the above. Hash, truncate, bucket (e.g. city-level geo instead of lat/long), or drop these fields before the event is considered shippable.
6. **A schema field added for a new feature is not "safe by default."** Every new or changed field must be re-evaluated against rules 4 and 5, not grandfathered in because prior fields passed review.

## Minimal safe review flow

1. Enumerate every distinct tracking/event-firing call site touched by the change (not just the SDK initialization).
2. For each call site, trace backwards to the actual conditional (if any) gating that call on consent state — confirm it reads a live consent value, not a stale/default-true flag.
3. For each event's payload fields, classify each field as essential, analytics, or marketing, and flag any field whose content could plausibly contain PII regardless of its name.
4. For any field flagged as potential PII, confirm it is hashed, truncated, generalized (e.g. geo bucketed to region), or removed before the event ships — do not accept "we'll filter it downstream" as sufficient, since the payload has already left the client by then.
5. If the deployment spans multiple origins/subdomains, confirm the consent-propagation mechanism explicitly, rather than assuming a shared consent state.

## Adversarial checklist

Before approving an instrumentation change as consent/PII compliant, answer these:

- Does the exact line of code that fires this tracking call check a live consent value, or does it fire unconditionally and rely on something else (a tag-manager rule, a server-side filter) to suppress it later?
- What is the default consent state before the user makes any choice — is non-essential tracking off by default?
- If this property has multiple subdomains or related origins, is there an explicit, documented mechanism sharing consent state, or is that being assumed?
- For every new field in this event's payload, what raw value does it actually carry at runtime — has anyone printed a sample payload, or is the field's safety being assumed from its name?
- If a field carries free text (search box, comment, form input), is there any scrubbing before it reaches the event payload, or does it pass through verbatim?

If any of these cannot be answered, the compliance posture is unverified, not confirmed.

## When to push back

Push back if the user asks to:

- fire analytics/marketing events before consent is granted "just to see the data, we'll filter it later,"
- treat a consent banner's presence on the page as sufficient evidence that a specific new tracking call is compliant,
- add a free-text or geolocation field to an event schema without a scrubbing/generalization step,
- assume a consent decision made on the main domain automatically applies to a newly added subdomain or partner property.

Those are not shortcuts. They are compliance and privacy exposure shipped as a schema change.
