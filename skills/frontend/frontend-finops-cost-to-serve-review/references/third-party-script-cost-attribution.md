# Third-Party Script Cost Attribution

Use this reference when ranking third-party scripts, tags, or dependencies by their cost and performance contribution against their measured business value — not just their bundle size.

## What people get wrong

The common bad assumption is:

> "Third-party scripts are free — they're hosted by the vendor, not by us."

That is wrong in two separate ways. First, the vendor's hosting cost is irrelevant; what matters is the cost *the frontend surface incurs on behalf of loading and running it*: the CDN egress/request count if self-hosted or proxied, the CI build-minute cost if bundled at build time, and — the largest and most commonly ignored cost — the Core Web Vitals and conversion-rate impact of the script's execution, which translates directly into revenue at the margin. Second, "third party" bundles wildly different cost profiles under one label: a 2 KB feature flag SDK and a 400 KB chat-widget bundle with its own CDN calls are not comparable line items.

## Officially grounded shape (web.dev / Lighthouse performance budgets)

Per current web.dev guidance:

- Performance budgets can be defined per resource type (script, image, font, stylesheet, third-party, total) and per resource *count*, not just size — a `resourceCounts` budget on `third-party` requests is a first-class Lighthouse budget primitive (`budget.json`), independent from a size-based budget.
- Quantity-based metrics (page weight, request count, third-party request count) are explicitly recommended as an early-stage, easy-to-communicate proxy — useful for a cost/business conversation with non-engineering stakeholders even before deeper timing-based metrics are modeled.
- A commonly cited critical-path budget is under ~170 KB of compressed/minified critical-path resources for a fast experience on constrained devices/networks — useful as a reference point when arguing that a given script's weight materially eats the surface's total budget, not just "feels heavy."

## Non-negotiable design rules

### 1. Attribute cost per script, not per "third-party" bucket

For each script, separately account for:

- **Bundle weight** — transferred bytes (compressed), which drives CDN egress cost if proxied/self-hosted, or drives page-load time regardless of hosting.
- **Request count** — each script may fan out to its own CDN/API calls (analytics beacons, ad-tech auctions, chat-widget polling) that are invisible in a simple bundle-size measurement.
- **Execution cost** — main-thread blocking time and its measured effect on Core Web Vitals (particularly INP and LCP), which is a performance cost, not a cloud-billing cost, but converts to a revenue/conversion cost.
- **Build-time cost** — if the script or its wrapper is bundled/transpiled/type-checked in CI, attribute a share of CI build-minute spend to it.

### 2. Require a stated business justification before ranking a script as "remove"

A script's cost is only meaningful relative to its value. Do not rank a script for removal purely by weight or execution cost — pair every cost figure with the script's stated business function (conversion tracking, support chat, personalization, fraud detection, ad revenue) and, where available, a measured lift/attribution figure. A high-cost script tied to a verified revenue driver ranks differently than an equally expensive script with no attributed owner or unclear purpose.

### 3. Separate "safe to remove," "safe to defer/lazy-load," and "requires owner sign-off"

- **Safe to remove**: no attributed business owner, duplicate functionality with another already-loaded script, or confirmed dead/unused (verify via network trace or tag-manager audit, not assumption).
- **Safe to defer/lazy-load**: legitimate business function but not required for above-the-fold interaction (e.g., load after first user interaction or via `next/script`'s lazy-loading strategies where the framework supports it) — verify the framework's specific lazy-loading API via Context7/official docs before recommending a specific implementation, since loading-strategy APIs vary by framework and version.
- **Requires owner sign-off**: anything tied to security (bot mitigation, fraud detection), compliance (consent management, accessibility overlays with a legal mandate), or a verified revenue driver — never recommend removing or deferring these unilaterally on cost grounds alone; escalate to the named business/security owner.

### 4. Do not conflate script cost with hosting cost

A script's own CDN bill is the vendor's problem. The frontend surface's cost exposure is: its own egress if self-hosting/proxying the script, its own CI cost if bundling it, and its own performance-to-revenue cost from execution. Keep these separate in the report so a reader does not think "removing this script saves us their hosting bill."

## Minimal safe attribution flow

1. Enumerate all third-party scripts/tags currently loaded (network trace, tag manager audit, or `package.json`/bundle-analyzer output for bundled third-party code).
2. For each script, capture: transferred bytes, request count/fan-out, measured main-thread/CWV impact if available (lab data from Lighthouse/WebPageTest, or field data from CrUX/RUM), and CI build-time share if bundled.
3. Attribute a stated business function and, if available, a measured value signal (conversion lift, support-ticket deflection, ad revenue) — mark as `owner-confirmed` or `unattributed` if no owner responds.
4. Rank by cost-to-value ratio, bucketing into safe-to-remove, safe-to-defer, and requires-sign-off.
5. Present dollar cost (CDN egress + build-minute share, evidence-labeled) alongside the performance/CWV cost and the stated business value for each item.

## High-risk assumptions to kill

- "It's third-party, so it doesn't cost us anything."
- "Nobody's complained about this script, so it must be fine to keep loading it eagerly."
- "This script is small, so its execution cost doesn't matter."
- "We can just lazy-load everything without checking what breaks."
- "Unattributed scripts are automatically safe to remove" — unattributed does not mean unused; verify before removing.

## When to push back

Push back if the user asks to:

- remove a script tied to fraud detection, bot mitigation, consent management, or an accessibility legal mandate purely for cost savings, without named security/compliance owner sign-off,
- rank scripts by bundle size alone without checking request fan-out or CWV execution impact,
- treat "no one knows what this does" as justification for silent removal in a live production surface without a verified owner check or a monitored rollback path.

Those are cost-cutting shortcuts that trade an unmeasured business or security risk for a measured dollar saving — surface the trade-off explicitly instead of making the call silently.
