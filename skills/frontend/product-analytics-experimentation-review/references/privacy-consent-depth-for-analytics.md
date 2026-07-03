# Privacy & Consent Depth for Analytics

Use this reference when the review needs standard-specific depth beyond the general consent/PII posture covered in `consent-and-pii-in-events.md` — specifically IAB TCF v2.2 purpose-granular consent, Google Consent Mode v2's default-denied timing requirement, Global Privacy Control / Do-Not-Track honoring, cookie categorization/expiry, and analytics-endpoint data residency. This is static-review guidance only; there is no detection corpus behind it (this skill is not a security-category skill).

## What people get wrong

The naive story is:

> "We call `gtag('consent', 'update', ...)` when the CMP loads, so we're Consent Mode compliant."

Wrong. Consent Mode v2 distinguishes **defaults** from **updates**. The *default* consent state — set before the CMP has rendered or the user has answered anything — must itself be `denied` for the relevant signals, and it must be set **synchronously**, before any `gtag`/GTM tag fragment can fire. An `update` call arriving later is the correct mechanism for what happens *after* the user answers; it does nothing to undo tags that already fired under a missing or async-set default.

A second naive story:

> "We have a TCF consent string cookie (`euconsent-v2`), so every downstream vendor call is covered."

Wrong. TCF v2.2 consent is granular **per purpose ID**, not a single yes/no. A vendor may be consented for "measurement" (Purpose 1/analytics-adjacent purposes) but not for "personalized ads" (Purpose 4) or cross-context targeting. A tracking call that fires because *a* TC string cookie exists, without checking whether *that specific vendor and purpose combination* is granted, is not actually gated — it is gated by presence, not by scope.

## Officially grounded shape

**Google Consent Mode v2 (doc-based, via Context7 `/websites/developers_google_tag-platform_security_guides`):**

- The default consent call must set at minimum all four parameters — `ad_storage`, `analytics_storage`, `ad_user_data`, `ad_personalization` — and per the consent-debugging guide, **"Ensure that the default consent statuses are not set asynchronously"** and the default block "should be placed at the top of your page, before any tag fragments or other code that might use consent settings."
- The documented pattern is: `gtag('consent', 'default', {ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied', analytics_storage: 'denied'})` executed synchronously, before the gtag.js/GTM snippet loads — then `gtag('consent', 'update', {...})` later once the user answers the CMP.
- An optional `wait_for_update` parameter (e.g. `500` ms) exists specifically to give an asynchronous CMP banner time to call `update` before tags evaluate the default state — its presence in an implementation is a signal the team is aware of the timing requirement, not a substitute for a synchronous default block.
- Treat any implementation that sets defaults inside an async-loaded script, inside a CMP callback, or after the main gtag/GTM snippet as **non-compliant with the documented requirement**, regardless of whether an `update` call exists later.

**IAB TCF v2.2 (standard-based — no TCF library resolved in Context7 at time of review; treat as standard-based inference, not Context7-verified):**

- Consent and legitimate-interest signals are structured **per purpose ID and per vendor**, encoded into a single TC string. A compliant integration checks a specific `(vendorId, purposeId)` pair — commonly via a decoded consent object exposing something equivalent to `vendorConsents.has(purposeId)` — not merely "does a TC string exist."
- The TC string is the standard mechanism for **communicating** consent state to downstream vendors/partners. A server-side or third-party tracking call that omits the TC string (or an equivalent consent signal) gives the receiving party no way to honor purpose-specific restriction, even if the client-side call itself was gated correctly.
- Purpose-level gating and vendor-level gating are independent: a vendor consented for one purpose is not automatically consented for another. Do not treat "CMP shows a green banner" as equivalent to "this specific vendor/purpose pair is granted."

## Non-negotiable design rules

1. **Consent Mode defaults must be synchronous and denied-by-default**, set in the page `<head>` before the gtag.js/GTM snippet and before any `gtag('event', ...)` call — not inside a CMP callback, not behind a `DOMContentLoaded`/async script boundary.
2. **No tracking call — pixel, `gtag`, `sendBeacon`, `fetch`, image-tag — fires before a consent signal exists**, whether that signal is a Consent Mode default/update pair or a decoded TCF TC string. "We'll gate it with CSS/display:none" or "we'll filter server-side" does not prevent the HTTP request from having already left the client with its payload.
3. **PII must never ride in event properties**, regardless of consent state: raw email address, full legal name, unhashed user ID, phone number, precise address. Consent governs *whether tracking may happen*, not *what may be sent once it does* — a consented-but-PII-laden event is still a data-minimization violation.
4. **Every cookie set by analytics/marketing code must be categorized (essential / functional / analytics / marketing) and carry an explicit, bounded expiry.** An uncategorized cookie cannot be selectively cleared by a CMP, and an unbounded/session-spanning expiry on a cookie that should be short-lived is itself a flag.
5. **Global Privacy Control (`navigator.globalPrivacyControl === true`) and Do-Not-Track (`navigator.doNotTrack === "1"`) must be checked and honored as an opt-out signal** for non-essential tracking, alongside — not instead of — explicit CMP consent state. Treat code that reads a TC string or Consent Mode flag but never checks `navigator.globalPrivacyControl`/`doNotTrack` as an incomplete opt-out surface.
6. **Data residency of the analytics endpoint must be identified, not assumed.** Note whether the destination (vendor domain, region-specific ingestion endpoint, self-hosted collector) is documented against any residency commitment made to users (e.g. "EU data stays in the EU"); flag payloads sent to a default/global endpoint when a residency-scoped endpoint is available and expected.

## Minimal safe review flow

1. Find the Consent Mode default-consent call (if Google tooling is in use). Confirm it (a) sets all relevant parameters to `denied`, (b) executes synchronously in `<head>`, before the gtag.js/GTM snippet — not inside a CMP callback or async script.
2. Find every tracking call site (pixel `<img>`/`new Image()`, `gtag('event', ...)`, `sendBeacon`, `fetch` to an analytics/marketing endpoint). For each, confirm it is temporally *after* a consent signal is available — not merely after the page has "loaded."
3. If TCF is in use, confirm the guarding logic checks a specific vendor/purpose pair from the decoded TC string, not just TC-string presence.
4. For each event's payload, scan for PII fields regardless of consent state (see `consent-and-pii-in-events.md` rule 5 for the field list).
5. List every cookie set by the code under review; confirm each has a declared category and an explicit `Max-Age`/`Expires` — flag any cookie set without either.
6. Confirm the code checks `navigator.globalPrivacyControl` and/or `navigator.doNotTrack` and suppresses non-essential tracking when either signals opt-out, independent of CMP state.
7. Identify the destination host/endpoint of each analytics call and note whether it matches any stated data-residency commitment; flag mismatches as unverified, not assumed-fine.

## Concrete sinks to grep for

- **Unguarded pixel/image tracking**: `<img src="https://` or `new Image().src =` pointed at a tracking/analytics domain, with no preceding consent/TC-string/Consent-Mode check in the same code path. Risk: the HTTP GET — and any query-string PII on it — is unrecoverable once sent, unlike a suppressed JS event.
- **`gtag('event', ...)` before `gtag('consent', 'default', ...)`**: search for `gtag('event'` occurrences and confirm a `gtag('consent', 'default'` call precedes them in load order (ideally same `<head>` block, before the gtag.js `src` script tag). Risk: Consent Mode has no default state yet, so Google's tags apply undefined/permissive behavior.
- **PII literals in event payload construction**: `email`, `user.email`, `fullName`, `userId` (raw, unhashed) passed directly into an analytics `track()`/`gtag('event', ...)`/`sendBeacon()` call's properties object. Risk: PII leaves the client in plaintext regardless of consent state.
- **Cookie set without `Max-Age`/`Expires` and without a category comment/constant**: `document.cookie = "<name>=<value>"` with no `Max-Age=`/`Expires=` attribute, or a cookie name with no mapping in a consent-category config. Risk: cannot be selectively honored, cleared, or excluded by a CMP.

## Adversarial checklist

- Is the Consent Mode (or equivalent) default set synchronously in `<head>`, or does it live behind an async script/CMP callback where tags could fire first?
- Does the code check a *specific* vendor/purpose grant from the TC string, or only "a TC string cookie is present"?
- Does any tracking call — pixel, beacon, fetch — execute before any consent signal (default, TCF string, or otherwise) is available at all?
- Does any event payload carry a raw email, name, phone number, or unhashed user ID, independent of whether consent was granted?
- Does every cookie set by this code have both a declared category and an explicit expiry, or are any "just set and never revisited"?
- Does the code check `navigator.globalPrivacyControl` / `navigator.doNotTrack` and suppress non-essential tracking on either signal, or does it only look at CMP-recorded consent?
- Is the destination endpoint for this analytics call known and checked against any data-residency commitment, or is "it's probably fine" being assumed?

If any of these cannot be answered from the code itself, treat the privacy/consent posture as **unverified**, not compliant.

## When to push back

Push back if the user asks to:

- ship a Consent Mode default that is set asynchronously or after the gtag.js/GTM snippet, "since we'll call update quickly anyway,"
- gate a tracking call on "a TC string cookie exists" rather than the specific vendor/purpose grant it needs,
- add a raw email/name/unhashed user ID to an event's properties because "it's useful for support lookups,"
- set a new analytics/marketing cookie without an expiry or category because "we'll clean it up later,"
- skip checking Global Privacy Control/Do-Not-Track because "the CMP already covers consent,"
- send analytics payloads to a default global endpoint when a region-scoped endpoint exists and residency commitments apply.

Those are not shortcuts. They are unrecoverable data exposure and standards non-compliance shipped as an analytics change.
