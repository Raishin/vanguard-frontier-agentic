# W3C Manifest Installability Checklist

Use this reference when validating a `manifest.json` field-by-field against W3C installability criteria, or when diagnosing why an install prompt never appears.

## What people get wrong

The common bad assumption is:

> "My `manifest.json` validates against the schema, so my app is installable."

That is incomplete. Schema validity (correct JSON shape, valid enum values) and installability (the specific subset of fields and runtime conditions a browser actually requires before it will fire `beforeinstallprompt`) are different bars. A manifest can be perfectly schema-valid and still never trigger an install prompt, because installability additionally depends on:

1. **transport** — the manifest and its origin must be served over HTTPS,
2. **service-worker presence** — most implementing browsers additionally require an active, fetch-handling service worker before treating the app as installable, not the manifest alone,
3. **specific field values**, not just field presence — a `display` value of `browser` is schema-valid but disqualifies the install flow by design.

## Field-by-field checklist (W3C manifest spec)

Work through each field and record a pass/fail with evidence label (`spec-cited`, `live evidence`, `documentation-based`, or `inference`):

- **`name` / `short_name`** — at least one must be present and non-empty; `short_name` is used where display space is constrained (home-screen label). Missing both is an outright installability blocker.
- **`icons`** — at least one icon meeting the browser's minimum size requirement (commonly a 192×192 and/or 512×512 PNG/SVG/WebP entry is expected by major implementations, though the spec itself does not hardcode a single universal minimum — cite the specific browser's documented threshold, do not assume one number applies everywhere). Check `purpose` values (`any`, `maskable`, `monochrome`) are correctly set — a maskable-only icon set with no `any` fallback can render badly on platforms that do not support maskable icons.
- **`start_url`** — must resolve to a same-origin (or explicitly permitted scope-relative) URL. Verify it is not a URL that 404s or redirects to a different origin.
- **`scope`** — must contain `start_url`. If `scope` is narrower than the app's actual navigable routes, navigations outside `scope` will open in a regular browser tab/window even from an installed app, breaking the installed-app experience.
- **`display`** — must be one of `standalone`, `fullscreen`, or `minimal-ui` for the app to be eligible for the install-prompt flow at all. `display: browser` is spec-valid but is explicitly the "opt out of app-like display" value; if found, treat it as a deliberate choice to confirm with the user, not a bug to silently patch.
- **`theme_color` / `background_color`** — not installability-blocking per spec, but their absence produces a visibly broken splash-screen/status-bar experience during install and first launch; flag as a quality issue, not a hard blocker.
- **`id`** (where supported) — used to distinguish app identity across updates to `start_url`; note if absent on a manifest that has changed `start_url` historically, since that can cause duplicate installs.

## HTTPS is not optional

The W3C manifest spec and every major implementing browser refuse to treat an app as installable over plain HTTP (the common documented exception is `localhost` for local development). This is a transport-layer precondition, not a manifest field — check it first, separately from the field-by-field pass, because no amount of manifest correctness compensates for a missing HTTPS origin.

## Service worker as a co-requirement

A manifest satisfying every field above does not by itself make most browsers fire `beforeinstallprompt`. The commonly documented additional requirement is a registered service worker that reaches the `activate` lifecycle state and handles at least the `fetch` event for the page. Confirm activation state directly (see `references/live-verification-protocol.md`) rather than assuming registration code in the source implies a running, activated worker in the deployed environment.

## Common false negatives to rule out before blaming the manifest

- The manifest `<link rel="manifest">` tag is present in HTML but points to a 404 or a path blocked by CSP/robots rules.
- The manifest is served with an incorrect `Content-Type` that the browser refuses to parse as a manifest.
- A framework PWA plugin (see `references/offline-fallback-precache-pattern.md`) generated a manifest with `display: browser` as its own default, unrelated to any explicit app configuration — verify the plugin's current documented default via Context7 before concluding the app team made this choice deliberately.
